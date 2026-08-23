//! Core libavutil API types.

use crate::ffi;

/// Wraps: AVPictureType
///
/// ABI-compatible picture coding type. The integer newtype accepts unknown
/// values from C without constructing an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVPictureType(ffi::AVPictureType);

impl AVPictureType {
    /// Undefined picture type.
    pub const NONE: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_NONE);
    /// Intra-coded picture.
    pub const I: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_I);
    /// Predicted picture.
    pub const P: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_P);
    /// Bidirectionally predicted picture.
    pub const B: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_B);
    /// MPEG-4 S(GMC)-VOP picture.
    pub const S: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_S);
    /// Switching intra picture.
    pub const SI: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_SI);
    /// Switching predicted picture.
    pub const SP: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_SP);
    /// BI picture.
    pub const BI: Self = Self(ffi::AVPictureType_AV_PICTURE_TYPE_BI);

    /// Wraps a raw libavutil picture-type value, including unknown values.
    #[must_use]
    pub const fn from_raw(value: ffi::AVPictureType) -> Self {
        Self(value)
    }

    /// Returns the raw libavutil picture-type value.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVPictureType {
        self.0
    }
}

impl Default for AVPictureType {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<ffi::AVPictureType> for AVPictureType {
    fn from(value: ffi::AVPictureType) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVPictureType> for ffi::AVPictureType {
    fn from(value: AVPictureType) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVMediaType
///
/// ABI-compatible media type. The integer newtype accepts both the named C
/// constants and unknown values without constructing an invalid Rust enum
/// discriminant; this preserves `av_get_media_type_string`'s documented
/// unknown-value case.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVMediaType(ffi::AVMediaType);

impl AVMediaType {
    /// An unknown type, usually treated as [`DATA`](Self::DATA).
    pub const UNKNOWN: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_UNKNOWN);
    /// Video data.
    pub const VIDEO: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_VIDEO);
    /// Audio data.
    pub const AUDIO: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_AUDIO);
    /// Opaque, usually continuous data.
    pub const DATA: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_DATA);
    /// Subtitle data.
    pub const SUBTITLE: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_SUBTITLE);
    /// Opaque, usually sparse attachment data.
    pub const ATTACHMENT: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_ATTACHMENT);
    /// Number of ordinary media-type values; a C API sentinel, not media.
    pub const NB: Self = Self(ffi::AVMediaType_AVMEDIA_TYPE_NB);

    /// Wraps a raw libavutil media-type value, including unknown values.
    #[must_use]
    pub const fn from_raw(value: ffi::AVMediaType) -> Self {
        Self(value)
    }

    /// Returns the raw libavutil media-type value.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVMediaType {
        self.0
    }
}

impl Default for AVMediaType {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl From<ffi::AVMediaType> for AVMediaType {
    fn from(value: ffi::AVMediaType) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVMediaType> for ffi::AVMediaType {
    fn from(value: AVMediaType) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_the_c_enum() {
        assert_eq!(size_of::<AVPictureType>(), size_of::<ffi::AVPictureType>());
        assert_eq!(
            align_of::<AVPictureType>(),
            align_of::<ffi::AVPictureType>()
        );
    }

    #[test]
    fn named_values_match_the_bindings() {
        assert_eq!(AVPictureType::NONE.as_raw(), 0);
        assert_eq!(AVPictureType::I.as_raw(), 1);
        assert_eq!(AVPictureType::BI.as_raw(), 7);
    }

    #[test]
    fn unknown_values_round_trip() {
        let raw = 99;
        assert_eq!(AVPictureType::from_raw(raw).as_raw(), raw);
        assert_eq!(ffi::AVPictureType::from(AVPictureType::from(raw)), raw);
    }

    #[test]
    fn media_type_layout_matches_the_c_enum() {
        assert_eq!(size_of::<AVMediaType>(), size_of::<ffi::AVMediaType>());
        assert_eq!(align_of::<AVMediaType>(), align_of::<ffi::AVMediaType>());
    }

    #[test]
    fn media_type_named_values_match_the_bindings() {
        assert_eq!(AVMediaType::UNKNOWN.as_raw(), -1);
        assert_eq!(AVMediaType::VIDEO.as_raw(), 0);
        assert_eq!(AVMediaType::ATTACHMENT.as_raw(), 4);
        assert_eq!(AVMediaType::NB.as_raw(), 5);
    }

    #[test]
    fn media_type_unknown_values_round_trip() {
        let raw = 99;
        assert_eq!(AVMediaType::from_raw(raw).as_raw(), raw);
        assert_eq!(ffi::AVMediaType::from(AVMediaType::from(raw)), raw);
    }
}
