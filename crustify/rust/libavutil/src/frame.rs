//! Wrappers for libavutil frames.

use core::ptr::{NonNull, addr_of};

use ffibox::{CSlice, CSliceMut, define_ctype};

use crate::buffer::AVBufferReferenceRef;
use crate::dict::{AVDictionaryMut, AVDictionaryRef};
use crate::ffi;

/// Wraps: AVFrameSideDataType
///
/// A layout-compatible value for libavutil's frame side-data type. The
/// transparent integer representation preserves values introduced by newer
/// libavutil versions instead of turning an unfamiliar C value into an invalid
/// Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVFrameSideDataType(ffi::AVFrameSideDataType);

impl AVFrameSideDataType {
    pub const PANSCAN: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_PANSCAN);
    pub const A53_CC: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_A53_CC);
    pub const STEREO3D: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_STEREO3D);
    pub const MATRIXENCODING: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MATRIXENCODING);
    pub const DOWNMIX_INFO: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_INFO);
    pub const REPLAYGAIN: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_REPLAYGAIN);
    pub const DISPLAYMATRIX: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DISPLAYMATRIX);
    pub const AFD: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AFD);
    pub const MOTION_VECTORS: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MOTION_VECTORS);
    pub const SKIP_SAMPLES: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SKIP_SAMPLES);
    pub const AUDIO_SERVICE_TYPE: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AUDIO_SERVICE_TYPE);
    pub const MASTERING_DISPLAY_METADATA: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MASTERING_DISPLAY_METADATA);
    pub const GOP_TIMECODE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_GOP_TIMECODE);
    pub const SPHERICAL: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SPHERICAL);
    pub const CONTENT_LIGHT_LEVEL: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_CONTENT_LIGHT_LEVEL);
    pub const ICC_PROFILE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_ICC_PROFILE);
    pub const S12M_TIMECODE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_S12M_TIMECODE);
    pub const DYNAMIC_HDR_PLUS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_PLUS);
    pub const REGIONS_OF_INTEREST: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_REGIONS_OF_INTEREST);
    pub const VIDEO_ENC_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIDEO_ENC_PARAMS);
    pub const SEI_UNREGISTERED: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SEI_UNREGISTERED);
    pub const FILM_GRAIN_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_FILM_GRAIN_PARAMS);
    pub const DETECTION_BBOXES: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DETECTION_BBOXES);
    pub const DOVI_RPU_BUFFER: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOVI_RPU_BUFFER);
    pub const DOVI_METADATA: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOVI_METADATA);
    pub const DYNAMIC_HDR_VIVID: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_VIVID);
    pub const AMBIENT_VIEWING_ENVIRONMENT: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT);
    pub const VIDEO_HINT: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIDEO_HINT);
    pub const LCEVC: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_LCEVC);
    pub const VIEW_ID: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIEW_ID);
    pub const REFERENCE_DISPLAYS_3D: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_3D_REFERENCE_DISPLAYS);
    pub const EXIF: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_EXIF);
    pub const DYNAMIC_HDR_SMPTE_2094_APP5: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5);
    pub const IAMF_MIX_GAIN_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM);
    pub const IAMF_DEMIXING_INFO_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM);
    pub const IAMF_RECON_GAIN_INFO_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM);
    pub const RAW_COLOR_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_RAW_COLOR_PARAMS);
    pub const DOWNMIX_MATRIX: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_MATRIX);

    /// Returns the raw side-data type accepted by libavutil.
    #[inline]
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVFrameSideDataType {
        self.0
    }

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVFrameSideDataType) -> Self {
        Self(raw)
    }
}

