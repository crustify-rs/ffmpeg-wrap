//! Wrappers for `libavutil/iamf.c`.

use core::ffi::c_void;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CBox, CDropped, CSlice, CSliceMut};

use crate::channel_layout::{AVChannelLayoutMut, AVChannelLayoutRef};
use crate::dict::{AVDictionary, AVDictionaryRef};
use crate::ffi;
use crate::log::AVClassRef;
use crate::rational::{AVRational, AVRationalMut, AVRationalRef};

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

ffibox::define_ctype!(
    /// Wraps: AVIAMFDemixingInfo
    ///
    /// Layout-compatible view of one demixing parameter subblock. Instances
    /// returned by libavutil are embedded in the allocation owned by their
    /// [`AVIAMFParamDefinition`], so this type has no independent destructor.
    AVIAMFDemixingInfo,
    AVIAMFDemixingInfoRef,
    AVIAMFDemixingInfoMut,
    ffi::AVIAMFDemixingInfo
);

ffibox::define_ctype!(
    /// Wraps: AVIAMFLayer
    ///
    /// Layout-compatible IAMF audio-element layer. A layer is owned by its
    /// parent audio element, which also releases the optional demixing matrix.
    /// Borrowed matrix views are therefore tied to the layer handle.
    AVIAMFLayer,
    AVIAMFLayerRef,
    AVIAMFLayerMut,
    ffi::AVIAMFLayer
);

ffibox::define_ctype!(
    /// Wraps: AVIAMFMixGain
    ///
    /// Layout-compatible view of one mix-gain parameter subblock. The value is
    /// embedded after its parent [`AVIAMFParamDefinition`] header and does not
    /// own storage independently of that allocation.
    AVIAMFMixGain,
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    ffi::AVIAMFMixGain
);

ffibox::define_ctype!(
    /// Wraps: AVIAMFParamDefinition
    ///
    /// Header for an IAMF parameter definition and its trailing subblock
    /// array. A pointer returned by libavutil can be owned as
    /// [`ffibox::CBox<AVIAMFParamDefinition>`]; dropping it releases the single
    /// allocation with [`ffi::av_free`].
    AVIAMFParamDefinition,
    AVIAMFParamDefinitionRef,
    AVIAMFParamDefinitionMut,
    ffi::AVIAMFParamDefinition
);

// SAFETY: a definition adopted into `CBox` must be the allocation base returned
// by `av_iamf_param_definition_alloc`. That routine obtains the header and its
// trailing subblocks in one av_malloc-family allocation, and none of the
// subblocks owns a separately disposed resource. `av_free` is its matching
// one-shot releaser.
unsafe impl CDropped for AVIAMFParamDefinition {
    unsafe fn c_drop(definition: NonNull<Self>) {
        // SAFETY: the trait contract transfers the uniquely owned allocation
        // base exactly once, and `av_free` accepts that allocation family.
        unsafe { ffi::av_free(definition.as_ptr().cast::<c_void>()) }
    }
}

macro_rules! class_field {
    ($(#[$meta:meta])* $shared:ident) => {
        impl<'a> $shared<'a> {
            $(#[$meta])*
            ///
            /// Libavutil-created values always contain their immutable static
            /// class. `None` also represents inert zero-initialized storage.
            #[must_use]
            pub fn av_class(&self) -> Option<AVClassRef<'a>> {
                // SAFETY: copying the const pointer through a raw projection
                // forms no reference. A non-null value is immutable class
                // metadata that remains live at least as long as this handle.
                let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
                // SAFETY: the class pointer is borrowed and immutable for the
                // returned handle's lifetime; null is represented as `None`.
                unsafe { AVClassRef::from_ptr(class.cast_mut()) }
            }
        }
    };
}

class_field!(
    /// Field: AVIAMFDemixingInfo.av_class
    AVIAMFDemixingInfoRef
);
class_field!(
    /// Field: AVIAMFLayer.av_class
    AVIAMFLayerRef
);
class_field!(
    /// Field: AVIAMFMixGain.av_class
    AVIAMFMixGainRef
);
class_field!(
    /// Field: AVIAMFParamDefinition.av_class
    AVIAMFParamDefinitionRef
);

macro_rules! scalar_field {
    ($(#[$meta:meta])* $shared:ident, $exclusive:ident, $field:ident, $setter:ident, $ty:ty) => {
        impl $shared<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> $ty {
                // SAFETY: the shared handle keeps a live initialized value;
                // raw projection copies one scalar and forms no reference to
                // C-visible storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl $exclusive<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](`", stringify!($shared), "::", stringify!($field), "`).")]
            pub fn $setter(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance to
                // the live field; raw projection writes only that scalar and
                // forms no reference to C-visible storage.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! readonly_scalar_field {
    ($(#[$meta:meta])* $shared:ident, $field:ident, $ty:ty) => {
        impl $shared<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> $ty {
                // SAFETY: the shared handle keeps a live initialized value;
                // raw projection copies one scalar and forms no reference.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
    };
}

macro_rules! enum_field {
    ($(#[$meta:meta])* $shared:ident, $exclusive:ident, $getter:ident, $setter:ident, $field:ident, $wrapper:ty) => {
        impl $shared<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $getter(&self) -> $wrapper {
                // SAFETY: bindgen represents the open C enum as an integer;
                // copying it through a raw projection forms no reference.
                <$wrapper>::from_raw(unsafe { addr_of!((*self.as_ptr()).$field).read() })
            }
        }

        impl $exclusive<'_> {
            #[doc = concat!("Sets [`", stringify!($getter), "`](`", stringify!($shared), "::", stringify!($getter), "`).")]
            pub fn $setter(&mut self, value: $wrapper) {
                // SAFETY: the exclusive handle permits replacing this integer
                // field, and the wrapper preserves every possible C value.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value.as_raw()) }
            }
        }
    };
}

macro_rules! rational_field {
    ($(#[$meta:meta])* $shared:ident, $exclusive:ident, $field:ident, $field_mut:ident) => {
        impl<'a> $shared<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> AVRationalRef<'a> {
                // SAFETY: the projected by-value rational is initialized and
                // remains live with its enclosing handle for `'a`. The helper
                // forms only another pointer-carrying handle.
                unsafe {
                    AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).$field).cast_mut())
                        .expect("an embedded field address is non-null")
                }
            }
        }

        impl $exclusive<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](`", stringify!($shared), "::", stringify!($field), "`).")]
            #[must_use]
            pub fn $field_mut(&mut self) -> AVRationalMut<'_> {
                // SAFETY: raw projection locates the initialized embedded
                // rational, and the mutable result is bound to this exclusive
                // reborrow of its enclosing handle.
                unsafe {
                    AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$field))
                        .expect("an embedded field address is non-null")
                }
            }
        }
    };
}

