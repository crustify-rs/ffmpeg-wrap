//! Wrappers for `libavutil/film_grain_params.c`.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CValued, define_ctype};

use crate::ffi;

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

impl AVFilmGrainH274Params {
    pub const COMPONENTS: usize = 3;
    pub const MAX_INTENSITY_INTERVALS: usize = 256;
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
            pub fn $set(&mut self, value: $ty) {
                if stringify!($field) != "log2_scale_factor" {
                    assert!((0..=1).contains(&value));
                }
                // SAFETY: the exclusive handle supplies write provenance for
                // this live scalar field; discriminator-like fields were
                // restricted to their two documented values above.
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
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
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
            /// Panics if `component` is not in `0..3`.
            pub fn $set(&mut self, component: usize, value: $ty) {
                assert_component(component);
                let maximum = match stringify!($field) {
                    "component_model_present" => 1,
                    "num_intensity_intervals" => {
                        AVFilmGrainH274Params::MAX_INTENSITY_INTERVALS
                    }
                    "num_model_values" => AVFilmGrainH274Params::MAX_MODEL_VALUES,
                    _ => unreachable!(),
                };
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
    i32
);
component_field!(
    /// Field: AVFilmGrainH274Params.num_intensity_intervals
    num_intensity_intervals,
    set_num_intensity_intervals,
    num_intensity_intervals,
    u16
);
component_field!(
    /// Field: AVFilmGrainH274Params.num_model_values
    num_model_values,
    set_num_model_values,
    num_model_values,
    u8
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
    /// The C header explicitly excludes this layout from the stable public
    /// ABI. Consequently the wrapper matches the headers used to build this
    /// crate, while the field meanings remain the published API contract.
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
        let mut params = AVFilmGrainH274Params::zeroed();
        // SAFETY: `params` is live and initialized, and this handle is its only
        // access path for the duration of the borrow.
        let mut params = unsafe {
            AVFilmGrainH274ParamsMut::from_ptr(
                addr_of_mut!(params).cast::<ffi::AVFilmGrainH274Params>(),
            )
            .expect("an inline field is non-null")
        };
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
