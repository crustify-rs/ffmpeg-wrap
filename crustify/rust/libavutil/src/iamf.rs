//! Wrappers for `libavutil/iamf.c`.

use crate::ffi;

/// Wraps: AVIAMFAmbisonicsMode
///
/// ABI-compatible IAMF ambisonics mode. The transparent integer newtype also
/// preserves values introduced by newer libavutil versions.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFAmbisonicsMode(ffi::AVIAMFAmbisonicsMode);

impl AVIAMFAmbisonicsMode {
    /// The channel mapping is defined by the associated channel layout.
    pub const MONO: Self = Self(ffi::AVIAMFAmbisonicsMode_AV_IAMF_AMBISONICS_MODE_MONO);
    /// The layer supplies a projection demixing matrix.
    pub const PROJECTION: Self = Self(ffi::AVIAMFAmbisonicsMode_AV_IAMF_AMBISONICS_MODE_PROJECTION);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFAmbisonicsMode) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFAmbisonicsMode {
        self.0
    }
}

impl From<ffi::AVIAMFAmbisonicsMode> for AVIAMFAmbisonicsMode {
    fn from(raw: ffi::AVIAMFAmbisonicsMode) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFAmbisonicsMode> for ffi::AVIAMFAmbisonicsMode {
    fn from(mode: AVIAMFAmbisonicsMode) -> Self {
        mode.as_raw()
    }
}

/// Wraps: AVIAMFAnimationType
///
/// ABI-compatible IAMF parameter animation type. Unknown integer values remain
/// valid Rust values so values from a newer linked libavutil can round-trip.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFAnimationType(ffi::AVIAMFAnimationType);

impl AVIAMFAnimationType {
    /// Apply the starting value for the entire parameter subblock.
    pub const STEP: Self = Self(ffi::AVIAMFAnimationType_AV_IAMF_ANIMATION_TYPE_STEP);
    /// Interpolate linearly between the starting and ending values.
    pub const LINEAR: Self = Self(ffi::AVIAMFAnimationType_AV_IAMF_ANIMATION_TYPE_LINEAR);
    /// Interpolate along the specified quadratic Bezier curve.
    pub const BEZIER: Self = Self(ffi::AVIAMFAnimationType_AV_IAMF_ANIMATION_TYPE_BEZIER);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFAnimationType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFAnimationType {
        self.0
    }
}

impl From<ffi::AVIAMFAnimationType> for AVIAMFAnimationType {
    fn from(raw: ffi::AVIAMFAnimationType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFAnimationType> for ffi::AVIAMFAnimationType {
    fn from(animation: AVIAMFAnimationType) -> Self {
        animation.as_raw()
    }
}

/// Wraps: AVIAMFAudioElementType
///
/// ABI-compatible IAMF audio-element type. The integer newtype avoids creating
/// an invalid Rust enum discriminant when C supplies a future value.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFAudioElementType(ffi::AVIAMFAudioElementType);

impl AVIAMFAudioElementType {
    /// A scalable channel audio element.
    pub const CHANNEL: Self = Self(ffi::AVIAMFAudioElementType_AV_IAMF_AUDIO_ELEMENT_TYPE_CHANNEL);
    /// A scene-based audio element.
    pub const SCENE: Self = Self(ffi::AVIAMFAudioElementType_AV_IAMF_AUDIO_ELEMENT_TYPE_SCENE);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFAudioElementType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFAudioElementType {
        self.0
    }
}

impl From<ffi::AVIAMFAudioElementType> for AVIAMFAudioElementType {
    fn from(raw: ffi::AVIAMFAudioElementType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFAudioElementType> for ffi::AVIAMFAudioElementType {
    fn from(element_type: AVIAMFAudioElementType) -> Self {
        element_type.as_raw()
    }
}

/// Wraps: AVIAMFHeadphonesMode
///
/// ABI-compatible IAMF headphones rendering mode. Unknown values are retained
/// rather than interpreted as invalid Rust enum discriminants.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFHeadphonesMode(ffi::AVIAMFHeadphonesMode);

impl AVIAMFHeadphonesMode {
    /// Render the referenced audio element to stereo loudspeakers.
    pub const STEREO: Self = Self(ffi::AVIAMFHeadphonesMode_AV_IAMF_HEADPHONES_MODE_STEREO);
    /// Render the referenced audio element with a binaural renderer.
    pub const BINAURAL: Self = Self(ffi::AVIAMFHeadphonesMode_AV_IAMF_HEADPHONES_MODE_BINAURAL);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFHeadphonesMode) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFHeadphonesMode {
        self.0
    }
}

impl From<ffi::AVIAMFHeadphonesMode> for AVIAMFHeadphonesMode {
    fn from(raw: ffi::AVIAMFHeadphonesMode) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFHeadphonesMode> for ffi::AVIAMFHeadphonesMode {
    fn from(mode: AVIAMFHeadphonesMode) -> Self {
        mode.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn wrappers_match_their_c_enum_layouts() {
        assert_eq!(
            size_of::<AVIAMFAmbisonicsMode>(),
            size_of::<ffi::AVIAMFAmbisonicsMode>()
        );
        assert_eq!(
            align_of::<AVIAMFAmbisonicsMode>(),
            align_of::<ffi::AVIAMFAmbisonicsMode>()
        );
        assert_eq!(
            size_of::<AVIAMFAnimationType>(),
            size_of::<ffi::AVIAMFAnimationType>()
        );
        assert_eq!(
            size_of::<AVIAMFAudioElementType>(),
            size_of::<ffi::AVIAMFAudioElementType>()
        );
        assert_eq!(
            size_of::<AVIAMFHeadphonesMode>(),
            size_of::<ffi::AVIAMFHeadphonesMode>()
        );
    }

    #[test]
    fn named_values_match_the_c_enumerators() {
        assert_eq!(AVIAMFAmbisonicsMode::MONO.as_raw(), 0);
        assert_eq!(AVIAMFAmbisonicsMode::PROJECTION.as_raw(), 1);

        assert_eq!(AVIAMFAnimationType::STEP.as_raw(), 0);
        assert_eq!(AVIAMFAnimationType::LINEAR.as_raw(), 1);
        assert_eq!(AVIAMFAnimationType::BEZIER.as_raw(), 2);

        assert_eq!(AVIAMFAudioElementType::CHANNEL.as_raw(), 0);
        assert_eq!(AVIAMFAudioElementType::SCENE.as_raw(), 1);

        assert_eq!(AVIAMFHeadphonesMode::STEREO.as_raw(), 0);
        assert_eq!(AVIAMFHeadphonesMode::BINAURAL.as_raw(), 1);
    }

    #[test]
    fn unknown_values_round_trip() {
        let future = 99;

        assert_eq!(
            ffi::AVIAMFAmbisonicsMode::from(AVIAMFAmbisonicsMode::from(future)),
            future
        );
        assert_eq!(
            ffi::AVIAMFAnimationType::from(AVIAMFAnimationType::from(future)),
            future
        );
        assert_eq!(
            ffi::AVIAMFAudioElementType::from(AVIAMFAudioElementType::from(future)),
            future
        );
        assert_eq!(
            ffi::AVIAMFHeadphonesMode::from(AVIAMFHeadphonesMode::from(future)),
            future
        );
    }
}
