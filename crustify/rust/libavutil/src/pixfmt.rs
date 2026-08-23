//! Libavutil pixel format types.

use crate::ffi;

/// Wraps: AVChromaLocation
///
/// Identifies where chroma samples are positioned relative to luma samples.
/// The transparent integer representation preserves values introduced by
/// newer libavutil versions instead of turning an unfamiliar C value into an
/// invalid Rust enum discriminant.
///
/// `AVCHROMA_LOC_NB` is deliberately not exposed: C documents it as not part
/// of the ABI, so a Rust constant for it would promise stability libavutil
/// does not offer. Values at or above it are exactly the ones for which
/// [`av_chroma_location_name`](crate::pixdesc::av_chroma_location_name)
/// returns `None`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVChromaLocation(ffi::AVChromaLocation);

impl AVChromaLocation {
    /// The chroma location is unspecified.
    pub const UNSPECIFIED: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_UNSPECIFIED);

    /// Chroma is horizontally co-sited with the left luma sample.
    pub const LEFT: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_LEFT);

    /// Chroma is centered between horizontal luma samples.
    pub const CENTER: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_CENTER);

    /// Chroma is co-sited with the top-left luma sample.
    pub const TOP_LEFT: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_TOPLEFT);

    /// Chroma is horizontally centered and vertically co-sited at the top.
    pub const TOP: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_TOP);

    /// Chroma is co-sited with the bottom-left luma sample.
    pub const BOTTOM_LEFT: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_BOTTOMLEFT);

    /// Chroma is horizontally centered and vertically co-sited at the bottom.
    pub const BOTTOM: Self = Self(ffi::AVChromaLocation_AVCHROMA_LOC_BOTTOM);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVChromaLocation) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVChromaLocation {
        self.0
    }
}

impl From<ffi::AVChromaLocation> for AVChromaLocation {
    fn from(raw: ffi::AVChromaLocation) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVChromaLocation> for ffi::AVChromaLocation {
    fn from(location: AVChromaLocation) -> Self {
        location.as_raw()
    }
}

/// Wraps: AVColorPrimaries
///
/// Identifies the chromaticity coordinates of the source color primaries, as
/// numbered by ITU-T H.273. The transparent representation preserves values
/// this crate does not name — the base range has unassigned gaps, libavutil
/// adds a custom extension range above [`EXT_BASE`](Self::EXT_BASE), and a
/// newer linked library may define more — instead of turning an unfamiliar C
/// value into an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorPrimaries(ffi::AVColorPrimaries);

impl AVColorPrimaries {
    /// Reserved by H.273.
    pub const RESERVED0: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_RESERVED0);

    /// Also ITU-R BT1361 / IEC 61966-2-4 / SMPTE RP 177 Annex B.
    pub const BT709: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT709);

    /// The primaries are unknown or deliberately unset.
    pub const UNSPECIFIED: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_UNSPECIFIED);

    /// Reserved by H.273.
    pub const RESERVED: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_RESERVED);

    /// Also FCC Title 47 Code of Federal Regulations 73.682 (a)(20).
    pub const BT470M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT470M);

    /// Also ITU-R BT601-6 625 / ITU-R BT1358 625 / ITU-R BT1700 625 PAL and SECAM.
    pub const BT470BG: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT470BG);

    /// Also ITU-R BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC.
    pub const SMPTE170M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE170M);

    /// The same primaries as [`SMPTE170M`](Self::SMPTE170M), also called
    /// "SMPTE C" even though it uses D65. It is a distinct H.273 value, so it
    /// does not compare equal to [`SMPTE170M`](Self::SMPTE170M).
    pub const SMPTE240M: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE240M);

    /// Colour filters using Illuminant C.
    pub const FILM: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_FILM);

    /// ITU-R BT2020.
    pub const BT2020: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_BT2020);

    /// SMPTE ST 428-1 (CIE 1931 XYZ).
    pub const SMPTE428: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE428);

    /// Alternative C spelling of [`SMPTE428`](Self::SMPTE428); the same value.
    pub const SMPTEST428_1: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTEST428_1);

    /// SMPTE ST 431-2 (2011) / DCI P3.
    pub const SMPTE431: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE431);

    /// SMPTE ST 432-1 (2010) / P3 D65 / Display P3.
    pub const SMPTE432: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_SMPTE432);

    /// EBU Tech. 3213-E / one of the JEDEC P22 group phosphors.
    pub const EBU3213: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_EBU3213);

    /// Alternative C spelling of [`EBU3213`](Self::EBU3213); the same value.
    pub const JEDEC_P22: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_JEDEC_P22);

    /// Exclusive upper bound of the H.273 base range; not part of the ABI.
    /// It is a bound, not a count: the values between
    /// [`SMPTE432`](Self::SMPTE432) and [`EBU3213`](Self::EBU3213) are
    /// unassigned and name nothing.
    pub const NB: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_NB);

    /// First value of libavutil's custom extension range, which lies above
    /// every H.273 value and is not part of that standard.
    pub const EXT_BASE: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_EXT_BASE);

    /// V-Gamut, the first custom extension and equal to
    /// [`EXT_BASE`](Self::EXT_BASE); libavutil names it `vgamut`.
    pub const V_GAMUT: Self = Self(ffi::AVColorPrimaries_AVCOL_PRI_V_GAMUT);

    /// Exclusive upper bound of the custom extension range; not part of the
    /// ABI.
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
/// ranges. The transparent representation keeps values unknown to this crate
/// representable, so a value from a newer linked libavutil round-trips instead
/// of becoming an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorRange(ffi::AVColorRange);

