//! Wrappers for `libavutil/hdr_dynamic_vivid_metadata.c`.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CSlice, CSliceMut, CVal, CValued};

use crate::ffi;
use crate::rational::{AVRationalMut, AVRationalRef};

ffibox::define_ctype!(
    /// Wraps: AVHDRVivid3SplineParams
    ///
    /// ABI-compatible inline HDR Vivid three-spline parameters. The structure
    /// owns no resources.
    AVHDRVivid3SplineParams,
    AVHDRVivid3SplineParamsRef,
    AVHDRVivid3SplineParamsMut,
    ffi::AVHDRVivid3SplineParams
);

// SAFETY: the C structure contains one integer and five inline rationals;
// none has a teardown operation.
unsafe impl CValued for AVHDRVivid3SplineParams {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVHDRVivid3SplineParams {
    /// Creates zero-initialized spline parameters in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

impl AVHDRVivid3SplineParamsRef<'_> {
    /// Field: AVHDRVivid3SplineParams.th_mode
    #[must_use]
    pub fn th_mode(&self) -> i32 {
        // SAFETY: the handle keeps an initialized object live and raw-place
        // projection copies the integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).th_mode).read() }
    }
}

impl AVHDRVivid3SplineParamsMut<'_> {
    /// Sets the three-spline mode.
    pub fn set_th_mode(&mut self, value: i32) {
        // SAFETY: the exclusive handle permits writing this integer field and
        // raw-place projection forms no reference to C-visible storage.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).th_mode).write(value) }
    }
}

macro_rules! rational_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVHDRVivid3SplineParamsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> AVRationalRef<'a> {
                // SAFETY: the projected field is an initialized inline
                // rational that lives for the parent handle's lifetime.
                unsafe {
                    AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).$field).cast_mut())
                }
                .expect("an inline field is non-null")
            }
        }

        impl AVHDRVivid3SplineParamsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVHDRVivid3SplineParamsRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> AVRationalMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to the initialized inline field for this reborrow.
                unsafe {
                    AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$field))
                }
                .expect("an inline field is non-null")
            }
        }
    };
}

rational_field!(
    /// Field: AVHDRVivid3SplineParams.th_enable_mb
    th_enable_mb,
    th_enable_mb_mut
);
rational_field!(
    /// Field: AVHDRVivid3SplineParams.th_enable
    th_enable,
    th_enable_mut
);
rational_field!(
    /// Field: AVHDRVivid3SplineParams.th_delta1
    th_delta1,
    th_delta1_mut
);
rational_field!(
    /// Field: AVHDRVivid3SplineParams.th_delta2
    th_delta2,
    th_delta2_mut
);
rational_field!(
    /// Field: AVHDRVivid3SplineParams.enable_strength
    enable_strength,
    enable_strength_mut
);

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_all_fields_match_c() {
        assert_eq!(
            size_of::<AVHDRVivid3SplineParams>(),
            size_of::<ffi::AVHDRVivid3SplineParams>()
        );
        assert_eq!(
            align_of::<AVHDRVivid3SplineParams>(),
            align_of::<ffi::AVHDRVivid3SplineParams>()
        );

        let mut params = AVHDRVivid3SplineParams::new();
        let mut view = params.as_mut();
        view.set_th_mode(2);
        view.th_enable_mb_mut().set_num(1);
        view.th_enable_mut().set_num(2);
        view.th_delta1_mut().set_num(3);
        view.th_delta2_mut().set_num(4);
        view.enable_strength_mut().set_num(5);

        let view = params.as_ref();
        assert_eq!(view.th_mode(), 2);
        assert_eq!(view.th_enable_mb().num(), 1);
        assert_eq!(view.th_enable().num(), 2);
        assert_eq!(view.th_delta1().num(), 3);
        assert_eq!(view.th_delta2().num(), 4);
        assert_eq!(view.enable_strength().num(), 5);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVHDRVividColorToneMappingParams
    ///
    /// ABI-compatible HDR Vivid tone-mapping parameters. All storage is
    /// inline, including the two three-spline parameter slots.
    AVHDRVividColorToneMappingParams,
    AVHDRVividColorToneMappingParamsRef,
    AVHDRVividColorToneMappingParamsMut,
    ffi::AVHDRVividColorToneMappingParams
);

