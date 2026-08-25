//! Wrappers for `libavutil/film_grain_params.c`.

use core::ffi::c_void;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CDropped, CValued, define_ctype};

use crate::ffi;
use crate::pixfmt::{AVColorPrimaries, AVColorRange, AVColorSpace, AVColorTransferCharacteristic};

/// Wraps: AVFilmGrainParamsType
///
/// Identifies the active member of `AVFilmGrainParams.codec`. The integer
/// newtype preserves values introduced by newer libavutil versions.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVFilmGrainParamsType(ffi::AVFilmGrainParamsType);

impl AVFilmGrainParamsType {
    pub const NONE: Self = Self(ffi::AVFilmGrainParamsType_AV_FILM_GRAIN_PARAMS_NONE);
    pub const AV1: Self = Self(ffi::AVFilmGrainParamsType_AV_FILM_GRAIN_PARAMS_AV1);
    pub const H274: Self = Self(ffi::AVFilmGrainParamsType_AV_FILM_GRAIN_PARAMS_H274);

    /// Wraps a raw C value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(value: ffi::AVFilmGrainParamsType) -> Self {
        Self(value)
    }

    /// Returns the raw value used by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVFilmGrainParamsType {
        self.0
    }
}

impl Default for AVFilmGrainParamsType {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<ffi::AVFilmGrainParamsType> for AVFilmGrainParamsType {
    fn from(value: ffi::AVFilmGrainParamsType) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVFilmGrainParamsType> for ffi::AVFilmGrainParamsType {
    fn from(value: AVFilmGrainParamsType) -> Self {
        value.as_raw()
    }
}

define_ctype!(
    /// Wraps: AVFilmGrainH274Params
    ///
    /// Layout-compatible H.274 film-grain parameters. This value is embedded
    /// in `AVFilmGrainParams`, so it owns no independent allocation and has no
    /// destructor. Its handles never form Rust references over C storage.
    AVFilmGrainH274Params,
    AVFilmGrainH274ParamsRef,
    AVFilmGrainH274ParamsMut,
    ffi::AVFilmGrainH274Params
);

// SAFETY: the structure contains only integers and fixed-size integer arrays,
// owns no resources, and has no C teardown operation. Disposing an inline
// value is therefore a no-op, exactly as for its `AVFilmGrainAOMParams` union
// sibling.
unsafe impl CValued for AVFilmGrainH274Params {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVFilmGrainH274Params {
    /// Number of colour components each per-component table is indexed by.
    pub const COMPONENTS: usize = 3;
    /// Capacity of the per-component intensity-interval tables.
    pub const MAX_INTENSITY_INTERVALS: usize = 256;
    /// Capacity of the per-interval model-value table.
    pub const MAX_MODEL_VALUES: usize = 6;
}

macro_rules! scalar_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVFilmGrainH274ParamsRef<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the handle addresses a live initialized H.274 value;
                // raw projection and the scalar read form no Rust reference.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVFilmGrainH274ParamsMut<'_> {
            #[doc = concat!("Sets `", stringify!($field), "`.")]
            ///
            /// The value is stored unchanged. The H.274 syntax elements this
            /// field carries are parsed as fixed-width unsigned fields, and
            /// libavcodec stores every value they can encode here, including
            /// the ones the specification reserves.
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance for
                // this live scalar field, and every `$ty` bit pattern is a
                // valid value of the C field.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

scalar_field!(
    /// Field: AVFilmGrainH274Params.log2_scale_factor
    log2_scale_factor,
    set_log2_scale_factor,
    log2_scale_factor,
    i32
);
scalar_field!(
    /// Field: AVFilmGrainH274Params.blending_mode_id
    blending_mode_id,
    set_blending_mode_id,
    blending_mode_id,
    i32
);
scalar_field!(
    /// Field: AVFilmGrainH274Params.model_id
    model_id,
    set_model_id,
    model_id,
    i32
);

macro_rules! component_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty, $maximum:expr) => {
        impl AVFilmGrainH274ParamsRef<'_> {
            $(#[$attr])*
            ///
            /// # Panics
            ///
            /// Panics if `component` is not in `0..3`.
            #[must_use]
            pub fn $get(&self, component: usize) -> $ty {
                assert_component(component);
                // SAFETY: the index was checked against the three-element C
                // array; raw projection and the scalar read form no reference.
                unsafe {
                    addr_of!((*self.as_ptr()).$field)
                        .cast::<$ty>()
                        .add(component)
                        .read()
                }
            }
        }

        impl AVFilmGrainH274ParamsMut<'_> {
            #[doc = concat!("Sets one entry of `", stringify!($field), "`.")]
            ///
            /// # Panics
            ///
            #[doc = concat!(
                "Panics if `component` is not in `0..3`, or if `value` exceeds `",
                stringify!($maximum),
                "`, the largest value libavcodec stores in this field.",
            )]
            pub fn $set(&mut self, component: usize, value: $ty) {
                assert_component(component);
                let maximum: usize = $maximum;
                assert!((value as usize) <= maximum);
                // SAFETY: both the index and count/indicator were checked
                // against the C field's documented capacity; the exclusive
                // handle supplies write provenance for the slot.
                unsafe {
                    addr_of_mut!((*self.as_mut_ptr()).$field)
                        .cast::<$ty>()
                        .add(component)
                        .write(value);
                }
            }
        }
    };
}

