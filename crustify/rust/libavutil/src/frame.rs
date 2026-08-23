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