impl From<ffi::AVFrameSideDataType> for AVFrameSideDataType {
    fn from(value: ffi::AVFrameSideDataType) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVFrameSideDataType> for ffi::AVFrameSideDataType {
    fn from(value: AVFrameSideDataType) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn side_data_types_are_layout_compatible_and_open() {
        let first = AVFrameSideDataType::from_raw(ffi::AVFrameSideDataType_AV_FRAME_DATA_PANSCAN);
        let last =
            AVFrameSideDataType::from_raw(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_MATRIX);

        assert_eq!(first, AVFrameSideDataType::PANSCAN);
        assert_eq!(last, AVFrameSideDataType::DOWNMIX_MATRIX);
        assert_eq!(first.as_raw(), 0);
        assert_eq!(last.as_raw(), 37);
        let future = ffi::AVFrameSideDataType::MAX;
        assert_eq!(AVFrameSideDataType::from_raw(future).as_raw(), future);
        assert_eq!(
            size_of::<AVFrameSideDataType>(),
            size_of::<ffi::AVFrameSideDataType>()
        );
        assert_eq!(
            align_of::<AVFrameSideDataType>(),
            align_of::<ffi::AVFrameSideDataType>()
        );
    }
}

define_ctype!(
    /// Wraps: AVFrameSideData
    ///
    /// A side-data header borrowed from an owning frame or side-data array.
    /// The header is not independently owned: its buffer reference and optional
    /// dictionary are released by the enclosing collection's lifecycle.
    AVFrameSideData,
    AVFrameSideDataRef,
    AVFrameSideDataMut,
    ffi::AVFrameSideData
);

impl<'a> AVFrameSideDataRef<'a> {
    /// Wraps: AVFrameSideData.type
    #[must_use]
    pub fn kind(&self) -> AVFrameSideDataType {
        // SAFETY: the integer-backed enum is copied through a raw projection;
        // AVFrameSideDataType preserves unknown ABI values.
        AVFrameSideDataType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Wraps: AVFrameSideData.buf
    ///
    /// Borrows the reference header that owns the backing byte allocation.
    #[must_use]
    pub fn buffer(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the field value is copied from the live side-data header. A
        // non-null reference header remains alive for this header's lifetime.
        let buffer = unsafe { addr_of!((*self.as_ptr()).buf).read() };
        // SAFETY: the side-data owner keeps the referenced header live for 'a.
        unsafe { AVBufferReferenceRef::from_ptr(buffer) }
    }

    /// Wraps: AVFrameSideData.size
    #[must_use]
    pub fn size(&self) -> usize {
        // SAFETY: the scalar is copied through a raw projection from the live
        // header and no reference to C storage is formed.
        unsafe { addr_of!((*self.as_ptr()).size).read() }
    }

    /// Wraps: AVFrameSideData.data
    ///
    /// Views the `size`-byte window kept alive by `buf`, without materializing
    /// a Rust slice over memory that C may mutate.
    #[must_use]
    pub fn data(&self) -> Option<CSlice<'a, u8>> {
        // SAFETY: both fields are copied from the live header. Its buffer owner
        // keeps the complete window alive for 'a.
        let (data, size) = unsafe {
            (
                addr_of!((*self.as_ptr()).data).read(),
                addr_of!((*self.as_ptr()).size).read(),
            )
        };
        NonNull::new(data).map(|data| {
            // SAFETY: side-data invariants establish `size` initialized bytes
            // at this (possibly interior) pointer, kept alive by `buf`.
            unsafe { CSlice::from_raw_parts(data, size) }
        })
    }