component_field!(
    /// Field: AVFilmGrainH274Params.component_model_present
    component_model_present,
    set_component_model_present,
    component_model_present,
    i32,
    1
);
component_field!(
    /// Field: AVFilmGrainH274Params.num_intensity_intervals
    num_intensity_intervals,
    set_num_intensity_intervals,
    num_intensity_intervals,
    u16,
    AVFilmGrainH274Params::MAX_INTENSITY_INTERVALS
);
component_field!(
    /// Field: AVFilmGrainH274Params.num_model_values
    num_model_values,
    set_num_model_values,
    num_model_values,
    u8,
    AVFilmGrainH274Params::MAX_MODEL_VALUES
);

macro_rules! interval_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident) => {
        impl AVFilmGrainH274ParamsRef<'_> {
            $(#[$attr])*
            ///
            /// # Panics
            ///
            /// Panics if either index exceeds its fixed C array bound.
            #[must_use]
            pub fn $get(&self, component: usize, interval: usize) -> u8 {
                let index = interval_index(component, interval);
                // SAFETY: `interval_index` checked both dimensions; raw
                // projection and the scalar read form no Rust reference.
                unsafe {
                    addr_of!((*self.as_ptr()).$field)
                        .cast::<u8>()
                        .add(index)
                        .read()
                }
            }
        }

        impl AVFilmGrainH274ParamsMut<'_> {
            #[doc = concat!("Sets one entry of `", stringify!($field), "`.")]
            pub fn $set(&mut self, component: usize, interval: usize, value: u8) {
                let index = interval_index(component, interval);
                // SAFETY: `interval_index` checked both dimensions and the
                // exclusive handle supplies write provenance for the slot.
                unsafe {
                    addr_of_mut!((*self.as_mut_ptr()).$field)
                        .cast::<u8>()
                        .add(index)
                        .write(value);
                }
            }
        }
    };
}

interval_field!(
    /// Field: AVFilmGrainH274Params.intensity_interval_upper_bound
    intensity_interval_upper_bound,
    set_intensity_interval_upper_bound,
    intensity_interval_upper_bound
);
interval_field!(
    /// Field: AVFilmGrainH274Params.intensity_interval_lower_bound
    intensity_interval_lower_bound,
    set_intensity_interval_lower_bound,
    intensity_interval_lower_bound
);

impl AVFilmGrainH274ParamsRef<'_> {
    /// Field: AVFilmGrainH274Params.comp_model_value
    ///
    /// Returns one entry indexed as `[component][interval][model value]`.
    ///
    /// # Panics
    ///
    /// Panics if any index exceeds its fixed C array bound.
    #[must_use]
    pub fn comp_model_value(&self, component: usize, interval: usize, value: usize) -> i16 {
        let index = model_value_index(component, interval, value);
        // SAFETY: `model_value_index` checked all three dimensions; raw
        // projection and the scalar read form no Rust reference.
        unsafe {
            addr_of!((*self.as_ptr()).comp_model_value)
                .cast::<i16>()
                .add(index)
                .read()
        }
    }
}

