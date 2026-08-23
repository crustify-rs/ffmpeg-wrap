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
}
