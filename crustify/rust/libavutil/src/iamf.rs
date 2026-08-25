//! Wrappers for `libavutil/iamf.c`.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::num::NonZeroU32;
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
    /// `av_iamf_param_definition_alloc` installs the immutable static
    /// `demixing_info_class` in every subblock it creates; a foreign instance
    /// must likewise keep equivalent class metadata live with the subblock.
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
    /// `av_iamf_audio_element_add_layer` installs the immutable static
    /// `layer_class`; a foreign instance must likewise keep equivalent class
    /// metadata live with the layer.
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
    /// `av_iamf_param_definition_alloc` installs the immutable static
    /// `mix_gain_class` in every subblock it creates; a foreign instance must
    /// likewise keep equivalent class metadata live with the subblock.
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
    /// `av_iamf_param_definition_alloc` installs the immutable static
    /// `param_definition_class` in the header and the matching subblock class
    /// in every trailing subblock; a foreign instance must likewise keep
    /// equivalent class metadata live with the allocation.
    ///
    /// Every construction contract for this wrapper additionally requires the
    /// trailing array to agree with the header it follows: the allocation
    /// covers `subblocks_offset + nb_subblocks * subblock_size` bytes, and
    /// each of those elements is an initialized, suitably aligned subblock of
    /// the struct type selected by `type`
    /// ([`AVIAMFMixGain`], [`AVIAMFDemixingInfo`] or [`AVIAMFReconGain`]).
    /// `av_iamf_param_definition_alloc` establishes exactly that, and it is
    /// what lets [`av_iamf_param_definition_get_subblock`] hand out a typed
    /// borrow of an element the C helper returns type-erased.
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
    ($(#[$meta:meta])* $shared:ident, $owner:literal) => {
        impl<'a> $shared<'a> {
            $(#[$meta])*
            ///
            /// Returns the immutable process-lifetime option metadata that the
            /// type's libavutil constructor installs. The slot is never null
            /// and never replaced, so the class outlives every borrow of the
            /// object it describes.
            ///
            /// # Panics
            ///
            /// Panics when the slot is null, which every construction contract
            /// for this wrapper already excludes.
            #[must_use]
            pub fn av_class(&self) -> AVClassRef<'a> {
                // SAFETY: copying the const pointer through a raw projection
                // forms no reference. The value is immutable class metadata
                // that remains live at least as long as this handle.
                let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
                // SAFETY: the wrapper's construction contract establishes that
                // the slot holds the address of a file-scope `static const
                // AVClass` in `libavutil/iamf.c`, borrowed and immutable for
                // longer than `'a`.
                unsafe { AVClassRef::from_ptr(class.cast_mut()) }
                    .expect(concat!($owner, " has a non-null AVClass"))
            }
        }
    };
}