impl AVFilmGrainH274ParamsMut<'_> {
    /// Sets one entry of `comp_model_value`.
    pub fn set_comp_model_value(
        &mut self,
        component: usize,
        interval: usize,
        value: usize,
        model_value: i16,
    ) {
        let index = model_value_index(component, interval, value);
        // SAFETY: `model_value_index` checked all dimensions and the exclusive
        // handle supplies write provenance for the selected scalar slot.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).comp_model_value)
                .cast::<i16>()
                .add(index)
                .write(model_value);
        }
    }
}

fn assert_component(component: usize) {
    assert!(component < AVFilmGrainH274Params::COMPONENTS);
}

fn interval_index(component: usize, interval: usize) -> usize {
    assert_component(component);
    assert!(interval < AVFilmGrainH274Params::MAX_INTENSITY_INTERVALS);
    component * AVFilmGrainH274Params::MAX_INTENSITY_INTERVALS + interval
}

fn model_value_index(component: usize, interval: usize, value: usize) -> usize {
    assert!(value < AVFilmGrainH274Params::MAX_MODEL_VALUES);
    interval_index(component, interval) * AVFilmGrainH274Params::MAX_MODEL_VALUES + value
}

define_ctype!(
    /// Wraps: AVFilmGrainAOMParams
    ///
    /// Layout-compatible AOM/AV1 film-grain parameters embedded by value in
    /// `AVFilmGrainParams::codec.aom`. The structure owns no pointers or other
    /// resources; all fixed-size tables are copied into and out of borrowed
    /// handles so no Rust reference covers memory that C may mutate.
    ///
    /// The C header excludes this struct's *size* from the public ABI, and
    /// requires it to be allocated as part of an `AVFilmGrainParams`.
    /// Consequently the wrapper matches the headers used to build this crate,
    /// while the field meanings remain the published API contract.
    AVFilmGrainAOMParams,
    AVFilmGrainAOMParamsRef,
    AVFilmGrainAOMParamsMut,
    ffi::AVFilmGrainAOMParams
);

// SAFETY: the structure contains only integers and fixed-size integer arrays,
// owns no resources, and has no C teardown operation. Disposing an inline
// value is therefore a no-op.
unsafe impl CValued for AVFilmGrainAOMParams {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVFilmGrainAOMParamsRef<'_> {
    /// Field: AVFilmGrainAOMParams.limit_output_range
    #[must_use]
    pub fn limit_output_range(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).limit_output_range).read() }
    }

    /// Field: AVFilmGrainAOMParams.overlap_flag
    #[must_use]
    pub fn overlap_flag(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).overlap_flag).read() }
    }

    /// Field: AVFilmGrainAOMParams.uv_offset
    #[must_use]
    pub fn uv_offset(&self) -> [i32; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).uv_offset).read() }
    }

    /// Field: AVFilmGrainAOMParams.uv_mult_luma
    #[must_use]
    pub fn uv_mult_luma(&self) -> [i32; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).uv_mult_luma).read() }
    }

    /// Field: AVFilmGrainAOMParams.uv_mult
    #[must_use]
    pub fn uv_mult(&self) -> [i32; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).uv_mult).read() }
    }

    /// Field: AVFilmGrainAOMParams.grain_scale_shift
    #[must_use]
    pub fn grain_scale_shift(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).grain_scale_shift).read() }
    }

    /// Field: AVFilmGrainAOMParams.ar_coeff_shift
    #[must_use]
    pub fn ar_coeff_shift(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).ar_coeff_shift).read() }
    }

    /// Field: AVFilmGrainAOMParams.ar_coeffs_uv
    #[must_use]
    pub fn ar_coeffs_uv(&self) -> [[i8; 25]; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).ar_coeffs_uv).read() }
    }

    /// Field: AVFilmGrainAOMParams.ar_coeffs_y
    #[must_use]
    pub fn ar_coeffs_y(&self) -> [i8; 24] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).ar_coeffs_y).read() }
    }

    /// Field: AVFilmGrainAOMParams.ar_coeff_lag
    #[must_use]
    pub fn ar_coeff_lag(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).ar_coeff_lag).read() }
    }

    /// Field: AVFilmGrainAOMParams.scaling_shift
    #[must_use]
    pub fn scaling_shift(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).scaling_shift).read() }
    }

    /// Field: AVFilmGrainAOMParams.uv_points
    #[must_use]
    pub fn uv_points(&self) -> [[[u8; 2]; 10]; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).uv_points).read() }
    }

    /// Field: AVFilmGrainAOMParams.num_uv_points
    #[must_use]
    pub fn num_uv_points(&self) -> [i32; 2] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).num_uv_points).read() }
    }

    /// Field: AVFilmGrainAOMParams.chroma_scaling_from_luma
    #[must_use]
    pub fn chroma_scaling_from_luma(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).chroma_scaling_from_luma).read() }
    }

    /// Field: AVFilmGrainAOMParams.y_points
    #[must_use]
    pub fn y_points(&self) -> [[u8; 2]; 14] {
        // SAFETY: the initialized handle permits a raw copy of this fixed array.
        unsafe { addr_of!((*self.as_ptr()).y_points).read() }
    }

    /// Field: AVFilmGrainAOMParams.num_y_points
    #[must_use]
    pub fn num_y_points(&self) -> i32 {
        // SAFETY: the initialized handle permits a raw copy of this integer.
        unsafe { addr_of!((*self.as_ptr()).num_y_points).read() }
    }
}

