//! Wrappers for libavutil rescaling utilities.

use crate::ffi;
use crate::rational::AVRationalRef;

/// Wraps: AVRounding
///
/// ABI-compatible representation of a valid FFmpeg rounding mode. The
/// pass-through flag is added with
/// [`with_pass_minmax`](Self::with_pass_minmax), preventing safe code from
/// constructing the invalid integer combinations that the C routines reject.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVRounding(ffi::AVRounding);

impl AVRounding {
    /// Round toward zero.
    pub const ZERO: Self = Self(ffi::AVRounding_AV_ROUND_ZERO);
    /// Round away from zero.
    pub const INF: Self = Self(ffi::AVRounding_AV_ROUND_INF);
    /// Round toward negative infinity.
    pub const DOWN: Self = Self(ffi::AVRounding_AV_ROUND_DOWN);
    /// Round toward positive infinity.
    pub const UP: Self = Self(ffi::AVRounding_AV_ROUND_UP);
    /// Round to nearest, with halfway cases away from zero.
    pub const NEAR_INF: Self = Self(ffi::AVRounding_AV_ROUND_NEAR_INF);

    /// Pass `i64::MIN` and `i64::MAX` through unchanged.
    #[must_use]
    pub const fn with_pass_minmax(self) -> Self {
        Self(self.0 | ffi::AVRounding_AV_ROUND_PASS_MINMAX)
    }

    /// Reports whether the min/max pass-through flag is enabled.
    #[must_use]
    pub const fn passes_minmax(self) -> bool {
        self.0 & ffi::AVRounding_AV_ROUND_PASS_MINMAX != 0
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVRounding {
        self.0
    }
}

impl From<AVRounding> for ffi::AVRounding {
    fn from(value: AVRounding) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_ffi() {
        assert_eq!(size_of::<AVRounding>(), size_of::<ffi::AVRounding>());
        assert_eq!(align_of::<AVRounding>(), align_of::<ffi::AVRounding>());
    }

    #[test]
    fn modes_and_pass_through_flag_match_ffi() {
        assert_eq!(AVRounding::ZERO.as_raw(), ffi::AVRounding_AV_ROUND_ZERO);
        assert_eq!(AVRounding::INF.as_raw(), ffi::AVRounding_AV_ROUND_INF);
        assert_eq!(AVRounding::DOWN.as_raw(), ffi::AVRounding_AV_ROUND_DOWN);
        assert_eq!(AVRounding::UP.as_raw(), ffi::AVRounding_AV_ROUND_UP);
        assert_eq!(
            AVRounding::NEAR_INF.as_raw(),
            ffi::AVRounding_AV_ROUND_NEAR_INF
        );

        let mode = AVRounding::UP.with_pass_minmax();
        assert_eq!(
            mode.as_raw(),
            ffi::AVRounding_AV_ROUND_UP | ffi::AVRounding_AV_ROUND_PASS_MINMAX
        );
        assert!(mode.passes_minmax());
        assert!(!AVRounding::UP.passes_minmax());
    }
}

/// Wraps: av_rescale_q
#[must_use]
pub fn av_rescale_q(a: i64, bq: AVRationalRef<'_>, cq: AVRationalRef<'_>) -> i64 {
    // SAFETY: the rationals are initialized by-value copies and C retains none.
    unsafe { ffi::av_rescale_q(a, bq.copy_ffi(), cq.copy_ffi()) }
}

/// Wraps: av_rescale_q_rnd
#[must_use]
pub fn av_rescale_q_rnd(
    a: i64,
    bq: AVRationalRef<'_>,
    cq: AVRationalRef<'_>,
    rounding: AVRounding,
) -> i64 {
    // SAFETY: all inputs are valid by-value ABI values and C retains none.
    unsafe { ffi::av_rescale_q_rnd(a, bq.copy_ffi(), cq.copy_ffi(), rounding.as_raw()) }
}

/// Wraps: av_rescale_rnd
#[must_use]
pub fn av_rescale_rnd(a: i64, b: i64, c: i64, rounding: AVRounding) -> i64 {
    // SAFETY: `AVRounding` only constructs modes accepted by this C routine.
    unsafe { ffi::av_rescale_rnd(a, b, c, rounding.as_raw()) }
}

#[cfg(test)]
mod scheduled_tests {
    use super::*;
    use crate::rational::AVRational;