    /// Wraps: AVFrameSideData.metadata
    ///
    /// Borrows the optional dictionary owned by this side-data entry.
    #[must_use]
    pub fn metadata(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: the pointer value is copied from the live header; a non-null
        // dictionary remains owned and live for this header's lifetime.
        let metadata = unsafe { addr_of!((*self.as_ptr()).metadata).read() };
        // SAFETY: the entry's owner keeps a non-null dictionary live for 'a.
        unsafe { AVDictionaryRef::from_ptr(metadata) }
    }
}

impl AVFrameSideDataMut<'_> {
    /// Exclusively views the byte window when its backing buffer is writable.
    #[must_use]
    pub fn data_mut(&mut self) -> Option<CSliceMut<'_, u8>> {
        let header = self.as_mut_ptr();
        // SAFETY: the field is copied through the exclusive live header. A
        // null buffer cannot license mutable data access.
        let buffer = unsafe { addr_of!((*header).buf).read() };
        if buffer.is_null() {
            return None;
        }
        // SAFETY: `buffer` is owned by the live header. The C predicate retains
        // nothing and verifies the underlying reference is uniquely writable.
        if unsafe { ffi::av_buffer_is_writable(buffer) } == 0 {
            return None;
        }
        // SAFETY: the positive writability result licenses mutation of the
        // header's complete initialized byte window.
        let (data, size) = unsafe {
            (
                addr_of!((*header).data).read(),
                addr_of!((*header).size).read(),
            )
        };
        NonNull::new(data).map(|data| {
            // SAFETY: the buffer predicate established unique writable access,
            // and the view is bound to this exclusive handle reborrow.
            unsafe { CSliceMut::from_raw_parts(data, size) }
        })
    }

    /// Exclusively borrows the optional dictionary owned by this entry.
    #[must_use]
    pub fn metadata_mut(&mut self) -> Option<AVDictionaryMut<'_>> {
        // SAFETY: the pointer is copied through the exclusive header; the
        // side-data contract gives the entry unique ownership of the dictionary.
        let metadata = unsafe { addr_of!((*self.as_mut_ptr()).metadata).read() };
        // SAFETY: a non-null dictionary is live and exclusively reached through
        // this mutable side-data handle for the returned lifetime.
        unsafe { AVDictionaryMut::from_ptr(metadata) }
    }
}

#[cfg(test)]
mod side_data_type_tests {
    use core::mem::{align_of, size_of};
    use core::ptr::addr_of_mut;

    use ffibox::CBox;

    use super::*;
    use crate::buffer::AVBufferReference;

    #[test]
    fn side_data_borrows_owners_and_gates_mutable_bytes() {
        // SAFETY: the returned fully constructed buffer reference is adopted
        // immediately by its matching reference-header lifecycle.
        let backing = unsafe {
            CBox::<AVBufferReference>::from_raw(ffi::av_buffer_allocz(4))
                .expect("four-byte side-data buffer")
        };
        let data = backing.as_ref().data().unwrap().as_elem_ptr();
        let mut raw = ffi::AVFrameSideData {
            type_: ffi::AVFrameSideDataType_AV_FRAME_DATA_A53_CC,
            data,
            size: 4,
            metadata: core::ptr::null_mut(),
            buf: backing.as_ptr(),
        };

        // SAFETY: `raw` and `backing` remain live and this exclusive handle is
        // the only access path to the side-data header for the scope.
        let mut side_data = unsafe { AVFrameSideDataMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(side_data.as_ref().kind(), AVFrameSideDataType::A53_CC);
        assert_eq!(side_data.as_ref().size(), 4);
        assert_eq!(
            side_data.as_ref().buffer().unwrap().as_ptr(),
            backing.as_ptr().cast_const()
        );
        assert!(side_data.as_ref().metadata().is_none());
        assert_eq!(side_data.as_ref().data().unwrap().elems().sum::<u8>(), 0);

        assert!(side_data.data_mut().unwrap().copy_from_slice(&[1, 2, 3, 4]));
        assert_eq!(side_data.as_ref().data().unwrap().elem(2), Some(3));
        assert_eq!(
            size_of::<AVFrameSideData>(),
            size_of::<ffi::AVFrameSideData>()
        );
        assert_eq!(
            align_of::<AVFrameSideData>(),
            align_of::<ffi::AVFrameSideData>()
        );
    }
}

define_ctype!(
    /// Wraps: AVFrame
    ///
    /// ABI-compatible frame storage. Owned frames use `CBox<AVFrame>`;
    /// borrowed access goes through pointer-carrying handles and never forms a
    /// Rust reference to memory libavutil may mutate.
    AVFrame,
    AVFrameRef,
    AVFrameMut,
    ffi::AVFrame
);

// SAFETY: av_frame_free consumes one initialized, independently owned frame,
// releases all of its fields and header storage, and nulls the local slot.
unsafe impl ffibox::CDropped for AVFrame {
    unsafe fn c_drop(obj: NonNull<Self>) {
        let mut frame = obj.as_ptr().cast::<ffi::AVFrame>();
        // SAFETY: the trait contract transfers one live owner to its matching
        // public destructor; the local pointer slot is writable.
        unsafe { ffi::av_frame_free(&raw mut frame) }
    }
}

// SAFETY: av_frame_clone leaves its source live and returns either NULL or a
// new frame header whose referenced resources can be independently released.
unsafe impl ffibox::CCloned for AVFrame {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: the trait contract supplies a live initialized source.
        NonNull::new(unsafe { ffi::av_frame_clone(obj.as_ptr().cast()) }.cast())
    }
}