class_field!(
    /// Field: AVIAMFDemixingInfo.av_class
    AVIAMFDemixingInfoRef,
    "AVIAMFDemixingInfo"
);
class_field!(
    /// Field: AVIAMFLayer.av_class
    AVIAMFLayerRef,
    "AVIAMFLayer"
);
class_field!(
    /// Field: AVIAMFMixGain.av_class
    AVIAMFMixGainRef,
    "AVIAMFMixGain"
);
class_field!(
    /// Field: AVIAMFParamDefinition.av_class
    AVIAMFParamDefinitionRef,
    "AVIAMFParamDefinition"
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

macro_rules! subblock_duration_field {
    ($(#[$meta:meta])* $shared:ident, $exclusive:ident) => {
        impl $shared<'_> {
            $(#[$meta])*
            ///
            /// The duration is expressed in units of
            /// `1 / AVIAMFParamDefinition::parameter_rate`. Libavutil's
            /// allocator zeroes the subblock and never applies the option
            /// default, so `None` is the not-yet-configured state that the
            /// documented `must not be 0` invariant excludes from a finished
            /// subblock.
            #[must_use]
            pub fn subblock_duration(&self) -> Option<NonZeroU32> {
                // SAFETY: the shared handle keeps a live initialized value;
                // raw projection copies one scalar and forms no reference to
                // C-visible storage.
                let duration = unsafe { addr_of!((*self.as_ptr()).subblock_duration).read() };
                NonZeroU32::new(duration)
            }
        }

        impl $exclusive<'_> {
            #[doc = concat!("Sets [`subblock_duration`](`", stringify!($shared), "::subblock_duration`), preserving its nonzero invariant.")]
            pub fn set_subblock_duration(&mut self, duration: NonZeroU32) {
                // SAFETY: the exclusive handle supplies write provenance to
                // the live field; raw projection writes only that scalar and
                // the argument type upholds the subblock's nonzero invariant.
                unsafe {
                    addr_of_mut!((*self.as_mut_ptr()).subblock_duration).write(duration.get())
                }
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

subblock_duration_field!(
    /// Field: AVIAMFDemixingInfo.subblock_duration
    AVIAMFDemixingInfoRef,
    AVIAMFDemixingInfoMut
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

subblock_duration_field!(
    /// Field: AVIAMFMixGain.subblock_duration
    AVIAMFMixGainRef,
    AVIAMFMixGainMut
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

class_field!(
    /// Field: AVIAMFReconGain.av_class
    AVIAMFReconGainRef,
    "AVIAMFReconGain"
);

impl<'a> AVIAMFReconGainRef<'a> {
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
}

subblock_duration_field!(
    /// Field: AVIAMFReconGain.subblock_duration
    AVIAMFReconGainRef,
    AVIAMFReconGainMut
);

impl AVIAMFReconGainMut<'_> {
    /// Replaces all reconstruction-gain entries.
    pub fn set_recon_gain(&mut self, gain: [[u8; 12]; 6]) {
        // SAFETY: the exclusive handle permits replacing this initialized
        // fixed array; raw-place projection forms no reference to C storage.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).recon_gain).write(gain);
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

class_field!(
    /// Field: AVIAMFSubmixLayout.av_class
    AVIAMFSubmixLayoutRef,
    "AVIAMFSubmixLayout"
);

impl<'a> AVIAMFSubmixLayoutRef<'a> {
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

    /// An inert stand-in for the file-scope `static const AVClass` values that
    /// libavutil installs, satisfying the wrappers' non-null class invariant.
    fn test_class() -> ffi::AVClass {
        // SAFETY: every AVClass field is an integer, a raw pointer, or an
        // optional function pointer, so the all-zero pattern is a valid, inert
        // class. The caller keeps it live longer than the object borrowing it.
        unsafe { core::mem::zeroed() }
    }

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
        let class = test_class();
        let mut raw = ffi::AVIAMFDemixingInfo {
            av_class: core::ptr::addr_of!(class),
            subblock_duration: 1,
            dmixp_mode: 2,
        };
        // SAFETY: `raw` remains live, its class outlives it, and this mutable
        // handle is the only access path used until it is dropped.
        let mut info = unsafe { AVIAMFDemixingInfoMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(
            info.as_ref().av_class().as_ptr(),
            core::ptr::addr_of!(class)
        );
        assert_eq!(
            info.as_ref().subblock_duration(),
            Some(NonZeroU32::new(1).unwrap())
        );
        assert_eq!(info.as_ref().dmixp_mode(), 2);
        info.set_subblock_duration(NonZeroU32::new(7).unwrap());
        info.set_dmixp_mode(6);
        assert_eq!(
            info.as_ref().subblock_duration(),
            Some(NonZeroU32::new(7).unwrap())
        );
        assert_eq!(info.as_ref().dmixp_mode(), 6);
    }

    #[test]
    fn an_unconfigured_subblock_duration_reads_as_none() {
        let class = test_class();
        let mut raw = ffi::AVIAMFDemixingInfo {
            av_class: core::ptr::addr_of!(class),
            subblock_duration: 0,
            dmixp_mode: 0,
        };
        // SAFETY: `raw` and its class remain live for the handle's scope and
        // the handle is the only access path to them.
        let info = unsafe { AVIAMFDemixingInfoRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(info.subblock_duration(), None);
    }

    #[test]
    fn mix_gain_projects_open_enum_and_embedded_rationals() {
        let class = test_class();
        let zero = ffi::AVRational { num: 0, den: 1 };
        let mut raw = ffi::AVIAMFMixGain {
            av_class: core::ptr::addr_of!(class),
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
        let class = test_class();
        let mut raw = ffi::AVIAMFLayer {
            av_class: core::ptr::addr_of!(class),
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
        let class = test_class();
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
                av_class: core::ptr::addr_of!(class),
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
        // `class` is declared first and therefore outlives the owner.
        let mut definition = unsafe { CBox::<AVIAMFParamDefinition>::from_raw(pointer) }.unwrap();
        assert_eq!(
            definition.as_ref().av_class().as_ptr(),
            core::ptr::addr_of!(class)
        );
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
        class: *const ffi::AVClass,
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
                av_class: class,
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

        let rejected = parameter_definition(AVIAMFParamDefinitionType::DEMIXING, addr_of!(class));
        assert!(view.replace_element_mix_config(Some(rejected)).is_err());
        assert!(view.as_ref().element_mix_config().is_none());

        let definition = parameter_definition(AVIAMFParamDefinitionType::MIX_GAIN, addr_of!(class));
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
/// A shared view of one runtime-typed parameter-definition subblock.
#[derive(Clone, Copy)]
pub enum AVIAMFParamSubblockRef<'a> {
    /// Mix-gain parameter data.
    MixGain(AVIAMFMixGainRef<'a>),
    /// Demixing parameter data.
    Demixing(AVIAMFDemixingInfoRef<'a>),
    /// Reconstruction-gain parameter data.
    ReconGain(AVIAMFReconGainRef<'a>),
}

/// An exclusive view of one runtime-typed parameter-definition subblock.
pub enum AVIAMFParamSubblockMut<'a> {
    /// Mix-gain parameter data.
    MixGain(AVIAMFMixGainMut<'a>),
    /// Demixing parameter data.
    Demixing(AVIAMFDemixingInfoMut<'a>),
    /// Reconstruction-gain parameter data.
    ReconGain(AVIAMFReconGainMut<'a>),
}

/// Wraps: av_iamf_param_definition_get_subblock
///
/// Returns the subblock at `idx`, tied to the lifetime of `par`. `None` means
/// that the index is out of range or that the definition carries an unknown
/// parameter type. Unlike the C inline helper, this checked form never aborts
/// for an out-of-range index and does not expose the helper's mutable pointer
/// through a shared borrow.
#[must_use]
pub fn av_iamf_param_definition_get_subblock<'a>(
    par: AVIAMFParamDefinitionRef<'a>,
    idx: u32,
) -> Option<AVIAMFParamSubblockRef<'a>> {
    if idx >= par.nb_subblocks() {
        return None;
    }

    let parameter_type = par.parameter_type();
    if parameter_type != AVIAMFParamDefinitionType::MIX_GAIN
        && parameter_type != AVIAMFParamDefinitionType::DEMIXING
        && parameter_type != AVIAMFParamDefinitionType::RECON_GAIN
    {
        return None;
    }

    // SAFETY: `par` is a live parameter definition, and the range check
    // establishes the C inline helper's index precondition. The returned
    // pointer selects initialized trailing storage within `par` and therefore
    // remains live for `'a`.
    let subblock =
        unsafe { ffi::crustify_av_iamf_param_definition_get_subblock(par.as_ptr(), idx) };

    if parameter_type == AVIAMFParamDefinitionType::MIX_GAIN {
        // SAFETY: the wrapper's construction contract binds `type` to the
        // struct type of every trailing subblock, so this erased pointer
        // addresses an initialized `AVIAMFMixGain` that lives for `'a`.
        unsafe { AVIAMFMixGainRef::from_void_ptr(subblock) }.map(AVIAMFParamSubblockRef::MixGain)
    } else if parameter_type == AVIAMFParamDefinitionType::DEMIXING {
        // SAFETY: the wrapper's construction contract binds `type` to the
        // struct type of every trailing subblock, so this erased pointer
        // addresses an initialized `AVIAMFDemixingInfo` that lives for `'a`.
        unsafe { AVIAMFDemixingInfoRef::from_void_ptr(subblock) }
            .map(AVIAMFParamSubblockRef::Demixing)
    } else {
        // SAFETY: the only remaining accepted discriminator, plus the same
        // construction contract, makes this erased pointer an initialized
        // `AVIAMFReconGain` that lives for `'a`.
        unsafe { AVIAMFReconGainRef::from_void_ptr(subblock) }
            .map(AVIAMFParamSubblockRef::ReconGain)
    }
}

/// Wraps: av_iamf_param_definition_get_subblock
///
/// Exclusive variant of [`av_iamf_param_definition_get_subblock`], representing
/// the writable pointer that the C helper returns even from a `const` header.
///
/// Consuming the exclusive parent handle ensures that no safe shared parent
/// view can coexist with the returned mutable subblock view.
#[must_use]
pub fn av_iamf_param_definition_get_subblock_mut<'a>(
    mut par: AVIAMFParamDefinitionMut<'a>,
    idx: u32,
) -> Option<AVIAMFParamSubblockMut<'a>> {
    if idx >= par.as_ref().nb_subblocks() {
        return None;
    }

    let parameter_type = par.as_ref().parameter_type();
    if parameter_type != AVIAMFParamDefinitionType::MIX_GAIN
        && parameter_type != AVIAMFParamDefinitionType::DEMIXING
        && parameter_type != AVIAMFParamDefinitionType::RECON_GAIN
    {
        return None;
    }

    // SAFETY: the exclusive live parent handle is consumed by this function,
    // and the range check establishes the inline helper's precondition. Its
    // trailing subblock remains exclusively borrowed for `'a`.
    let subblock =
        unsafe { ffi::crustify_av_iamf_param_definition_get_subblock(par.as_mut_ptr(), idx) };

    if parameter_type == AVIAMFParamDefinitionType::MIX_GAIN {
        // SAFETY: the construction contract binds `type` to the trailing
        // subblock struct type, and the consumed exclusive parent handle makes
        // this a live, exclusive `AVIAMFMixGain` borrow for `'a`.
        unsafe { AVIAMFMixGainMut::from_ptr(subblock.cast()) }.map(AVIAMFParamSubblockMut::MixGain)
    } else if parameter_type == AVIAMFParamDefinitionType::DEMIXING {
        // SAFETY: the construction contract binds `type` to the trailing
        // subblock struct type, and the consumed exclusive parent handle makes
        // this a live, exclusive `AVIAMFDemixingInfo` borrow for `'a`.
        unsafe { AVIAMFDemixingInfoMut::from_ptr(subblock.cast()) }
            .map(AVIAMFParamSubblockMut::Demixing)
    } else {
        // SAFETY: the remaining accepted discriminator, the construction
        // contract for the trailing array, and the consumed exclusive parent
        // handle make this a live, exclusive `AVIAMFReconGain` borrow for `'a`.
        unsafe { AVIAMFReconGainMut::from_ptr(subblock.cast()) }
            .map(AVIAMFParamSubblockMut::ReconGain)
    }
}

#[cfg(test)]
mod subblock_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CBox;

    use super::*;

    /// An inert stand-in for the file-scope `static const AVClass` values that
    /// `av_iamf_param_definition_alloc` installs in the header and in every
    /// trailing subblock.
    fn test_class() -> ffi::AVClass {
        // SAFETY: every AVClass field is an integer, a raw pointer, or an
        // optional function pointer, so the all-zero pattern is a valid, inert
        // class. The caller keeps it live longer than the objects using it.
        unsafe { core::mem::zeroed() }
    }

    fn definition_with_subblock(
        parameter_type: AVIAMFParamDefinitionType,
        subblock_size: usize,
        subblock_align: usize,
        class: *const ffi::AVClass,
    ) -> CBox<AVIAMFParamDefinition> {
        let header_size = size_of::<ffi::AVIAMFParamDefinition>();
        let subblocks_offset = (header_size + subblock_align - 1) & !(subblock_align - 1);
        let allocation_size = subblocks_offset + subblock_size;

        // SAFETY: `av_mallocz` accepts this nonzero byte count and returns
        // suitably aligned zeroed storage or null. The allocation is retained
        // by the `CBox` constructed below and released with matching `av_free`.
        let definition =
            unsafe { ffi::av_mallocz(allocation_size) }.cast::<ffi::AVIAMFParamDefinition>();
        assert!(!definition.is_null());

        // SAFETY: the allocation covers the complete header and one aligned
        // subblock. Zero is valid for every remaining bindgen C field; these
        // raw writes establish the trailing-array metadata read by the helper.
        unsafe {
            addr_of_mut!((*definition).av_class).write(class);
            addr_of_mut!((*definition).subblocks_offset).write(subblocks_offset);
            addr_of_mut!((*definition).subblock_size).write(subblock_size);
            addr_of_mut!((*definition).nb_subblocks).write(1);
            addr_of_mut!((*definition).type_).write(parameter_type.as_raw());
        }

        // SAFETY: `subblocks_offset` is rounded up to the subblock alignment
        // and the allocation extends `subblock_size` bytes beyond it. All three
        // subblock structures start with a `const AVClass *`, so this writes
        // that leading slot exactly as `av_iamf_param_definition_alloc` does.
        unsafe {
            definition
                .cast::<u8>()
                .add(subblocks_offset)
                .cast::<*const ffi::AVClass>()
                .write(class);
        }

        // SAFETY: the pointer is the base of one fully initialized,
        // av_malloc-family allocation. The definition and its zero-initialized
        // inline subblock require no teardown beyond `av_free`.
        unsafe { CBox::from_raw(definition) }.expect("av_mallocz returned non-null")
    }

    #[test]
    fn checked_subblock_views_preserve_type_bounds_and_exclusivity() {
        let class = test_class();
        let class = addr_of!(class);
        let mut mix_gain = definition_with_subblock(
            AVIAMFParamDefinitionType::MIX_GAIN,
            size_of::<ffi::AVIAMFMixGain>(),
            align_of::<ffi::AVIAMFMixGain>(),
            class,
        );

        assert!(
            av_iamf_param_definition_get_subblock(mix_gain.as_ref(), 1).is_none(),
            "an out-of-range index must not reach the aborting C helper"
        );
        {
            let subblock = av_iamf_param_definition_get_subblock_mut(mix_gain.as_mut(), 0).unwrap();
            match subblock {
                AVIAMFParamSubblockMut::MixGain(mut subblock) => {
                    assert_eq!(subblock.as_ref().av_class().as_ptr(), class);
                    assert_eq!(subblock.as_ref().subblock_duration(), None);
                    subblock.set_subblock_duration(NonZeroU32::new(37).unwrap());
                }
                _ => panic!("mix-gain discriminator returned the wrong variant"),
            }
        }
        let shared = av_iamf_param_definition_get_subblock(mix_gain.as_ref(), 0).unwrap();
        // A shared subblock view is a copyable handle, so observing it once
        // must not consume it.
        for view in [shared, shared] {
            match view {
                AVIAMFParamSubblockRef::MixGain(subblock) => {
                    assert_eq!(
                        subblock.subblock_duration(),
                        Some(NonZeroU32::new(37).unwrap())
                    );
                }
                _ => panic!("mix-gain discriminator returned the wrong variant"),
            }
        }

        let demixing = definition_with_subblock(
            AVIAMFParamDefinitionType::DEMIXING,
            size_of::<ffi::AVIAMFDemixingInfo>(),
            align_of::<ffi::AVIAMFDemixingInfo>(),
            class,
        );
        assert!(matches!(
            av_iamf_param_definition_get_subblock(demixing.as_ref(), 0),
            Some(AVIAMFParamSubblockRef::Demixing(_))
        ));

        let recon_gain = definition_with_subblock(
            AVIAMFParamDefinitionType::RECON_GAIN,
            size_of::<ffi::AVIAMFReconGain>(),
            align_of::<ffi::AVIAMFReconGain>(),
            class,
        );
        assert!(matches!(
            av_iamf_param_definition_get_subblock(recon_gain.as_ref(), 0),
            Some(AVIAMFParamSubblockRef::ReconGain(_))
        ));

        let unknown = definition_with_subblock(
            AVIAMFParamDefinitionType::from_raw(99),
            size_of::<ffi::AVIAMFMixGain>(),
            align_of::<ffi::AVIAMFMixGain>(),
            class,
        );
        assert!(av_iamf_param_definition_get_subblock(unknown.as_ref(), 0).is_none());
    }
}

ffibox::define_ctype!(
    /// Wraps: AVIAMFAudioElement
    ///
    /// Layout-compatible IAMF audio element. Values allocated by libavutil can
    /// be adopted into [`CBox<AVIAMFAudioElement>`], which releases the layer
    /// container, its layers, and both optional parameter definitions.
    AVIAMFAudioElement,
    AVIAMFAudioElementRef,
    AVIAMFAudioElementMut,
    ffi::AVIAMFAudioElement
);

// SAFETY: a value adopted into `CBox` must be the allocation returned by
// `av_iamf_audio_element_alloc`, together with only the owned children allowed
// by the public API. The matching destructor accepts its address as an owning
// in/out slot, releases every child exactly once, and clears that local slot.
unsafe impl CDropped for AVIAMFAudioElement {
    unsafe fn c_drop(audio_element: NonNull<Self>) {
        let mut pointer = audio_element.as_ptr().cast::<ffi::AVIAMFAudioElement>();
        // SAFETY: the trait contract transfers the unique live allocation to
        // its matching destructor exactly once. The local slot is non-null and
        // writable for the duration of the call.
        unsafe { ffi::av_iamf_audio_element_free(addr_of_mut!(pointer)) }
    }
}

/// Borrowed random-access view of an audio element's owned layer pointers.
///
/// The view never exposes the pointer container itself; each successful lookup
/// returns a handle tied to the audio element's shared borrow.
#[derive(Clone, Copy)]
pub struct AVIAMFLayers<'a> {
    data: Option<NonNull<*mut ffi::AVIAMFLayer>>,
    len: usize,
    _borrow: PhantomData<&'a ()>,
}

impl AVIAMFLayers<'_> {
    /// Returns the number of layer pointers in the parent-owned container.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the audio element contains no layers.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a> AVIAMFLayers<'a> {
    /// Borrows one layer, or returns `None` when `index` is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<AVIAMFLayerRef<'a>> {
        if index >= self.len {
            return None;
        }
        let data = self.data?;
        // SAFETY: a valid audio element owns a `len`-entry pointer container;
        // the bounds check makes this pointer read valid. Each counted entry
        // is a live layer owned by the parent for `'a`.
        let layer = unsafe { data.as_ptr().add(index).read() };
        // SAFETY: the layer remains owned and live with the shared parent, and
        // this result grants shared access only. Null is handled as `None`.
        unsafe { AVIAMFLayerRef::from_ptr(layer) }
    }
}

impl<'a> AVIAMFAudioElementRef<'a> {
    /// Field: AVIAMFAudioElement.av_class
    ///
    /// Returns the immutable process-lifetime option metadata installed by
    /// libavutil's constructor.
    #[must_use]
    pub fn av_class(&self) -> AVClassRef<'a> {
        // SAFETY: the audio-element invariant makes this a non-null pointer to
        // immutable static class metadata. Copying it forms no reference.
        let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: the static pointee is immutable and outlives the returned
        // handle. Construction through libavutil establishes non-nullness.
        unsafe { AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFAudioElement has a non-null AVClass")
    }

    /// Field: AVIAMFAudioElement.recon_gain_info
    ///
    /// Borrows the optional reconstruction-gain definition owned by the audio
    /// element.
    #[must_use]
    pub fn recon_gain_info(&self) -> Option<AVIAMFParamDefinitionRef<'a>> {
        // SAFETY: raw projection copies the nullable owned pointer without
        // forming a reference; the parent keeps a non-null pointee live.
        let definition = unsafe { addr_of!((*self.as_ptr()).recon_gain_info).read() };
        // SAFETY: any non-null definition is owned by and live with the parent
        // and the result provides shared access only.
        unsafe { AVIAMFParamDefinitionRef::from_ptr(definition) }
    }

    /// Field: AVIAMFAudioElement.demixing_info
    ///
    /// Borrows the optional demixing definition owned by the audio element.
    #[must_use]
    pub fn demixing_info(&self) -> Option<AVIAMFParamDefinitionRef<'a>> {
        // SAFETY: raw projection copies the nullable owned pointer without
        // forming a reference; the parent keeps a non-null pointee live.
        let definition = unsafe { addr_of!((*self.as_ptr()).demixing_info).read() };
        // SAFETY: any non-null definition is owned by and live with the parent
        // and the result provides shared access only.
        unsafe { AVIAMFParamDefinitionRef::from_ptr(definition) }
    }

    /// Field: AVIAMFAudioElement.nb_layers
    #[must_use]
    pub fn nb_layers(&self) -> u32 {
        // SAFETY: raw-place projection copies one initialized scalar and forms
        // no reference to C-visible storage.
        unsafe { addr_of!((*self.as_ptr()).nb_layers).read() }
    }

    /// Field: AVIAMFAudioElement.layers
    ///
    /// Borrows the complete parent-owned layer pointer container.
    #[must_use]
    pub fn layers(&self) -> AVIAMFLayers<'a> {
        // SAFETY: raw-place projection copies the container pointer only.
        let data = unsafe { addr_of!((*self.as_ptr()).layers).read() };
        AVIAMFLayers {
            data: NonNull::new(data),
            len: self.nb_layers() as usize,
            _borrow: PhantomData,
        }
    }
}

scalar_field!(
    /// Field: AVIAMFAudioElement.default_w
    AVIAMFAudioElementRef,
    AVIAMFAudioElementMut,
    default_w,
    set_default_w,
    u32
);

enum_field!(
    /// Field: AVIAMFAudioElement.audio_element_type
    AVIAMFAudioElementRef,
    AVIAMFAudioElementMut,
    audio_element_type,
    set_audio_element_type,
    audio_element_type,
    AVIAMFAudioElementType
);

impl AVIAMFAudioElementMut<'_> {
    /// Exclusively borrows the optional reconstruction-gain definition.
    #[must_use]
    pub fn recon_gain_info_mut(&mut self) -> Option<AVIAMFParamDefinitionMut<'_>> {
        // SAFETY: raw projection copies the nullable pointer from an exclusive
        // parent handle; a non-null result is uniquely reborrowed from it.
        let definition = unsafe { addr_of!((*self.as_mut_ptr()).recon_gain_info).read() };
        // SAFETY: the exclusive parent reborrow prevents competing access and
        // the parent owns the pointee for the returned handle's lifetime.
        unsafe { AVIAMFParamDefinitionMut::from_ptr(definition) }
    }

    /// Exclusively borrows the optional demixing definition.
    #[must_use]
    pub fn demixing_info_mut(&mut self) -> Option<AVIAMFParamDefinitionMut<'_>> {
        // SAFETY: raw projection copies the nullable pointer from an exclusive
        // parent handle; a non-null result is uniquely reborrowed from it.
        let definition = unsafe { addr_of!((*self.as_mut_ptr()).demixing_info).read() };
        // SAFETY: the exclusive parent reborrow prevents competing access and
        // the parent owns the pointee for the returned handle's lifetime.
        unsafe { AVIAMFParamDefinitionMut::from_ptr(definition) }
    }

    /// Replaces the owned reconstruction-gain definition, dropping the old
    /// value. A mismatched definition is returned unchanged.
    pub fn set_recon_gain_info(
        &mut self,
        definition: Option<CBox<AVIAMFParamDefinition>>,
    ) -> Result<(), CBox<AVIAMFParamDefinition>> {
        if definition.as_ref().is_some_and(|definition| {
            definition.as_ref().parameter_type() != AVIAMFParamDefinitionType::RECON_GAIN
        }) {
            return Err(definition.expect("the checked option is populated"));
        }
        let replacement = definition.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot. The replacement
        // transfers one compatible owner to the parent, and the prior pointer
        // is cleared from the field before being adopted and dropped once.
        let old = unsafe {
            let slot = addr_of_mut!((*self.as_mut_ptr()).recon_gain_info);
            let old = slot.read();
            slot.write(replacement);
            CBox::<AVIAMFParamDefinition>::from_raw(old)
        };
        drop(old);
        Ok(())
    }

    /// Replaces the owned demixing definition, dropping the old value. A
    /// mismatched definition is returned unchanged.
    pub fn set_demixing_info(
        &mut self,
        definition: Option<CBox<AVIAMFParamDefinition>>,
    ) -> Result<(), CBox<AVIAMFParamDefinition>> {
        if definition.as_ref().is_some_and(|definition| {
            definition.as_ref().parameter_type() != AVIAMFParamDefinitionType::DEMIXING
        }) {
            return Err(definition.expect("the checked option is populated"));
        }
        let replacement = definition.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: as `set_recon_gain_info`, for the demixing owner slot.
        let old = unsafe {
            let slot = addr_of_mut!((*self.as_mut_ptr()).demixing_info);
            let old = slot.read();
            slot.write(replacement);
            CBox::<AVIAMFParamDefinition>::from_raw(old)
        };
        drop(old);
        Ok(())
    }

    /// Removes and returns the owned reconstruction-gain definition.
    #[must_use]
    pub fn take_recon_gain_info(&mut self) -> Option<CBox<AVIAMFParamDefinition>> {
        // SAFETY: the exclusive handle owns the slot. Clearing it before
        // adoption transfers the unique pointee owner out of the parent.
        unsafe {
            let slot = addr_of_mut!((*self.as_mut_ptr()).recon_gain_info);
            let definition = slot.read();
            slot.write(core::ptr::null_mut());
            CBox::from_raw(definition)
        }
    }

    /// Removes and returns the owned demixing definition.
    #[must_use]
    pub fn take_demixing_info(&mut self) -> Option<CBox<AVIAMFParamDefinition>> {
        // SAFETY: as `take_recon_gain_info`, for the demixing owner slot.
        unsafe {
            let slot = addr_of_mut!((*self.as_mut_ptr()).demixing_info);
            let definition = slot.read();
            slot.write(core::ptr::null_mut());
            CBox::from_raw(definition)
        }
    }

    /// Exclusively borrows one parent-owned layer.
    #[must_use]
    pub fn layer_mut(&mut self, index: usize) -> Option<AVIAMFLayerMut<'_>> {
        let len = self.as_ref().nb_layers() as usize;
        if index >= len {
            return None;
        }
        // SAFETY: raw-place projection copies the container pointer only.
        let data = NonNull::new(unsafe { addr_of!((*self.as_mut_ptr()).layers).read() })?;
        // SAFETY: the index is within the valid `len`-entry container. The
        // exclusive parent handle uniquely reborrows the counted live child.
        let layer = unsafe { data.as_ptr().add(index).read() };
        // SAFETY: the non-null child remains owned by the exclusively borrowed
        // parent for the returned handle's lifetime.
        unsafe { AVIAMFLayerMut::from_ptr(layer) }
    }
}

#[cfg(test)]
mod audio_element_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    fn test_class() -> ffi::AVClass {
        // SAFETY: all callback pointers are optional and every all-zero scalar
        // value is representable in this synthetic immutable class record.
        unsafe { core::mem::zeroed() }
    }

    fn definition(kind: AVIAMFParamDefinitionType) -> CBox<AVIAMFParamDefinition> {
        // SAFETY: this requests a finite header-sized allocation from the
        // allocator paired with `AVIAMFParamDefinition`'s drop implementation.
        let pointer = unsafe { ffi::av_malloc(size_of::<ffi::AVIAMFParamDefinition>()) }
            .cast::<ffi::AVIAMFParamDefinition>();
        assert!(!pointer.is_null());
        // SAFETY: the allocation is suitably aligned and large enough, and
        // this write initializes every field before ownership is adopted.
        unsafe {
            pointer.write(ffi::AVIAMFParamDefinition {
                av_class: core::ptr::null(),
                subblocks_offset: size_of::<ffi::AVIAMFParamDefinition>(),
                subblock_size: 0,
                nb_subblocks: 0,
                type_: kind.as_raw(),
                parameter_id: 0,
                parameter_rate: 0,
                duration: 0,
                constant_subblock_duration: 0,
            });
            CBox::from_raw(pointer).expect("av_malloc returned non-null")
        }
    }

    #[test]
    fn layout_scalars_and_layer_views_match_c() {
        assert_eq!(
            size_of::<AVIAMFAudioElement>(),
            size_of::<ffi::AVIAMFAudioElement>()
        );
        assert_eq!(
            align_of::<AVIAMFAudioElement>(),
            align_of::<ffi::AVIAMFAudioElement>()
        );

        let class = test_class();
        // SAFETY: the all-zero layers have valid scalar representations and no
        // owned allocations. They remain live throughout the parent views.
        let mut first = unsafe { core::mem::zeroed::<ffi::AVIAMFLayer>() };
        // SAFETY: as `first`, for an independent second layer.
        let mut second = unsafe { core::mem::zeroed::<ffi::AVIAMFLayer>() };
        first.flags = 1;
        second.flags = 2;
        let mut layer_pointers = [addr_of_mut!(first), addr_of_mut!(second)];
        let mut raw = ffi::AVIAMFAudioElement {
            av_class: addr_of!(class),
            layers: layer_pointers.as_mut_ptr(),
            nb_layers: 2,
            demixing_info: core::ptr::null_mut(),
            recon_gain_info: core::ptr::null_mut(),
            audio_element_type: AVIAMFAudioElementType::CHANNEL.as_raw(),
            default_w: 7,
        };

        // SAFETY: the parent, class, container, and children remain live, and
        // this is the only access path while the exclusive handle exists.
        let mut element = unsafe { AVIAMFAudioElementMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(element.as_ref().av_class().as_ptr(), addr_of!(class));
        assert_eq!(element.as_ref().default_w(), 7);
        assert_eq!(
            element.as_ref().audio_element_type(),
            AVIAMFAudioElementType::CHANNEL
        );
        assert_eq!(element.as_ref().layers().len(), 2);
        assert_eq!(element.as_ref().layers().get(1).unwrap().flags(), 2);
        assert!(element.as_ref().layers().get(2).is_none());
        element.layer_mut(0).unwrap().set_flags(9);
        element.set_default_w(11);
        element.set_audio_element_type(AVIAMFAudioElementType::SCENE);
        assert_eq!(element.as_ref().layers().get(0).unwrap().flags(), 9);
        assert_eq!(element.as_ref().default_w(), 11);
        assert_eq!(
            element.as_ref().audio_element_type(),
            AVIAMFAudioElementType::SCENE
        );
    }

    #[test]
    fn owned_definition_fields_validate_replace_and_take() {
        let class = test_class();
        let mut raw = ffi::AVIAMFAudioElement {
            av_class: addr_of!(class),
            layers: core::ptr::null_mut(),
            nb_layers: 0,
            demixing_info: core::ptr::null_mut(),
            recon_gain_info: core::ptr::null_mut(),
            audio_element_type: AVIAMFAudioElementType::CHANNEL.as_raw(),
            default_w: 0,
        };
        // SAFETY: `raw` is initialized and exclusively accessed through this
        // handle. Every transferred child is taken back before the stack
        // parent leaves scope.
        let mut element = unsafe { AVIAMFAudioElementMut::from_ptr(addr_of_mut!(raw)) }.unwrap();

        let wrong = definition(AVIAMFParamDefinitionType::DEMIXING);
        let wrong = element.set_recon_gain_info(Some(wrong)).unwrap_err();
        assert!(element.as_ref().recon_gain_info().is_none());
        drop(wrong);

        element
            .set_demixing_info(Some(definition(AVIAMFParamDefinitionType::DEMIXING)))
            .unwrap();
        element
            .set_recon_gain_info(Some(definition(AVIAMFParamDefinitionType::RECON_GAIN)))
            .unwrap();
        assert_eq!(
            element.as_ref().demixing_info().unwrap().parameter_type(),
            AVIAMFParamDefinitionType::DEMIXING
        );
        assert_eq!(
            element
                .recon_gain_info_mut()
                .unwrap()
                .as_ref()
                .parameter_type(),
            AVIAMFParamDefinitionType::RECON_GAIN
        );
        drop(element.take_demixing_info().unwrap());
        drop(element.take_recon_gain_info().unwrap());
        assert!(element.as_ref().demixing_info().is_none());
        assert!(element.as_ref().recon_gain_info().is_none());
    }

    #[test]
    fn cbox_uses_the_published_audio_element_destructor() {
        let class = test_class();
        // SAFETY: request one correctly aligned audio-element allocation.
        let pointer = unsafe { ffi::av_malloc(size_of::<ffi::AVIAMFAudioElement>()) }
            .cast::<ffi::AVIAMFAudioElement>();
        assert!(!pointer.is_null());
        // SAFETY: initialize every field according to the destructor's empty
        // element contract, then transfer the allocation exactly once.
        let owner = unsafe {
            pointer.write(ffi::AVIAMFAudioElement {
                av_class: addr_of!(class),
                layers: core::ptr::null_mut(),
                nb_layers: 0,
                demixing_info: core::ptr::null_mut(),
                recon_gain_info: core::ptr::null_mut(),
                audio_element_type: AVIAMFAudioElementType::CHANNEL.as_raw(),
                default_w: 0,
            });
            CBox::<AVIAMFAudioElement>::from_raw(pointer).unwrap()
        };
        drop(owner);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVIAMFSubmix
    ///
    /// Layout-compatible view of a submix owned by an enclosing
    /// `AVIAMFMixPresentation`. Libavutil creates these values through
    /// `av_iamf_mix_presentation_add_submix` and releases the submix, its two
    /// pointer tables, every entry in them, and its optional output mix
    /// definition with that parent, so this type deliberately has no
    /// independent owning handle.
    ///
    /// A valid value carries non-null immutable static `AVClass` metadata; an
    /// `elements` / `layouts` table that is non-null whenever the matching
    /// `nb_elements` / `nb_layouts` count is nonzero and whose counted entries
    /// are distinct, non-null, live child allocations; and an
    /// `output_mix_config` that is null or a uniquely owned `av_malloc`-family
    /// mix-gain parameter definition. Only libavutil's add helpers may change
    /// a table or its count, so the accessors below expose the entries through
    /// lifetime-bound handles and never the container itself.
    AVIAMFSubmix,
    AVIAMFSubmixRef,
    AVIAMFSubmixMut,
    ffi::AVIAMFSubmix
);

macro_rules! submix_pointer_iterators {
    ($shared_iter:ident, $exclusive_iter:ident, $raw:ty, $shared:ident, $exclusive:ident) => {
        /// Iterator over objects owned indirectly by an IAMF submix.
        pub struct $shared_iter<'a> {
            table: *mut *mut $raw,
            index: usize,
            len: usize,
            _borrow: PhantomData<&'a AVIAMFSubmix>,
        }

        impl<'a> Iterator for $shared_iter<'a> {
            type Item = $shared<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == self.len {
                    return None;
                }
                let index = self.index;
                self.index += 1;
                // SAFETY: the submix invariant supplies `len` initialized
                // entries, each a non-null live object kept alive by the parent.
                let pointer = unsafe { self.table.add(index).read() };
                // SAFETY: validity, non-nullness, and lifetime follow from the
                // same constructor-established submix invariant.
                let handle = unsafe { $shared::from_ptr(pointer) };
                Some(handle.expect("submix pointer-array entries are non-null"))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.len - self.index;
                (remaining, Some(remaining))
            }
        }
        impl ExactSizeIterator for $shared_iter<'_> {}
        impl core::iter::FusedIterator for $shared_iter<'_> {}

        /// Exclusive iterator over distinct objects owned by an IAMF submix.
        pub struct $exclusive_iter<'a> {
            table: *mut *mut $raw,
            index: usize,
            len: usize,
            _borrow: PhantomData<&'a mut AVIAMFSubmix>,
        }

        impl<'a> Iterator for $exclusive_iter<'a> {
            type Item = $exclusive<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == self.len {
                    return None;
                }
                let index = self.index;
                self.index += 1;
                // SAFETY: libavutil constructs the table from distinct live
                // allocations and public callers must not modify the table.
                let pointer = unsafe { self.table.add(index).read() };
                // SAFETY: the iterator holds the sole parent borrow, and the
                // distinct-allocation invariant permits this mutable handle.
                let handle = unsafe { $exclusive::from_ptr(pointer) };
                Some(handle.expect("submix pointer-array entries are non-null"))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.len - self.index;
                (remaining, Some(remaining))
            }
        }
        impl ExactSizeIterator for $exclusive_iter<'_> {}
        impl core::iter::FusedIterator for $exclusive_iter<'_> {}
    };
}