    #[test]
    fn rescaling_accepts_only_valid_rounding_modes() {
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::UP), 2);
        let ms = AVRational::new(1, 1000);
        let seconds = AVRational::new(1, 1);
        assert_eq!(av_rescale_q(1500, ms.as_ref(), seconds.as_ref()), 2);
        assert_eq!(
            av_rescale_q_rnd(1500, ms.as_ref(), seconds.as_ref(), AVRounding::DOWN),
            1
        );
    }

    #[test]
    fn each_rounding_mode_rounds_the_way_its_name_claims() {
        // The constants are checked against what C does with them, not
        // against the bindings they were built from: 3/2 separates every mode.
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::ZERO), 1);
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::INF), 2);
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::DOWN), 1);
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::UP), 2);
        assert_eq!(av_rescale_rnd(3, 1, 2, AVRounding::NEAR_INF), 2);

        // Negative inputs separate toward-zero from toward-negative-infinity,
        // which the positive cases above cannot.
        assert_eq!(av_rescale_rnd(-3, 1, 2, AVRounding::ZERO), -1);
        assert_eq!(av_rescale_rnd(-3, 1, 2, AVRounding::INF), -2);
        assert_eq!(av_rescale_rnd(-3, 1, 2, AVRounding::DOWN), -2);
        assert_eq!(av_rescale_rnd(-3, 1, 2, AVRounding::UP), -1);

        // The pass-through flag is what `with_pass_minmax` claims it is: the
        // extremes come back untouched instead of being rescaled.
        assert_eq!(
            av_rescale_rnd(i64::MAX, 1, 2, AVRounding::NEAR_INF.with_pass_minmax()),
            i64::MAX
        );
        assert_eq!(
            av_rescale_rnd(i64::MIN, 1, 2, AVRounding::NEAR_INF.with_pass_minmax()),
            i64::MIN
        );
        // Without it, `i64::MAX` is rescaled like any other value.
        assert_eq!(
            av_rescale_rnd(i64::MAX, 1, 2, AVRounding::ZERO),
            i64::MAX / 2
        );
        // And the flag changes nothing for an ordinary value.
        assert_eq!(
            av_rescale_rnd(3, 1, 2, AVRounding::UP.with_pass_minmax()),
            2
        );
    }

    #[test]
    fn rescaling_answers_its_sentinel_rather_than_overflowing() {
        // `AVRounding` makes the invalid mode words unconstructible, so what
        // is left for safe code to reach is the argument range. C guards it
        // itself: a non-positive divisor, a negative multiplier and a product
        // that leaves `int64_t` all return `INT64_MIN` instead of trapping, so
        // these need no wrapper-side rejection — this pins that they stay
        // total under the campaign's UBSan build.
        assert_eq!(av_rescale_rnd(1, 1, 0, AVRounding::ZERO), i64::MIN);
        assert_eq!(av_rescale_rnd(1, 1, -1, AVRounding::ZERO), i64::MIN);
        assert_eq!(av_rescale_rnd(1, -1, 1, AVRounding::ZERO), i64::MIN);
        assert_eq!(
            av_rescale_rnd(i64::MAX, i64::MAX, 1, AVRounding::ZERO),
            i64::MIN
        );
        assert_eq!(
            av_rescale_rnd(i64::MIN, i64::MAX, 1, AVRounding::ZERO),
            i64::MIN
        );

        // `av_rescale_q*` reach the same guard through the cross product of
        // the two time bases, so an undefined or reversed base is an answer
        // too. `int * (int64_t)int` cannot overflow, so nothing upstream of it
        // can.
        let undefined = AVRational::new(0, 0);
        let ms = AVRational::new(1, 1000);
        assert_eq!(
            av_rescale_q(1500, undefined.as_ref(), ms.as_ref()),
            i64::MIN
        );
        assert_eq!(
            av_rescale_q(1500, ms.as_ref(), undefined.as_ref()),
            i64::MIN
        );
        let extreme = AVRational::new(i32::MIN, i32::MIN);
        assert_eq!(
            av_rescale_q_rnd(i64::MIN, extreme.as_ref(), extreme.as_ref(), AVRounding::UP),
            -i64::MAX
        );
    }
}
