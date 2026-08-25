//! Wrappers for `libavutil/hdr_dynamic_metadata.c`.

use crate::ffi;

/// Wraps: AVHDRPlusOverlapProcessOption
///
/// Selects how overlapping HDR10+ processing windows are combined. The
/// integer newtype remains ABI-compatible and preserves unknown values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVHDRPlusOverlapProcessOption(ffi::AVHDRPlusOverlapProcessOption);

impl AVHDRPlusOverlapProcessOption {
    /// Blend contributions from every overlapping processing window.
    pub const WEIGHTED_AVERAGING: Self =
        Self(ffi::AVHDRPlusOverlapProcessOption_AV_HDR_PLUS_OVERLAP_PROCESS_WEIGHTED_AVERAGING);
    /// Apply overlapping processing windows in layers.
    pub const LAYERING: Self =
        Self(ffi::AVHDRPlusOverlapProcessOption_AV_HDR_PLUS_OVERLAP_PROCESS_LAYERING);

    /// Wraps a raw C value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(value: ffi::AVHDRPlusOverlapProcessOption) -> Self {
        Self(value)
    }

    /// Returns the raw value used by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVHDRPlusOverlapProcessOption {
        self.0
    }
}

impl Default for AVHDRPlusOverlapProcessOption {
    fn default() -> Self {
        Self::WEIGHTED_AVERAGING
    }
}

