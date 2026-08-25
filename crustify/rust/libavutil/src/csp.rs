//! Wrappers for `libavutil/csp.c`.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CVal, CValued};

use crate::ffi;
use crate::rational::{AVRationalMut, AVRationalRef};

ffibox::define_ctype!(
    /// Wraps: AVLumaCoefficients
    ///
    /// ABI-compatible inline luma coefficients used by colorspace conversion.
    /// The structure owns no resources.
    AVLumaCoefficients,
    AVLumaCoefficientsRef,
    AVLumaCoefficientsMut,
    ffi::AVLumaCoefficients
);

// SAFETY: all three fields are inline `AVRational` values with no teardown.
unsafe impl CValued for AVLumaCoefficients {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVLumaCoefficients {
    /// Creates zero-initialized coefficients in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! rational_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVLumaCoefficientsRef<'a> {
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

        impl AVLumaCoefficientsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVLumaCoefficientsRef::", stringify!($field), ").")]
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
    /// Field: AVLumaCoefficients.cb
    cb,
    cb_mut
);
rational_field!(
    /// Field: AVLumaCoefficients.cg
    cg,
    cg_mut
);
rational_field!(
    /// Field: AVLumaCoefficients.cr
    cr,
    cr_mut
);

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_rational_fields_match_c() {
        assert_eq!(
            size_of::<AVLumaCoefficients>(),
            size_of::<ffi::AVLumaCoefficients>()
        );
        assert_eq!(
            align_of::<AVLumaCoefficients>(),
            align_of::<ffi::AVLumaCoefficients>()
        );

        let mut coefficients = AVLumaCoefficients::new();
        let mut view = coefficients.as_mut();
        view.cr_mut().set_num(299);
        view.cg_mut().set_num(587);
        view.cb_mut().set_num(114);
        view.cr_mut().set_den(1_000);
        view.cg_mut().set_den(1_000);
        view.cb_mut().set_den(1_000);

        let view = coefficients.as_ref();
        assert_eq!((view.cr().num(), view.cr().den()), (299, 1_000));
        assert_eq!((view.cg().num(), view.cg().den()), (587, 1_000));
        assert_eq!((view.cb().num(), view.cb().den()), (114, 1_000));
    }
}