scalar_field!(
    /// Field: AVIAMFDemixingInfo.subblock_duration
    AVIAMFDemixingInfoRef,
    AVIAMFDemixingInfoMut,
    subblock_duration,
    set_subblock_duration,
    u32
);
scalar_field!(
    /// Field: AVIAMFDemixingInfo.dmixp_mode
    AVIAMFDemixingInfoRef,
    AVIAMFDemixingInfoMut,
    dmixp_mode,
    set_dmixp_mode,
    u32
);

scalar_field!(
    /// Field: AVIAMFLayer.flags
    AVIAMFLayerRef,
    AVIAMFLayerMut,
    flags,
    set_flags,
    u32
);

impl<'a> AVIAMFLayerRef<'a> {
    /// Field: AVIAMFLayer.ch_layout
    #[must_use]
    pub fn ch_layout(&self) -> AVChannelLayoutRef<'a> {
        // SAFETY: the by-value channel layout is live for `'a` with the layer.
        // A libavutil-created layer contains a valid initialized layout, while
        // the wrapper's zeroed value is the valid UNSPEC layout.
        unsafe {
            AVChannelLayoutRef::from_ptr(addr_of!((*self.as_ptr()).ch_layout).cast_mut())
                .expect("an embedded field address is non-null")
        }
    }
}

impl AVIAMFLayerMut<'_> {
    /// Exclusively borrows [`ch_layout`](AVIAMFLayerRef::ch_layout).
    #[must_use]
    pub fn ch_layout_mut(&mut self) -> AVChannelLayoutMut<'_> {
        // SAFETY: the exclusive layer handle provides the only access to this
        // initialized embedded layout for the duration of the returned view.
        unsafe {
            AVChannelLayoutMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).ch_layout))
                .expect("an embedded field address is non-null")
        }
    }
}

readonly_scalar_field!(
    /// Field: AVIAMFLayer.nb_demixing_matrix
    AVIAMFLayerRef,
    nb_demixing_matrix,
    u32
);

impl<'a> AVIAMFLayerRef<'a> {
    /// Field: AVIAMFLayer.demixing_matrix
    ///
    /// Borrows the optional owned matrix using `nb_demixing_matrix` as its
    /// element count. Wrapped rational elements are exposed as handles rather
    /// than as a Rust slice over storage C retains.
    #[must_use]
    pub fn demixing_matrix(&self) -> Option<CSlice<'a, AVRational>> {
        let len = usize::try_from(self.nb_demixing_matrix()).ok()?;
        // SAFETY: only the pointer value is copied through the raw projection.
        let pointer = unsafe { addr_of!((*self.as_ptr()).demixing_matrix).read() };
        NonNull::new(pointer.cast::<AVRational>()).map(|pointer| {
            // SAFETY: a valid layer owns `len` contiguous initialized rational
            // values at a non-null pointer. Its shared handle keeps the parent
            // and therefore this allocation live and unmodified for `'a`.
            unsafe { CSlice::from_raw_parts(pointer, len) }
        })
    }
}

impl AVIAMFLayerMut<'_> {
    /// Exclusively borrows [`demixing_matrix`](AVIAMFLayerRef::demixing_matrix).
    #[must_use]
    pub fn demixing_matrix_mut(&mut self) -> Option<CSliceMut<'_, AVRational>> {
        let len = usize::try_from(self.as_ref().nb_demixing_matrix()).ok()?;
        // SAFETY: only the pointer value is copied from the live exclusive
        // layer through a raw projection.
        let pointer = unsafe { addr_of!((*self.as_mut_ptr()).demixing_matrix).read() };
        NonNull::new(pointer.cast::<AVRational>()).map(|pointer| {
            // SAFETY: a valid layer owns `len` contiguous initialized values,
            // and the mutable layer reborrow makes this the exclusive access
            // path to them for the returned view's lifetime.
            unsafe { CSliceMut::from_raw_parts(pointer, len) }
        })
    }
}

enum_field!(
    /// Field: AVIAMFLayer.ambisonics_mode
    AVIAMFLayerRef,
    AVIAMFLayerMut,
    ambisonics_mode,
    set_ambisonics_mode,
    ambisonics_mode,
    AVIAMFAmbisonicsMode
);
rational_field!(
    /// Field: AVIAMFLayer.output_gain
    AVIAMFLayerRef,
    AVIAMFLayerMut,
    output_gain,
    output_gain_mut
);
scalar_field!(
    /// Field: AVIAMFLayer.output_gain_flags
    AVIAMFLayerRef,
    AVIAMFLayerMut,
    output_gain_flags,
    set_output_gain_flags,
    u32
);

