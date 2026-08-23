//! Wrappers for libavutil rational arithmetic.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CVal, CValued};

use crate::ffi;

ffibox::define_ctype!(
    /// Wraps: AVRational
    ///
    /// ABI-compatible layout for FFmpeg's numerator/denominator pair.
    AVRational,
    AVRationalRef,
    AVRationalMut,
    ffi::AVRational
);

// SAFETY: `AVRational` has no owned resources or teardown operation. Disposing
// an inline value is therefore a no-op and always leaves its storage valid.
unsafe impl CValued for AVRational {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVRational {
    /// Creates an owned inline rational without reducing it.
    #[must_use]
    pub fn new(num: i32, den: i32) -> CVal<Self> {
        Self::from_ffi(ffi::AVRational { num, den })
    }

    pub(crate) fn from_ffi(value: ffi::AVRational) -> CVal<Self> {
        CVal::new(Self(ffibox::CType::new(value)))
    }
}

impl AVRationalRef<'_> {
    /// Wraps: AVRational.num
    ///
    /// Returns the numerator.
    #[must_use]
    pub fn num(&self) -> i32 {
        // SAFETY: the handle guarantees a live, initialized `AVRational`; the
        // raw-place projection reads its integer field without forming a
        // reference to the wrapped object or field.
        unsafe { addr_of!((*self.as_ptr()).num).read() }
    }

    /// Wraps: AVRational.den
    ///
    /// Returns the denominator.
    #[must_use]
    pub fn den(&self) -> i32 {
        // SAFETY: the handle guarantees a live, initialized `AVRational`; the
        // raw-place projection reads its integer field without forming a
        // reference to the wrapped object or field.
        unsafe { addr_of!((*self.as_ptr()).den).read() }
    }
}

impl AVRationalMut<'_> {
    /// Sets the numerator.
    pub fn set_num(&mut self, num: i32) {
        // SAFETY: the exclusive handle supplies write provenance to a live
        // `AVRational`; the raw-place projection writes only its integer field
        // and forms no reference to the wrapped object or field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).num).write(num) }
    }

    /// Sets the denominator.
    pub fn set_den(&mut self, den: i32) {
        // SAFETY: the exclusive handle supplies write provenance to a live
        // `AVRational`; the raw-place projection writes only its integer field
        // and forms no reference to the wrapped object or field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).den).write(den) }
    }
}

/// Result of reducing a fraction with [`av_reduce`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReducedRational {
    pub numerator: i32,
    pub denominator: i32,
    pub exact: bool,
}

/// Wraps: av_reduce
///
/// Returns both C out-parameters as one initialized Rust value.
#[must_use]
pub fn av_reduce(numerator: i64, denominator: i64, max: i64) -> ReducedRational {
    let mut reduced_num = 0;
    let mut reduced_den = 0;
    // SAFETY: both out-pointers address distinct live `i32` slots and remain
    // valid for the call; the remaining arguments are values.
    let exact = unsafe {
        ffi::av_reduce(
            &raw mut reduced_num,
            &raw mut reduced_den,
            numerator,
            denominator,
            max,
        ) != 0
    };
    ReducedRational {
        numerator: reduced_num,
        denominator: reduced_den,
        exact,
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_ffi() {
        assert_eq!(size_of::<AVRational>(), size_of::<ffi::AVRational>());
        assert_eq!(align_of::<AVRational>(), align_of::<ffi::AVRational>());
    }

    #[test]
    fn owned_value_supports_shared_and_mutable_handles() {
        let mut value = AVRational::new(1, 2);
        assert_eq!(value.as_ref().num(), 1);
        assert_eq!(value.as_ref().den(), 2);

        value.as_mut().set_num(-3);
        value.as_mut().set_den(7);

        let view = value.as_ref();
        assert_eq!(view.num(), -3);
        assert_eq!(view.den(), 7);
        // SAFETY: `view` keeps the initialized inline rational live, and the
        // bindgen layout is `Copy`, so reading duplicates the by-value pair.
        let raw = unsafe { view.as_ptr().read() };
        assert_eq!(raw.num, -3);
        assert_eq!(raw.den, 7);
    }

    #[test]
    fn reduces_exactly_and_approximately() {
        assert_eq!(
            av_reduce(10, 20, 100),
            ReducedRational {
                numerator: 1,
                denominator: 2,
                exact: true
            }
        );
        let approximate = av_reduce(1, 3, 2);
        assert!(!approximate.exact);
        assert!(approximate.numerator.abs() <= 2 && approximate.denominator <= 2);
    }
}
