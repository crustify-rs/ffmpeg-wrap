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
    pub(crate) fn copy_ffi(&self) -> ffi::AVRational {
        // SAFETY: the handle keeps an initialized rational live and the C
        // representation is a pair of copyable integers.
        unsafe { self.as_ptr().read() }
    }

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
///
/// `max` bounds two `int` out-parameters, so it is taken as [`i32`] rather than
/// C's `int64_t`. C reduces into `int64_t` accumulators and only truncates on
/// the final stores, so a wider `max` makes it report an exact reduction while
/// writing a wrapped value: `av_reduce(1 << 40, 1, i64::MAX)` stores `0/1` and
/// returns 1. A non-positive `max` is rejected for the same reason — it is
/// below every reachable denominator, so C's own
/// `av_assert2(a1.num <= max && a1.den <= max)` cannot hold.
///
/// [`i64::MIN`] is refused for either half of the fraction. C starts by taking
/// `FFABS` of both, which negates a negative input, and negating [`i64::MIN`]
/// overflows — undefined behaviour this campaign's UBSan build reports against
/// `rational.c` before the reduction produces anything. Nothing else is lost to
/// the check: every other magnitude reduces normally, and `max` is never
/// negated.
pub fn av_reduce(
    numerator: i64,
    denominator: i64,
    max: i32,
) -> Result<ReducedRational, RationalError> {
    if max <= 0 {
        return Err(RationalError::NonPositiveMaximum);
    }
    if numerator == i64::MIN || denominator == i64::MIN {
        return Err(RationalError::UnnegatableInput);
    }
    let mut reduced_num = 0;
    let mut reduced_den = 0;
    // SAFETY: both out-pointers address distinct live `i32` slots and remain
    // valid for the call; the remaining arguments are values, and a `max` in
    // `1..=i32::MAX` keeps both results inside the `int` slots C writes.
    let exact = unsafe {
        ffi::av_reduce(
            &raw mut reduced_num,
            &raw mut reduced_den,
            numerator,
            denominator,
            i64::from(max),
        ) != 0
    };
    Ok(ReducedRational {
        numerator: reduced_num,
        denominator: reduced_den,
        exact,
    })
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
            av_reduce(10, 20, 100).unwrap(),
            ReducedRational {
                numerator: 1,
                denominator: 2,
                exact: true
            }
        );
        let approximate = av_reduce(1, 3, 2).unwrap();
        assert!(!approximate.exact);
        assert!(approximate.numerator.abs() <= 2 && approximate.denominator <= 2);

        // The sign is carried separately from the reduced magnitude, and an
        // undefined denominator survives the round trip rather than dividing
        // by zero.
        assert_eq!(
            av_reduce(-10, 20, 100).unwrap(),
            ReducedRational {
                numerator: -1,
                denominator: 2,
                exact: true
            }
        );
        assert_eq!(
            av_reduce(1, 0, i32::MAX).unwrap(),
            ReducedRational {
                numerator: 1,
                denominator: 0,
                exact: true
            }
        );
    }

    #[test]
    fn rejects_the_one_magnitude_c_cannot_negate() {
        // Reaching C with either of these makes `FFABS` negate `i64::MIN`,
        // which the UBSan build reports as
        // "negation of -9223372036854775808 cannot be represented in type
        // 'long int'" at rational.c's `av_gcd(FFABS(num), FFABS(den))`.
        assert_eq!(
            av_reduce(i64::MIN, 1, i32::MAX),
            Err(RationalError::UnnegatableInput)
        );
        assert_eq!(
            av_reduce(1, i64::MIN, i32::MAX),
            Err(RationalError::UnnegatableInput)
        );
        // The neighbouring magnitude is fine, so the guard is exactly one
        // value wide.
        assert!(av_reduce(i64::MIN + 1, 1, i32::MAX).is_ok());
    }

    #[test]
    fn reduction_bound_is_clamped_to_the_int_out_parameters() {
        assert_eq!(av_reduce(1, 3, 0), Err(RationalError::NonPositiveMaximum));
        assert_eq!(av_reduce(1, 3, -1), Err(RationalError::NonPositiveMaximum));

        // The widest accepted bound still lands inside the `int` slots.
        let wide = av_reduce(1 << 40, 1, i32::MAX).unwrap();
        assert_eq!(wide.denominator, 1);
        assert_eq!(wide.numerator, i32::MAX);
        assert!(!wide.exact);
    }
}

