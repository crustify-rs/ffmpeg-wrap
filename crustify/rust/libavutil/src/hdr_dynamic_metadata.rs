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

use ffibox::{CSlice, CSliceMut, CVal, CValued};

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
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.num_bezier_curve_anchors
    num_bezier_curve_anchors,
    set_num_bezier_curve_anchors: u8
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
hdr_plus_scalar_field!(
    /// Field: AVHDRPlusColorTransformParams.num_distribution_maxrgb_percentiles
    num_distribution_maxrgb_percentiles,
    set_num_distribution_maxrgb_percentiles: u8
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
