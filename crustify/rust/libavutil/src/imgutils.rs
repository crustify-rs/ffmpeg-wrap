//! Wrappers for libavutil image utilities.

use core::marker::PhantomData;

use crate::ffi;
use crate::pixfmt::{AVColorRange, AVPixelFormat};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageError {
    LengthOverflow,
    BufferTooSmall { required: usize },
    Library(i32),
}

fn size_result(status: i32) -> Result<usize, ImageError> {
    if status < 0 {
        Err(ImageError::Library(status))
    } else {
        Ok(status as usize)
    }
}

/// Plane pointers and strides borrowed from one contiguous image buffer.
pub struct ImagePlanes<'a> {
    data: [*mut u8; 4],
    linesizes: [i32; 4],
    format: AVPixelFormat,
    width: i32,
    height: i32,
    _buffer: PhantomData<&'a [u8]>,
}

impl ImagePlanes<'_> {
    #[must_use]
    pub fn linesizes(&self) -> [i32; 4] {
        self.linesizes
    }
    #[must_use]
    pub fn format(&self) -> AVPixelFormat {
        self.format
    }
    #[must_use]
    pub fn width(&self) -> i32 {
        self.width
    }
    #[must_use]
    pub fn height(&self) -> i32 {
        self.height
    }
}

/// Opaque result of computing plane pointers from caller-provided strides.
pub struct ImagePointers<'a> {
    data: [*mut u8; 4],
    linesizes: [i32; 4],
    required: usize,
    _buffer: PhantomData<&'a mut [u8]>,
}

impl ImagePointers<'_> {
    #[must_use]
    pub fn linesizes(&self) -> [i32; 4] {
        self.linesizes
    }
    #[must_use]
    pub fn required_bytes(&self) -> usize {
        self.required
    }
    #[must_use]
    pub fn plane_count(&self) -> usize {
        self.data.iter().filter(|p| !p.is_null()).count()
    }
}

/// Wraps: av_image_copy_to_buffer
pub fn av_image_copy_to_buffer(
    destination: &mut [u8],
    source: &ImagePlanes<'_>,
    align: i32,
) -> Result<usize, ImageError> {
    let dst_size = i32::try_from(destination.len()).map_err(|_| ImageError::LengthOverflow)?;
    // SAFETY: `source` was produced by `av_image_fill_arrays` only after its
    // required extent was checked against the borrowed buffer. The destination
    // has exactly `dst_size` writable bytes and neither borrow is retained.
    size_result(unsafe {
        ffi::av_image_copy_to_buffer(
            destination.as_mut_ptr(),
            dst_size,
            source.data.as_ptr().cast(),
            source.linesizes.as_ptr(),
            source.format.as_raw(),
            source.width,
            source.height,
            align,
        )
    })
}

/// Wraps: av_image_fill_arrays
pub fn av_image_fill_arrays<'a>(
    source: &'a [u8],
    format: AVPixelFormat,
    width: i32,
    height: i32,
    align: i32,
) -> Result<ImagePlanes<'a>, ImageError> {
    let mut data = [core::ptr::null_mut(); 4];
    let mut linesizes = [0; 4];
    // SAFETY: both output arrays have four writable elements. A null source
    // requests the extent without deriving pointers outside any allocation.
    let required = size_result(unsafe {
        ffi::av_image_fill_arrays(
            data.as_mut_ptr(),
            linesizes.as_mut_ptr(),
            core::ptr::null(),
            format.as_raw(),
            width,
            height,
            align,
        )
    })?;
    if required > source.len() {
        return Err(ImageError::BufferTooSmall { required });
    }
    // SAFETY: the preflight proved the entire derived layout lies within
    // `source`; output arrays remain writable and the bytes are not read.
    size_result(unsafe {
        ffi::av_image_fill_arrays(
            data.as_mut_ptr(),
            linesizes.as_mut_ptr(),
            source.as_ptr(),
            format.as_raw(),
            width,
            height,
            align,
        )
    })?;
    Ok(ImagePlanes {
        data,
        linesizes,
        format,
        width,
        height,
        _buffer: PhantomData,
    })
}