impl AVFilmGrainAOMParamsMut<'_> {
    /// Sets whether synthesis output is clipped to limited color levels.
    pub fn set_limit_output_range(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).limit_output_range).write(value) }
    }

    /// Sets whether adjacent film-grain blocks overlap.
    pub fn set_overlap_flag(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).overlap_flag).write(value) }
    }

    /// Sets the Cb and Cr scaling offsets.
    pub fn set_uv_offset(&mut self, value: [i32; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).uv_offset).write(value) }
    }

    /// Sets the luma multipliers used for the chroma scaling indices.
    pub fn set_uv_mult_luma(&mut self, value: [i32; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).uv_mult_luma).write(value) }
    }

    /// Sets the Cb and Cr multipliers used for the chroma scaling indices.
    pub fn set_uv_mult(&mut self, value: [i32; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).uv_mult).write(value) }
    }

    /// Sets the downshift applied to generated Gaussian values.
    pub fn set_grain_scale_shift(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).grain_scale_shift).write(value) }
    }

    /// Sets the range shift for auto-regression coefficients.
    pub fn set_ar_coeff_shift(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).ar_coeff_shift).write(value) }
    }

    /// Replaces both chroma auto-regression coefficient tables.
    pub fn set_ar_coeffs_uv(&mut self, value: [[i8; 25]; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).ar_coeffs_uv).write(value) }
    }

    /// Replaces the luma auto-regression coefficient table.
    pub fn set_ar_coeffs_y(&mut self, value: [i8; 24]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).ar_coeffs_y).write(value) }
    }

    /// Sets the auto-regression lag.
    pub fn set_ar_coeff_lag(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).ar_coeff_lag).write(value) }
    }

    /// Sets the piecewise-linear scaling shift.
    pub fn set_scaling_shift(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).scaling_shift).write(value) }
    }

    /// Replaces the Cb and Cr piecewise-linear scaling tables.
    pub fn set_uv_points(&mut self, value: [[[u8; 2]; 10]; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).uv_points).write(value) }
    }

    /// Sets the number of active Cb and Cr scaling points.
    pub fn set_num_uv_points(&mut self, value: [i32; 2]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).num_uv_points).write(value) }
    }

    /// Sets whether chroma scaling is derived from luma.
    pub fn set_chroma_scaling_from_luma(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).chroma_scaling_from_luma).write(value) }
    }

    /// Replaces the luma piecewise-linear scaling table.
    pub fn set_y_points(&mut self, value: [[u8; 2]; 14]) {
        // SAFETY: the exclusive handle permits a raw write to this fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).y_points).write(value) }
    }

    /// Sets the number of active luma scaling points.
    pub fn set_num_y_points(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits a raw write to this field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).num_y_points).write(value) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn enum_values_and_unknown_values_round_trip() {
        assert_eq!(
            AVFilmGrainParamsType::default(),
            AVFilmGrainParamsType::NONE
        );
        assert_eq!(AVFilmGrainParamsType::AV1.as_raw(), 1);
        assert_eq!(AVFilmGrainParamsType::H274.as_raw(), 2);
        assert_eq!(AVFilmGrainParamsType::from_raw(91).as_raw(), 91);
    }

    #[test]
    fn h274_fields_round_trip_at_array_boundaries() {
        let mut owner = CVal::new(AVFilmGrainH274Params::zeroed());
        let mut params = owner.as_mut();
        params.set_model_id(1);
        params.set_blending_mode_id(1);
        params.set_log2_scale_factor(7);
        params.set_component_model_present(2, 1);
        params.set_num_intensity_intervals(2, 256);
        params.set_num_model_values(2, 6);
        params.set_intensity_interval_lower_bound(2, 255, 17);
        params.set_intensity_interval_upper_bound(2, 255, 219);
        params.set_comp_model_value(2, 255, 5, -1234);

        let shared = params.as_ref();
        assert_eq!(shared.model_id(), 1);
        assert_eq!(shared.blending_mode_id(), 1);
        assert_eq!(shared.log2_scale_factor(), 7);
        assert_eq!(shared.component_model_present(2), 1);
        assert_eq!(shared.num_intensity_intervals(2), 256);
        assert_eq!(shared.num_model_values(2), 6);
        assert_eq!(shared.intensity_interval_lower_bound(2, 255), 17);
        assert_eq!(shared.intensity_interval_upper_bound(2, 255), 219);
        assert_eq!(shared.comp_model_value(2, 255, 5), -1234);
    }

    /// `film_grain_model_id` and `blending_mode_id` are two-bit SEI syntax
    /// elements. `libavcodec/h2645_sei.c` reads them with `get_bits(gb, 2)`
    /// and copies them into this structure without clamping, so the wrapper
    /// must round-trip the reserved values 2 and 3 as well.
    #[test]
    fn h274_mode_identifiers_round_trip_every_two_bit_value() {
        let mut owner = CVal::new(AVFilmGrainH274Params::zeroed());
        for raw in 0..4 {
            let mut params = owner.as_mut();
            params.set_model_id(raw);
            params.set_blending_mode_id(raw);
            assert_eq!(params.as_ref().model_id(), raw);
            assert_eq!(params.as_ref().blending_mode_id(), raw);
        }

        // `log2_scale_factor` is a four-bit element with the same contract.
        let mut params = owner.as_mut();
        params.set_log2_scale_factor(15);
        assert_eq!(params.as_ref().log2_scale_factor(), 15);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn h274_rejects_an_interval_count_the_tables_cannot_hold() {
        let mut owner = CVal::new(AVFilmGrainH274Params::zeroed());
        owner.as_mut().set_num_intensity_intervals(0, 257);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn h274_rejects_a_model_value_count_the_tables_cannot_hold() {
        let mut owner = CVal::new(AVFilmGrainH274Params::zeroed());
        owner.as_mut().set_num_model_values(0, 7);
    }

    #[test]
    fn h274_wrapper_preserves_c_layout() {
        assert_eq!(
            size_of::<AVFilmGrainH274Params>(),
            size_of::<ffi::AVFilmGrainH274Params>()
        );
        assert_eq!(
            align_of::<AVFilmGrainH274Params>(),
            align_of::<ffi::AVFilmGrainH274Params>()
        );
    }
}

#[cfg(test)]
mod aom_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn layout_matches_the_generated_c_structure() {
        assert_eq!(
            size_of::<AVFilmGrainAOMParams>(),
            size_of::<ffi::AVFilmGrainAOMParams>()
        );
        assert_eq!(
            align_of::<AVFilmGrainAOMParams>(),
            align_of::<ffi::AVFilmGrainAOMParams>()
        );
    }

    #[test]
    fn owned_inline_value_supports_scalar_and_table_access() {
        let mut value = CVal::new(AVFilmGrainAOMParams::zeroed());
        value.as_mut().set_num_y_points(2);
        value.as_mut().set_y_points([
            [1, 11],
            [2, 22],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
        ]);
        value.as_mut().set_num_uv_points([1, 1]);
        value.as_mut().set_uv_offset([-256, 255]);
        value.as_mut().set_ar_coeffs_y([7; 24]);
        value.as_mut().set_ar_coeffs_uv([[3; 25], [4; 25]]);

        let view = value.as_ref();
        assert_eq!(view.num_y_points(), 2);
        assert_eq!(view.y_points()[..2], [[1, 11], [2, 22]]);
        assert_eq!(view.num_uv_points(), [1, 1]);
        assert_eq!(view.uv_offset(), [-256, 255]);
        assert_eq!(view.ar_coeffs_y(), [7; 24]);
        assert_eq!(view.ar_coeffs_uv(), [[3; 25], [4; 25]]);
    }
}