impl AVFrame {
    /// Allocates a frame initialized to libavutil's documented defaults.
    pub fn new() -> Option<ffibox::CBox<Self>> {
        // SAFETY: a non-null result is a fully initialized unique allocation
        // matched by AVFrame's destructor implementation.
        unsafe { ffibox::CBox::from_raw(ffi::av_frame_alloc()) }
    }
}

/// Identity-only address of a frame data plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVFramePlane<'a> {
    pointer: NonNull<u8>,
    _lifetime: core::marker::PhantomData<&'a ()>,
}

impl AVFramePlane<'_> {
    /// Returns the address. Dereferencing remains unsafe because the plane's
    /// extent depends on the media type, format, dimensions and signed stride.
    #[must_use]
    pub const fn as_non_null(self) -> NonNull<u8> {
        self.pointer
    }
}

/// Lifetime-bound identity for an application-managed frame cookie.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVFrameOpaque<'a> {
    pointer: NonNull<core::ffi::c_void>,
    _lifetime: core::marker::PhantomData<&'a ()>,
}

impl AVFrameOpaque<'_> {
    #[must_use]
    pub const fn as_non_null(self) -> NonNull<core::ffi::c_void> {
        self.pointer
    }
}

macro_rules! frame_scalar {
    ($(#[$meta:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the scalar is copied from a live frame through a raw
                // projection, without forming a reference to C storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
        impl AVFrameMut<'_> {
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle permits replacing this scalar.
                unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! frame_scalar_readonly {
    ($(#[$meta:meta])* $get:ident, $field:ident, $ty:ty) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the scalar is copied from a live frame through a raw
                // projection, without forming a reference to C storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
    };
}

macro_rules! frame_enum {
    ($(#[$meta:meta])* $get:ident, $set:ident, $field:ident, $ty:path) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the integer-backed ABI value is copied through a raw projection.
                <$ty>::from_raw(unsafe { addr_of!((*self.as_ptr()).$field).read() })
            }
        }
        impl AVFrameMut<'_> {
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle permits storing the ABI value.
                unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).$field).write(value.as_raw()) }
            }
        }
    };
}

