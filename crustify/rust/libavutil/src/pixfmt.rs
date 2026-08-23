//! Libavutil pixel format types.

use crate::ffi;

/// Wraps: AVColorPrimaries
///
/// Identifies the chromaticity coordinates of source color primaries. The
/// transparent representation preserves extension and unknown values without
/// turning an unfamiliar C value into an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorPrimaries(ffi::AVColorPrimaries);

impl AVColorPrimaries {
    pub const RESERVED0: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_RESERVED0);
    pub const BT709: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT709);
    pub const UNSPECIFIED: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_UNSPECIFIED);
    pub const RESERVED: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_RESERVED);
    pub const BT470M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT470M);
    pub const BT470BG: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT470BG);
    pub const SMPTE170M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE170M);
    pub const SMPTE240M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE240M);
    pub const FILM: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_FILM);
    pub const BT2020: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT2020);
    pub const SMPTE428: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE428);
    pub const SMPTEST428_1: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTEST428_1);
    pub const SMPTE431: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE431);
    pub const SMPTE432: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE432);
    pub const EBU3213: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_EBU3213);
    pub const JEDEC_P22: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_JEDEC_P22);
    /// Sentinel for the number of base values; not part of the stable C ABI.
    pub const NB: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_NB);
    pub const EXT_BASE: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_EXT_BASE);
    pub const V_GAMUT: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_V_GAMUT);
    /// Sentinel for the number of extension values; not part of the stable C ABI.
    pub const EXT_NB: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_EXT_NB);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVColorPrimaries) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVColorPrimaries {
        self.0
    }
}

impl From<ffi::AVColorPrimaries> for AVColorPrimaries {
    fn from(raw: ffi::AVColorPrimaries) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVColorPrimaries> for ffi::AVColorPrimaries {
    fn from(value: AVColorPrimaries) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVColorRange
///
/// Describes whether visual content uses narrow, full, or unspecified sample
/// ranges. The transparent representation keeps unknown C values representable
/// for forward compatibility.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorRange(ffi::AVColorRange);

impl AVColorRange {
    pub const UNSPECIFIED: Self = Self(ffi::AVColorRange_AVCOL_RANGE_UNSPECIFIED);
    /// Narrow or limited range content.
    pub const MPEG: Self = Self(ffi::AVColorRange_AVCOL_RANGE_MPEG);
    /// Full range content.
    pub const JPEG: Self = Self(ffi::AVColorRange_AVCOL_RANGE_JPEG);
    /// Sentinel for the number of values; not part of the stable C ABI.
    pub const NB: Self = Self(ffi::AVColorRange_AVCOL_RANGE_NB);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVColorRange) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVColorRange {
        self.0
    }
}

impl From<ffi::AVColorRange> for AVColorRange {
    fn from(raw: ffi::AVColorRange) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVColorRange> for ffi::AVColorRange {
    fn from(value: AVColorRange) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn color_primaries_is_layout_compatible_and_round_trips() {
        assert_eq!(
            size_of::<AVColorPrimaries>(),
            size_of::<ffi::AVColorPrimaries>()
        );
        assert_eq!(
            align_of::<AVColorPrimaries>(),
            align_of::<ffi::AVColorPrimaries>()
        );
        assert_eq!(
            AVColorPrimaries::BT2020.as_raw(),
            ffi::AVColorPrimaries_AVCOL_PRI_BT2020
        );
        assert_eq!(AVColorPrimaries::SMPTE428, AVColorPrimaries::SMPTEST428_1);
        assert_eq!(AVColorPrimaries::EBU3213, AVColorPrimaries::JEDEC_P22);

        let unknown = ffi::AVColorPrimaries::MAX;
        assert_eq!(AVColorPrimaries::from_raw(unknown).as_raw(), unknown);
    }

    #[test]
    fn color_range_is_layout_compatible_and_round_trips() {
        assert_eq!(size_of::<AVColorRange>(), size_of::<ffi::AVColorRange>());
        assert_eq!(align_of::<AVColorRange>(), align_of::<ffi::AVColorRange>());
        assert_eq!(
            AVColorRange::JPEG.as_raw(),
            ffi::AVColorRange_AVCOL_RANGE_JPEG
        );

        let unknown = ffi::AVColorRange::MAX;
        assert_eq!(AVColorRange::from_raw(unknown).as_raw(), unknown);
    }
}