impl AVColorRange {
    /// The sample range is unknown or deliberately unset.
    pub const UNSPECIFIED: Self = Self(ffi::AVColorRange_AVCOL_RANGE_UNSPECIFIED);

    /// Narrow or limited range content: luma is `(219 * E + 16) * 2^(n-8)`
    /// and chroma is `(224 * E + 128) * 2^(n-8)`, so 8-bit luma occupies
    /// 16-235 and 8-bit chroma 16-240.
    pub const MPEG: Self = Self(ffi::AVColorRange_AVCOL_RANGE_MPEG);

    /// Full range content: RGB and luma are `(2^n - 1) * E` and chroma is
    /// `(2^n - 1) * E + 2^(n-1)`, so 8-bit luma occupies 0-255 and 8-bit
    /// chroma 1-255.
    pub const JPEG: Self = Self(ffi::AVColorRange_AVCOL_RANGE_JPEG);

    /// Exclusive upper bound of the defined values; not part of the ABI.
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

/// Wraps: AVColorSpace
///
/// A YUV color-space identifier. The transparent integer representation is
/// intentional: C callers may pass reserved or future values, so modelling
/// this as a closed Rust `enum` would make otherwise valid FFI values invalid
/// Rust values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorSpace(ffi::AVColorSpace);

impl AVColorSpace {
    pub const RGB: Self = Self(ffi::AVColorSpace_AVCOL_SPC_RGB);
    pub const BT709: Self = Self(ffi::AVColorSpace_AVCOL_SPC_BT709);
    pub const UNSPECIFIED: Self = Self(ffi::AVColorSpace_AVCOL_SPC_UNSPECIFIED);
    pub const RESERVED: Self = Self(ffi::AVColorSpace_AVCOL_SPC_RESERVED);
    pub const FCC: Self = Self(ffi::AVColorSpace_AVCOL_SPC_FCC);
    pub const BT470BG: Self = Self(ffi::AVColorSpace_AVCOL_SPC_BT470BG);
    pub const SMPTE170M: Self = Self(ffi::AVColorSpace_AVCOL_SPC_SMPTE170M);
    pub const SMPTE240M: Self = Self(ffi::AVColorSpace_AVCOL_SPC_SMPTE240M);
    pub const YCGCO: Self = Self(ffi::AVColorSpace_AVCOL_SPC_YCGCO);
    pub const YCOCG: Self = Self(ffi::AVColorSpace_AVCOL_SPC_YCOCG);
    pub const BT2020_NCL: Self = Self(ffi::AVColorSpace_AVCOL_SPC_BT2020_NCL);
    pub const BT2020_CL: Self = Self(ffi::AVColorSpace_AVCOL_SPC_BT2020_CL);
    pub const SMPTE2085: Self = Self(ffi::AVColorSpace_AVCOL_SPC_SMPTE2085);
    pub const CHROMA_DERIVED_NCL: Self = Self(ffi::AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_NCL);
    pub const CHROMA_DERIVED_CL: Self = Self(ffi::AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_CL);
    pub const ICTCP: Self = Self(ffi::AVColorSpace_AVCOL_SPC_ICTCP);
    pub const IPT_C2: Self = Self(ffi::AVColorSpace_AVCOL_SPC_IPT_C2);
    pub const YCGCO_RE: Self = Self(ffi::AVColorSpace_AVCOL_SPC_YCGCO_RE);
    pub const YCGCO_RO: Self = Self(ffi::AVColorSpace_AVCOL_SPC_YCGCO_RO);
    /// Number of standard color-space identifiers; not part of the C ABI.
    pub const NB: Self = Self(ffi::AVColorSpace_AVCOL_SPC_NB);

    /// Preserves any value received from C, including reserved and future
    /// identifiers.
    pub const fn from_raw(raw: ffi::AVColorSpace) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVColorSpace {
        self.0
    }
}

impl From<ffi::AVColorSpace> for AVColorSpace {
    fn from(raw: ffi::AVColorSpace) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVColorSpace> for ffi::AVColorSpace {
    fn from(value: AVColorSpace) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVColorTransferCharacteristic
///
/// A color transfer characteristic. Like the corresponding C enum, this is
/// an open integer value: reserved, custom-extension and future identifiers
/// can cross the FFI boundary without creating an invalid Rust value.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVColorTransferCharacteristic(ffi::AVColorTransferCharacteristic);

impl AVColorTransferCharacteristic {
    pub const RESERVED0: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_RESERVED0);
    pub const BT709: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_BT709);
    pub const UNSPECIFIED: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_UNSPECIFIED);
    pub const RESERVED: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_RESERVED);
    pub const GAMMA22: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_GAMMA22);
    pub const GAMMA28: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_GAMMA28);
    pub const SMPTE170M: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTE170M);
    pub const SMPTE240M: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTE240M);
    pub const LINEAR: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_LINEAR);
    pub const LOG: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_LOG);
    pub const LOG_SQRT: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_LOG_SQRT);
    pub const IEC61966_2_4: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_4);
    pub const BT1361_ECG: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_BT1361_ECG);
    pub const IEC61966_2_1: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_1);
    pub const BT2020_10: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_BT2020_10);
    pub const BT2020_12: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_BT2020_12);
    pub const SMPTE2084: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTE2084);
    pub const SMPTEST2084: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTEST2084);
    pub const SMPTE428: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTE428);
    pub const SMPTEST428_1: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_SMPTEST428_1);
    pub const ARIB_STD_B67: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_ARIB_STD_B67);
    /// Number of standard transfer-characteristic identifiers; not part of
    /// the C ABI.
    pub const NB: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_NB);
    pub const EXT_BASE: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_EXT_BASE);
    pub const V_LOG: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_V_LOG);
    /// End of the custom extension range; not part of the C ABI.
    pub const EXT_NB: Self = Self(ffi::AVColorTransferCharacteristic_AVCOL_TRC_EXT_NB);

    /// Preserves any value received from C, including reserved and future
    /// identifiers.
    pub const fn from_raw(raw: ffi::AVColorTransferCharacteristic) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVColorTransferCharacteristic {
        self.0
    }
}