impl From<ffi::AVHDRPlusOverlapProcessOption> for AVHDRPlusOverlapProcessOption {
    fn from(value: ffi::AVHDRPlusOverlapProcessOption) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVHDRPlusOverlapProcessOption> for ffi::AVHDRPlusOverlapProcessOption {
    fn from(value: AVHDRPlusOverlapProcessOption) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod overlap_option_tests {
    use super::*;

    #[test]
    fn overlap_options_and_unknown_values_round_trip() {
        assert_eq!(
            AVHDRPlusOverlapProcessOption::default(),
            AVHDRPlusOverlapProcessOption::WEIGHTED_AVERAGING
        );
        assert_eq!(AVHDRPlusOverlapProcessOption::LAYERING.as_raw(), 1);
        assert_eq!(AVHDRPlusOverlapProcessOption::from_raw(37).as_raw(), 37);
    }
}

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CDropped, CSlice, CSliceMut, CVal, CValued};

use crate::rational::{AVRationalMut, AVRationalRef};

ffibox::define_ctype!(
    /// Wraps: AVDynamicHDRSmpte2094App5
    ///
    /// ABI-compatible SMPTE ST 2094-50 dynamic HDR metadata. The structure is
    /// plain by-value storage: it contains no pointers or other resources.
    AVDynamicHDRSmpte2094App5,
    AVDynamicHDRSmpte2094App5Ref,
    AVDynamicHDRSmpte2094App5Mut,
    ffi::AVDynamicHDRSmpte2094App5
);

// SAFETY: the C structure contains only integer scalars and fixed-size integer
// arrays, so disposing an inline value requires no action.
unsafe impl CValued for AVDynamicHDRSmpte2094App5 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDynamicHDRSmpte2094App5 {
    /// Creates zero-initialized metadata in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! scalar_field {
    ($(#[$meta:meta])* $field:ident, $setter:ident: $ty:ty) => {
        impl AVDynamicHDRSmpte2094App5Ref<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> $ty {
                // SAFETY: the handle keeps a live initialized metadata object;
                // this raw-place projection copies the selected integer field
                // without forming a reference to C-owned storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVDynamicHDRSmpte2094App5Mut<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](AVDynamicHDRSmpte2094App5Ref::", stringify!($field), ").")]
            pub fn $setter(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance to a
                // live metadata object; this raw-place projection writes only
                // the selected integer field and forms no reference to it.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! array_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident: $element:ty, $len:expr) => {
        impl<'a> AVDynamicHDRSmpte2094App5Ref<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> CSlice<'a, $element> {
                // SAFETY: the projected fixed-size C array contains exactly
                // the declared number of initialized elements and lives for
                // the metadata handle's lifetime. `CSlice` forms no reference
                // to that storage and only copies plain elements out.
                unsafe {
                    let pointer = addr_of!((*self.as_ptr()).$field)
                        .cast::<$element>()
                        .cast_mut();
                    CSlice::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }

        impl AVDynamicHDRSmpte2094App5Mut<'_> {
            #[doc = concat!("Returns exclusive element-wise access to [`", stringify!($field), "`](AVDynamicHDRSmpte2094App5Ref::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> CSliceMut<'_, $element> {
                // SAFETY: the projected fixed-size C array contains exactly
                // the declared number of initialized elements and the mutable
                // handle provides exclusive access for the returned view's
                // lifetime.
                unsafe {
                    let pointer = addr_of_mut!((*self.as_mut_ptr()).$field).cast::<$element>();
                    CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }
    };
}

scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.application_version
    application_version,
    set_application_version: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.minimum_application_version
    minimum_application_version,
    set_minimum_application_version: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.has_custom_hdr_reference_white_flag
    has_custom_hdr_reference_white_flag,
    set_has_custom_hdr_reference_white_flag: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.has_adaptive_tone_map_flag
    has_adaptive_tone_map_flag,
    set_has_adaptive_tone_map_flag: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.hdr_reference_white
    hdr_reference_white,
    set_hdr_reference_white: u16
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.baseline_hdr_headroom
    baseline_hdr_headroom,
    set_baseline_hdr_headroom: u16
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.use_reference_white_tone_mapping_flag
    use_reference_white_tone_mapping_flag,
    set_use_reference_white_tone_mapping_flag: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.num_alternate_images
    num_alternate_images,
    set_num_alternate_images: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_application_space_chromaticities_flag
    gain_application_space_chromaticities_flag,
    set_gain_application_space_chromaticities_flag: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.has_common_component_mix_params_flag
    has_common_component_mix_params_flag,
    set_has_common_component_mix_params_flag: u8
);
scalar_field!(
    /// Field: AVDynamicHDRSmpte2094App5.has_common_curve_params_flag
    has_common_curve_params_flag,
    set_has_common_curve_params_flag: u8
);

array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_application_space_chromaticities
    gain_application_space_chromaticities,
    gain_application_space_chromaticities_mut: u16,
    8
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.alternate_hdr_headrooms
    alternate_hdr_headrooms,
    alternate_hdr_headrooms_mut: u16,
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.component_mixing_type
    component_mixing_type,
    component_mixing_type_mut: u8,
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.has_component_mixing_coefficient_flag
    has_component_mixing_coefficient_flag,
    has_component_mixing_coefficient_flag_mut: [u8; 6],
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.component_mixing_coefficient
    component_mixing_coefficient,
    component_mixing_coefficient_mut: [u16; 6],
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_curve_num_control_points_minus_1
    gain_curve_num_control_points_minus_1,
    gain_curve_num_control_points_minus_1_mut: u8,
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_curve_use_pchip_slope_flag
    gain_curve_use_pchip_slope_flag,
    gain_curve_use_pchip_slope_flag_mut: u8,
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_curve_control_points_x
    gain_curve_control_points_x,
    gain_curve_control_points_x_mut: [u16; 32],
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_curve_control_points_y
    gain_curve_control_points_y,
    gain_curve_control_points_y_mut: [u16; 32],
    4
);
array_field!(
    /// Field: AVDynamicHDRSmpte2094App5.gain_curve_control_points_theta
    gain_curve_control_points_theta,
    gain_curve_control_points_theta_mut: [u16; 32],
    4
);

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_ffi() {
        assert_eq!(
            size_of::<AVDynamicHDRSmpte2094App5>(),
            size_of::<ffi::AVDynamicHDRSmpte2094App5>()
        );
        assert_eq!(
            align_of::<AVDynamicHDRSmpte2094App5>(),
            align_of::<ffi::AVDynamicHDRSmpte2094App5>()
        );
    }

    #[test]
    fn scalar_fields_round_trip() {
        let mut metadata = AVDynamicHDRSmpte2094App5::new();
        let mut view = metadata.as_mut();
        view.set_application_version(1);
        view.set_minimum_application_version(2);
        view.set_has_custom_hdr_reference_white_flag(3);
        view.set_has_adaptive_tone_map_flag(4);
        view.set_hdr_reference_white(5);
        view.set_baseline_hdr_headroom(6);
        view.set_use_reference_white_tone_mapping_flag(7);
        view.set_num_alternate_images(8);
        view.set_gain_application_space_chromaticities_flag(9);
        view.set_has_common_component_mix_params_flag(10);
        view.set_has_common_curve_params_flag(11);

        let view = metadata.as_ref();
        assert_eq!(view.application_version(), 1);
        assert_eq!(view.minimum_application_version(), 2);
        assert_eq!(view.has_custom_hdr_reference_white_flag(), 3);
        assert_eq!(view.has_adaptive_tone_map_flag(), 4);
        assert_eq!(view.hdr_reference_white(), 5);
        assert_eq!(view.baseline_hdr_headroom(), 6);
        assert_eq!(view.use_reference_white_tone_mapping_flag(), 7);
        assert_eq!(view.num_alternate_images(), 8);
        assert_eq!(view.gain_application_space_chromaticities_flag(), 9);
        assert_eq!(view.has_common_component_mix_params_flag(), 10);
        assert_eq!(view.has_common_curve_params_flag(), 11);
    }

    #[test]
    fn array_fields_use_bounded_copying_views() {
        let mut metadata = AVDynamicHDRSmpte2094App5::new();
        let mut view = metadata.as_mut();

        assert!(
            view.gain_application_space_chromaticities_mut()
                .copy_from_slice(&[1; 8])
        );
        assert!(view.alternate_hdr_headrooms_mut().copy_from_slice(&[2; 4]));
        assert!(view.component_mixing_type_mut().copy_from_slice(&[3; 4]));
        assert!(
            view.has_component_mixing_coefficient_flag_mut()
                .set_elem(1, [4; 6])
        );
        assert!(view.component_mixing_coefficient_mut().set_elem(2, [5; 6]));
        assert!(
            view.gain_curve_num_control_points_minus_1_mut()
                .copy_from_slice(&[6; 4])
        );
        assert!(
            view.gain_curve_use_pchip_slope_flag_mut()
                .copy_from_slice(&[7; 4])
        );
        assert!(view.gain_curve_control_points_x_mut().set_elem(0, [8; 32]));
        assert!(view.gain_curve_control_points_y_mut().set_elem(1, [9; 32]));
        assert!(
            view.gain_curve_control_points_theta_mut()
                .set_elem(3, [10; 32])
        );

        let view = metadata.as_ref();
        assert_eq!(view.gain_application_space_chromaticities().len(), 8);
        assert_eq!(
            view.gain_application_space_chromaticities().elem(7),
            Some(1)
        );
        assert_eq!(view.alternate_hdr_headrooms().elem(3), Some(2));
        assert_eq!(view.component_mixing_type().elem(0), Some(3));
        assert_eq!(
            view.has_component_mixing_coefficient_flag().elem(1),
            Some([4; 6])
        );
        assert_eq!(view.component_mixing_coefficient().elem(2), Some([5; 6]));
        assert_eq!(
            view.gain_curve_num_control_points_minus_1().elem(3),
            Some(6)
        );
        assert_eq!(view.gain_curve_use_pchip_slope_flag().elem(2), Some(7));
        assert_eq!(view.gain_curve_control_points_x().elem(0), Some([8; 32]));
        assert_eq!(view.gain_curve_control_points_y().elem(1), Some([9; 32]));
        assert_eq!(
            view.gain_curve_control_points_theta().elem(3),
            Some([10; 32])
        );
        assert_eq!(view.gain_curve_control_points_theta().elem(4), None);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVHDRPlusPercentile
    ///
    /// ABI-compatible percentile entry embedded in HDR10+ metadata. It is
    /// plain by-value storage and owns no resources.
    AVHDRPlusPercentile,
    AVHDRPlusPercentileRef,
    AVHDRPlusPercentileMut,
    ffi::AVHDRPlusPercentile
);

// SAFETY: this C structure contains only an integer and an inline
// `AVRational`; neither field has a teardown operation.
unsafe impl CValued for AVHDRPlusPercentile {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVHDRPlusPercentile {
    /// Creates a zero-initialized percentile entry in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

impl<'a> AVHDRPlusPercentileRef<'a> {
    /// Field: AVHDRPlusPercentile.percentile
    ///
    /// Borrows the inline linearized maxRGB value.
    #[must_use]
    pub fn percentile(&self) -> AVRationalRef<'a> {
        // SAFETY: the projected field is an initialized inline `AVRational`
        // that remains live for the enclosing handle's lifetime.
        unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).percentile).cast_mut()) }
            .expect("an inline field is non-null")
    }