impl<'a> AVFrameRef<'a> {
    /// Wraps: AVFrame.buf
    #[must_use]
    pub fn buffer(&self, index: usize) -> Option<AVBufferReferenceRef<'a>> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        let buffers = unsafe { addr_of!((*self.as_ptr()).buf).read() };
        // SAFETY: a non-null header is owned and kept live by the frame.
        unsafe { AVBufferReferenceRef::from_ptr(buffers[index]) }
    }

    /// Clones an owned reference header while leaving the frame slot valid.
    #[must_use]
    pub fn owned_buffer(
        &self,
        index: usize,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.buffer(index)?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrame.data
    ///
    /// Returns only plane identity because no format-independent byte extent exists.
    #[must_use]
    pub fn data_plane(&self, index: usize) -> Option<AVFramePlane<'a>> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        let data = unsafe { addr_of!((*self.as_ptr()).data).read() };
        NonNull::new(data[index]).map(|pointer| AVFramePlane {
            pointer,
            _lifetime: core::marker::PhantomData,
        })
    }

    /// Wraps: AVFrame.side_data
    #[must_use]
    pub fn side_data(&self, index: usize) -> Option<AVFrameSideDataRef<'a>> {
        let len = usize::try_from(self.nb_side_data()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: a valid frame owns a `len`-entry table; index is checked.
        let entry = unsafe {
            addr_of!((*self.as_ptr()).side_data)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: a non-null entry remains owned and live with the frame.
        unsafe { AVFrameSideDataRef::from_ptr(entry) }
    }

    /// Wraps: AVFrame.opaque
    #[must_use]
    pub fn opaque(&self) -> Option<AVFrameOpaque<'a>> {
        // SAFETY: only the identity is copied; libavutil never dereferences it.
        NonNull::new(unsafe { addr_of!((*self.as_ptr()).opaque).read() }).map(|pointer| {
            AVFrameOpaque {
                pointer,
                _lifetime: core::marker::PhantomData,
            }
        })
    }

    /// Wraps: AVFrame.metadata
    #[must_use]
    pub fn metadata(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).metadata).read() };
        // SAFETY: a non-null dictionary remains live for the frame borrow.
        unsafe { AVDictionaryRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.ch_layout
    #[must_use]
    pub fn channel_layout(&self) -> crate::channel_layout::AVChannelLayoutRef<'a> {
        // SAFETY: the initialized layout is embedded in the live frame.
        unsafe {
            crate::channel_layout::AVChannelLayoutRef::from_ptr(
                addr_of!((*self.as_ptr()).ch_layout).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.linesize
    #[must_use]
    pub fn line_size(&self, index: usize) -> Option<i32> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        Some(unsafe { addr_of!((*self.as_ptr()).linesize).read() }[index])
    }

    /// Wraps: AVFrame.private_ref
    ///
    /// Public callers may only test presence of this internal RefStruct reference.
    #[must_use]
    pub fn has_private_reference(&self) -> bool {
        // SAFETY: only the address is copied; it is neither exposed nor dereferenced.
        !unsafe { addr_of!((*self.as_ptr()).private_ref).read() }.is_null()
    }

    /// Wraps: AVFrame.opaque_ref
    #[must_use]
    pub fn opaque_reference(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).opaque_ref).read() };
        // SAFETY: a non-null header remains live for the frame borrow.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.hw_frames_ctx
    #[must_use]
    pub fn hardware_frames_context(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).hw_frames_ctx).read() };
        // SAFETY: a non-null header remains live for the frame borrow.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.extended_buf
    #[must_use]
    pub fn extended_buffer(&self, index: usize) -> Option<AVBufferReferenceRef<'a>> {
        let len = usize::try_from(self.nb_extended_buffers()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: a valid frame owns a `len`-entry table; index is checked.
        let pointer = unsafe {
            addr_of!((*self.as_ptr()).extended_buf)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: a non-null header remains live for the frame borrow.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Clones an owned extra reference header while leaving the frame valid.
    #[must_use]
    pub fn owned_extended_buffer(
        &self,
        index: usize,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.extended_buffer(index)?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrame.time_base
    #[must_use]
    pub fn time_base(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the initialized rational is embedded in the live frame.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).time_base).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.sample_aspect_ratio
    #[must_use]
    pub fn sample_aspect_ratio(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the initialized rational is embedded in the live frame.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).sample_aspect_ratio).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.extended_data
    #[must_use]
    pub fn extended_data_plane(&self, index: usize) -> Option<AVFramePlane<'a>> {
        let frame = self.as_ptr();
        // SAFETY: the table address is copied and the inline address projected
        // from the same live frame without forming references.
        let (table, inline) = unsafe {
            (
                addr_of!((*frame).extended_data).read(),
                addr_of!((*frame).data).cast::<*mut u8>(),
            )
        };
        let len = if table.cast_const() == inline {
            8
        } else {
            usize::try_from(self.channel_layout().nb_channels()).ok()?
        };
        if table.is_null() || index >= len {
            return None;
        }
        // SAFETY: a valid frame has a `len`-entry table; index is checked.
        NonNull::new(unsafe { table.add(index).read() }).map(|pointer| AVFramePlane {
            pointer,
            _lifetime: core::marker::PhantomData,
        })
    }
}