scalar_field!(
    /// Field: AVIAMFMixGain.subblock_duration
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    subblock_duration,
    set_subblock_duration,
    u32
);
rational_field!(
    /// Field: AVIAMFMixGain.control_point_relative_time
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    control_point_relative_time,
    control_point_relative_time_mut
);
rational_field!(
    /// Field: AVIAMFMixGain.control_point_value
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    control_point_value,
    control_point_value_mut
);
rational_field!(
    /// Field: AVIAMFMixGain.end_point_value
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    end_point_value,
    end_point_value_mut
);
rational_field!(
    /// Field: AVIAMFMixGain.start_point_value
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    start_point_value,
    start_point_value_mut
);
enum_field!(
    /// Field: AVIAMFMixGain.animation_type
    AVIAMFMixGainRef,
    AVIAMFMixGainMut,
    animation_type,
    set_animation_type,
    animation_type,
    AVIAMFAnimationType
);

impl AVIAMFParamDefinitionRef<'_> {
    /// Field: AVIAMFParamDefinition.type
    #[must_use]
    pub fn parameter_type(&self) -> AVIAMFParamDefinitionType {
        // SAFETY: bindgen uses an integer for the open C enum. Copying it
        // through a raw projection forms no reference to C-owned storage.
        AVIAMFParamDefinitionType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }
}

scalar_field!(
    /// Field: AVIAMFParamDefinition.duration
    AVIAMFParamDefinitionRef,
    AVIAMFParamDefinitionMut,
    duration,
    set_duration,
    u32
);
scalar_field!(
    /// Field: AVIAMFParamDefinition.constant_subblock_duration
    AVIAMFParamDefinitionRef,
    AVIAMFParamDefinitionMut,
    constant_subblock_duration,
    set_constant_subblock_duration,
    u32
);
scalar_field!(
    /// Field: AVIAMFParamDefinition.parameter_rate
    AVIAMFParamDefinitionRef,
    AVIAMFParamDefinitionMut,
    parameter_rate,
    set_parameter_rate,
    u32
);
scalar_field!(
    /// Field: AVIAMFParamDefinition.parameter_id
    AVIAMFParamDefinitionRef,
    AVIAMFParamDefinitionMut,
    parameter_id,
    set_parameter_id,
    u32
);
readonly_scalar_field!(
    /// Field: AVIAMFParamDefinition.nb_subblocks
    AVIAMFParamDefinitionRef,
    nb_subblocks,
    u32
);
readonly_scalar_field!(
    /// Field: AVIAMFParamDefinition.subblock_size
    AVIAMFParamDefinitionRef,
    subblock_size,
    usize
);
readonly_scalar_field!(
    /// Field: AVIAMFParamDefinition.subblocks_offset
    AVIAMFParamDefinitionRef,
    subblocks_offset,
    usize
);

ffibox::define_ctype!(
    /// Wraps: AVIAMFReconGain
    ///
    /// Borrowed view of reconstruction-gain subblock storage embedded in an
    /// [`AVIAMFParamDefinition`](ffi::AVIAMFParamDefinition) allocation.
    /// Libavutil initializes every such subblock with a non-null pointer to
    /// immutable static `AVClass` metadata. Its duration starts at zero and
    /// must be configured before the subblock is consumed. The subblock has no
    /// independent allocation or teardown operation. Foreign instances must
    /// keep equivalent immutable class metadata live with the subblock.
    AVIAMFReconGain,
    AVIAMFReconGainRef,
    AVIAMFReconGainMut,
    ffi::AVIAMFReconGain
);

impl<'a> AVIAMFReconGainRef<'a> {
    /// Field: AVIAMFReconGain.av_class
    ///
    /// Returns the immutable process-lifetime option metadata for this
    /// reconstruction-gain subblock.
    #[must_use]
    pub fn av_class(&self) -> crate::log::AVClassRef<'a> {
        // SAFETY: the type invariant establishes that this field is a non-null
        // pointer to immutable class metadata live for at least `'a`.
        let class = unsafe { core::ptr::addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: that same invariant establishes initialization, immutability,
        // non-nullness, and the returned handle's lifetime. Libavutil's own
        // instances point at the translation unit's static `recon_gain_class`.
        unsafe { crate::log::AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFReconGain has a non-null AVClass")
    }

    /// Field: AVIAMFReconGain.recon_gain
    ///
    /// Copies the six layers of twelve channel gains. Entries belonging to a
    /// layer without reconstruction gain have unspecified semantics, but their
    /// zero-initialized bytes remain safe to copy.
    #[must_use]
    pub fn recon_gain(&self) -> [[u8; 12]; 6] {
        // SAFETY: libavutil zero-initializes the complete subblock allocation;
        // raw-place projection copies the fixed array without forming a
        // reference to C-visible storage.
        unsafe { core::ptr::addr_of!((*self.as_ptr()).recon_gain).read() }
    }

    /// Field: AVIAMFReconGain.subblock_duration
    ///
    /// Returns the duration in units of the parent definition's parameter
    /// rate. A freshly allocated subblock is zero until its required duration
    /// is configured.
    #[must_use]
    pub fn subblock_duration(&self) -> Option<core::num::NonZeroU32> {
        // SAFETY: the scalar is copied through a raw projection from the live
        // subblock. Zero is retained as the not-yet-configured state.
        let duration = unsafe { core::ptr::addr_of!((*self.as_ptr()).subblock_duration).read() };
        core::num::NonZeroU32::new(duration)
    }
}

impl AVIAMFReconGainMut<'_> {
    /// Replaces all reconstruction-gain entries.
    pub fn set_recon_gain(&mut self, gain: [[u8; 12]; 6]) {
        // SAFETY: the exclusive handle permits replacing this initialized
        // fixed array; raw-place projection forms no reference to C storage.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).recon_gain).write(gain);
        }
    }

    /// Sets the duration while preserving its documented nonzero invariant.
    pub fn set_subblock_duration(&mut self, duration: core::num::NonZeroU32) {
        // SAFETY: the exclusive handle permits writing this scalar and the
        // argument's type preserves the subblock invariant.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).subblock_duration).write(duration.get());
        }
    }
}