/// Replaces: av_inv_q
#[must_use]
pub fn av_inv_q(q: AVRationalRef<'_>) -> CVal<AVRational> {
    AVRational::new(q.den(), q.num())
}

/// Replaces: av_make_q
#[must_use]
pub fn av_make_q(num: i32, den: i32) -> CVal<AVRational> {
    AVRational::new(num, den)
}

/// Wraps: av_mul_q
#[must_use]
pub fn av_mul_q(b: AVRationalRef<'_>, c: AVRationalRef<'_>) -> CVal<AVRational> {
    // SAFETY: both by-value arguments were copied from live rational handles.
    AVRational::from_ffi(unsafe { ffi::av_mul_q(b.copy_ffi(), c.copy_ffi()) })
}

/// Wraps: av_nearer_q
#[must_use]
pub fn av_nearer_q(q: AVRationalRef<'_>, q1: AVRationalRef<'_>, q2: AVRationalRef<'_>) -> i32 {
    // SAFETY: all arguments are initialized by-value copies; C retains none.
    unsafe { ffi::av_nearer_q(q.copy_ffi(), q1.copy_ffi(), q2.copy_ffi()) }
}

/// Replaces: av_q2d
#[must_use]
pub fn av_q2d(q: AVRationalRef<'_>) -> f64 {
    f64::from(q.num()) / f64::from(q.den())
}

/// Wraps: av_q2intfloat
#[must_use]
pub fn av_q2intfloat(q: AVRationalRef<'_>) -> u32 {
    // SAFETY: the argument is an initialized by-value copy and is not retained.
    unsafe { ffi::av_q2intfloat(q.copy_ffi()) }
}

/// Wraps: av_sub_q
#[must_use]
pub fn av_sub_q(b: AVRationalRef<'_>, c: AVRationalRef<'_>) -> CVal<AVRational> {
    // SAFETY: both by-value arguments were copied from live rational handles.
    AVRational::from_ffi(unsafe { ffi::av_sub_q(b.copy_ffi(), c.copy_ffi()) })
}

#[cfg(test)]
mod scheduled_tests {
    use super::*;

    #[test]
    fn scheduled_rational_operations_use_typed_values() {
        let half = av_make_q(1, 2);
        let third = av_make_q(1, 3);
        assert_eq!(av_q2d(half.as_ref()), 0.5);
        let product = av_mul_q(half.as_ref(), third.as_ref());
        assert_eq!((product.as_ref().num(), product.as_ref().den()), (1, 6));
        let difference = av_sub_q(half.as_ref(), third.as_ref());
        assert_eq!(
            (difference.as_ref().num(), difference.as_ref().den()),
            (1, 6)
        );
        let inverse = av_inv_q(half.as_ref());
        assert_eq!((inverse.as_ref().num(), inverse.as_ref().den()), (2, 1));
    }
}

/// Wraps: av_add_q
#[must_use]
pub fn av_add_q(left: AVRationalRef<'_>, right: AVRationalRef<'_>) -> CVal<AVRational> {
    // SAFETY: both by-value arguments were copied from live rational handles.
    AVRational::from_ffi(unsafe { ffi::av_add_q(left.copy_ffi(), right.copy_ffi()) })
}

/// Replaces: av_cmp_q
///
/// This header-inline operation is evaluated directly with the same widened
/// arithmetic and special handling for infinities and undefined rationals.
///
/// The widened arithmetic cannot overflow, so this stays total for every
/// `AVRational` pair: an `i32` cross product lies in
/// `-((1 << 62) - (1 << 31))..=(1 << 62)`, and the widest difference of two
/// such products is `(1 << 63) - (1 << 31)`, still inside `i64`.
#[must_use]
pub fn av_cmp_q(left: AVRationalRef<'_>, right: AVRationalRef<'_>) -> i32 {
    let a_num = left.num();
    let a_den = left.den();
    let b_num = right.num();
    let b_den = right.den();
    let difference = i64::from(a_num) * i64::from(b_den) - i64::from(b_num) * i64::from(a_den);
    if difference != 0 {
        (((difference ^ i64::from(a_den) ^ i64::from(b_den)) >> 63) as i32) | 1
    } else if b_den != 0 && a_den != 0 {
        0
    } else if a_num != 0 && b_num != 0 {
        (a_num >> 31) - (b_num >> 31)
    } else {
        i32::MIN
    }
}