impl AVFrameMut<'_> {
    /// Replaces one owned buffer header and returns the prior owner.
    ///
    /// # Safety
    /// The resulting fixed table must remain contiguous, and every stored data
    /// plane must remain inside one of the frame's remaining buffer owners.
    pub unsafe fn replace_buffer(
        &mut self,
        index: usize,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Result<
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    > {
        if index >= 8 {
            return Err(replacement);
        }
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: index is checked; the caller guarantees the relational frame
        // invariants, and this handle exclusively owns the pointer slot.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).buf)
                .cast::<*mut ffi::AVBufferRef>()
                .add(index)
                .replace(replacement)
        };
        // SAFETY: a non-null previous slot was one owned reference header, now
        // removed from the frame and transferred to the returned owner.
        Ok(unsafe { ffibox::CBox::from_raw(previous) })
    }

    pub fn side_data_mut(&mut self, index: usize) -> Option<AVFrameSideDataMut<'_>> {
        let len = usize::try_from(self.as_ref().nb_side_data()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: the exclusive frame owns a `len`-entry table; index is checked.
        let entry = unsafe {
            addr_of!((*self.as_mut_ptr()).side_data)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: the mutable frame reborrow prevents competing access.
        unsafe { AVFrameSideDataMut::from_ptr(entry) }
    }

    pub fn metadata_mut(&mut self) -> Option<AVDictionaryMut<'_>> {
        // SAFETY: the pointer is copied through the exclusive frame handle.
        let pointer = unsafe { addr_of!((*self.as_mut_ptr()).metadata).read() };
        // SAFETY: the mutable frame reborrow prevents competing access.
        unsafe { AVDictionaryMut::from_ptr(pointer) }
    }

    /// Replaces the optional owned dictionary and returns the prior owner.
    pub fn replace_metadata(
        &mut self,
        replacement: Option<ffibox::CBox<crate::dict::AVDictionary>>,
    ) -> Option<ffibox::CBox<crate::dict::AVDictionary>> {
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot; replacing it
        // transfers the prior nullable dictionary out without freeing it.
        let previous =
            unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).metadata).replace(replacement) };
        // SAFETY: a non-null prior value was one frame-owned dictionary and is
        // no longer reachable from the frame.
        unsafe { ffibox::CBox::from_raw(previous) }
    }

    /// Replaces the optional reference-counted opaque owner.
    pub fn replace_opaque_reference(
        &mut self,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque_ref).replace(replacement)
        };
        // SAFETY: a non-null prior value was one frame-owned reference header.
        unsafe { ffibox::CBox::from_raw(previous) }
    }

    /// Replaces the optional hardware-frames-context reference owner.
    pub fn replace_hardware_frames_context(
        &mut self,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).hw_frames_ctx).replace(replacement)
        };
        // SAFETY: a non-null prior value was one frame-owned reference header.
        unsafe { ffibox::CBox::from_raw(previous) }
    }

    /// Replaces one extra owned buffer header and returns the prior owner.
    ///
    /// # Safety
    /// The replacement and all data-plane addresses must preserve the frame's
    /// buffer-containment invariants for the full frame lifetime.
    pub unsafe fn replace_extended_buffer(
        &mut self,
        index: usize,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Result<
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    > {
        let len = usize::try_from(self.as_ref().nb_extended_buffers()).unwrap_or(0);
        if index >= len {
            return Err(replacement);
        }
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: index is checked against the live table; the caller preserves
        // the relational invariants and this frame handle is exclusive.
        let previous = unsafe {
            core::ptr::addr_of!((*self.as_mut_ptr()).extended_buf)
                .read()
                .add(index)
                .replace(replacement)
        };
        // SAFETY: a non-null previous entry was owned by the frame and has now
        // been transferred out of its table slot.
        Ok(unsafe { ffibox::CBox::from_raw(previous) })
    }

    pub fn channel_layout_mut(&mut self) -> crate::channel_layout::AVChannelLayoutMut<'_> {
        // SAFETY: the embedded layout is live and exclusively reborrowed.
        unsafe {
            crate::channel_layout::AVChannelLayoutMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).ch_layout
            ))
            .expect("embedded field is non-null")
        }
    }

    pub fn set_line_size(&mut self, index: usize, value: i32) -> bool {
        if index >= 8 {
            return false;
        }
        // SAFETY: index is in bounds and the exclusive handle permits the write.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).linesize)
                .cast::<i32>()
                .add(index)
                .write(value)
        }
        true
    }

    pub fn clear_opaque(&mut self) {
        // SAFETY: the pointee is application-owned and the exclusive handle permits clearing it.
        unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque).write(core::ptr::null_mut()) }
    }

    /// Stores an application-managed cookie address.
    ///
    /// # Safety
    /// The pointee must outlive this frame and every clone retaining the address.
    pub unsafe fn set_opaque(&mut self, value: Option<NonNull<core::ffi::c_void>>) {
        // SAFETY: the caller provides the external lifetime; this handle is exclusive.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque)
                .write(value.map_or(core::ptr::null_mut(), NonNull::as_ptr))
        }
    }

    pub fn time_base_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the embedded rational is live and exclusively reborrowed.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).time_base
            ))
            .expect("embedded field is non-null")
        }
    }

    pub fn sample_aspect_ratio_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the embedded rational is live and exclusively reborrowed.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).sample_aspect_ratio
            ))
            .expect("embedded field is non-null")
        }
    }
}