ffibox::define_ctype!(
    /// Wraps: AVIAMFSubmixLayout
    ///
    /// Borrowed view of a submix layout owned by its enclosing
    /// `AVIAMFMixPresentation`. Libavutil allocates and zero-initializes the
    /// object, installs immutable static `AVClass` metadata, initializes the
    /// embedded channel layout, and later disposes it with its parent. Foreign
    /// instances must keep equivalent immutable class metadata live with the
    /// layout. This type deliberately has no independent owning handle.
    AVIAMFSubmixLayout,
    AVIAMFSubmixLayoutRef,
    AVIAMFSubmixLayoutMut,
    ffi::AVIAMFSubmixLayout
);

impl<'a> AVIAMFSubmixLayoutRef<'a> {
    /// Field: AVIAMFSubmixLayout.av_class
    ///
    /// Returns the immutable process-lifetime option metadata for this layout.
    #[must_use]
    pub fn av_class(&self) -> crate::log::AVClassRef<'a> {
        // SAFETY: the type invariant establishes that this field is a non-null
        // pointer to immutable class metadata live for at least `'a`.
        let class = unsafe { core::ptr::addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: that same invariant establishes initialization, immutability,
        // non-nullness, and the returned handle's lifetime. Libavutil's own
        // instances point at the translation unit's static `layout_class`.
        unsafe { crate::log::AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFSubmixLayout has a non-null AVClass")
    }

    /// Field: AVIAMFSubmixLayout.layout_type
    #[must_use]
    pub fn layout_type(&self) -> AVIAMFSubmixLayoutType {
        // SAFETY: raw-place projection copies the integer-backed enum from the
        // live layout without forming a reference to C storage.
        AVIAMFSubmixLayoutType::from_raw(unsafe {
            core::ptr::addr_of!((*self.as_ptr()).layout_type).read()
        })
    }

    /// Field: AVIAMFSubmixLayout.sound_system
    ///
    /// Borrows the initialized channel layout embedded in this submix layout.
    #[must_use]
    pub fn sound_system(&self) -> crate::channel_layout::AVChannelLayoutRef<'a> {
        // SAFETY: libavutil initializes the by-value channel layout and it
        // remains live with the enclosing layout for `'a`.
        unsafe {
            crate::channel_layout::AVChannelLayoutRef::from_ptr(
                core::ptr::addr_of!((*self.as_ptr()).sound_system).cast_mut(),
            )
        }
        .expect("an embedded field address is non-null")
    }
}

impl AVIAMFSubmixLayoutMut<'_> {
    /// Sets whether this is a loudspeaker or binaural layout.
    pub fn set_layout_type(&mut self, layout_type: AVIAMFSubmixLayoutType) {
        // SAFETY: the exclusive handle permits replacing this integer-backed
        // enum and raw-place projection forms no reference to C storage.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).layout_type).write(layout_type.as_raw());
        }
    }

    /// Exclusively borrows the embedded channel layout.
    #[must_use]
    pub fn sound_system_mut(&mut self) -> crate::channel_layout::AVChannelLayoutMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to the
        // initialized inline layout for the duration of this reborrow.
        unsafe {
            crate::channel_layout::AVChannelLayoutMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).sound_system
            ))
        }
        .expect("an embedded field address is non-null")
    }
}

macro_rules! submix_rational_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVIAMFSubmixLayoutRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> crate::rational::AVRationalRef<'a> {
                // SAFETY: the projected inline rational is initialized and
                // remains live for the enclosing layout handle's lifetime.
                unsafe {
                    crate::rational::AVRationalRef::from_ptr(
                        core::ptr::addr_of!((*self.as_ptr()).$field).cast_mut(),
                    )
                }
                .expect("an embedded field address is non-null")
            }
        }

        impl AVIAMFSubmixLayoutMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVIAMFSubmixLayoutRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to this initialized inline rational for the
                // duration of the returned reborrow.
                unsafe {
                    crate::rational::AVRationalMut::from_ptr(core::ptr::addr_of_mut!(
                        (*self.as_mut_ptr()).$field
                    ))
                }
                .expect("an embedded field address is non-null")
            }
        }
    };
}

submix_rational_field!(
    /// Field: AVIAMFSubmixLayout.album_anchored_loudness
    album_anchored_loudness,
    album_anchored_loudness_mut
);
submix_rational_field!(
    /// Field: AVIAMFSubmixLayout.dialogue_anchored_loudness
    dialogue_anchored_loudness,
    dialogue_anchored_loudness_mut
);
submix_rational_field!(
    /// Field: AVIAMFSubmixLayout.true_peak
    true_peak,
    true_peak_mut
);
submix_rational_field!(
    /// Field: AVIAMFSubmixLayout.digital_peak
    digital_peak,
    digital_peak_mut
);
submix_rational_field!(
    /// Field: AVIAMFSubmixLayout.integrated_loudness
    integrated_loudness,
    integrated_loudness_mut
);