define_ctype!(
    /// Wraps: AVFilmGrainParams
    ///
    /// Layout-compatible top-level film-grain parameters. Independently
    /// allocated values are owned by `ffibox::CBox<AVFilmGrainParams>` and
    /// released with `av_free`; frame side-data values remain borrowed from
    /// their frame owner.
    AVFilmGrainParams,
    AVFilmGrainParamsRef,
    AVFilmGrainParamsMut,
    ffi::AVFilmGrainParams
);

// SAFETY: independently owned instances are allocated by
// `av_film_grain_params_alloc` with `av_mallocz`. The structure contains no
// separately allocated fields, and `av_free` is its documented releaser.
unsafe impl CDropped for AVFilmGrainParams {
    unsafe fn c_drop(object: NonNull<Self>) {
        // SAFETY: the trait contract supplies sole ownership of an independent
        // allocation from the av_malloc family. Borrowed frame side-data is
        // never adopted into `CBox`, so it cannot reach this implementation.
        unsafe { ffi::av_free(object.as_ptr().cast::<c_void>()) }
    }
}

/// A discriminator-checked shared view of `AVFilmGrainParams.codec`.
pub enum AVFilmGrainCodecRef<'a> {
    None,
    Aom(AVFilmGrainAOMParamsRef<'a>),
    H274(AVFilmGrainH274ParamsRef<'a>),
    Unknown(AVFilmGrainParamsType),
}

/// A discriminator-checked exclusive view of `AVFilmGrainParams.codec`.
pub enum AVFilmGrainCodecMut<'a> {
    None,
    Aom(AVFilmGrainAOMParamsMut<'a>),
    H274(AVFilmGrainH274ParamsMut<'a>),
    Unknown(AVFilmGrainParamsType),
}

macro_rules! params_scalar_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVFilmGrainParamsRef<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the shared handle addresses an initialized value;
                // raw-place projection copies one scalar without a reference.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVFilmGrainParamsMut<'_> {
            #[doc = concat!("Sets `", stringify!($field), "`.")]
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle permits a raw write to the
                // scalar field without forming a reference to C storage.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

params_scalar_field!(
    /// Field: AVFilmGrainParams.seed
    seed,
    set_seed,
    seed,
    u64
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.width
    width,
    set_width,
    width,
    i32
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.height
    height,
    set_height,
    height,
    i32
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.subsampling_x
    subsampling_x,
    set_subsampling_x,
    subsampling_x,
    i32
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.subsampling_y
    subsampling_y,
    set_subsampling_y,
    subsampling_y,
    i32
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.bit_depth_luma
    bit_depth_luma,
    set_bit_depth_luma,
    bit_depth_luma,
    i32
);
params_scalar_field!(
    /// Field: AVFilmGrainParams.bit_depth_chroma
    bit_depth_chroma,
    set_bit_depth_chroma,
    bit_depth_chroma,
    i32
);

macro_rules! params_color_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty, $raw:ty) => {
        impl AVFilmGrainParamsRef<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: bindgen exposes the open C enum as its integer ABI
                // type, which is copied through a raw-place projection.
                <$ty>::from_raw(unsafe { addr_of!((*self.as_ptr()).$field).read() })
            }
        }

        impl AVFilmGrainParamsMut<'_> {
            #[doc = concat!("Sets `", stringify!($field), "`.")]
            pub fn $set(&mut self, value: $ty) {
                let raw: $raw = value.as_raw();
                // SAFETY: the exclusive handle permits a raw ABI-integer write
                // and the open wrapper preserves every possible value.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(raw) }
            }
        }
    };
}