submix_pointer_iterators!(
    AVIAMFSubmixElements,
    AVIAMFSubmixElementsMut,
    ffi::AVIAMFSubmixElement,
    AVIAMFSubmixElementRef,
    AVIAMFSubmixElementMut
);
submix_pointer_iterators!(
    AVIAMFSubmixLayouts,
    AVIAMFSubmixLayoutsMut,
    ffi::AVIAMFSubmixLayout,
    AVIAMFSubmixLayoutRef,
    AVIAMFSubmixLayoutMut
);

impl<'a> AVIAMFSubmixRef<'a> {
    /// Field: AVIAMFSubmix.av_class
    #[must_use]
    pub fn av_class(&self) -> AVClassRef<'a> {
        // SAFETY: the type invariant makes this copied pointer non-null static metadata.
        let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: the constructor-established invariant supplies validity and lifetime.
        unsafe { AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFSubmix has a non-null AVClass")
    }

    /// Field: AVIAMFSubmix.default_mix_gain
    #[must_use]
    pub fn default_mix_gain(&self) -> AVRationalRef<'a> {
        // SAFETY: the initialized inline rational remains live with the submix.
        unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).default_mix_gain).cast_mut()) }
            .expect("an embedded field address is non-null")
    }

    /// Field: AVIAMFSubmix.output_mix_config
    #[must_use]
    pub fn output_mix_config(&self) -> Option<AVIAMFParamDefinitionRef<'a>> {
        // SAFETY: a non-null copied pointer is initialized and parent-owned for `'a`.
        let definition = unsafe { addr_of!((*self.as_ptr()).output_mix_config).read() };
        // SAFETY: the field invariant supplies pointee validity and lifetime.
        unsafe { AVIAMFParamDefinitionRef::from_ptr(definition) }
    }

    /// Field: AVIAMFSubmix.nb_layouts
    #[must_use]
    pub fn nb_layouts(&self) -> usize {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).nb_layouts).read() as usize }
    }

    /// Field: AVIAMFSubmix.layouts
    #[must_use]
    pub fn layouts(&self) -> AVIAMFSubmixLayouts<'a> {
        let len = self.nb_layouts();
        // SAFETY: the field invariant makes the copied table non-null if non-empty.
        let table = unsafe { addr_of!((*self.as_ptr()).layouts).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty layout table is null"
        );
        AVIAMFSubmixLayouts {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }

    /// Field: AVIAMFSubmix.nb_elements
    #[must_use]
    pub fn nb_elements(&self) -> usize {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).nb_elements).read() as usize }
    }

    /// Field: AVIAMFSubmix.elements
    #[must_use]
    pub fn elements(&self) -> AVIAMFSubmixElements<'a> {
        let len = self.nb_elements();
        // SAFETY: the field invariant makes the copied table non-null if non-empty.
        let table = unsafe { addr_of!((*self.as_ptr()).elements).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty element table is null"
        );
        AVIAMFSubmixElements {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }
}