impl From<ffi::AVColorTransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(raw: ffi::AVColorTransferCharacteristic) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVColorTransferCharacteristic> for ffi::AVColorTransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVAlphaMode
///
/// Describes how an alpha channel relates to its color channels. The
/// transparent representation preserves unknown values introduced by newer
/// libavutil versions instead of turning them into invalid Rust enum values.
///
/// The C enum's trailing `AVALPHA_MODE_NB` is a count, documented as not part
/// of the ABI, so it is deliberately not exposed here. Values outside the three
/// named modes — including `NB` itself and the negative `AVERROR` that
/// `av_alpha_mode_from_name` returns for an unrecognized name — are
/// representable and round-trip, but name no alpha mode.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVAlphaMode(ffi::AVAlphaMode);

impl AVAlphaMode {
    /// Alpha handling is unknown, or the format has no alpha channel.
    pub const UNSPECIFIED: Self = Self(ffi::AVAlphaMode_AVALPHA_MODE_UNSPECIFIED);

    /// Color values have already been multiplied by alpha.
    pub const PREMULTIPLIED: Self = Self(ffi::AVAlphaMode_AVALPHA_MODE_PREMULTIPLIED);

    /// Alpha is independent of the color values.
    pub const STRAIGHT: Self = Self(ffi::AVAlphaMode_AVALPHA_MODE_STRAIGHT);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVAlphaMode) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVAlphaMode {
        self.0
    }
}

