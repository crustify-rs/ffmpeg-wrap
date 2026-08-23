//! Wrappers for libavutil rescaling utilities.

use crate::ffi;

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