impl AVIAMFSubmixMut<'_> {
    /// Exclusively borrows the inline default gain rational.
    #[must_use]
    pub fn default_mix_gain_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: the exclusive submix reborrow supplies sole access to this field.
        unsafe { AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).default_mix_gain)) }
            .expect("an embedded field address is non-null")
    }

    /// Exclusively borrows the optional output mix definition.
    #[must_use]
    pub fn output_mix_config_mut(&mut self) -> Option<AVIAMFParamDefinitionMut<'_>> {
        // SAFETY: the copied pointer remains owned by the exclusively borrowed parent.
        let definition = unsafe { addr_of!((*self.as_mut_ptr()).output_mix_config).read() };
        // SAFETY: the field invariant and exclusive reborrow supply validity and lifetime.
        unsafe { AVIAMFParamDefinitionMut::from_ptr(definition) }
    }

    /// Replaces the owned output mix definition and returns the previous one.
    pub fn replace_output_mix_config(
        &mut self,
        definition: Option<CBox<AVIAMFParamDefinition>>,
    ) -> Result<Option<CBox<AVIAMFParamDefinition>>, CBox<AVIAMFParamDefinition>> {
        if definition.as_ref().is_some_and(|definition| {
            definition.as_ref().parameter_type() != AVIAMFParamDefinitionType::MIX_GAIN
        }) {
            return Err(definition.expect("the rejected definition is present"));
        }
        let definition = definition.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: exclusive access permits moving the new owner in and old owner out.
        let old =
            unsafe { addr_of_mut!((*self.as_mut_ptr()).output_mix_config).replace(definition) };
        // SAFETY: `old` is null or the unique allocation removed from this owner slot.
        Ok(unsafe { CBox::from_raw(old) })
    }

    /// Iterates the distinct parent-owned layouts with exclusive handles.
    #[must_use]
    pub fn layouts_mut(&mut self) -> AVIAMFSubmixLayoutsMut<'_> {
        let len = self.as_ref().nb_layouts();
        // SAFETY: the copied table belongs to this exclusively borrowed parent.
        let table = unsafe { addr_of!((*self.as_mut_ptr()).layouts).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty layout table is null"
        );
        AVIAMFSubmixLayoutsMut {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }

    /// Iterates the distinct parent-owned elements with exclusive handles.
    #[must_use]
    pub fn elements_mut(&mut self) -> AVIAMFSubmixElementsMut<'_> {
        let len = self.as_ref().nb_elements();
        // SAFETY: the copied table belongs to this exclusively borrowed parent.
        let table = unsafe { addr_of!((*self.as_mut_ptr()).elements).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty element table is null"
        );
        AVIAMFSubmixElementsMut {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }
}