    /// Field: AVHDRPlusPercentile.percentage
    ///
    /// Returns the percentage corresponding to the percentile.
    #[must_use]
    pub fn percentage(&self) -> u8 {
        // SAFETY: the handle keeps the object initialized and live; raw-place
        // projection copies the integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).percentage).read() }
    }
}

impl AVHDRPlusPercentileMut<'_> {
    /// Exclusively borrows the inline linearized maxRGB value.
    #[must_use]
    pub fn percentile_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to
        // the projected initialized inline field for this reborrow.
        unsafe { AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).percentile)) }
            .expect("an inline field is non-null")
    }

    /// Sets the percentage corresponding to the percentile.
    pub fn set_percentage(&mut self, value: u8) {
        // SAFETY: the exclusive handle permits writing this integer field and
        // the raw-place projection forms no reference to C-visible storage.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).percentage).write(value) }
    }
}

#[cfg(test)]
mod percentile_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_fields_match_c() {
        assert_eq!(
            size_of::<AVHDRPlusPercentile>(),
            size_of::<ffi::AVHDRPlusPercentile>()
        );
        assert_eq!(
            align_of::<AVHDRPlusPercentile>(),
            align_of::<ffi::AVHDRPlusPercentile>()
        );

        let mut entry = AVHDRPlusPercentile::new();
        let mut view = entry.as_mut();
        view.set_percentage(73);
        let mut percentile = view.percentile_mut();
        percentile.set_num(12_345);
        percentile.set_den(100_000);

        let view = entry.as_ref();
        assert_eq!(view.percentage(), 73);
        assert_eq!(view.percentile().num(), 12_345);
        assert_eq!(view.percentile().den(), 100_000);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVHDRPlusColorTransformParams
    ///
    /// ABI-compatible HDR10+ color-transform parameters for one processing
    /// window. All storage is inline and contains no owned resources.
    AVHDRPlusColorTransformParams,
    AVHDRPlusColorTransformParamsRef,
    AVHDRPlusColorTransformParamsMut,
    ffi::AVHDRPlusColorTransformParams
);

// SAFETY: the structure consists solely of integer scalars, an integer enum,
// and fixed arrays of by-value structures with no teardown operations.
unsafe impl CValued for AVHDRPlusColorTransformParams {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVHDRPlusColorTransformParams {
    /// Number of entries in the fixed maxRGB-percentile table.
    pub const MAX_DISTRIBUTION_MAXRGB_PERCENTILES: usize = 15;
    /// Number of entries in the fixed Bezier-anchor table.
    pub const MAX_BEZIER_CURVE_ANCHORS: usize = 15;