#[cfg(test)]
mod struct_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CBox;

    use super::*;

    #[test]
    fn wrapped_struct_layouts_match_bindgen() {
        assert_eq!(
            size_of::<AVIAMFDemixingInfo>(),
            size_of::<ffi::AVIAMFDemixingInfo>()
        );
        assert_eq!(
            align_of::<AVIAMFDemixingInfo>(),
            align_of::<ffi::AVIAMFDemixingInfo>()
        );
        assert_eq!(size_of::<AVIAMFLayer>(), size_of::<ffi::AVIAMFLayer>());
        assert_eq!(align_of::<AVIAMFLayer>(), align_of::<ffi::AVIAMFLayer>());
        assert_eq!(size_of::<AVIAMFMixGain>(), size_of::<ffi::AVIAMFMixGain>());
        assert_eq!(
            align_of::<AVIAMFMixGain>(),
            align_of::<ffi::AVIAMFMixGain>()
        );
        assert_eq!(
            size_of::<AVIAMFParamDefinition>(),
            size_of::<ffi::AVIAMFParamDefinition>()
        );
        assert_eq!(
            align_of::<AVIAMFParamDefinition>(),
            align_of::<ffi::AVIAMFParamDefinition>()
        );
    }

    #[test]
    fn demixing_scalar_handles_read_and_write() {
        let mut raw = ffi::AVIAMFDemixingInfo {
            av_class: core::ptr::null(),
            subblock_duration: 1,
            dmixp_mode: 2,
        };
        // SAFETY: `raw` remains live and this mutable handle is the only access
        // path used until it is dropped.
        let mut info = unsafe { AVIAMFDemixingInfoMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert!(info.as_ref().av_class().is_none());
        assert_eq!(info.as_ref().subblock_duration(), 1);
        assert_eq!(info.as_ref().dmixp_mode(), 2);
        info.set_subblock_duration(7);
        info.set_dmixp_mode(6);
        assert_eq!(info.as_ref().subblock_duration(), 7);
        assert_eq!(info.as_ref().dmixp_mode(), 6);
    }

    #[test]
    fn mix_gain_projects_open_enum_and_embedded_rationals() {
        let zero = ffi::AVRational { num: 0, den: 1 };
        let mut raw = ffi::AVIAMFMixGain {
            av_class: core::ptr::null(),
            subblock_duration: 1,
            animation_type: ffi::AVIAMFAnimationType_AV_IAMF_ANIMATION_TYPE_STEP,
            start_point_value: zero,
            end_point_value: zero,
            control_point_value: zero,
            control_point_relative_time: zero,
        };
        // SAFETY: `raw` remains live and exclusively accessed through `gain`.
        let mut gain = unsafe { AVIAMFMixGainMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        gain.set_animation_type(AVIAMFAnimationType::from_raw(99));
        gain.start_point_value_mut().set_num(-3);
        gain.start_point_value_mut().set_den(2);
        assert_eq!(gain.as_ref().animation_type().as_raw(), 99);
        assert_eq!(gain.as_ref().start_point_value().num(), -3);
        assert_eq!(gain.as_ref().start_point_value().den(), 2);
    }

    #[test]
    fn layer_borrows_layout_and_matrix_without_forming_slices() {
        let mut matrix = [
            ffi::AVRational { num: 1, den: 2 },
            ffi::AVRational { num: 3, den: 4 },
        ];
        let mut raw = ffi::AVIAMFLayer {
            av_class: core::ptr::null(),
            ch_layout: ffi::AVChannelLayout {
                order: ffi::AVChannelOrder_AV_CHANNEL_ORDER_NATIVE,
                nb_channels: 2,
                u: ffi::AVChannelLayout__bindgen_ty_1 { mask: 3 },
                opaque: core::ptr::null_mut(),
            },
            flags: 0,
            output_gain_flags: 0,
            output_gain: ffi::AVRational { num: 0, den: 1 },
            ambisonics_mode: ffi::AVIAMFAmbisonicsMode_AV_IAMF_AMBISONICS_MODE_PROJECTION,
            demixing_matrix: matrix.as_mut_ptr(),
            nb_demixing_matrix: matrix.len() as u32,
        };
        // SAFETY: `raw` and its two initialized matrix entries remain live and
        // are exclusively accessed through `layer` for this scope.
        let mut layer = unsafe { AVIAMFLayerMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(layer.as_ref().ch_layout().nb_channels(), 2);
        let matrix_view = layer.as_ref().demixing_matrix().unwrap();
        assert_eq!(matrix_view.len(), 2);
        assert_eq!(matrix_view.get(1).unwrap().num(), 3);
        layer
            .demixing_matrix_mut()
            .unwrap()
            .get_mut(0)
            .unwrap()
            .set_num(5);
        assert_eq!(
            layer
                .as_ref()
                .demixing_matrix()
                .unwrap()
                .get(0)
                .unwrap()
                .num(),
            5
        );
    }

    #[test]
    fn parameter_definition_has_typed_ownership_and_readonly_extent() {
        // SAFETY: the requested extent is finite; `av_malloc` either returns
        // null or a fresh allocation with libavutil's maximum alignment.
        let pointer = unsafe { ffi::av_malloc(size_of::<ffi::AVIAMFParamDefinition>()) }
            .cast::<ffi::AVIAMFParamDefinition>();
        assert!(!pointer.is_null());
        // SAFETY: `av_malloc` returned a suitably aligned allocation large
        // enough for this header; writing initializes every field before the
        // allocation is adopted by `CBox`.
        unsafe {
            pointer.write(ffi::AVIAMFParamDefinition {
                av_class: core::ptr::null(),
                subblocks_offset: size_of::<ffi::AVIAMFParamDefinition>(),
                subblock_size: size_of::<ffi::AVIAMFMixGain>(),
                nb_subblocks: 0,
                type_: ffi::AVIAMFParamDefinitionType_AV_IAMF_PARAMETER_DEFINITION_MIX_GAIN,
                parameter_id: 4,
                parameter_rate: 48_000,
                duration: 0,
                constant_subblock_duration: 0,
            });
        }
        // SAFETY: the initialized pointer is the unique base of an
        // av_malloc-family allocation, transferred exactly once to this owner.
        let mut definition = unsafe { CBox::<AVIAMFParamDefinition>::from_raw(pointer) }.unwrap();
        assert_eq!(
            definition.as_ref().parameter_type(),
            AVIAMFParamDefinitionType::MIX_GAIN
        );
        assert_eq!(definition.as_ref().nb_subblocks(), 0);
        assert_eq!(
            definition.as_ref().subblocks_offset(),
            size_of::<ffi::AVIAMFParamDefinition>()
        );
        definition.as_mut().set_parameter_id(9);
        assert_eq!(definition.as_ref().parameter_id(), 9);
        // `CBox` drops through the allocator-matched `av_free` here.
    }
}

#[cfg(test)]
mod scheduled_struct_tests {
    use core::num::NonZeroU32;
    use core::ptr::{addr_of, addr_of_mut};

    use crate::channel_layout::AVChannelOrder;

    use super::*;

    fn test_class() -> ffi::AVClass {
        // SAFETY: every bit pattern of the scalar and pointer fields in AVClass
        // is valid, and all callback pointers are optional. The test keeps this
        // value live longer than the synthetic object that borrows it.
        unsafe { core::mem::zeroed::<ffi::AVClass>() }
    }

    fn rational(num: i32, den: i32) -> ffi::AVRational {
        ffi::AVRational { num, den }
    }

    fn unspecified_layout() -> ffi::AVChannelLayout {
        ffi::AVChannelLayout {
            order: ffi::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: ffi::AVChannelLayout__bindgen_ty_1 { mask: 0 },
            opaque: core::ptr::null_mut(),
        }
    }

    #[test]
    fn scheduled_struct_layouts_match_bindgen() {
        assert_eq!(
            core::mem::size_of::<AVIAMFReconGain>(),
            core::mem::size_of::<ffi::AVIAMFReconGain>()
        );
        assert_eq!(
            core::mem::align_of::<AVIAMFReconGain>(),
            core::mem::align_of::<ffi::AVIAMFReconGain>()
        );
        assert_eq!(
            core::mem::size_of::<AVIAMFSubmixLayout>(),
            core::mem::size_of::<ffi::AVIAMFSubmixLayout>()
        );
        assert_eq!(
            core::mem::align_of::<AVIAMFSubmixLayout>(),
            core::mem::align_of::<ffi::AVIAMFSubmixLayout>()
        );
    }

    #[test]
    fn recon_gain_fields_are_copied_and_mutated_without_references() {
        let class = test_class();
        let class = addr_of!(class);
        let mut raw = ffi::AVIAMFReconGain {
            av_class: class,
            subblock_duration: 0,
            recon_gain: [[0; 12]; 6],
        };
        // SAFETY: `raw` is initialized according to AVIAMFReconGain's
        // invariants and this is its only access path for the handle's life.
        let view = unsafe { AVIAMFReconGainRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(view.av_class().as_ptr(), class);
        assert_eq!(view.subblock_duration(), None);
        assert_eq!(view.recon_gain(), [[0; 12]; 6]);

        // SAFETY: the shared handle is no longer used and `raw` remains a live,
        // initialized subblock reached only through this exclusive handle.
        let mut view = unsafe { AVIAMFReconGainMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        let mut gains = [[0; 12]; 6];
        gains[5][11] = 37;
        view.set_recon_gain(gains);
        view.set_subblock_duration(NonZeroU32::new(9).unwrap());
        assert_eq!(view.as_ref().recon_gain()[5][11], 37);
        assert_eq!(view.as_ref().subblock_duration().unwrap().get(), 9);
    }

    #[test]
    fn submix_layout_projects_typed_embedded_fields() {
        let class = test_class();
        let class = addr_of!(class);
        let mut raw = ffi::AVIAMFSubmixLayout {
            av_class: class,
            layout_type: AVIAMFSubmixLayoutType::LOUDSPEAKERS.as_raw(),
            sound_system: unspecified_layout(),
            integrated_loudness: rational(-23, 1),
            digital_peak: rational(-2, 1),
            true_peak: rational(-1, 1),
            dialogue_anchored_loudness: rational(-24, 1),
            album_anchored_loudness: rational(-22, 1),
        };
        // SAFETY: every embedded value is initialized, the class remains live,
        // and this shared handle is the only access path for its duration.
        let view = unsafe { AVIAMFSubmixLayoutRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(view.av_class().as_ptr(), class);
        assert_eq!(view.layout_type(), AVIAMFSubmixLayoutType::LOUDSPEAKERS);
        assert_eq!(view.sound_system().order(), AVChannelOrder::UNSPECIFIED);
        assert_eq!(
            (
                view.integrated_loudness().num(),
                view.integrated_loudness().den()
            ),
            (-23, 1)
        );
        assert_eq!(view.album_anchored_loudness().num(), -22);

        // SAFETY: the shared handle is no longer used and the live initialized
        // object is reached only through this exclusive handle.
        let mut view = unsafe { AVIAMFSubmixLayoutMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        view.set_layout_type(AVIAMFSubmixLayoutType::BINAURAL);
        view.true_peak_mut().set_num(-3);
        assert_eq!(
            view.sound_system_mut().as_ref().order(),
            AVChannelOrder::UNSPECIFIED
        );
        assert_eq!(
            view.as_ref().layout_type(),
            AVIAMFSubmixLayoutType::BINAURAL
        );
        assert_eq!(view.as_ref().true_peak().num(), -3);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVIAMFSubmixElement
    ///
    /// Layout-compatible view of an element owned by an `AVIAMFSubmix`
    /// within a mix presentation. Libavutil creates these values through
    /// `av_iamf_submix_add_element`, installs immutable static class metadata,
    /// and releases the element, its optional dictionary, and its optional
    /// parameter definition with the enclosing mix presentation. It therefore
    /// has borrowed handles but no independent owning handle.
    AVIAMFSubmixElement,
    AVIAMFSubmixElementRef,
    AVIAMFSubmixElementMut,
    ffi::AVIAMFSubmixElement
);

impl<'a> AVIAMFSubmixElementRef<'a> {
    /// Field: AVIAMFSubmixElement.annotations
    ///
    /// Borrows the optional dictionary owned by this element.
    #[must_use]
    pub fn annotations(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: raw-place projection only copies the pointer from the live
        // element. Its invariant makes a non-null pointer one fully formed
        // dictionary kept alive by the enclosing mix presentation for `'a`.
        let annotations = unsafe { addr_of!((*self.as_ptr()).annotations).read() };
        // SAFETY: the ownership invariant above supplies validity and lifetime;
        // null is represented by `None`.
        unsafe { AVDictionaryRef::from_ptr(annotations) }
    }

    /// Field: AVIAMFSubmixElement.av_class
    ///
    /// Returns the immutable process-lifetime option metadata installed by
    /// libavutil's element constructor.
    #[must_use]
    pub fn av_class(&self) -> AVClassRef<'a> {
        // SAFETY: raw-place projection copies the pointer without forming a
        // reference. The type invariant makes it non-null static metadata.
        let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: the constructor-established invariant supplies non-nullness,
        // immutability, initialization, and a lifetime exceeding `'a`.
        unsafe { AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFSubmixElement has a non-null AVClass")
    }

    /// Field: AVIAMFSubmixElement.default_mix_gain
    ///
    /// Borrows the inline default gain rational.
    #[must_use]
    pub fn default_mix_gain(&self) -> AVRationalRef<'a> {
        // SAFETY: the inline rational is initialized and remains live with the
        // element for `'a`; the returned handle contains only its raw address.
        unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).default_mix_gain).cast_mut()) }
            .expect("an embedded field address is non-null")
    }

    /// Field: AVIAMFSubmixElement.headphones_rendering_mode
    #[must_use]
    pub fn headphones_rendering_mode(&self) -> AVIAMFHeadphonesMode {
        // SAFETY: bindgen represents this open C enum as an integer. Raw-place
        // projection copies it without forming a reference to C storage.
        AVIAMFHeadphonesMode::from_raw(unsafe {
            addr_of!((*self.as_ptr()).headphones_rendering_mode).read()
        })
    }

    /// Field: AVIAMFSubmixElement.element_mix_config
    ///
    /// Borrows the optional parameter definition owned by this element.
    #[must_use]
    pub fn element_mix_config(&self) -> Option<AVIAMFParamDefinitionRef<'a>> {
        // SAFETY: raw-place projection only copies the pointer. A non-null
        // value is an initialized definition kept alive by the element for
        // the lifetime of this shared handle.
        let definition = unsafe { addr_of!((*self.as_ptr()).element_mix_config).read() };
        // SAFETY: the field invariant establishes validity and lifetime; null
        // is represented by `None`.
        unsafe { AVIAMFParamDefinitionRef::from_ptr(definition) }
    }

    /// Field: AVIAMFSubmixElement.audio_element_id
    #[must_use]
    pub fn audio_element_id(&self) -> u32 {
        // SAFETY: raw-place projection copies one initialized scalar without
        // forming a reference to C-visible storage.
        unsafe { addr_of!((*self.as_ptr()).audio_element_id).read() }
    }
}

impl AVIAMFSubmixElementMut<'_> {
    /// Replaces the owned annotations dictionary and returns the previous one.
    pub fn replace_annotations(
        &mut self,
        annotations: Option<CBox<AVDictionary>>,
    ) -> Option<CBox<AVDictionary>> {
        let annotations = annotations.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: this exclusive handle permits replacing the owner slot. The
        // incoming pointer transfers exactly one ownership unit to the field;
        // `replace` returns the old pointer without releasing it.
        let old = unsafe { addr_of_mut!((*self.as_mut_ptr()).annotations).replace(annotations) };
        // SAFETY: the field invariant makes `old` null or the unique live
        // dictionary owner removed from the slot above.
        unsafe { CBox::from_raw(old) }
    }

    /// Exclusively borrows the inline default gain rational.
    #[must_use]
    pub fn default_mix_gain_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: the exclusive element reborrow supplies the sole access to
        // this initialized inline rational for the returned handle's lifetime.
        unsafe { AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).default_mix_gain)) }
            .expect("an embedded field address is non-null")
    }

    /// Sets the headphones rendering mode while preserving unknown C values.
    pub fn set_headphones_rendering_mode(&mut self, mode: AVIAMFHeadphonesMode) {
        // SAFETY: the exclusive handle permits replacing this integer-backed
        // enum, and the wrapper represents every possible C value.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).headphones_rendering_mode).write(mode.as_raw());
        }
    }

    /// Exclusively borrows the optional owned parameter definition.
    #[must_use]
    pub fn element_mix_config_mut(&mut self) -> Option<AVIAMFParamDefinitionMut<'_>> {
        // SAFETY: raw-place projection copies the pointer from the exclusive
        // element. A non-null definition remains owned by the element and the
        // returned handle is bound to this exclusive reborrow.
        let definition = unsafe { addr_of!((*self.as_mut_ptr()).element_mix_config).read() };
        // SAFETY: the field invariant supplies validity and this method's
        // exclusive borrow supplies the returned mutable lifetime.
        unsafe { AVIAMFParamDefinitionMut::from_ptr(definition) }
    }

    /// Replaces the owned parameter definition and returns the previous one.
    ///
    /// A non-null definition must describe mix gain, as required by the C
    /// structure contract. An owner of another parameter type is returned in
    /// `Err` without changing the element.
    pub fn replace_element_mix_config(
        &mut self,
        definition: Option<CBox<AVIAMFParamDefinition>>,
    ) -> Result<Option<CBox<AVIAMFParamDefinition>>, CBox<AVIAMFParamDefinition>> {
        if definition.as_ref().is_some_and(|definition| {
            definition.as_ref().parameter_type() != AVIAMFParamDefinitionType::MIX_GAIN
        }) {
            return Err(definition.expect("the rejected definition is present"));
        }
        let definition = definition.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: this exclusive handle permits replacing the owner slot. The
        // incoming pointer transfers one ownership unit into the element and
        // `replace` moves the old unit out without releasing it.
        let old =
            unsafe { addr_of_mut!((*self.as_mut_ptr()).element_mix_config).replace(definition) };
        // SAFETY: the field invariant makes `old` null or the unique live
        // av_malloc-family allocation removed from the slot above.
        Ok(unsafe { CBox::from_raw(old) })
    }

    /// Sets the referenced audio-element identifier.
    pub fn set_audio_element_id(&mut self, id: u32) {
        // SAFETY: the exclusive handle permits replacing this scalar and raw
        // projection forms no reference to C-visible storage.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).audio_element_id).write(id) }
    }
}