frame_scalar!(/// Wraps: AVFrame.height
    height, set_height, height, i32);
frame_scalar!(/// Wraps: AVFrame.format
    format, set_format, format, i32);
frame_scalar!(/// Wraps: AVFrame.nb_samples
    sample_count, set_sample_count, nb_samples, i32);
frame_scalar!(/// Wraps: AVFrame.flags
    flags, set_flags, flags, i32);
frame_scalar!(/// Wraps: AVFrame.width
    width, set_width, width, i32);
frame_enum!(/// Wraps: AVFrame.pict_type
    picture_type, set_picture_type, pict_type, crate::avutil::AVPictureType);
frame_scalar!(/// Wraps: AVFrame.duration
    duration, set_duration, duration, i64);
frame_scalar_readonly!(/// Wraps: AVFrame.nb_extended_buf
    nb_extended_buffers, nb_extended_buf, i32);
frame_enum!(/// Wraps: AVFrame.alpha_mode
    alpha_mode, set_alpha_mode, alpha_mode, crate::pixfmt::AVAlphaMode);
frame_scalar!(/// Wraps: AVFrame.crop_right
    crop_right, set_crop_right, crop_right, usize);
frame_scalar!(/// Wraps: AVFrame.crop_left
    crop_left, set_crop_left, crop_left, usize);
frame_scalar!(/// Wraps: AVFrame.crop_bottom
    crop_bottom, set_crop_bottom, crop_bottom, usize);
frame_scalar!(/// Wraps: AVFrame.crop_top
    crop_top, set_crop_top, crop_top, usize);
frame_scalar!(/// Wraps: AVFrame.decode_error_flags
    decode_error_flags, set_decode_error_flags, decode_error_flags, i32);
frame_scalar!(/// Wraps: AVFrame.best_effort_timestamp
    best_effort_timestamp, set_best_effort_timestamp, best_effort_timestamp, i64);
frame_enum!(/// Wraps: AVFrame.chroma_location
    chroma_location, set_chroma_location, chroma_location, crate::pixfmt::AVChromaLocation);