    /// Creates zero-initialized parameters in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! hdr_plus_scalar_field {
    ($(#[$meta:meta])* $field:ident, $setter:ident: $ty:ty) => {
        impl AVHDRPlusColorTransformParamsRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> $ty {
                // SAFETY: the shared handle keeps initialized parameters live;
                // raw-place projection copies the scalar without forming a
                // reference to the C-visible object or field.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVHDRPlusColorTransformParamsMut<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](AVHDRPlusColorTransformParamsRef::", stringify!($field), ").")]
            pub fn $setter(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance to
                // initialized parameters; raw-place projection writes only
                // the selected scalar and forms no reference to it.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! hdr_plus_count_field {
    ($(#[$meta:meta])* $field:ident, $setter:ident, $limit:expr, $message:literal) => {
        impl AVHDRPlusColorTransformParamsRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> u8 {
                // SAFETY: the shared handle keeps initialized parameters live;
                // raw-place projection copies the count without forming a
                // reference to the C-visible object or field.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVHDRPlusColorTransformParamsMut<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](AVHDRPlusColorTransformParamsRef::", stringify!($field), ").")]
            ///
            /// # Panics
            ///
            #[doc = $message]
            pub fn $setter(&mut self, value: u8) {
                assert!(usize::from(value) <= $limit, $message);
                // SAFETY: the exclusive handle supplies write provenance to
                // initialized parameters, and the validated count cannot make
                // a C consumer overrun the fixed array it bounds.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! hdr_plus_rational_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVHDRPlusColorTransformParamsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> AVRationalRef<'a> {
                // SAFETY: the projected inline rational is initialized and
                // remains live for the enclosing parameters handle's lifetime.
                unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).$field).cast_mut()) }
                    .expect("an inline field is non-null")
            }
        }

        impl AVHDRPlusColorTransformParamsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVHDRPlusColorTransformParamsRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> AVRationalMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to this initialized inline rational for the
                // duration of the returned reborrow.
                unsafe { AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$field)) }
                    .expect("an inline field is non-null")
            }
        }
    };
}

macro_rules! hdr_plus_array_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident: $element:ty, $len:expr) => {
        impl<'a> AVHDRPlusColorTransformParamsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> CSlice<'a, $element> {
                // SAFETY: raw-place projection locates the fixed initialized
                // array without forming a reference. Its `$len` elements live
                // for the enclosing parameters handle's lifetime.
                unsafe {
                    let pointer = addr_of!((*self.as_ptr()).$field)
                        .cast::<$element>()
                        .cast_mut();
                    CSlice::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }

        impl AVHDRPlusColorTransformParamsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVHDRPlusColorTransformParamsRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> CSliceMut<'_, $element> {
                // SAFETY: the exclusive handle supplies write provenance to
                // every element of the fixed initialized array, and the view
                // remains bound to this mutable reborrow.
                unsafe {
                    let pointer = addr_of_mut!((*self.as_mut_ptr()).$field).cast::<$element>();
                    CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }
    };
}

hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.color_saturation_mapping_flag
    color_saturation_mapping_flag,
    set_color_saturation_mapping_flag: u8
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.average_maxrgb
    average_maxrgb,
    average_maxrgb_mut
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.color_saturation_weight
    color_saturation_weight,
    color_saturation_weight_mut
);
hdr_plus_array_field!(
    /// Field: AVHDRPlusColorTransformParams.bezier_curve_anchors
    bezier_curve_anchors,
    bezier_curve_anchors_mut: crate::rational::AVRational,
    AVHDRPlusColorTransformParams::MAX_BEZIER_CURVE_ANCHORS
);
hdr_plus_count_field!(
    /// Field: AVHDRPlusColorTransformParams.num_bezier_curve_anchors
    num_bezier_curve_anchors,
    set_num_bezier_curve_anchors,
    AVHDRPlusColorTransformParams::MAX_BEZIER_CURVE_ANCHORS,
    "num_bezier_curve_anchors must not exceed 15"
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.knee_point_y
    knee_point_y,
    knee_point_y_mut
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.knee_point_x
    knee_point_x,
    knee_point_x_mut
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.tone_mapping_flag
    tone_mapping_flag,
    set_tone_mapping_flag: u8
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.fraction_bright_pixels
    fraction_bright_pixels,
    fraction_bright_pixels_mut
);
hdr_plus_array_field!(
    /// Field: AVHDRPlusColorTransformParams.distribution_maxrgb
    distribution_maxrgb,
    distribution_maxrgb_mut: AVHDRPlusPercentile,
    AVHDRPlusColorTransformParams::MAX_DISTRIBUTION_MAXRGB_PERCENTILES
);
hdr_plus_count_field!(
    /// Field: AVHDRPlusColorTransformParams.num_distribution_maxrgb_percentiles
    num_distribution_maxrgb_percentiles,
    set_num_distribution_maxrgb_percentiles,
    AVHDRPlusColorTransformParams::MAX_DISTRIBUTION_MAXRGB_PERCENTILES,
    "num_distribution_maxrgb_percentiles must not exceed 15"
);
hdr_plus_array_field!(
    /// Field: AVHDRPlusColorTransformParams.maxscl
    maxscl,
    maxscl_mut: crate::rational::AVRational,
    3
);

impl AVHDRPlusColorTransformParamsRef<'_> {
    /// Field: AVHDRPlusColorTransformParams.overlap_process_option
    #[must_use]
    pub fn overlap_process_option(&self) -> AVHDRPlusOverlapProcessOption {
        // SAFETY: the shared handle keeps initialized parameters live;
        // raw-place projection copies the integer enum representation without
        // forming a reference to C-visible storage.
        let raw = unsafe { addr_of!((*self.as_ptr()).overlap_process_option).read() };
        AVHDRPlusOverlapProcessOption::from_raw(raw)
    }
}