#[cfg(test)]
mod submix_tests {
    use super::*;
    use core::mem::{align_of, size_of};

    fn test_class() -> ffi::AVClass {
        // SAFETY: every field is an integer, raw pointer, or optional callback.
        unsafe { core::mem::zeroed() }
    }

    fn mix_gain_definition() -> CBox<AVIAMFParamDefinition> {
        // SAFETY: this finite header size is valid for `av_malloc`, which
        // returns null or a fresh suitably aligned allocation.
        let pointer = unsafe { ffi::av_malloc(size_of::<ffi::AVIAMFParamDefinition>()) }
            .cast::<ffi::AVIAMFParamDefinition>();
        assert!(!pointer.is_null());
        // SAFETY: the fresh allocation is large and aligned for the complete
        // write, which initializes every header field before adoption.
        unsafe {
            pointer.write(ffi::AVIAMFParamDefinition {
                av_class: core::ptr::null(),
                subblocks_offset: size_of::<ffi::AVIAMFParamDefinition>(),
                subblock_size: size_of::<ffi::AVIAMFMixGain>(),
                nb_subblocks: 0,
                type_: AVIAMFParamDefinitionType::MIX_GAIN.as_raw(),
                parameter_id: 11,
                parameter_rate: 48_000,
                duration: 0,
                constant_subblock_duration: 0,
            });
        }
        // SAFETY: this is the initialized unique av_malloc allocation base
        // required by `AVIAMFParamDefinition`'s drop implementation.
        unsafe { CBox::from_raw(pointer) }.expect("allocation is non-null")
    }