/// Wraps: av_image_fill_black
pub fn av_image_fill_black(
    destination: &mut [u8],
    format: AVPixelFormat,
    range: AVColorRange,
    width: i32,
    height: i32,
    align: i32,
) -> Result<(), ImageError> {
    let mut data = [core::ptr::null_mut(); 4];
    let mut linesizes = [0_i32; 4];
    // SAFETY: the output arrays have four slots; null performs a size preflight.
    let required = size_result(unsafe {
        ffi::av_image_fill_arrays(
            data.as_mut_ptr(),
            linesizes.as_mut_ptr(),
            core::ptr::null(),
            format.as_raw(),
            width,
            height,
            align,
        )
    })?;
    if required > destination.len() {
        return Err(ImageError::BufferTooSmall { required });
    }
    // SAFETY: preflight proved the derived layout fits; the mutable pointer
    // preserves the exclusive provenance later used by the black-fill call.
    size_result(unsafe {
        ffi::av_image_fill_arrays(
            data.as_mut_ptr(),
            linesizes.as_mut_ptr(),
            destination.as_mut_ptr(),
            format.as_raw(),
            width,
            height,
            align,
        )
    })?;
    let strides = linesizes.map(isize::try_from).map(|v| v.unwrap_or(0));
    // SAFETY: the preceding call proved the layout fits the exclusive buffer;
    // the same dimensions and format are used, so every write remains inside.
    let status = unsafe {
        ffi::av_image_fill_black(
            data.as_ptr(),
            strides.as_ptr(),
            format.as_raw(),
            range.as_raw(),
            width,
            height,
        )
    };
    if status < 0 {
        Err(ImageError::Library(status))
    } else {
        Ok(())
    }
}

/// Wraps: av_image_fill_pointers
pub fn av_image_fill_pointers<'a>(
    buffer: &'a mut [u8],
    format: AVPixelFormat,
    height: i32,
    linesizes: [i32; 4],
) -> Result<ImagePointers<'a>, ImageError> {
    let mut data = [core::ptr::null_mut(); 4];
    // SAFETY: null requests the extent without deriving out-of-allocation
    // pointers; the output table itself has four writable slots.
    let required = size_result(unsafe {
        ffi::av_image_fill_pointers(
            data.as_mut_ptr(),
            format.as_raw(),
            height,
            core::ptr::null_mut(),
            linesizes.as_ptr(),
        )
    })?;
    if required > buffer.len() {
        return Err(ImageError::BufferTooSmall { required });
    }
    // SAFETY: preflight proved every derived pointer stays within the buffer.
    size_result(unsafe {
        ffi::av_image_fill_pointers(
            data.as_mut_ptr(),
            format.as_raw(),
            height,
            buffer.as_mut_ptr(),
            linesizes.as_ptr(),
        )
    })?;
    Ok(ImagePointers {
        data,
        linesizes,
        required,
        _buffer: PhantomData,
    })
}