impl AVHDRPlusColorTransformParamsMut<'_> {
    /// Sets the overlap-processing option, preserving unknown C values.
    pub fn set_overlap_process_option(&mut self, value: AVHDRPlusOverlapProcessOption) {
        // SAFETY: the exclusive handle permits writing this integer enum field
        // and the raw-place projection forms no reference to it.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).overlap_process_option).write(value.as_raw()) }
    }
}

hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.semiminor_axis_external_ellipse
    semiminor_axis_external_ellipse,
    set_semiminor_axis_external_ellipse: u16
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.semimajor_axis_external_ellipse
    semimajor_axis_external_ellipse,
    set_semimajor_axis_external_ellipse: u16
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.semimajor_axis_internal_ellipse
    semimajor_axis_internal_ellipse,
    set_semimajor_axis_internal_ellipse: u16
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.rotation_angle
    rotation_angle,
    set_rotation_angle: u8
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.center_of_ellipse_y
    center_of_ellipse_y,
    set_center_of_ellipse_y: u16
);
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.center_of_ellipse_x
    center_of_ellipse_x,
    set_center_of_ellipse_x: u16
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.window_lower_right_corner_y
    window_lower_right_corner_y,
    window_lower_right_corner_y_mut
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.window_lower_right_corner_x
    window_lower_right_corner_x,
    window_lower_right_corner_x_mut
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.window_upper_left_corner_y
    window_upper_left_corner_y,
    window_upper_left_corner_y_mut
);
hdr_plus_rational_field!(
    /// Field: AVHDRPlusColorTransformParams.window_upper_left_corner_x
    window_upper_left_corner_x,
    window_upper_left_corner_x_mut
);

#[cfg(test)]
mod color_transform_params_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_scalar_fields_match_c() {
        assert_eq!(
            size_of::<AVHDRPlusColorTransformParams>(),
            size_of::<ffi::AVHDRPlusColorTransformParams>()
        );
        assert_eq!(
            align_of::<AVHDRPlusColorTransformParams>(),
            align_of::<ffi::AVHDRPlusColorTransformParams>()
        );

        let mut params = AVHDRPlusColorTransformParams::new();
        let mut view = params.as_mut();
        view.set_center_of_ellipse_x(101);
        view.set_center_of_ellipse_y(102);
        view.set_rotation_angle(103);
        view.set_semimajor_axis_internal_ellipse(104);
        view.set_semimajor_axis_external_ellipse(105);
        view.set_semiminor_axis_external_ellipse(106);
        view.set_overlap_process_option(AVHDRPlusOverlapProcessOption::LAYERING);
        view.set_num_distribution_maxrgb_percentiles(15);
        view.set_tone_mapping_flag(1);
        view.set_num_bezier_curve_anchors(14);
        view.set_color_saturation_mapping_flag(1);

        let view = params.as_ref();
        assert_eq!(view.center_of_ellipse_x(), 101);
        assert_eq!(view.center_of_ellipse_y(), 102);
        assert_eq!(view.rotation_angle(), 103);
        assert_eq!(view.semimajor_axis_internal_ellipse(), 104);
        assert_eq!(view.semimajor_axis_external_ellipse(), 105);
        assert_eq!(view.semiminor_axis_external_ellipse(), 106);
        assert_eq!(
            view.overlap_process_option(),
            AVHDRPlusOverlapProcessOption::LAYERING
        );
        assert_eq!(view.num_distribution_maxrgb_percentiles(), 15);
        assert_eq!(view.tone_mapping_flag(), 1);
        assert_eq!(view.num_bezier_curve_anchors(), 14);
        assert_eq!(view.color_saturation_mapping_flag(), 1);
    }

    #[test]
    #[should_panic(expected = "num_distribution_maxrgb_percentiles must not exceed 15")]
    fn a_percentile_count_beyond_the_fixed_table_is_rejected() {
        let mut params = AVHDRPlusColorTransformParams::new();
        params
            .as_mut()
            .set_num_distribution_maxrgb_percentiles(
                AVHDRPlusColorTransformParams::MAX_DISTRIBUTION_MAXRGB_PERCENTILES as u8 + 1,
            );
    }

    #[test]
    #[should_panic(expected = "num_bezier_curve_anchors must not exceed 15")]
    fn a_bezier_anchor_count_beyond_the_fixed_table_is_rejected() {
        let mut params = AVHDRPlusColorTransformParams::new();
        params.as_mut().set_num_bezier_curve_anchors(
            AVHDRPlusColorTransformParams::MAX_BEZIER_CURVE_ANCHORS as u8 + 1,
        );
    }

    #[test]
    fn inline_rationals_and_fixed_arrays_use_lifetime_bound_handles() {
        let mut params = AVHDRPlusColorTransformParams::new();
        let mut view = params.as_mut();

        view.window_upper_left_corner_x_mut().set_num(1);
        view.window_upper_left_corner_y_mut().set_num(2);
        view.window_lower_right_corner_x_mut().set_num(3);
        view.window_lower_right_corner_y_mut().set_num(4);
        view.average_maxrgb_mut().set_num(5);
        view.fraction_bright_pixels_mut().set_num(6);
        view.knee_point_x_mut().set_num(7);
        view.knee_point_y_mut().set_num(8);
        view.color_saturation_weight_mut().set_num(9);

        view.maxscl_mut().get_mut(2).unwrap().set_num(10);
        view.distribution_maxrgb_mut()
            .get_mut(14)
            .unwrap()
            .set_percentage(11);
        view.bezier_curve_anchors_mut()
            .get_mut(14)
            .unwrap()
            .set_num(12);

        let view = params.as_ref();
        assert_eq!(view.window_upper_left_corner_x().num(), 1);
        assert_eq!(view.window_upper_left_corner_y().num(), 2);
        assert_eq!(view.window_lower_right_corner_x().num(), 3);
        assert_eq!(view.window_lower_right_corner_y().num(), 4);
        assert_eq!(view.average_maxrgb().num(), 5);
        assert_eq!(view.fraction_bright_pixels().num(), 6);
        assert_eq!(view.knee_point_x().num(), 7);
        assert_eq!(view.knee_point_y().num(), 8);
        assert_eq!(view.color_saturation_weight().num(), 9);
        assert_eq!(view.maxscl().get(2).unwrap().num(), 10);
        assert_eq!(view.distribution_maxrgb().get(14).unwrap().percentage(), 11);
        assert_eq!(view.bezier_curve_anchors().get(14).unwrap().num(), 12);
        assert!(view.maxscl().get(3).is_none());
        assert!(view.distribution_maxrgb().get(15).is_none());
        assert!(view.bezier_curve_anchors().get(15).is_none());
    }
}