params_color_field!(
    /// Field: AVFilmGrainParams.color_range
    color_range,
    set_color_range,
    color_range,
    AVColorRange,
    ffi::AVColorRange
);
params_color_field!(
    /// Field: AVFilmGrainParams.color_primaries
    color_primaries,
    set_color_primaries,
    color_primaries,
    AVColorPrimaries,
    ffi::AVColorPrimaries
);
params_color_field!(
    /// Field: AVFilmGrainParams.color_trc
    color_trc,
    set_color_trc,
    color_trc,
    AVColorTransferCharacteristic,
    ffi::AVColorTransferCharacteristic
);
params_color_field!(
    /// Field: AVFilmGrainParams.color_space
    color_space,
    set_color_space,
    color_space,
    AVColorSpace,
    ffi::AVColorSpace
);

impl<'a> AVFilmGrainParamsRef<'a> {
    /// Field: AVFilmGrainParams.type
    #[must_use]
    pub fn params_type(&self) -> AVFilmGrainParamsType {
        // SAFETY: bindgen represents the open C enum as an integer and the
        // initialized discriminator is copied through a raw projection.
        AVFilmGrainParamsType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Field: AVFilmGrainParams.codec
    ///
    /// Returns the active union member selected by [`Self::params_type`].
    #[must_use]
    pub fn codec(&self) -> AVFilmGrainCodecRef<'a> {
        let params_type = self.params_type();
        // SAFETY: the shared handle addresses a live initialized parent;
        // projecting the union's address forms no reference to its storage.
        let codec = unsafe { addr_of!((*self.as_ptr()).codec) };
        if params_type == AVFilmGrainParamsType::NONE {
            AVFilmGrainCodecRef::None
        } else if params_type == AVFilmGrainParamsType::AV1 {
            // SAFETY: the public C contract says AV1 selects `codec.aom`; the
            // member begins at the union address and remains live for `'a`.
            let member = unsafe {
                AVFilmGrainAOMParamsRef::from_ptr(
                    codec.cast::<ffi::AVFilmGrainAOMParams>().cast_mut(),
                )
                .expect("an inline union member is non-null")
            };
            AVFilmGrainCodecRef::Aom(member)
        } else if params_type == AVFilmGrainParamsType::H274 {
            // SAFETY: the public C contract says H274 selects `codec.h274`; the
            // member begins at the union address and remains live for `'a`.
            let member = unsafe {
                AVFilmGrainH274ParamsRef::from_ptr(
                    codec.cast::<ffi::AVFilmGrainH274Params>().cast_mut(),
                )
                .expect("an inline union member is non-null")
            };
            AVFilmGrainCodecRef::H274(member)
        } else {
            AVFilmGrainCodecRef::Unknown(params_type)
        }
    }
}