// SAFETY: the C structure contains only integers, inline rationals, and an
// inline array of resource-free `AVHDRVivid3SplineParams` values.
unsafe impl CValued for AVHDRVividColorToneMappingParams {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVHDRVividColorToneMappingParams {
    /// Creates zero-initialized tone-mapping parameters in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! tone_mapping_scalar_field {
    ($(#[$meta:meta])* $method:ident, $setter:ident, $raw_field:ident) => {
        impl AVHDRVividColorToneMappingParamsRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $method(&self) -> i32 {
                // SAFETY: the handle keeps an initialized object live and
                // raw-place projection copies one integer without forming a
                // reference to the C-visible storage.
                unsafe { addr_of!((*self.as_ptr()).$raw_field).read() }
            }
        }

        impl AVHDRVividColorToneMappingParamsMut<'_> {
            #[doc = concat!("Sets [`", stringify!($method), "`](AVHDRVividColorToneMappingParamsRef::", stringify!($method), ").")]
            pub fn $setter(&mut self, value: i32) {
                // SAFETY: the exclusive handle supplies write provenance and
                // raw-place projection writes only this integer field.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$raw_field).write(value) }
            }
        }
    };
}

macro_rules! tone_mapping_rational_field {
    ($(#[$meta:meta])* $method:ident, $method_mut:ident, $raw_field:ident) => {
        impl<'a> AVHDRVividColorToneMappingParamsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $method(&self) -> AVRationalRef<'a> {
                // SAFETY: raw-place projection locates an initialized inline
                // rational that lives for the parent handle's lifetime.
                unsafe {
                    AVRationalRef::from_ptr(
                        addr_of!((*self.as_ptr()).$raw_field).cast_mut(),
                    )
                    .expect("an embedded field is non-null")
                }
            }
        }

        impl AVHDRVividColorToneMappingParamsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($method), "`](AVHDRVividColorToneMappingParamsRef::", stringify!($method), ").")]
            #[must_use]
            pub fn $method_mut(&mut self) -> AVRationalMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to this initialized inline rational for the
                // returned reborrow.
                unsafe {
                    AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$raw_field))
                        .expect("an embedded field is non-null")
                }
            }
        }
    };
}

tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.targeted_system_display_maximum_luminance
    targeted_system_display_maximum_luminance,
    targeted_system_display_maximum_luminance_mut,
    targeted_system_display_maximum_luminance
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_enable_flag
    base_enable_flag,
    set_base_enable_flag,
    base_enable_flag
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_m_p
    base_param_m_p,
    base_param_m_p_mut,
    base_param_m_p
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_m_m
    base_param_m_m,
    base_param_m_m_mut,
    base_param_m_m
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_m_a
    base_param_m_a,
    base_param_m_a_mut,
    base_param_m_a
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_m_b
    base_param_m_b,
    base_param_m_b_mut,
    base_param_m_b
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_m_n
    base_param_m_n,
    base_param_m_n_mut,
    base_param_m_n
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_k1
    base_param_k1,
    set_base_param_k1,
    base_param_k1
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_k2
    base_param_k2,
    set_base_param_k2,
    base_param_k2
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_k3
    base_param_k3,
    set_base_param_k3,
    base_param_k3
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_Delta_enable_mode
    base_param_delta_enable_mode,
    set_base_param_delta_enable_mode,
    base_param_Delta_enable_mode
);
tone_mapping_rational_field!(
    /// Field: AVHDRVividColorToneMappingParams.base_param_Delta
    base_param_delta,
    base_param_delta_mut,
    base_param_Delta
);
tone_mapping_scalar_field!(
    /// Field: AVHDRVividColorToneMappingParams.three_Spline_enable_flag
    three_spline_enable_flag,
    set_three_spline_enable_flag,
    three_Spline_enable_flag
);

impl AVHDRVividColorToneMappingParamsRef<'_> {
    /// Field: AVHDRVividColorToneMappingParams.three_Spline_num
    #[must_use]
    pub fn three_spline_num(&self) -> i32 {
        // SAFETY: the handle keeps initialized metadata live and raw-place
        // projection copies the integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).three_Spline_num).read() }
    }
}