ffibox::define_ctype!(
    /// Wraps: AVDynamicHDRPlus
    ///
    /// ABI-compatible HDR10+ metadata. The complete metadata structure is
    /// inline storage and contains no owned pointers or other resources.
    AVDynamicHDRPlus,
    AVDynamicHDRPlusRef,
    AVDynamicHDRPlusMut,
    ffi::AVDynamicHDRPlus
);

// SAFETY: this structure contains only integer scalars, inline rationals, and
// fixed arrays of resource-free by-value structures.
unsafe impl CValued for AVDynamicHDRPlus {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

// SAFETY: owned pointers to this type originate from
// `av_dynamic_hdr_plus_alloc`, which uses the `av_malloc` family. `av_free`
// is its matching storage destructor and the type has no field teardown.
unsafe impl CDropped for AVDynamicHDRPlus {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract provides unique ownership of a live
        // `av_malloc`-family allocation and transfers it exactly once here.
        unsafe { ffi::av_free(obj.as_ptr().cast()) }
    }
}

impl AVDynamicHDRPlus {
    /// Number of processing-window parameter slots.
    pub const MAX_WINDOWS: usize = 3;
    /// Width and height of each fixed actual-peak-luminance grid.
    pub const PEAK_LUMINANCE_GRID_SIDE: usize = 25;
    /// Total entries in each flattened actual-peak-luminance grid.
    pub const PEAK_LUMINANCE_GRID_LEN: usize =
        Self::PEAK_LUMINANCE_GRID_SIDE * Self::PEAK_LUMINANCE_GRID_SIDE;