#[cfg(test)]
mod submix_element_tests {
    use core::mem::{align_of, size_of};
    use core::ptr::{addr_of, addr_of_mut};

    use super::*;

    fn test_class() -> ffi::AVClass {
        // SAFETY: every field is an integer, raw pointer, or optional callback,
        // so zero initializes a valid inert class used only as borrowed test
        // metadata.
        unsafe { core::mem::zeroed() }
    }

    fn raw_element(class: *const ffi::AVClass) -> ffi::AVIAMFSubmixElement {
        ffi::AVIAMFSubmixElement {
            av_class: class,
            audio_element_id: 7,
            element_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational { num: -3, den: 2 },
            headphones_rendering_mode: AVIAMFHeadphonesMode::STEREO.as_raw(),
            annotations: core::ptr::null_mut(),
        }
    }

    fn parameter_definition(
        parameter_type: AVIAMFParamDefinitionType,
    ) -> CBox<AVIAMFParamDefinition> {
        // SAFETY: the requested finite size fits one header; av_malloc returns
        // null or a fresh allocation with sufficient alignment.
        let pointer = unsafe { ffi::av_malloc(size_of::<ffi::AVIAMFParamDefinition>()) }
            .cast::<ffi::AVIAMFParamDefinition>();
        assert!(!pointer.is_null());
        // SAFETY: the fresh allocation is large and aligned for this write,
        // which initializes every header field before ownership is adopted.
        unsafe {
            pointer.write(ffi::AVIAMFParamDefinition {
                av_class: core::ptr::null(),
                subblocks_offset: size_of::<ffi::AVIAMFParamDefinition>(),
                subblock_size: size_of::<ffi::AVIAMFMixGain>(),
                nb_subblocks: 0,
                type_: parameter_type.as_raw(),
                parameter_id: 11,
                parameter_rate: 48_000,
                duration: 0,
                constant_subblock_duration: 0,
            });
        }
        // SAFETY: the initialized pointer is the unique allocation base from
        // av_malloc and transfers exactly once into the matching CBox owner.
        unsafe { CBox::from_raw(pointer) }.unwrap()
    }

