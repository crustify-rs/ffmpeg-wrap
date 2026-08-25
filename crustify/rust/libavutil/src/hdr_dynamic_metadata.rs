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