/// Argument rejected by a fallible rational wrapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalError {
    /// A reduction bound was not at least 1.
    NonPositiveMaximum,
    /// Both operands of [`av_gcd_q`] had a zero denominator, which would make
    /// C divide by `av_gcd(0, 0) == 0`.
    BothDenominatorsZero,
    /// `i64::MIN` was passed where C takes `FFABS` of the value.
    UnnegatableInput,
}

/// Wraps: av_d2q
///
/// `max` bounds the reduction C performs internally, so it is rejected on the
/// same terms as [`av_reduce`]; the `int` parameter already caps it.
pub fn av_d2q(value: f64, max: i32) -> Result<CVal<AVRational>, RationalError> {
    if max <= 0 {
        return Err(RationalError::NonPositiveMaximum);
    }
    // SAFETY: a positive maximum satisfies `av_reduce`'s bound invariant; the
    // function accepts both arguments by value and returns an initialized pair.
    Ok(AVRational::from_ffi(unsafe { ffi::av_d2q(value, max) }))
}

/// Wraps: av_div_q
#[must_use]
pub fn av_div_q(dividend: AVRationalRef<'_>, divisor: AVRationalRef<'_>) -> CVal<AVRational> {
    // SAFETY: both by-value arguments were copied from live rational handles.
    AVRational::from_ffi(unsafe { ffi::av_div_q(dividend.copy_ffi(), divisor.copy_ffi()) })
}

/// Wraps: av_gcd_q
pub fn av_gcd_q(
    left: AVRationalRef<'_>,
    right: AVRationalRef<'_>,
    max_denominator: i32,
    default: AVRationalRef<'_>,
) -> Result<CVal<AVRational>, RationalError> {
    if left.den() == 0 && right.den() == 0 {
        return Err(RationalError::BothDenominatorsZero);
    }
    // SAFETY: all rational arguments are initialized by-value copies. At least
    // one denominator is nonzero, so C's `a.den / gcd` cannot divide by zero.
    Ok(AVRational::from_ffi(unsafe {
        ffi::av_gcd_q(
            left.copy_ffi(),
            right.copy_ffi(),
            max_denominator,
            default.copy_ffi(),
        )
    }))
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn arithmetic_wrappers_return_owned_values() {
        let half = AVRational::new(1, 2);
        let third = AVRational::new(1, 3);
        let sum = av_add_q(half.as_ref(), third.as_ref());
        assert_eq!((sum.as_ref().num(), sum.as_ref().den()), (5, 6));

        let quotient = av_div_q(half.as_ref(), third.as_ref());
        assert_eq!((quotient.as_ref().num(), quotient.as_ref().den()), (3, 2));
        assert_eq!(av_cmp_q(half.as_ref(), third.as_ref()), 1);
        assert_eq!(av_cmp_q(half.as_ref(), half.as_ref()), 0);

        let converted = av_d2q(0.5, 1000).unwrap();
        assert_eq!((converted.as_ref().num(), converted.as_ref().den()), (1, 2));

        let fallback = AVRational::new(0, 1);
        let gcd = av_gcd_q(half.as_ref(), third.as_ref(), 100, fallback.as_ref()).unwrap();
        assert_eq!((gcd.as_ref().num(), gcd.as_ref().den()), (1, 6));
    }

    #[test]
    fn comparison_is_total_over_the_whole_int_range() {
        // Widest cross-product difference reachable from `i32` fields; the
        // values match a two's-complement C build of the inline `av_cmp_q`.
        let a = AVRational::new(i32::MIN, i32::MAX);
        let b = AVRational::new(i32::MIN, i32::MIN);
        assert_eq!(av_cmp_q(a.as_ref(), b.as_ref()), -1);

        let c = AVRational::new(i32::MAX, i32::MIN);
        let d = AVRational::new(i32::MIN, i32::MAX);
        assert_eq!(av_cmp_q(c.as_ref(), d.as_ref()), 1);

        // `0 / 0` on either side is the documented `INT_MIN` sentinel.
        let undefined = AVRational::new(0, 0);
        assert_eq!(av_cmp_q(undefined.as_ref(), undefined.as_ref()), i32::MIN);

        // Two infinities of opposite sign compare by numerator sign alone.
        let plus_inf = AVRational::new(1, 0);
        let minus_inf = AVRational::new(-1, 0);
        assert_eq!(av_cmp_q(plus_inf.as_ref(), minus_inf.as_ref()), 1);
        assert_eq!(av_cmp_q(minus_inf.as_ref(), plus_inf.as_ref()), -1);
    }
}