impl AVHDRVividColorToneMappingParamsMut<'_> {
    /// Sets the number of active three-spline entries.
    ///
    /// # Panics
    ///
    /// Panics when `value` is outside `0..=2`, because the inline array has
    /// exactly two entries. Zero is retained as the inactive/initial state.
    pub fn set_three_spline_num(&mut self, value: i32) {
        assert!(
            (0..=2).contains(&value),
            "three_spline_num must be in 0..=2"
        );
        // SAFETY: the exclusive handle supplies write provenance and the
        // validated value cannot make a C consumer overrun `three_spline`.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).three_Spline_num).write(value) }
    }
}

impl<'a> AVHDRVividColorToneMappingParamsRef<'a> {
    /// Field: AVHDRVividColorToneMappingParams.three_spline
    #[must_use]
    pub fn three_spline(&self) -> CSlice<'a, AVHDRVivid3SplineParams> {
        // SAFETY: raw-place projection locates the fixed array without forming
        // a reference; both initialized entries live for the parent handle's
        // lifetime and the returned view grants shared access only.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).three_spline)
                .cast::<AVHDRVivid3SplineParams>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 2)
        }
    }
}

impl AVHDRVividColorToneMappingParamsMut<'_> {
    /// Exclusively borrows the two inline three-spline parameter entries.
    #[must_use]
    pub fn three_spline_mut(&mut self) -> CSliceMut<'_, AVHDRVivid3SplineParams> {
        // SAFETY: raw-place projection locates both initialized entries, and
        // the exclusive parent handle prevents any competing access for the
        // returned view's lifetime.
        unsafe {
            let pointer =
                addr_of_mut!((*self.as_mut_ptr()).three_spline).cast::<AVHDRVivid3SplineParams>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 2)
        }
    }
}

#[cfg(test)]
mod tone_mapping_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_all_fields_match_c() {
        assert_eq!(
            size_of::<AVHDRVividColorToneMappingParams>(),
            size_of::<ffi::AVHDRVividColorToneMappingParams>()
        );
        assert_eq!(
            align_of::<AVHDRVividColorToneMappingParams>(),
            align_of::<ffi::AVHDRVividColorToneMappingParams>()
        );

        let mut params = AVHDRVividColorToneMappingParams::new();
        let mut view = params.as_mut();
        view.targeted_system_display_maximum_luminance_mut()
            .set_num(1);
        view.set_base_enable_flag(2);
        view.base_param_m_p_mut().set_num(3);
        view.base_param_m_m_mut().set_num(4);
        view.base_param_m_a_mut().set_num(5);
        view.base_param_m_b_mut().set_num(6);
        view.base_param_m_n_mut().set_num(7);
        view.set_base_param_k1(8);
        view.set_base_param_k2(9);
        view.set_base_param_k3(10);
        view.set_base_param_delta_enable_mode(11);
        view.base_param_delta_mut().set_num(12);
        view.set_three_spline_enable_flag(13);
        view.set_three_spline_num(2);
        view.three_spline_mut()
            .get_mut(1)
            .expect("the second inline spline exists")
            .set_th_mode(3);

        let view = params.as_ref();
        assert_eq!(view.targeted_system_display_maximum_luminance().num(), 1);
        assert_eq!(view.base_enable_flag(), 2);
        assert_eq!(view.base_param_m_p().num(), 3);
        assert_eq!(view.base_param_m_m().num(), 4);
        assert_eq!(view.base_param_m_a().num(), 5);
        assert_eq!(view.base_param_m_b().num(), 6);
        assert_eq!(view.base_param_m_n().num(), 7);
        assert_eq!(view.base_param_k1(), 8);
        assert_eq!(view.base_param_k2(), 9);
        assert_eq!(view.base_param_k3(), 10);
        assert_eq!(view.base_param_delta_enable_mode(), 11);
        assert_eq!(view.base_param_delta().num(), 12);
        assert_eq!(view.three_spline_enable_flag(), 13);
        assert_eq!(view.three_spline_num(), 2);
        assert_eq!(view.three_spline().len(), 2);
        assert_eq!(
            view.three_spline()
                .get(1)
                .expect("the second inline spline exists")
                .th_mode(),
            3
        );
        assert!(view.three_spline().get(2).is_none());
    }
}