impl From<ffi::AVAlphaMode> for AVAlphaMode {
    fn from(raw: ffi::AVAlphaMode) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVAlphaMode> for ffi::AVAlphaMode {
    fn from(value: AVAlphaMode) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn alpha_mode_is_layout_compatible_and_round_trips() {
        assert_eq!(size_of::<AVAlphaMode>(), size_of::<ffi::AVAlphaMode>());
        assert_eq!(align_of::<AVAlphaMode>(), align_of::<ffi::AVAlphaMode>());

        // Pin the wire values `libavutil/pixfmt.h` assigns explicitly, so a
        // renumbering or a new enumerator inserted ahead of them fails here
        // rather than silently retagging pixel data.
        assert_eq!(AVAlphaMode::UNSPECIFIED.as_raw(), 0);
        assert_eq!(AVAlphaMode::PREMULTIPLIED.as_raw(), 1);
        assert_eq!(AVAlphaMode::STRAIGHT.as_raw(), 2);

        // `AVALPHA_MODE_NB` is a count, not a mode, and must stay unnamed.
        let count = ffi::AVAlphaMode_AVALPHA_MODE_NB;
        assert_eq!(count, 3);
        for mode in [
            AVAlphaMode::UNSPECIFIED,
            AVAlphaMode::PREMULTIPLIED,
            AVAlphaMode::STRAIGHT,
        ] {
            assert!(mode.as_raw() < count);
        }

        // Out-of-range values stay representable instead of becoming invalid
        // discriminants: `NB`, a future mode, and the `AVERROR(EINVAL)` that
        // `av_alpha_mode_from_name` returns through this unsigned enum.
        for unknown in [count, ffi::AVAlphaMode::MAX, (-22_i32) as ffi::AVAlphaMode] {
            assert_eq!(AVAlphaMode::from_raw(unknown).as_raw(), unknown);
            assert_eq!(
                ffi::AVAlphaMode::from(AVAlphaMode::from_raw(unknown)),
                unknown
            );
        }
    }

    #[test]
    fn color_primaries_matches_the_h273_numbering() {
        assert_eq!(
            size_of::<AVColorPrimaries>(),
            size_of::<ffi::AVColorPrimaries>()
        );
        assert_eq!(
            align_of::<AVColorPrimaries>(),
            align_of::<ffi::AVColorPrimaries>()
        );

        // Pinned against libavutil/pixfmt.h so a bindgen or header change that
        // renumbered a value is a test failure rather than a silent mismatch
        // with the linked library.
        for (value, expected) in [
            (AVColorPrimaries::RESERVED0, 0),
            (AVColorPrimaries::BT709, 1),
            (AVColorPrimaries::UNSPECIFIED, 2),
            (AVColorPrimaries::RESERVED, 3),
            (AVColorPrimaries::BT470M, 4),
            (AVColorPrimaries::BT470BG, 5),
            (AVColorPrimaries::SMPTE170M, 6),
            (AVColorPrimaries::SMPTE240M, 7),
            (AVColorPrimaries::FILM, 8),
            (AVColorPrimaries::BT2020, 9),
            (AVColorPrimaries::SMPTE428, 10),
            (AVColorPrimaries::SMPTE431, 11),
            (AVColorPrimaries::SMPTE432, 12),
            (AVColorPrimaries::EBU3213, 22),
            (AVColorPrimaries::NB, 23),
            (AVColorPrimaries::EXT_BASE, 256),
            (AVColorPrimaries::EXT_NB, 257),
        ] {
            assert_eq!(value.as_raw(), expected);
        }

        // The C header defines these as alternative spellings, not as values
        // of their own.
        assert_eq!(AVColorPrimaries::SMPTE428, AVColorPrimaries::SMPTEST428_1);
        assert_eq!(AVColorPrimaries::EBU3213, AVColorPrimaries::JEDEC_P22);
        assert_eq!(AVColorPrimaries::V_GAMUT, AVColorPrimaries::EXT_BASE);

        // `NB` bounds the base range but does not count it: the values below
        // it between SMPTE432 and EBU3213 are unassigned.
        assert!(AVColorPrimaries::SMPTE432 < AVColorPrimaries::EBU3213);
        assert!(AVColorPrimaries::NB < AVColorPrimaries::EXT_BASE);

        let unknown = ffi::AVColorPrimaries::MAX;
        assert_eq!(AVColorPrimaries::from_raw(unknown).as_raw(), unknown);
    }

    #[test]
    fn color_primaries_cross_the_abi_over_the_whole_value_space() {
        use crate::pixdesc::av_color_primaries_name;

        // A named base value, and the extension value 256 — which only
        // resolves if the wrapper hands C an `unsigned int` of the width the
        // linked library's enum uses.
        assert_eq!(
            av_color_primaries_name(AVColorPrimaries::BT709),
            Some(c"bt709")
        );
        assert_eq!(
            av_color_primaries_name(AVColorPrimaries::V_GAMUT),
            Some(c"vgamut")
        );

        // Unnamed values stay valid Rust values and are rejected by C rather
        // than indexing its tables: 13 falls in the base range's unassigned
        // hole, `NB` and `EXT_NB` are the exclusive bounds, and the gap
        // between the ranges is empty.
        for value in [
            AVColorPrimaries::from_raw(13),
            AVColorPrimaries::NB,
            AVColorPrimaries::from_raw(255),
            AVColorPrimaries::EXT_NB,
            AVColorPrimaries::from_raw(ffi::AVColorPrimaries::MAX),
        ] {
            assert_eq!(av_color_primaries_name(value), None);
        }
    }

    #[test]
    fn color_range_matches_the_c_numbering_and_crosses_the_abi() {
        use crate::pixdesc::av_color_range_name;

        assert_eq!(size_of::<AVColorRange>(), size_of::<ffi::AVColorRange>());
        assert_eq!(align_of::<AVColorRange>(), align_of::<ffi::AVColorRange>());

        for (value, expected) in [
            (AVColorRange::UNSPECIFIED, 0),
            (AVColorRange::MPEG, 1),
            (AVColorRange::JPEG, 2),
            (AVColorRange::NB, 3),
        ] {
            assert_eq!(value.as_raw(), expected);
        }

        // libavutil names the narrow and full ranges after their container
        // conventions; the mapping proves the values reach C unchanged.
        assert_eq!(
            av_color_range_name(AVColorRange::UNSPECIFIED),
            Some(c"unknown")
        );
        assert_eq!(av_color_range_name(AVColorRange::MPEG), Some(c"tv"));
        assert_eq!(av_color_range_name(AVColorRange::JPEG), Some(c"pc"));

        // `NB` is the exclusive bound, so C names nothing for it or above.
        assert_eq!(av_color_range_name(AVColorRange::NB), None);

        let unknown = ffi::AVColorRange::MAX;
        assert_eq!(AVColorRange::from_raw(unknown).as_raw(), unknown);
        assert_eq!(av_color_range_name(AVColorRange::from_raw(unknown)), None);
    }

    #[test]
    fn color_space_is_layout_compatible_and_open() {
        assert_eq!(size_of::<AVColorSpace>(), size_of::<ffi::AVColorSpace>());
        assert_eq!(align_of::<AVColorSpace>(), align_of::<ffi::AVColorSpace>());
        assert_eq!(AVColorSpace::YCOCG, AVColorSpace::YCGCO);

        let future = ffi::AVColorSpace::MAX;
        assert_eq!(AVColorSpace::from_raw(future).as_raw(), future);
    }

    #[test]
    fn transfer_characteristic_is_layout_compatible_and_open() {
        assert_eq!(
            size_of::<AVColorTransferCharacteristic>(),
            size_of::<ffi::AVColorTransferCharacteristic>()
        );
        assert_eq!(
            align_of::<AVColorTransferCharacteristic>(),
            align_of::<ffi::AVColorTransferCharacteristic>()
        );
        assert_eq!(
            AVColorTransferCharacteristic::SMPTEST2084,
            AVColorTransferCharacteristic::SMPTE2084
        );
        assert_eq!(
            AVColorTransferCharacteristic::SMPTEST428_1,
            AVColorTransferCharacteristic::SMPTE428
        );
        assert_eq!(
            AVColorTransferCharacteristic::V_LOG,
            AVColorTransferCharacteristic::EXT_BASE
        );

        let future = ffi::AVColorTransferCharacteristic::MAX;
        assert_eq!(
            AVColorTransferCharacteristic::from_raw(future).as_raw(),
            future
        );
    }

    #[test]
    fn color_space_identifiers_are_the_ones_libavutil_resolves() {
        use crate::pixdesc::av_color_space_name;

        assert_eq!(av_color_space_name(AVColorSpace::RGB), Some(c"gbr"));
        assert_eq!(av_color_space_name(AVColorSpace::BT709), Some(c"bt709"));
        assert_eq!(
            av_color_space_name(AVColorSpace::CHROMA_DERIVED_CL),
            Some(c"chroma-derived-c")
        );
        assert_eq!(
            av_color_space_name(AVColorSpace::YCGCO_RO),
            Some(c"ycgco-ro")
        );
        // `YCOCG` is an alternate spelling of `YCGCO`, not a separate value.
        assert_eq!(av_color_space_name(AVColorSpace::YCOCG), Some(c"ycgco"));

        // `NB` is a count, so libavutil names it no more than it names a
        // reserved or future identifier.
        assert_eq!(av_color_space_name(AVColorSpace::NB), None);
        assert_eq!(
            av_color_space_name(AVColorSpace::from_raw(ffi::AVColorSpace::MAX)),
            None
        );
    }

    #[test]
    fn transfer_characteristic_identifiers_are_the_ones_libavutil_resolves() {
        use crate::pixdesc::av_color_transfer_name;

        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::BT709),
            Some(c"bt709")
        );
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::IEC61966_2_1),
            Some(c"iec61966-2-1")
        );
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::ARIB_STD_B67),
            Some(c"arib-std-b67")
        );
        // The custom extensions sit far above the standard identifiers, so the
        // constant carries the offset rather than a table index.
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::V_LOG),
            Some(c"vlog")
        );

        // Both sentinels bound a range rather than naming a value, and the gap
        // between them holds identifiers libavutil does not name either.
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::NB),
            None
        );
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::EXT_NB),
            None
        );
        let between = AVColorTransferCharacteristic::from_raw(100);
        assert_eq!(av_color_transfer_name(between), None);
        assert_eq!(between.as_raw(), 100);
    }
}