    /// Creates zero-initialized HDR10+ metadata in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! hdr_plus_metadata_scalar_field {
    ($(#[$meta:meta])* $field:ident, $setter:ident) => {
        impl AVDynamicHDRPlusRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> u8 {
                // SAFETY: the shared handle keeps initialized metadata live;
                // raw-place projection copies one byte without forming a
                // reference to the C-visible object or field.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVDynamicHDRPlusMut<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](AVDynamicHDRPlusRef::", stringify!($field), ").")]
            pub fn $setter(&mut self, value: u8) {
                // SAFETY: the exclusive handle supplies write provenance to
                // initialized metadata and raw-place projection writes only
                // the selected byte.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

hdr_plus_metadata_scalar_field!(
    /// Field: AVDynamicHDRPlus.itu_t_t35_country_code
    itu_t_t35_country_code,
    set_itu_t_t35_country_code
);
hdr_plus_metadata_scalar_field!(
    /// Field: AVDynamicHDRPlus.application_version
    application_version,
    set_application_version
);
hdr_plus_metadata_scalar_field!(
    /// Field: AVDynamicHDRPlus.targeted_system_display_actual_peak_luminance_flag
    targeted_system_display_actual_peak_luminance_flag,
    set_targeted_system_display_actual_peak_luminance_flag
);
hdr_plus_metadata_scalar_field!(
    /// Field: AVDynamicHDRPlus.mastering_display_actual_peak_luminance_flag
    mastering_display_actual_peak_luminance_flag,
    set_mastering_display_actual_peak_luminance_flag
);

macro_rules! hdr_plus_metadata_count_field {
    ($(#[$meta:meta])* $field:ident, $setter:ident, $valid:expr, $message:literal) => {
        impl AVDynamicHDRPlusRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> u8 {
                // SAFETY: the shared handle keeps initialized metadata live;
                // raw-place projection copies one byte without forming a
                // reference to C-visible storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVDynamicHDRPlusMut<'_> {
            #[doc = concat!("Sets [`", stringify!($field), "`](AVDynamicHDRPlusRef::", stringify!($field), ").")]
            ///
            /// # Panics
            ///
            #[doc = $message]
            pub fn $setter(&mut self, value: u8) {
                assert!($valid(value), $message);
                // SAFETY: the exclusive handle supplies write provenance and
                // the validated count cannot make a C consumer overrun its
                // corresponding fixed array.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

hdr_plus_metadata_count_field!(
    /// Field: AVDynamicHDRPlus.num_windows
    num_windows,
    set_num_windows,
    |value: u8| (1..=AVDynamicHDRPlus::MAX_WINDOWS as u8).contains(&value),
    "num_windows must be in 1..=3"
);
hdr_plus_metadata_count_field!(
    /// Field: AVDynamicHDRPlus.num_rows_targeted_system_display_actual_peak_luminance
    num_rows_targeted_system_display_actual_peak_luminance,
    set_num_rows_targeted_system_display_actual_peak_luminance,
    |value: u8| (2..=AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_SIDE as u8).contains(&value),
    "the targeted-system-display row count must be in 2..=25"
);
hdr_plus_metadata_count_field!(
    /// Field: AVDynamicHDRPlus.num_cols_targeted_system_display_actual_peak_luminance
    num_cols_targeted_system_display_actual_peak_luminance,
    set_num_cols_targeted_system_display_actual_peak_luminance,
    |value: u8| (2..=AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_SIDE as u8).contains(&value),
    "the targeted-system-display column count must be in 2..=25"
);
hdr_plus_metadata_count_field!(
    /// Field: AVDynamicHDRPlus.num_rows_mastering_display_actual_peak_luminance
    num_rows_mastering_display_actual_peak_luminance,
    set_num_rows_mastering_display_actual_peak_luminance,
    |value: u8| (2..=AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_SIDE as u8).contains(&value),
    "the mastering-display row count must be in 2..=25"
);
hdr_plus_metadata_count_field!(
    /// Field: AVDynamicHDRPlus.num_cols_mastering_display_actual_peak_luminance
    num_cols_mastering_display_actual_peak_luminance,
    set_num_cols_mastering_display_actual_peak_luminance,
    |value: u8| (2..=AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_SIDE as u8).contains(&value),
    "the mastering-display column count must be in 2..=25"
);

impl<'a> AVDynamicHDRPlusRef<'a> {
    /// Field: AVDynamicHDRPlus.params
    ///
    /// Borrows all three inline processing-window parameter slots.
    #[must_use]
    pub fn params(&self) -> CSlice<'a, AVHDRPlusColorTransformParams> {
        // SAFETY: raw-place projection locates the three initialized inline
        // slots without forming a reference. They live for the parent handle's
        // lifetime and the returned view grants shared access only.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).params)
                .cast::<AVHDRPlusColorTransformParams>()
                .cast_mut();
            CSlice::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::MAX_WINDOWS,
            )
        }
    }

    /// Field: AVDynamicHDRPlus.targeted_system_display_maximum_luminance
    #[must_use]
    pub fn targeted_system_display_maximum_luminance(&self) -> AVRationalRef<'a> {
        // SAFETY: raw-place projection locates an initialized inline rational
        // that remains live for the parent metadata handle's lifetime.
        unsafe {
            AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).targeted_system_display_maximum_luminance).cast_mut(),
            )
            .expect("an inline field is non-null")
        }
    }

    /// Field: AVDynamicHDRPlus.targeted_system_display_actual_peak_luminance
    ///
    /// Borrows the row-major 25-by-25 grid as a flat bounded view.
    #[must_use]
    pub fn targeted_system_display_actual_peak_luminance(
        &self,
    ) -> CSlice<'a, crate::rational::AVRational> {
        // SAFETY: C arrays are contiguous in row-major order. Raw-place
        // projection locates all 625 initialized inline rationals without
        // forming a reference, and they live for the parent handle's lifetime.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).targeted_system_display_actual_peak_luminance)
                .cast::<crate::rational::AVRational>()
                .cast_mut();
            CSlice::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_LEN,
            )
        }
    }

    /// Field: AVDynamicHDRPlus.mastering_display_actual_peak_luminance
    ///
    /// Borrows the row-major 25-by-25 grid as a flat bounded view.
    #[must_use]
    pub fn mastering_display_actual_peak_luminance(
        &self,
    ) -> CSlice<'a, crate::rational::AVRational> {
        // SAFETY: as the targeted-system grid, this fixed C array is 625
        // contiguous initialized rationals that live for the parent lifetime.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).mastering_display_actual_peak_luminance)
                .cast::<crate::rational::AVRational>()
                .cast_mut();
            CSlice::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_LEN,
            )
        }
    }
}

