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
mod tests {
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