/// Wraps: AVPixelFormat
///
/// ABI-compatible pixel-format value. This is an integer newtype rather than a
/// Rust enum because libavutil may pass values introduced by a newer linked
/// version. Unknown values therefore remain valid and round-trip through
/// [`from_raw`](Self::from_raw) and [`as_raw`](Self::as_raw).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVPixelFormat(ffi::AVPixelFormat);

impl AVPixelFormat {
    /// The sentinel meaning "no pixel format"; also the terminator libavutil
    /// writes at the end of a format list.
    pub const NONE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NONE);
    pub const YUV420P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P);
    pub const YUYV422: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUYV422);
    pub const RGB24: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB24);
    pub const BGR24: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR24);
    pub const YUV422P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P);
    pub const YUV444P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P);
    pub const YUV410P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV410P);
    pub const YUV411P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV411P);
    pub const GRAY8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY8);
    pub const MONOWHITE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_MONOWHITE);
    pub const MONOBLACK: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_MONOBLACK);
    pub const PAL8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_PAL8);
    /// Deprecated in C in favour of [`YUV420P`](Self::YUV420P) plus
    /// [`AVColorRange::JPEG`].
    pub const YUVJ420P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ420P);
    /// Deprecated in C in favour of [`YUV422P`](Self::YUV422P) plus
    /// [`AVColorRange::JPEG`].
    pub const YUVJ422P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ422P);
    /// Deprecated in C in favour of [`YUV444P`](Self::YUV444P) plus
    /// [`AVColorRange::JPEG`].
    pub const YUVJ444P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ444P);
    pub const UYVY422: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_UYVY422);
    pub const UYYVYY411: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_UYYVYY411);
    pub const BGR8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR8);
    pub const BGR4: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR4);
    pub const BGR4_BYTE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR4_BYTE);
    pub const RGB8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB8);
    pub const RGB4: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB4);
    pub const RGB4_BYTE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB4_BYTE);
    pub const NV12: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV12);
    pub const NV21: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV21);
    pub const ARGB: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_ARGB);
    pub const RGBA: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBA);
    pub const ABGR: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_ABGR);
    pub const BGRA: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGRA);
    pub const GRAY16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY16BE);
    pub const GRAY16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY16LE);
    pub const YUV440P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV440P);
    /// Deprecated in C in favour of [`YUV440P`](Self::YUV440P) plus
    /// [`AVColorRange::JPEG`].
    pub const YUVJ440P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ440P);
    pub const YUVA420P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P);
    pub const RGB48BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB48BE);
    pub const RGB48LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB48LE);
    pub const RGB565BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB565BE);
    pub const RGB565LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB565LE);
    pub const RGB555BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB555BE);
    pub const RGB555LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB555LE);
    pub const BGR565BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR565BE);
    pub const BGR565LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR565LE);
    pub const BGR555BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR555BE);
    pub const BGR555LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR555LE);
    pub const VAAPI: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VAAPI);
    pub const YUV420P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P16LE);
    pub const YUV420P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P16BE);
    pub const YUV422P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P16LE);
    pub const YUV422P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P16BE);
    pub const YUV444P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P16LE);
    pub const YUV444P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P16BE);
    pub const DXVA2_VLD: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_DXVA2_VLD);
    pub const RGB444LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB444LE);
    pub const RGB444BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB444BE);
    pub const BGR444LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR444LE);
    pub const BGR444BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR444BE);
    pub const YA8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YA8);
    pub const Y400A: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y400A);
    pub const GRAY8A: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY8A);
    pub const BGR48BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR48BE);
    pub const BGR48LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR48LE);
    pub const YUV420P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P9BE);
    pub const YUV420P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P9LE);
    pub const YUV420P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P10BE);
    pub const YUV420P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P10LE);
    pub const YUV422P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P10BE);
    pub const YUV422P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P10LE);
    pub const YUV444P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P9BE);
    pub const YUV444P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P9LE);
    pub const YUV444P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P10BE);
    pub const YUV444P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P10LE);
    pub const YUV422P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P9BE);
    pub const YUV422P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P9LE);
    pub const GBRP: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP);
    pub const GBR24P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBR24P);
    pub const GBRP9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP9BE);
    pub const GBRP9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP9LE);
    pub const GBRP10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP10BE);
    pub const GBRP10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP10LE);
    pub const GBRP16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP16BE);
    pub const GBRP16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP16LE);
    pub const YUVA422P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P);
    pub const YUVA444P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P);
    pub const YUVA420P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P9BE);
    pub const YUVA420P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P9LE);
    pub const YUVA422P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P9BE);
    pub const YUVA422P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P9LE);
    pub const YUVA444P9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P9BE);
    pub const YUVA444P9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P9LE);
    pub const YUVA420P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P10BE);
    pub const YUVA420P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P10LE);
    pub const YUVA422P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P10BE);
    pub const YUVA422P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P10LE);
    pub const YUVA444P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P10BE);
    pub const YUVA444P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P10LE);
    pub const YUVA420P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P16BE);
    pub const YUVA420P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA420P16LE);
    pub const YUVA422P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P16BE);
    pub const YUVA422P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P16LE);
    pub const YUVA444P16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P16BE);
    pub const YUVA444P16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P16LE);
    pub const VDPAU: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VDPAU);
    pub const XYZ12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XYZ12LE);
    pub const XYZ12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XYZ12BE);
    pub const NV16: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV16);
    pub const NV20LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV20LE);
    pub const NV20BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV20BE);
    pub const RGBA64BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBA64BE);
    pub const RGBA64LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBA64LE);
    pub const BGRA64BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGRA64BE);
    pub const BGRA64LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGRA64LE);
    pub const YVYU422: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YVYU422);
    pub const YA16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YA16BE);
    pub const YA16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YA16LE);
    pub const GBRAP: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP);
    pub const GBRAP16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP16BE);
    pub const GBRAP16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP16LE);
    pub const QSV: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_QSV);
    pub const MMAL: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_MMAL);
    pub const D3D11VA_VLD: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_D3D11VA_VLD);
    pub const CUDA: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_CUDA);
    pub const _0RGB: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_0RGB);
    pub const RGB0: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB0);
    pub const _0BGR: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_0BGR);
    pub const BGR0: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BGR0);
    pub const YUV420P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P12BE);
    pub const YUV420P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P12LE);
    pub const YUV420P14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P14BE);
    pub const YUV420P14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV420P14LE);
    pub const YUV422P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P12BE);
    pub const YUV422P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P12LE);
    pub const YUV422P14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P14BE);
    pub const YUV422P14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV422P14LE);
    pub const YUV444P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P12BE);
    pub const YUV444P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P12LE);
    pub const YUV444P14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P14BE);
    pub const YUV444P14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P14LE);
    pub const GBRP12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP12BE);
    pub const GBRP12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP12LE);
    pub const GBRP14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP14BE);
    pub const GBRP14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP14LE);
    /// Deprecated in C in favour of [`YUV411P`](Self::YUV411P) plus
    /// [`AVColorRange::JPEG`].
    pub const YUVJ411P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ411P);
    pub const BAYER_BGGR8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_BGGR8);
    pub const BAYER_RGGB8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_RGGB8);
    pub const BAYER_GBRG8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GBRG8);
    pub const BAYER_GRBG8: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GRBG8);
    pub const BAYER_BGGR16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16LE);
    pub const BAYER_BGGR16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16BE);
    pub const BAYER_RGGB16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16LE);
    pub const BAYER_RGGB16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16BE);
    pub const BAYER_GBRG16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16LE);
    pub const BAYER_GBRG16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16BE);
    pub const BAYER_GRBG16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16LE);
    pub const BAYER_GRBG16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16BE);
    pub const YUV440P10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV440P10LE);
    pub const YUV440P10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV440P10BE);
    pub const YUV440P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV440P12LE);
    pub const YUV440P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV440P12BE);
    pub const AYUV64LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_AYUV64LE);
    pub const AYUV64BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_AYUV64BE);
    pub const VIDEOTOOLBOX: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VIDEOTOOLBOX);
    pub const P010LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P010LE);
    pub const P010BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P010BE);
    pub const GBRAP12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP12BE);
    pub const GBRAP12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP12LE);
    pub const GBRAP10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP10BE);
    pub const GBRAP10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP10LE);
    pub const MEDIACODEC: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_MEDIACODEC);
    pub const GRAY12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY12BE);
    pub const GRAY12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY12LE);
    pub const GRAY10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY10BE);
    pub const GRAY10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY10LE);
    pub const P016LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P016LE);
    pub const P016BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P016BE);
    pub const D3D11: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_D3D11);
    pub const GRAY9BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY9BE);
    pub const GRAY9LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY9LE);
    pub const GBRPF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRPF32BE);
    pub const GBRPF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRPF32LE);
    pub const GBRAPF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAPF32BE);
    pub const GBRAPF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAPF32LE);
    pub const DRM_PRIME: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_DRM_PRIME);
    pub const OPENCL: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_OPENCL);
    pub const GRAY14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY14BE);
    pub const GRAY14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY14LE);
    pub const GRAYF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAYF32BE);
    pub const GRAYF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAYF32LE);
    pub const YUVA422P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P12BE);
    pub const YUVA422P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA422P12LE);
    pub const YUVA444P12BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P12BE);
    pub const YUVA444P12LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVA444P12LE);
    pub const NV24: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV24);
    pub const NV42: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NV42);
    pub const VULKAN: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VULKAN);
    pub const Y210BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y210BE);
    pub const Y210LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y210LE);
    pub const X2RGB10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_X2RGB10LE);
    pub const X2RGB10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_X2RGB10BE);
    pub const X2BGR10LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_X2BGR10LE);
    pub const X2BGR10BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_X2BGR10BE);
    pub const P210BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P210BE);
    pub const P210LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P210LE);
    pub const P410BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P410BE);
    pub const P410LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P410LE);
    pub const P216BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P216BE);
    pub const P216LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P216LE);
    pub const P416BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P416BE);
    pub const P416LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P416LE);
    pub const VUYA: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VUYA);
    pub const RGBAF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBAF16BE);
    pub const RGBAF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBAF16LE);
    pub const VUYX: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VUYX);
    pub const P012LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P012LE);
    pub const P012BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P012BE);
    pub const Y212BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y212BE);
    pub const Y212LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y212LE);
    pub const XV30BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV30BE);
    pub const XV30LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV30LE);
    pub const XV36BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV36BE);
    pub const XV36LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV36LE);
    pub const RGBF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBF32BE);
    pub const RGBF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBF32LE);
    pub const RGBAF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBAF32BE);
    pub const RGBAF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBAF32LE);
    pub const P212BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P212BE);
    pub const P212LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P212LE);
    pub const P412BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P412BE);
    pub const P412LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_P412LE);
    pub const GBRAP14BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP14BE);
    pub const GBRAP14LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP14LE);
    pub const D3D12: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_D3D12);
    pub const AYUV: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_AYUV);
    pub const UYVA: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_UYVA);
    pub const VYU444: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_VYU444);
    pub const V30XBE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_V30XBE);
    pub const V30XLE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_V30XLE);
    pub const RGBF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBF16BE);
    pub const RGBF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBF16LE);
    pub const RGBA128BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBA128BE);
    pub const RGBA128LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGBA128LE);
    pub const RGB96BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB96BE);
    pub const RGB96LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_RGB96LE);
    pub const Y216BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y216BE);
    pub const Y216LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_Y216LE);
    pub const XV48BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV48BE);
    pub const XV48LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_XV48LE);
    pub const GBRPF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRPF16BE);
    pub const GBRPF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRPF16LE);
    pub const GBRAPF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAPF16BE);
    pub const GBRAPF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAPF16LE);
    pub const GRAYF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAYF16BE);
    pub const GRAYF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAYF16LE);
    pub const AMF_SURFACE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_AMF_SURFACE);
    pub const GRAY32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY32BE);
    pub const GRAY32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GRAY32LE);
    pub const YAF32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YAF32BE);
    pub const YAF32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YAF32LE);
    pub const YAF16BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YAF16BE);
    pub const YAF16LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YAF16LE);
    pub const GBRAP32BE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP32BE);
    pub const GBRAP32LE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRAP32LE);
    pub const YUV444P10MSBBE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P10MSBBE);
    pub const YUV444P10MSBLE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P10MSBLE);
    pub const YUV444P12MSBBE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P12MSBBE);
    pub const YUV444P12MSBLE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUV444P12MSBLE);
    pub const GBRP10MSBBE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP10MSBBE);
    pub const GBRP10MSBLE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP10MSBLE);
    pub const GBRP12MSBBE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP12MSBBE);
    pub const GBRP12MSBLE: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_GBRP12MSBLE);
    pub const OHCODEC: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_OHCODEC);
    pub const CUARRAY: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_CUARRAY);
    /// The number of pixel formats known to the headers this crate was built
    /// against. C documents it as unusable for shared linking, and that
    /// applies here: the linked libavutil may know more formats than this
    /// compile-time bound, so it is not a valid upper limit on a value
    /// received from C.
    pub const NB: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_NB);

    /// Wraps a raw libavutil pixel-format value, including unknown values.
    #[must_use]
    pub const fn from_raw(value: ffi::AVPixelFormat) -> Self {
        Self(value)
    }

    /// Returns the raw libavutil pixel-format value.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVPixelFormat {
        self.0
    }
}

