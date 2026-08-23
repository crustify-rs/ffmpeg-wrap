//! Wrappers for libavutil frames.

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