    #[test]
    fn layout_matches_bindgen() {
        assert_eq!(size_of::<AVIAMFSubmix>(), size_of::<ffi::AVIAMFSubmix>());
        assert_eq!(align_of::<AVIAMFSubmix>(), align_of::<ffi::AVIAMFSubmix>());
    }

    #[test]
    fn pointer_tables_and_inline_gain_use_borrowed_handles() {
        let class = test_class();
        let mut element = ffi::AVIAMFSubmixElement {
            av_class: &class,
            audio_element_id: 7,
            element_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational { num: 0, den: 1 },
            headphones_rendering_mode: AVIAMFHeadphonesMode::STEREO.as_raw(),
            annotations: core::ptr::null_mut(),
        };
        // SAFETY: zero is the documented empty channel-layout state.
        let mut layout: ffi::AVIAMFSubmixLayout = unsafe { core::mem::zeroed() };
        layout.av_class = &class;
        let mut elements = [addr_of_mut!(element)];
        let mut layouts = [addr_of_mut!(layout)];
        let mut raw = ffi::AVIAMFSubmix {
            av_class: &class,
            elements: elements.as_mut_ptr(),
            nb_elements: 1,
            layouts: layouts.as_mut_ptr(),
            nb_layouts: 1,
            output_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational { num: -3, den: 2 },
        };
        // SAFETY: all storage and distinct children are initialized and exclusive.
        let mut submix = unsafe { AVIAMFSubmixMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(
            submix
                .as_ref()
                .elements()
                .next()
                .unwrap()
                .audio_element_id(),
            7
        );
        assert_eq!(submix.as_ref().layouts().count(), 1);
        assert_eq!(submix.as_ref().default_mix_gain().num(), -3);
        assert!(submix.as_ref().output_mix_config().is_none());
        assert!(
            submix
                .replace_output_mix_config(Some(mix_gain_definition()))
                .unwrap()
                .is_none()
        );
        assert_eq!(
            submix
                .as_ref()
                .output_mix_config()
                .unwrap()
                .parameter_type(),
            AVIAMFParamDefinitionType::MIX_GAIN
        );
        let definition = submix
            .replace_output_mix_config(None)
            .unwrap()
            .expect("the installed owner is returned");
        drop(definition);
        submix
            .elements_mut()
            .next()
            .unwrap()
            .set_audio_element_id(9);
        let mut gain = submix.default_mix_gain_mut();
        gain.set_num(5);
        gain.set_den(4);
        assert_eq!(
            submix
                .as_ref()
                .elements()
                .next()
                .unwrap()
                .audio_element_id(),
            9
        );
        assert_eq!(submix.as_ref().default_mix_gain().num(), 5);
    }

    #[test]
    fn empty_pointer_tables_may_be_null() {
        let class = test_class();
        let mut raw = ffi::AVIAMFSubmix {
            av_class: &class,
            elements: core::ptr::null_mut(),
            nb_elements: 0,
            layouts: core::ptr::null_mut(),
            nb_layouts: 0,
            output_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational { num: 0, den: 1 },
        };
        // SAFETY: zero-length null tables and the remaining initialized fields are valid.
        let submix = unsafe { AVIAMFSubmixRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(submix.elements().count(), 0);
        assert_eq!(submix.layouts().count(), 0);
    }

    #[test]
    fn shared_projections_outlive_the_temporary_handle_they_came_from() {
        let class = test_class();
        // SAFETY: zero is the documented empty channel-layout state.
        let mut layout: ffi::AVIAMFSubmixLayout = unsafe { core::mem::zeroed() };
        layout.av_class = &class;
        layout.layout_type = AVIAMFSubmixLayoutType::BINAURAL.as_raw();
        layout.true_peak = ffi::AVRational { num: -3, den: 1 };
        let mut layouts = [addr_of_mut!(layout)];
        let mut raw = ffi::AVIAMFSubmix {
            av_class: &class,
            elements: core::ptr::null_mut(),
            nb_elements: 0,
            layouts: layouts.as_mut_ptr(),
            nb_layouts: 1,
            output_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational { num: 1, den: 2 },
        };
        // SAFETY: the submix and its single distinct child are initialized and
        // reached only through this exclusive handle.
        let submix = unsafe { AVIAMFSubmixMut::from_ptr(addr_of_mut!(raw)) }.unwrap();

        // Each shared projection borrows the submix itself, not the temporary
        // `as_ref()` copy it was reached through, so all three outlive it.
        let gain = submix.as_ref().default_mix_gain();
        let class_handle = submix.as_ref().av_class();
        let first = submix.as_ref().layouts().next().expect("one layout");
        assert_eq!(gain.den(), 2);
        assert_eq!(class_handle.as_ptr(), addr_of!(class));
        assert_eq!(first.layout_type(), AVIAMFSubmixLayoutType::BINAURAL);
        assert_eq!(first.true_peak().num(), -3);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVIAMFMixPresentation
    ///
    /// Layout-compatible IAMF mix presentation. A valid value is allocated by
    /// libavutil, carries non-null immutable class metadata, and owns its
    /// optional annotations dictionary plus a pointer table of distinct owned
    /// submix allocations. Only libavutil's add operation may change the table
    /// or its count; borrowed handles expose its elements without exposing the
    /// container itself.
    AVIAMFMixPresentation,
    AVIAMFMixPresentationRef,
    AVIAMFMixPresentationMut,
    ffi::AVIAMFMixPresentation
);

// SAFETY: a value adopted into `CBox` must be the unique fully initialized
// allocation returned by `av_iamf_mix_presentation_alloc`, with only child
// owners accepted by the public IAMF API. Its matching destructor accepts an
// owning in/out slot, releases every child and container exactly once, releases
// the presentation allocation, and clears the slot.
unsafe impl CDropped for AVIAMFMixPresentation {
    unsafe fn c_drop(presentation: NonNull<Self>) {
        let mut pointer = presentation.as_ptr().cast::<ffi::AVIAMFMixPresentation>();
        // SAFETY: the trait contract transfers this unique, live presentation
        // to its matching destructor. The local pointer slot is writable and
        // remains live for the complete call.
        unsafe { ffi::av_iamf_mix_presentation_free(addr_of_mut!(pointer)) }
    }
}

/// Shared iterator over the submixes owned by an IAMF mix presentation.
pub struct AVIAMFSubmixes<'a> {
    table: *mut *mut ffi::AVIAMFSubmix,
    index: usize,
    len: usize,
    _borrow: PhantomData<&'a AVIAMFMixPresentation>,
}

impl<'a> Iterator for AVIAMFSubmixes<'a> {
    type Item = AVIAMFSubmixRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }
        let index = self.index;
        self.index += 1;
        // SAFETY: the presentation invariant supplies `len` initialized table
        // entries, each naming a non-null live submix owned by the parent.
        let submix = unsafe { self.table.add(index).read() };
        // SAFETY: the parent keeps the submix live for `'a`, and shared
        // iteration grants no write access to it or its pointer container.
        Some(
            unsafe { AVIAMFSubmixRef::from_ptr(submix) }
                .expect("presentation submix entries are non-null"),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AVIAMFSubmixes<'_> {}
impl core::iter::FusedIterator for AVIAMFSubmixes<'_> {}

/// Exclusive iterator over the distinct submixes owned by a presentation.
pub struct AVIAMFSubmixesMut<'a> {
    table: *mut *mut ffi::AVIAMFSubmix,
    index: usize,
    len: usize,
    _borrow: PhantomData<&'a mut AVIAMFMixPresentation>,
}

impl<'a> Iterator for AVIAMFSubmixesMut<'a> {
    type Item = AVIAMFSubmixMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }
        let index = self.index;
        self.index += 1;
        // SAFETY: the presentation invariant supplies distinct live submix
        // allocations in every counted entry, and the iterator owns the sole
        // presentation borrow for `'a`.
        let submix = unsafe { self.table.add(index).read() };
        // SAFETY: distinctness permits each yielded exclusive handle to remain
        // live while iteration proceeds to another entry.
        Some(
            unsafe { AVIAMFSubmixMut::from_ptr(submix) }
                .expect("presentation submix entries are non-null"),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AVIAMFSubmixesMut<'_> {}
impl core::iter::FusedIterator for AVIAMFSubmixesMut<'_> {}

impl<'a> AVIAMFMixPresentationRef<'a> {
    /// Field: AVIAMFMixPresentation.annotations
    ///
    /// Borrows the optional annotations dictionary owned by the presentation.
    #[must_use]
    pub fn annotations(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: raw-place projection copies the nullable owner pointer. A
        // non-null dictionary remains live with this presentation for `'a`.
        let annotations = unsafe { addr_of!((*self.as_ptr()).annotations).read() };
        // SAFETY: the presentation invariant supplies validity and lifetime;
        // the handle constructor maps null to `None`.
        unsafe { AVDictionaryRef::from_ptr(annotations) }
    }

    /// Field: AVIAMFMixPresentation.nb_submixes
    ///
    /// Returns the number of initialized pointers in the owned submix table.
    #[must_use]
    pub fn nb_submixes(&self) -> usize {
        // SAFETY: raw-place projection copies one initialized scalar without
        // forming a reference to C-visible storage.
        unsafe { addr_of!((*self.as_ptr()).nb_submixes).read() as usize }
    }

    /// Field: AVIAMFMixPresentation.submixes
    ///
    /// Iterates the submixes while keeping every result tied to this shared
    /// presentation borrow. The pointer container itself is never exposed.
    #[must_use]
    pub fn submixes(&self) -> AVIAMFSubmixes<'a> {
        let len = self.nb_submixes();
        // SAFETY: raw-place projection copies the table pointer. The type
        // invariant requires it to be non-null whenever `len` is nonzero.
        let table = unsafe { addr_of!((*self.as_ptr()).submixes).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty submix table is null"
        );
        AVIAMFSubmixes {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }

    /// Field: AVIAMFMixPresentation.av_class
    ///
    /// Returns the immutable process-lifetime option metadata installed by
    /// libavutil's constructor.
    #[must_use]
    pub fn av_class(&self) -> AVClassRef<'a> {
        // SAFETY: raw-place projection copies the pointer without forming a
        // reference. The presentation invariant makes it non-null and static.
        let class = unsafe { addr_of!((*self.as_ptr()).av_class).read() };
        // SAFETY: constructor-established non-nullness, immutability, and
        // static lifetime satisfy this shared handle's contract.
        unsafe { AVClassRef::from_ptr(class.cast_mut()) }
            .expect("AVIAMFMixPresentation has a non-null AVClass")
    }
}

impl AVIAMFMixPresentationMut<'_> {
    /// Replaces the owned annotations dictionary and returns the prior owner.
    pub fn replace_annotations(
        &mut self,
        annotations: Option<CBox<AVDictionary>>,
    ) -> Option<CBox<AVDictionary>> {
        let annotations = annotations.map_or(core::ptr::null_mut(), CBox::into_raw);
        // SAFETY: the exclusive presentation handle permits moving an owner
        // into this slot and moving its previous nullable owner out.
        let old = unsafe { addr_of_mut!((*self.as_mut_ptr()).annotations).replace(annotations) };
        // SAFETY: `old` is null or the unique live dictionary removed from the
        // presentation's owner slot, so it may be adopted exactly once.
        unsafe { CBox::from_raw(old) }
    }

    /// Iterates the distinct owned submixes through exclusive handles.
    #[must_use]
    pub fn submixes_mut(&mut self) -> AVIAMFSubmixesMut<'_> {
        let len = self.as_ref().nb_submixes();
        // SAFETY: raw-place projection copies the table pointer through the
        // sole presentation borrow. A non-empty table is non-null.
        let table = unsafe { addr_of!((*self.as_mut_ptr()).submixes).read() };
        assert!(
            len == 0 || !table.is_null(),
            "non-empty submix table is null"
        );
        AVIAMFSubmixesMut {
            table,
            index: 0,
            len,
            _borrow: PhantomData,
        }
    }
}