impl AVFilmGrainParamsMut<'_> {
    /// Returns an exclusive view of the currently active codec member.
    #[must_use]
    pub fn codec_mut(&mut self) -> AVFilmGrainCodecMut<'_> {
        let params_type = self.as_ref().params_type();
        // SAFETY: the exclusive handle addresses a live initialized parent;
        // projecting the union's address forms no reference to its storage.
        let codec = unsafe { addr_of_mut!((*self.as_mut_ptr()).codec) };
        if params_type == AVFilmGrainParamsType::NONE {
            AVFilmGrainCodecMut::None
        } else if params_type == AVFilmGrainParamsType::AV1 {
            // SAFETY: AV1 selects `codec.aom`; the exclusive reborrow keeps the
            // returned member handle from outliving or aliasing this handle.
            let member = unsafe {
                AVFilmGrainAOMParamsMut::from_ptr(codec.cast::<ffi::AVFilmGrainAOMParams>())
                    .expect("an inline union member is non-null")
            };
            AVFilmGrainCodecMut::Aom(member)
        } else if params_type == AVFilmGrainParamsType::H274 {
            // SAFETY: H274 selects `codec.h274`; the exclusive reborrow keeps
            // the returned member handle from outliving or aliasing this one.
            let member = unsafe {
                AVFilmGrainH274ParamsMut::from_ptr(codec.cast::<ffi::AVFilmGrainH274Params>())
                    .expect("an inline union member is non-null")
            };
            AVFilmGrainCodecMut::H274(member)
        } else {
            AVFilmGrainCodecMut::Unknown(params_type)
        }
    }

    /// Field: AVFilmGrainParams.codec.aom
    ///
    /// Zero-initializes and selects the AOM/AV1 member, returning it for
    /// further initialization.
    pub fn activate_aom(&mut self) -> AVFilmGrainAOMParamsMut<'_> {
        let ptr = self.as_mut_ptr();
        // SAFETY: every field of the C member is an integer or integer array,
        // so all-zero is valid. The exclusive handle permits overwriting the
        // union member and discriminator before the new view is returned.
        unsafe {
            let codec = addr_of_mut!((*ptr).codec).cast::<ffi::AVFilmGrainAOMParams>();
            codec.write(core::mem::zeroed());
            addr_of_mut!((*ptr).type_).write(AVFilmGrainParamsType::AV1.as_raw());
            AVFilmGrainAOMParamsMut::from_ptr(codec).expect("an inline union member is non-null")
        }
    }

    /// Field: AVFilmGrainParams.codec.h274
    ///
    /// Zero-initializes and selects the H.274 member, returning it for further
    /// initialization.
    pub fn activate_h274(&mut self) -> AVFilmGrainH274ParamsMut<'_> {
        let ptr = self.as_mut_ptr();
        // SAFETY: every field of the C member is an integer or integer array,
        // so all-zero is valid. The exclusive handle permits overwriting the
        // union member and discriminator before the new view is returned.
        unsafe {
            let codec = addr_of_mut!((*ptr).codec).cast::<ffi::AVFilmGrainH274Params>();
            codec.write(core::mem::zeroed());
            addr_of_mut!((*ptr).type_).write(AVFilmGrainParamsType::H274.as_raw());
            AVFilmGrainH274ParamsMut::from_ptr(codec).expect("an inline union member is non-null")
        }
    }

    /// Marks the union as inactive without reading its stored bytes.
    pub fn clear_codec(&mut self) {
        // SAFETY: the exclusive handle permits updating the discriminator;
        // the union bytes may remain initialized but are inactive under NONE.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).type_).write(AVFilmGrainParamsType::NONE.as_raw());
        }
    }
}