impl Default for AVPixelFormat {
    fn default() -> Self {
        Self::NONE
    }
}

// SAFETY: the newtype is `repr(transparent)` over `ffi::AVPixelFormat`, a
// `c_int`, so every bit pattern of it is a valid `AVPixelFormat` — unknown
// values included, which is exactly what the open representation promises.
unsafe impl ffibox::CElem for AVPixelFormat {}

impl From<ffi::AVPixelFormat> for AVPixelFormat {
    fn from(value: ffi::AVPixelFormat) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVPixelFormat> for ffi::AVPixelFormat {
    fn from(value: AVPixelFormat) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod pixel_format_tests {
    use core::ffi::c_int;
    use core::mem::{align_of, size_of};

    use super::*;
    use crate::pixdesc::{
        av_get_pix_fmt, av_get_pix_fmt_name, av_pix_fmt_desc_get_id, av_pix_fmt_desc_next,
    };

    #[test]
    fn layout_is_the_c_enum_abi() {
        // `repr(transparent)` already ties the newtype to `ffi::AVPixelFormat`,
        // so the claim worth testing is the one it is transparent over: this
        // enum travels as a plain `int` in the C ABI.
        assert_eq!(size_of::<AVPixelFormat>(), size_of::<c_int>());
        assert_eq!(align_of::<AVPixelFormat>(), align_of::<c_int>());
    }

    #[test]
    fn named_values_and_aliases_match_the_bindings() {
        assert_eq!(AVPixelFormat::NONE.as_raw(), -1);
        assert_eq!(AVPixelFormat::YUV420P.as_raw(), 0);
        assert_eq!(AVPixelFormat::Y400A, AVPixelFormat::YA8);
        assert_eq!(AVPixelFormat::GRAY8A, AVPixelFormat::YA8);
        assert_eq!(AVPixelFormat::GBR24P, AVPixelFormat::GBRP);
    }

    #[test]
    fn unknown_values_round_trip() {
        let raw = ffi::AVPixelFormat_AV_PIX_FMT_NB + 17;
        assert_eq!(AVPixelFormat::from_raw(raw).as_raw(), raw);
        assert_eq!(ffi::AVPixelFormat::from(AVPixelFormat::from(raw)), raw);
    }

    /// The constants come from the headers; the values arriving at runtime come
    /// from the linked shared object. Walk the library's own descriptor table
    /// and check the two agree on every format it publishes.
    #[test]
    fn named_constants_agree_with_the_linked_library() {
        let mut entry = av_pix_fmt_desc_next(None);
        let mut seen = 0usize;
        while let Some(descriptor) = entry {
            let format = av_pix_fmt_desc_get_id(descriptor);
            assert_ne!(format, AVPixelFormat::NONE);

            // A name the library recognises must map back to the same value,
            // so a constant bound to the wrong enumerator cannot hide here.
            let name = av_get_pix_fmt_name(format).expect("table entry is named");
            assert_eq!(av_get_pix_fmt(name), format);

            seen += 1;
            entry = av_pix_fmt_desc_next(Some(descriptor));
        }
        assert!(seen > 0, "descriptor table is nonempty");
    }

    /// Each constant is bound to a bindgen name, so a constant carrying the
    /// wrong enumerator compiles cleanly. Ask the linked library what these
    /// values actually are; the spread covers the start, the middle, the
    /// deprecated group, the leading-digit names and the header's upper bound.
    #[test]
    fn named_constants_denote_the_formats_they_are_named_after() {
        for (format, name) in [
            (AVPixelFormat::YUV420P, c"yuv420p"),
            (AVPixelFormat::RGB24, c"rgb24"),
            (AVPixelFormat::BGR24, c"bgr24"),
            (AVPixelFormat::NV12, c"nv12"),
            (AVPixelFormat::NV21, c"nv21"),
            (AVPixelFormat::YA8, c"ya8"),
            (AVPixelFormat::_0RGB, c"0rgb"),
            (AVPixelFormat::BGR0, c"bgr0"),
            (AVPixelFormat::YUVJ411P, c"yuvj411p"),
            (AVPixelFormat::GBRAP32LE, c"gbrap32le"),
            (AVPixelFormat::XYZ12BE, c"xyz12be"),
        ] {
            assert_eq!(av_get_pix_fmt_name(format), Some(name));
            assert_eq!(av_get_pix_fmt(name), format);
        }

        // The header's upper bound is one past the last format this build
        // names, so the value below it must still be a format C knows.
        let last = AVPixelFormat::from_raw(AVPixelFormat::NB.as_raw() - 1);
        assert!(av_get_pix_fmt_name(last).is_some());
    }
}

#[cfg(test)]
mod chroma_location_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn chroma_location_is_layout_compatible_and_open() {
        assert_eq!(
            size_of::<AVChromaLocation>(),
            size_of::<ffi::AVChromaLocation>()
        );
        assert_eq!(
            align_of::<AVChromaLocation>(),
            align_of::<ffi::AVChromaLocation>()
        );

        let future = ffi::AVChromaLocation::MAX;
        assert_eq!(AVChromaLocation::from_raw(future).as_raw(), future);
        assert_eq!(
            ffi::AVChromaLocation::from(AVChromaLocation::from(future)),
            future
        );
    }

    #[test]
    fn named_locations_match_the_bindings() {
        for (named, raw) in [
            (
                AVChromaLocation::UNSPECIFIED,
                ffi::AVChromaLocation_AVCHROMA_LOC_UNSPECIFIED,
            ),
            (
                AVChromaLocation::LEFT,
                ffi::AVChromaLocation_AVCHROMA_LOC_LEFT,
            ),
            (
                AVChromaLocation::CENTER,
                ffi::AVChromaLocation_AVCHROMA_LOC_CENTER,
            ),
            (
                AVChromaLocation::TOP_LEFT,
                ffi::AVChromaLocation_AVCHROMA_LOC_TOPLEFT,
            ),
            (
                AVChromaLocation::TOP,
                ffi::AVChromaLocation_AVCHROMA_LOC_TOP,
            ),
            (
                AVChromaLocation::BOTTOM_LEFT,
                ffi::AVChromaLocation_AVCHROMA_LOC_BOTTOMLEFT,
            ),
            (
                AVChromaLocation::BOTTOM,
                ffi::AVChromaLocation_AVCHROMA_LOC_BOTTOM,
            ),
        ] {
            assert_eq!(named.as_raw(), raw);
            assert_eq!(AVChromaLocation::from_raw(raw), named);
        }
    }
}