/// Wraps: av_image_get_buffer_size
pub fn av_image_get_buffer_size(
    format: AVPixelFormat,
    width: i32,
    height: i32,
    align: i32,
) -> Result<usize, ImageError> {
    // SAFETY: all inputs are plain values and C retains nothing.
    size_result(unsafe { ffi::av_image_get_buffer_size(format.as_raw(), width, height, align) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_and_copies_a_contiguous_rgb_image() {
        let size = av_image_get_buffer_size(AVPixelFormat::RGB24, 2, 2, 1).unwrap();
        assert_eq!(size, 12);
        let source = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let planes = av_image_fill_arrays(&source, AVPixelFormat::RGB24, 2, 2, 1).unwrap();
        let mut destination = [0; 12];
        assert_eq!(
            av_image_copy_to_buffer(&mut destination, &planes, 1),
            Ok(size)
        );
        assert_eq!(destination, source);
    }

    #[test]
    fn black_fill_is_bounded_by_the_slice() {
        let mut bytes = [0xff; 4];
        av_image_fill_black(
            &mut bytes,
            AVPixelFormat::GRAY8,
            AVColorRange::JPEG,
            2,
            2,
            1,
        )
        .unwrap();
        assert_eq!(bytes, [0; 4]);
        assert!(matches!(
            av_image_fill_arrays(&bytes[..3], AVPixelFormat::GRAY8, 2, 2, 1),
            Err(ImageError::BufferTooSmall { required: 4 })
        ));
    }
}

use crate::mem::AvFree;
use core::mem::MaybeUninit;
use ffibox::CVec;

/// A single av_malloc-owned image allocation and the plane metadata that
/// points within it. Pixel bytes remain `MaybeUninit<u8>` because
/// `av_image_alloc` allocates storage but does not initialize ordinary pixels.
pub struct AllocatedImage {
    storage: CVec<MaybeUninit<u8>, AvFree>,
    plane_offsets: [Option<usize>; 4],
    linesizes: [i32; 4],
}

impl AllocatedImage {
    #[must_use]
    pub fn storage(&self) -> &[MaybeUninit<u8>] {
        self.storage.as_slice()
    }

    #[must_use]
    pub fn storage_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.storage.as_mut_slice()
    }

    #[must_use]
    pub const fn plane_offset(&self, plane: usize) -> Option<usize> {
        if plane < 4 {
            self.plane_offsets[plane]
        } else {
            None
        }
    }

    #[must_use]
    pub const fn linesizes(&self) -> &[i32; 4] {
        &self.linesizes
    }
}

/// Wraps: av_image_alloc
pub fn av_image_alloc(
    width: i32,
    height: i32,
    format: AVPixelFormat,
    align: i32,
) -> Result<AllocatedImage, i32> {
    if align <= 0 || !(align as u32).is_power_of_two() {
        return Err(-22);
    }
    let mut pointers = [core::ptr::null_mut(); 4];
    let mut linesizes = [0_i32; 4];
    // SAFETY: both output arrays provide four writable elements as required by
    // the API. On success C transfers one allocation through `pointers[0]` and
    // makes the remaining non-null pointers interior to that same allocation.
    let status = unsafe {
        ffi::av_image_alloc(
            pointers.as_mut_ptr(),
            linesizes.as_mut_ptr(),
            width,
            height,
            format.as_raw(),
            align,
        )
    };
    if status < 0 {
        return Err(status);
    }
    let length = status as usize;
    let base = pointers[0];
    let mut offsets = [None; 4];
    for (slot, pointer) in offsets.iter_mut().zip(pointers) {
        if !pointer.is_null() {
            // SAFETY: the successful C contract makes every non-null plane
            // pointer interior to the one allocation beginning at `base`.
            let offset = unsafe { pointer.offset_from(base) };
            *slot = usize::try_from(offset).ok();
        }
    }
    // SAFETY: success transfers the non-null av_malloc allocation beginning at
    // `base`. Treating each byte as `MaybeUninit` accurately preserves that C
    // initialized only format-specific metadata/padding, not ordinary pixels.
    let storage = unsafe { CVec::<MaybeUninit<u8>, AvFree>::from_raw_parts(base.cast(), length) }
        .expect("successful av_image_alloc returned a null base pointer");
    Ok(AllocatedImage {
        storage,
        plane_offsets: offsets,
        linesizes,
    })
}

#[cfg(test)]
mod scheduled_alloc_tests {
    use super::*;

    #[test]
    fn allocated_image_owns_uninitialized_pixel_storage() {
        let image = av_image_alloc(16, 16, AVPixelFormat::YUV420P, 32).unwrap();
        assert!(!image.storage().is_empty());
        assert_eq!(image.plane_offset(0), Some(0));
        assert!(image.linesizes()[0] >= 16);
    }

    #[test]
    fn invalid_dimensions_return_the_c_error() {
        assert!(av_image_alloc(-1, 16, AVPixelFormat::YUV420P, 32).is_err());
    }
}
