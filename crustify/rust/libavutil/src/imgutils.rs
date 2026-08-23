//! Wrappers for libavutil image utilities.

use core::marker::PhantomData;

use crate::ffi;
use crate::pixfmt::{AVColorRange, AVPixelFormat};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageError {
    LengthOverflow,
    BufferTooSmall {
        required: usize,
    },
    /// A stride alignment was below 1.
    ///
    /// Every extent in this module is `FFALIGN(linesize, align)` summed over
    /// the planes, and `FFALIGN(x, a)` is `(x + a - 1) & ~(a - 1)`, which is
    /// `>= x` only for `a >= 1`. At `align == 0` the mask is zero, so the whole
    /// image reports an extent of zero bytes while C still copies or fills a
    /// full unaligned `linesize` per row; a negative alignment under-reports
    /// the same way. C never range-checks the parameter, so the wrappers do.
    NonPositiveAlignment,
    /// A plane height was below 1.
    ///
    /// `av_image_fill_plane_sizes` divides `SIZE_MAX` by the height to test
    /// each stride for overflow. `av_image_fill_arrays` reaches it only
    /// through `av_image_check_size`, which rejects a non-positive height
    /// first, but `av_image_fill_pointers` is a public entry point of its own
    /// and has no such guard.
    NonPositiveHeight,
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
///
/// `align` describes the packing of `destination`, not of `source`: C sizes
/// the destination with `av_image_get_buffer_size(.., align)` and refuses to
/// run when that exceeds `destination.len()`. A non-positive `align` defeats
/// exactly that guard — the computed size collapses to zero, the length check
/// passes for any destination, and C then writes one unaligned `linesize` per
/// row into it. Rejected here rather than at the C seam, where this campaign's
/// ASan build reports it as a heap-buffer-overflow write at `imgutils.c:529`.
pub fn av_image_copy_to_buffer(
    destination: &mut [u8],
    source: &ImagePlanes<'_>,
    align: i32,
) -> Result<usize, ImageError> {
    if align <= 0 {
        return Err(ImageError::NonPositiveAlignment);
    }
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
///
/// Derives the plane table for an image packed into `source`, after a
/// null-source preflight has proved the whole derived extent lies inside it.
///
/// A non-positive `align` is refused: it makes every aligned stride zero, so
/// the preflight reports an extent of zero bytes that any slice satisfies —
/// including an empty one — while the strides recorded in the result still
/// describe rows C will later read in full through [`av_image_copy_to_buffer`].
pub fn av_image_fill_arrays<'a>(
    source: &'a [u8],
    format: AVPixelFormat,
    width: i32,
    height: i32,
    align: i32,
) -> Result<ImagePlanes<'a>, ImageError> {
    if align <= 0 {
        return Err(ImageError::NonPositiveAlignment);
    }
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
///
/// Lays an image of the given geometry over `destination` and fills every
/// plane with the format's black, so the caller never handles the plane table
/// C requires.
///
/// A non-positive `align` is refused for the same reason as in
/// [`av_image_fill_arrays`], and here it is a write: `av_image_fill_color`
/// derives its per-row byte width from `av_image_get_linesize`, which ignores
/// `align` entirely, so a zero-byte extent still memsets a full row —
/// a heap-buffer-overflow write at `imgutils.c:569` under this campaign's ASan
/// build.
pub fn av_image_fill_black(
    destination: &mut [u8],
    format: AVPixelFormat,
    range: AVColorRange,
    width: i32,
    height: i32,
    align: i32,
) -> Result<(), ImageError> {
    if align <= 0 {
        return Err(ImageError::NonPositiveAlignment);
    }
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
///
/// The caller supplies the strides here instead of an alignment, so C's own
/// overflow checks cover them: a stride whose plane extent leaves the `int`
/// return range comes back as `EINVAL`, negative strides included.
///
/// What those checks do not cover is the height they divide by. Unlike
/// [`av_image_fill_arrays`], this entry point never reaches
/// `av_image_check_size`, so `height == 0` becomes `SIZE_MAX / height` in
/// `av_image_fill_plane_sizes` — reported by this campaign's UBSan build as
/// "division by zero" at `imgutils.c:122` — and a negative height can reach
/// the same division through `AV_CEIL_RSHIFT`, which rounds `-1 >> 1` to zero
/// for a subsampled plane. A positive height keeps every derived plane height
/// at 1 or more.
pub fn av_image_fill_pointers<'a>(
    buffer: &'a mut [u8],
    format: AVPixelFormat,
    height: i32,
    linesizes: [i32; 4],
) -> Result<ImagePointers<'a>, ImageError> {
    if height <= 0 {
        return Err(ImageError::NonPositiveHeight);
    }
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
///
/// Returns the byte extent a packed image of this geometry occupies, which is
/// what a caller sizes a buffer for. A non-positive `align` is refused rather
/// than answered: C would report zero for an image of any size, and a caller
/// who believes that answer allocates a buffer the very next call overruns.
pub fn av_image_get_buffer_size(
    format: AVPixelFormat,
    width: i32,
    height: i32,
    align: i32,
) -> Result<usize, ImageError> {
    if align <= 0 {
        return Err(ImageError::NonPositiveAlignment);
    }
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

    #[test]
    fn a_non_positive_alignment_is_refused_by_every_extent() {
        // `FFALIGN(x, 0)` masks with `~(0 - 1)`, i.e. with zero, so C reports a
        // four-byte image as needing no bytes at all. Each of these calls used
        // to succeed and then let C run past the slice it was handed: the fill
        // is an ASan heap-buffer-overflow write at `imgutils.c:569`, the copy
        // one at `imgutils.c:529`, and the fill-arrays case an over-read of the
        // empty source once the planes reach the copy.
        for align in [0, -1, i32::MIN] {
            assert_eq!(
                av_image_get_buffer_size(AVPixelFormat::GRAY8, 2, 2, align),
                Err(ImageError::NonPositiveAlignment)
            );
            assert_eq!(
                av_image_fill_arrays(&[0; 4], AVPixelFormat::GRAY8, 2, 2, align)
                    .err()
                    .expect("a non-positive alignment is refused"),
                ImageError::NonPositiveAlignment
            );
            assert_eq!(
                av_image_fill_black(
                    &mut [0xff; 4],
                    AVPixelFormat::GRAY8,
                    AVColorRange::JPEG,
                    2,
                    2,
                    align,
                ),
                Err(ImageError::NonPositiveAlignment)
            );

            let source = [0; 12];
            let planes = av_image_fill_arrays(&source, AVPixelFormat::RGB24, 2, 2, 1).unwrap();
            assert_eq!(
                av_image_copy_to_buffer(&mut [0; 12], &planes, align),
                Err(ImageError::NonPositiveAlignment)
            );
        }

        // The smallest accepted alignment is the one C's own macro is total
        // for, and it still produces the packed extent.
        assert_eq!(
            av_image_get_buffer_size(AVPixelFormat::GRAY8, 2, 2, 1),
            Ok(4)
        );
    }

    #[test]
    fn fill_pointers_refuses_the_height_c_divides_by() {
        // `av_image_fill_plane_sizes` tests each stride with `SIZE_MAX / height`
        // and `av_image_fill_pointers` reaches it with no size check in front,
        // so `height == 0` is a division by zero — reported by the UBSan build
        // as `imgutils.c:122:33: runtime error: division by zero`. A negative
        // height reaches the same division for a subsampled plane, because
        // `AV_CEIL_RSHIFT(-1, 1)` is zero.
        let mut buffer = [0; 64];
        for height in [0, -1, i32::MIN] {
            assert_eq!(
                av_image_fill_pointers(&mut buffer, AVPixelFormat::GRAY8, height, [4, 0, 0, 0])
                    .err(),
                Some(ImageError::NonPositiveHeight)
            );
            assert_eq!(
                av_image_fill_pointers(&mut buffer, AVPixelFormat::YUV420P, height, [4, 2, 2, 0])
                    .err(),
                Some(ImageError::NonPositiveHeight)
            );
        }

        // One row is accepted and reports the extent of exactly that row.
        let pointers =
            av_image_fill_pointers(&mut buffer, AVPixelFormat::GRAY8, 1, [4, 0, 0, 0]).unwrap();
        assert_eq!(pointers.required_bytes(), 4);
        assert_eq!(pointers.plane_count(), 1);

        // C's own stride checks still do their half of the work.
        assert!(matches!(
            av_image_fill_pointers(&mut buffer, AVPixelFormat::GRAY8, 1, [-4, 0, 0, 0]),
            Err(ImageError::Library(_))
        ));
    }

    #[test]
    fn a_buffer_shorter_than_the_derived_layout_is_refused() {
        let mut small = [0; 3];
        assert_eq!(
            av_image_fill_pointers(&mut small, AVPixelFormat::GRAY8, 1, [4, 0, 0, 0]).err(),
            Some(ImageError::BufferTooSmall { required: 4 })
        );
        assert_eq!(
            av_image_fill_black(
                &mut small,
                AVPixelFormat::GRAY8,
                AVColorRange::JPEG,
                2,
                2,
                1,
            ),
            Err(ImageError::BufferTooSmall { required: 4 })
        );
    }
}

use crate::mem::AvFree;
use core::mem::MaybeUninit;
use ffibox::CVec;

/// A single av_malloc-owned image allocation and the plane metadata that
/// points within it. Pixel bytes remain `MaybeUninit<u8>` because
/// `av_image_alloc` allocates storage but does not initialize ordinary pixels.
///
/// [`storage`](Self::storage) spans the sum of the plane sizes, which is what
/// `av_image_alloc` returns and where its last plane ends. C over-allocates
/// that by `align` bytes of slack, so the view is shorter than the allocation
/// and never longer: every plane offset it reports is addressable within it.
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
///
/// `align` must be a positive power of two. C aligns every line size with
/// `FFALIGN`, whose mask arithmetic assumes that, and seeds the allocation with
/// `align` itself; anything else yields line sizes and a total size that agree
/// with each other but describe no image — zero everywhere for `align == 0`,
/// and a `size_t` overflow rejected as an allocation failure for a negative
/// one. It is refused here with `AVERROR(EINVAL)` (-22) rather than passed on
/// as a size question.
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

    #[test]
    fn every_plane_offset_lies_inside_the_owned_storage() {
        // The claim `storage` rests on: C returns the summed plane sizes, so
        // each plane's first line is addressable through the returned view.
        let image = av_image_alloc(16, 16, AVPixelFormat::YUV420P, 32).unwrap();
        let length = image.storage().len();
        for plane in 0..3 {
            let offset = image.plane_offset(plane).expect("a planar YUV plane");
            let linesize = image.linesizes()[plane] as usize;
            assert!(offset + linesize <= length);
        }
        assert_eq!(image.plane_offset(3), None);
        assert_eq!(image.plane_offset(4), None);
    }

    #[test]
    fn a_non_power_of_two_alignment_is_refused() {
        for align in [0, -32, 3, 12] {
            assert_eq!(
                av_image_alloc(16, 16, AVPixelFormat::YUV420P, align).err(),
                Some(-22)
            );
        }
        assert!(av_image_alloc(16, 16, AVPixelFormat::YUV420P, 1).is_ok());
    }
}
