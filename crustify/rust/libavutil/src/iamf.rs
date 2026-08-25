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


/// Wraps: AVIAMFParamDefinitionType
///
/// Selects the concrete subblock type stored after an IAMF parameter
/// definition. The transparent integer representation preserves values from
/// newer libavutil versions without creating an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFParamDefinitionType(ffi::AVIAMFParamDefinitionType);

impl AVIAMFParamDefinitionType {
    /// Subblocks contain mix-gain parameters.
    pub const MIX_GAIN: Self =
        Self(ffi::AVIAMFParamDefinitionType_AV_IAMF_PARAMETER_DEFINITION_MIX_GAIN);
    /// Subblocks contain demixing information.
    pub const DEMIXING: Self =
        Self(ffi::AVIAMFParamDefinitionType_AV_IAMF_PARAMETER_DEFINITION_DEMIXING);
    /// Subblocks contain reconstruction-gain parameters.
    pub const RECON_GAIN: Self =
        Self(ffi::AVIAMFParamDefinitionType_AV_IAMF_PARAMETER_DEFINITION_RECON_GAIN);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFParamDefinitionType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFParamDefinitionType {
        self.0
    }
}

impl From<ffi::AVIAMFParamDefinitionType> for AVIAMFParamDefinitionType {
    fn from(raw: ffi::AVIAMFParamDefinitionType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFParamDefinitionType> for ffi::AVIAMFParamDefinitionType {
    fn from(value: AVIAMFParamDefinitionType) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVIAMFSubmixLayoutType
///
/// Describes whether an IAMF submix layout names a loudspeaker sound system
/// or binaural rendering. Unknown integer values remain representable for ABI
/// compatibility with newer linked libraries.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVIAMFSubmixLayoutType(ffi::AVIAMFSubmixLayoutType);

impl AVIAMFSubmixLayoutType {
    /// The layout follows an ITU-2051-3 loudspeaker sound system.
    pub const LOUDSPEAKERS: Self =
        Self(ffi::AVIAMFSubmixLayoutType_AV_IAMF_SUBMIX_LAYOUT_TYPE_LOUDSPEAKERS);
    /// The layout is binaural.
    pub const BINAURAL: Self =
        Self(ffi::AVIAMFSubmixLayoutType_AV_IAMF_SUBMIX_LAYOUT_TYPE_BINAURAL);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVIAMFSubmixLayoutType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVIAMFSubmixLayoutType {
        self.0
    }
}

impl From<ffi::AVIAMFSubmixLayoutType> for AVIAMFSubmixLayoutType {
    fn from(raw: ffi::AVIAMFSubmixLayoutType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVIAMFSubmixLayoutType> for ffi::AVIAMFSubmixLayoutType {
    fn from(value: AVIAMFSubmixLayoutType) -> Self {
        value.as_raw()
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

    #[test]
    fn parameter_definition_values_match_c_and_preserve_unknowns() {
        assert_eq!(AVIAMFParamDefinitionType::MIX_GAIN.as_raw(), 0);
        assert_eq!(AVIAMFParamDefinitionType::DEMIXING.as_raw(), 1);
        assert_eq!(AVIAMFParamDefinitionType::RECON_GAIN.as_raw(), 2);
        assert_eq!(AVIAMFParamDefinitionType::from_raw(99).as_raw(), 99);
    }

    #[test]
    fn submix_layout_values_match_c_and_preserve_unknowns() {
        assert_eq!(AVIAMFSubmixLayoutType::LOUDSPEAKERS.as_raw(), 2);
        assert_eq!(AVIAMFSubmixLayoutType::BINAURAL.as_raw(), 3);
        assert_eq!(AVIAMFSubmixLayoutType::from_raw(99).as_raw(), 99);
    }
}