    #[test]
    fn layout_and_borrowed_fields_match_c() {
        assert_eq!(
            size_of::<AVIAMFSubmixElement>(),
            size_of::<ffi::AVIAMFSubmixElement>()
        );
        assert_eq!(
            align_of::<AVIAMFSubmixElement>(),
            align_of::<ffi::AVIAMFSubmixElement>()
        );

        let class = test_class();
        let mut raw = raw_element(addr_of!(class));
        // SAFETY: `raw` is fully initialized according to the wrapper's
        // invariant and remains live and exclusively accessed through `view`.
        let mut view = unsafe { AVIAMFSubmixElementMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(view.as_ref().av_class().as_ptr(), addr_of!(class));
        assert_eq!(view.as_ref().audio_element_id(), 7);
        assert!(view.as_ref().annotations().is_none());
        assert!(view.as_ref().element_mix_config().is_none());
        assert_eq!(view.as_ref().default_mix_gain().num(), -3);
        assert_eq!(view.as_ref().default_mix_gain().den(), 2);
        assert_eq!(
            view.as_ref().headphones_rendering_mode(),
            AVIAMFHeadphonesMode::STEREO
        );

        view.set_audio_element_id(9);
        view.set_headphones_rendering_mode(AVIAMFHeadphonesMode::BINAURAL);
        view.default_mix_gain_mut().set_den(4);
        assert_eq!(view.as_ref().audio_element_id(), 9);
        assert_eq!(
            view.as_ref().headphones_rendering_mode(),
            AVIAMFHeadphonesMode::BINAURAL
        );
        assert_eq!(view.as_ref().default_mix_gain().den(), 4);
    }

    #[test]
    fn owned_fields_can_be_replaced_and_taken() {
        let class = test_class();
        let mut raw = raw_element(addr_of!(class));
        // SAFETY: `raw` is fully initialized and only this handle accesses it
        // until all installed owners are taken back out.
        let mut view = unsafe { AVIAMFSubmixElementMut::from_ptr(addr_of_mut!(raw)) }.unwrap();

        let rejected = parameter_definition(AVIAMFParamDefinitionType::DEMIXING);
        assert!(view.replace_element_mix_config(Some(rejected)).is_err());
        assert!(view.as_ref().element_mix_config().is_none());

        let definition = parameter_definition(AVIAMFParamDefinitionType::MIX_GAIN);
        assert!(
            view.replace_element_mix_config(Some(definition))
                .unwrap()
                .is_none()
        );
        view.element_mix_config_mut().unwrap().set_parameter_id(13);
        assert_eq!(
            view.as_ref().element_mix_config().unwrap().parameter_id(),
            13
        );
        let definition = view.replace_element_mix_config(None).unwrap().unwrap();
        assert_eq!(definition.as_ref().parameter_id(), 13);
        drop(definition);

        let mut dictionary = crate::dict::Dictionary::default();
        crate::dict::av_dict_set(&mut dictionary, c"en", Some(c"name"), 0).unwrap();
        assert!(view.replace_annotations(dictionary.into_owner()).is_none());
        assert_eq!(crate::dict::av_dict_count(view.as_ref().annotations()), 1);
        let annotations = view.replace_annotations(None).unwrap();
        assert_eq!(crate::dict::av_dict_count(Some(annotations.as_ref())), 1);
        drop(annotations);
    }
}
