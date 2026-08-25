//! Wrappers for `libavutil/hdr_dynamic_vivid_metadata.c`.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CVal, CValued};

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