#[cfg(test)]
mod mix_presentation_tests {
    use super::*;
    use core::mem::{align_of, size_of};

    fn test_class() -> ffi::AVClass {
        // SAFETY: every field is an integer, raw pointer, or optional callback.
        // The inert class remains borrowed and is never invoked by these tests.
        unsafe { core::mem::zeroed() }
    }

    fn raw_submix(class: *const ffi::AVClass, numerator: i32) -> ffi::AVIAMFSubmix {
        ffi::AVIAMFSubmix {
            av_class: class,
            elements: core::ptr::null_mut(),
            nb_elements: 0,
            layouts: core::ptr::null_mut(),
            nb_layouts: 0,
            output_mix_config: core::ptr::null_mut(),
            default_mix_gain: ffi::AVRational {
                num: numerator,
                den: 1,
            },
        }
    }

    #[test]
    fn layout_matches_bindgen() {
        assert_eq!(
            size_of::<AVIAMFMixPresentation>(),
            size_of::<ffi::AVIAMFMixPresentation>()
        );
        assert_eq!(
            align_of::<AVIAMFMixPresentation>(),
            align_of::<ffi::AVIAMFMixPresentation>()
        );
    }

    #[test]
    fn fields_and_submixes_use_lifetime_bound_handles() {
        let class = test_class();
        let mut first = raw_submix(addr_of!(class), 1);
        let mut second = raw_submix(addr_of!(class), 2);
        let mut submixes = [addr_of_mut!(first), addr_of_mut!(second)];
        let mut raw = ffi::AVIAMFMixPresentation {
            av_class: addr_of!(class),
            submixes: submixes.as_mut_ptr(),
            nb_submixes: 2,
            annotations: core::ptr::null_mut(),
        };
        // SAFETY: the presentation and its two distinct children are fully
        // initialized and exclusively accessed through this handle.
        let mut presentation =
            unsafe { AVIAMFMixPresentationMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(presentation.as_ref().av_class().as_ptr(), addr_of!(class));
        assert!(presentation.as_ref().annotations().is_none());
        assert_eq!(presentation.as_ref().nb_submixes(), 2);
        assert_eq!(
            presentation
                .as_ref()
                .submixes()
                .map(|submix| submix.default_mix_gain().num())
                .sum::<i32>(),
            3
        );
        for (index, mut submix) in presentation.submixes_mut().enumerate() {
            submix.default_mix_gain_mut().set_num(index as i32 + 4);
        }
        assert_eq!(presentation.as_ref().submixes().count(), 2);
        assert_eq!(first.default_mix_gain.num, 4);
        assert_eq!(second.default_mix_gain.num, 5);
    }

    #[test]
    fn annotations_owner_can_be_installed_and_removed() {
        let class = test_class();
        let mut raw = ffi::AVIAMFMixPresentation {
            av_class: addr_of!(class),
            submixes: core::ptr::null_mut(),
            nb_submixes: 0,
            annotations: core::ptr::null_mut(),
        };
        // SAFETY: this is a fully initialized empty presentation accessed only
        // through the exclusive handle until the installed owner is removed.
        let mut presentation =
            unsafe { AVIAMFMixPresentationMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        let mut dictionary = crate::dict::Dictionary::default();
        crate::dict::av_dict_set(&mut dictionary, c"en", Some(c"mix"), 0).unwrap();
        assert!(
            presentation
                .replace_annotations(dictionary.into_owner())
                .is_none()
        );
        assert_eq!(
            crate::dict::av_dict_count(presentation.as_ref().annotations()),
            1
        );
        let dictionary = presentation.replace_annotations(None).unwrap();
        assert_eq!(crate::dict::av_dict_count(Some(dictionary.as_ref())), 1);
        drop(dictionary);
    }

    #[test]
    fn cbox_runs_the_published_destructor_over_a_libavutil_graph() {
        // SAFETY: the published constructor returns null or one fresh, fully
        // initialized presentation whose sole ownership transfers to the
        // caller, which is exactly what this owner adopts.
        let mut owner = unsafe {
            CBox::<AVIAMFMixPresentation>::from_raw(ffi::av_iamf_mix_presentation_alloc())
        }
        .expect("av_iamf_mix_presentation_alloc succeeds");
        assert_eq!(
            owner.as_ref().av_class().class_name(),
            Some(c"AVIAMFMixPresentation")
        );
        assert_eq!(owner.as_ref().nb_submixes(), 0);
        assert_eq!(owner.as_ref().submixes().count(), 0);

        // SAFETY: each call mutably borrows the live presentation for its own
        // duration only. This is the sole routine allowed to grow the owned
        // table, and the children it appends are released with the parent.
        let added = [
            unsafe { ffi::av_iamf_mix_presentation_add_submix(owner.as_ptr()) },
            unsafe { ffi::av_iamf_mix_presentation_add_submix(owner.as_ptr()) },
        ];
        assert!(added.iter().all(|submix| !submix.is_null()));

        // The wrapper's distinctness invariant is what makes the exclusive
        // iterator sound, so check it against the pointers libavutil stored.
        assert_eq!(owner.as_ref().nb_submixes(), 2);
        let mut submixes = owner.as_ref().submixes();
        let first = submixes.next().expect("the first added submix");
        let second = submixes.next().expect("the second added submix");
        assert!(submixes.next().is_none());
        assert_eq!(first.as_ptr(), added[0]);
        assert_eq!(second.as_ptr(), added[1]);
        assert_ne!(first.as_ptr(), second.as_ptr());

        for (index, mut submix) in owner.as_mut().submixes_mut().enumerate() {
            submix.default_mix_gain_mut().set_num(index as i32 + 1);
        }
        assert_eq!(
            owner
                .as_ref()
                .submixes()
                .map(|submix| submix.default_mix_gain().num())
                .sum::<i32>(),
            3
        );

        // Installing an owner in the annotations slot hands it to the C
        // destructor, which reaches it through this class's option table.
        let mut dictionary = crate::dict::Dictionary::default();
        crate::dict::av_dict_set(&mut dictionary, c"en", Some(c"mix"), 0).unwrap();
        assert!(
            owner
                .as_mut()
                .replace_annotations(dictionary.into_owner())
                .is_none()
        );
        assert_eq!(crate::dict::av_dict_count(owner.as_ref().annotations()), 1);

        // Releases the annotations dictionary, both submixes, the pointer
        // table, and the presentation allocation.
        drop(owner);
    }
}