frame_enum!(/// Wraps: AVFrame.colorspace
    color_space, set_color_space, colorspace, crate::pixfmt::AVColorSpace);
frame_enum!(/// Wraps: AVFrame.color_trc
    color_transfer, set_color_transfer, color_trc, crate::pixfmt::AVColorTransferCharacteristic);
frame_enum!(/// Wraps: AVFrame.color_primaries
    color_primaries, set_color_primaries, color_primaries, crate::pixfmt::AVColorPrimaries);
frame_enum!(/// Wraps: AVFrame.color_range
    color_range, set_color_range, color_range, crate::pixfmt::AVColorRange);
frame_scalar_readonly!(/// Wraps: AVFrame.nb_side_data
    nb_side_data, nb_side_data, i32);
frame_scalar!(/// Wraps: AVFrame.sample_rate
    sample_rate, set_sample_rate, sample_rate, i32);
frame_scalar!(/// Wraps: AVFrame.repeat_pict
    repeat_picture, set_repeat_picture, repeat_pict, i32);
frame_scalar!(/// Wraps: AVFrame.quality
    quality, set_quality, quality, i32);
frame_scalar!(/// Wraps: AVFrame.pkt_dts
    packet_dts, set_packet_dts, pkt_dts, i64);
frame_scalar!(/// Wraps: AVFrame.pts
    pts, set_pts, pts, i64);

#[cfg(test)]
mod frame_type_tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn owned_defaults_mutation_and_clone_use_typed_handles() {
        let mut frame = AVFrame::new().expect("frame allocation");
        assert_eq!(frame.as_ref().format(), -1);
        assert_eq!(frame.as_ref().time_base().den(), 1);
        assert!(frame.as_ref().data_plane(0).is_none());
        assert!(frame.as_ref().buffer(0).is_none());
        frame.as_mut().set_width(16);
        frame.as_mut().set_height(16);
        frame
            .as_mut()
            .set_format(crate::pixfmt::AVPixelFormat::RGBA.as_raw());
        // SAFETY: the frame is newly allocated and has valid video dimensions
        // and format; no buffers are already attached, and the owner adopts
        // every allocation installed by the successful call.
        let allocated = unsafe { ffi::av_frame_get_buffer(frame.as_mut().as_mut_ptr(), 32) };
        assert_eq!(allocated, 0);
        let opaque_owner = frame
            .as_ref()
            .owned_buffer(0)
            .expect("independent buffer reference");
        assert!(
            frame
                .as_mut()
                .replace_opaque_reference(Some(opaque_owner))
                .is_none()
        );
        assert!(frame.as_ref().opaque_reference().is_some());
        drop(frame.as_mut().replace_opaque_reference(None));
        frame.as_mut().set_pts(42);
        assert!(!frame.as_mut().set_line_size(8, 1));
        frame.as_mut().time_base_mut().set_num(1);
        frame.as_mut().time_base_mut().set_den(25);
        let cloned = frame.try_clone().expect("frame clone");
        assert_eq!(
            (cloned.as_ref().width(), cloned.as_ref().height()),
            (16, 16)
        );
        assert!(
            cloned
                .as_ref()
                .line_size(0)
                .is_some_and(|stride| stride > 0)
        );
        assert_eq!(
            (
                cloned.as_ref().time_base().num(),
                cloned.as_ref().time_base().den()
            ),
            (1, 25)
        );
    }

    #[test]
    fn layout_and_handles_match_ffi() {
        assert_eq!(size_of::<AVFrame>(), size_of::<ffi::AVFrame>());
        assert_eq!(align_of::<AVFrame>(), align_of::<ffi::AVFrame>());
        assert_eq!(
            size_of::<AVFrameRef<'_>>(),
            size_of::<*const ffi::AVFrame>()
        );
        assert_eq!(size_of::<AVFrameMut<'_>>(), size_of::<*mut ffi::AVFrame>());
    }
}