#[cfg(test)]
mod params_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_scalar_and_tagged_union_access_round_trip() {
        assert_eq!(
            size_of::<AVFilmGrainParams>(),
            size_of::<ffi::AVFilmGrainParams>()
        );
        assert_eq!(
            align_of::<AVFilmGrainParams>(),
            align_of::<ffi::AVFilmGrainParams>()
        );

        let mut params = AVFilmGrainParams::zeroed();
        // SAFETY: `params` is live and initialized, and the returned exclusive
        // handle is its only access path for this borrow.
        let mut params = unsafe {
            AVFilmGrainParamsMut::from_ptr(addr_of_mut!(params).cast::<ffi::AVFilmGrainParams>())
                .expect("a stack value is non-null")
        };
        params.set_seed(42);
        params.set_width(1920);
        params.set_height(1080);
        params.set_subsampling_x(1);
        params.set_subsampling_y(1);
        params.set_color_range(AVColorRange::MPEG);
        params.set_color_primaries(AVColorPrimaries::BT2020);
        params.set_color_trc(AVColorTransferCharacteristic::SMPTE2084);
        params.set_color_space(AVColorSpace::BT2020_NCL);
        params.set_bit_depth_luma(10);
        params.set_bit_depth_chroma(10);
        params.activate_aom().set_num_y_points(2);

        let view = params.as_ref();
        assert_eq!(view.seed(), 42);
        assert_eq!(view.width(), 1920);
        assert_eq!(view.height(), 1080);
        assert_eq!(view.color_range(), AVColorRange::MPEG);
        assert_eq!(view.params_type(), AVFilmGrainParamsType::AV1);
        match view.codec() {
            AVFilmGrainCodecRef::Aom(aom) => assert_eq!(aom.num_y_points(), 2),
            _ => panic!("AOM discriminator did not expose the AOM member"),
        }

        params.activate_h274().set_model_id(1);
        match params.codec_mut() {
            AVFilmGrainCodecMut::H274(mut h274) => h274.set_log2_scale_factor(7),
            _ => panic!("H.274 discriminator did not expose the H.274 member"),
        }
        match params.as_ref().codec() {
            AVFilmGrainCodecRef::H274(h274) => {
                assert_eq!(h274.model_id(), 1);
                assert_eq!(h274.log2_scale_factor(), 7);
            }
            _ => panic!("H.274 member was not retained"),
        }

        params.clear_codec();
        assert!(matches!(params.as_ref().codec(), AVFilmGrainCodecRef::None));
    }
}