impl AVDynamicHDRPlusMut<'_> {
    /// Exclusively borrows all three inline processing-window parameter slots.
    #[must_use]
    pub fn params_mut(&mut self) -> CSliceMut<'_, AVHDRPlusColorTransformParams> {
        // SAFETY: raw-place projection locates all initialized slots and the
        // exclusive parent handle prevents competing access for this reborrow.
        unsafe {
            let pointer =
                addr_of_mut!((*self.as_mut_ptr()).params).cast::<AVHDRPlusColorTransformParams>();
            CSliceMut::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::MAX_WINDOWS,
            )
        }
    }

    /// Exclusively borrows the nominal maximum targeted-display luminance.
    #[must_use]
    pub fn targeted_system_display_maximum_luminance_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to
        // this initialized inline rational for the returned reborrow.
        unsafe {
            AVRationalMut::from_ptr(addr_of_mut!(
                (*self.as_mut_ptr()).targeted_system_display_maximum_luminance
            ))
            .expect("an inline field is non-null")
        }
    }

    /// Exclusively borrows the flattened targeted-display peak-luminance grid.
    #[must_use]
    pub fn targeted_system_display_actual_peak_luminance_mut(
        &mut self,
    ) -> CSliceMut<'_, crate::rational::AVRational> {
        // SAFETY: raw-place projection locates all contiguous initialized grid
        // entries and the exclusive parent handle prevents competing access.
        unsafe {
            let pointer =
                addr_of_mut!((*self.as_mut_ptr()).targeted_system_display_actual_peak_luminance)
                    .cast::<crate::rational::AVRational>();
            CSliceMut::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_LEN,
            )
        }
    }

    /// Exclusively borrows the flattened mastering-display peak-luminance grid.
    #[must_use]
    pub fn mastering_display_actual_peak_luminance_mut(
        &mut self,
    ) -> CSliceMut<'_, crate::rational::AVRational> {
        // SAFETY: as the targeted-system grid, all fixed initialized entries
        // are covered by this exclusive reborrow and no references are formed.
        unsafe {
            let pointer =
                addr_of_mut!((*self.as_mut_ptr()).mastering_display_actual_peak_luminance)
                    .cast::<crate::rational::AVRational>();
            CSliceMut::from_raw_parts(
                NonNull::new_unchecked(pointer),
                AVDynamicHDRPlus::PEAK_LUMINANCE_GRID_LEN,
            )
        }
    }
}

#[cfg(test)]
mod dynamic_hdr_plus_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_all_fields_match_c() {
        assert_eq!(
            size_of::<AVDynamicHDRPlus>(),
            size_of::<ffi::AVDynamicHDRPlus>()
        );
        assert_eq!(
            align_of::<AVDynamicHDRPlus>(),
            align_of::<ffi::AVDynamicHDRPlus>()
        );

        let mut metadata = AVDynamicHDRPlus::new();
        let mut view = metadata.as_mut();
        view.set_itu_t_t35_country_code(0xb5);
        view.set_application_version(1);
        view.set_num_windows(3);
        view.set_targeted_system_display_actual_peak_luminance_flag(1);
        view.set_num_rows_targeted_system_display_actual_peak_luminance(25);
        view.set_num_cols_targeted_system_display_actual_peak_luminance(24);
        view.set_mastering_display_actual_peak_luminance_flag(1);
        view.set_num_rows_mastering_display_actual_peak_luminance(23);
        view.set_num_cols_mastering_display_actual_peak_luminance(22);
        view.targeted_system_display_maximum_luminance_mut()
            .set_num(10_000);
        view.params_mut()
            .get_mut(2)
            .expect("the third window exists")
            .set_center_of_ellipse_x(321);
        view.targeted_system_display_actual_peak_luminance_mut()
            .get_mut(624)
            .expect("the last targeted-display grid entry exists")
            .set_num(15);
        view.mastering_display_actual_peak_luminance_mut()
            .get_mut(0)
            .expect("the first mastering-display grid entry exists")
            .set_den(17);

        let view = metadata.as_ref();
        assert_eq!(view.itu_t_t35_country_code(), 0xb5);
        assert_eq!(view.application_version(), 1);
        assert_eq!(view.num_windows(), 3);
        assert_eq!(
            view.targeted_system_display_maximum_luminance().num(),
            10_000
        );
        assert_eq!(view.targeted_system_display_actual_peak_luminance_flag(), 1);
        assert_eq!(
            view.num_rows_targeted_system_display_actual_peak_luminance(),
            25
        );
        assert_eq!(
            view.num_cols_targeted_system_display_actual_peak_luminance(),
            24
        );
        assert_eq!(view.mastering_display_actual_peak_luminance_flag(), 1);
        assert_eq!(view.num_rows_mastering_display_actual_peak_luminance(), 23);
        assert_eq!(view.num_cols_mastering_display_actual_peak_luminance(), 22);
        assert_eq!(view.params().get(2).unwrap().center_of_ellipse_x(), 321);
        assert_eq!(
            view.targeted_system_display_actual_peak_luminance()
                .get(624)
                .unwrap()
                .num(),
            15
        );
        assert_eq!(
            view.mastering_display_actual_peak_luminance()
                .get(0)
                .unwrap()
                .den(),
            17
        );
        assert!(view.params().get(3).is_none());
        assert!(
            view.targeted_system_display_actual_peak_luminance()
                .get(625)
                .is_none()
        );
    }

    #[test]
    fn owned_allocation_uses_the_av_malloc_destructor() {
        // SAFETY: allocating exactly the generated C layout size is within
        // `av_malloc`'s contract; NULL is handled before the pointer is used.
        let raw = unsafe { ffi::av_malloc(size_of::<ffi::AVDynamicHDRPlus>()) }
            .cast::<ffi::AVDynamicHDRPlus>();
        assert!(!raw.is_null());
        // SAFETY: `raw` is suitably aligned writable storage for exactly one
        // C layout value. Every field admits the zero representation used by
        // libavutil's own allocator, so this finishes initialization.
        unsafe { raw.write(core::mem::zeroed()) };
        // SAFETY: the initialized pointer is a uniquely owned av_malloc-family
        // allocation whose matching `CDropped` implementation calls av_free.
        let mut metadata = unsafe { ffibox::CBox::<AVDynamicHDRPlus>::from_raw(raw) }
            .expect("av_malloc returned a non-null pointer");

        metadata.as_mut().set_num_windows(1);
        assert_eq!(metadata.as_ref().num_windows(), 1);
        // `metadata` releases the allocation through `av_free` here.
    }
}
