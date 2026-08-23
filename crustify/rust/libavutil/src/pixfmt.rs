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
    pub const YUVJ420P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ420P);
    pub const YUVJ422P: Self = Self(ffi::AVPixelFormat_AV_PIX_FMT_YUVJ422P);
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
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_the_c_enum() {
        assert_eq!(size_of::<AVPixelFormat>(), size_of::<ffi::AVPixelFormat>());
        assert_eq!(
            align_of::<AVPixelFormat>(),
            align_of::<ffi::AVPixelFormat>()
        );
    }

    #[test]
    fn named_values_and_aliases_match_the_bindings() {
        assert_eq!(AVPixelFormat::NONE.as_raw(), -1);
        assert_eq!(AVPixelFormat::YUV420P.as_raw(), 0);
        assert_eq!(AVPixelFormat::Y400A, AVPixelFormat::YA8);
        assert_eq!(AVPixelFormat::GRAY8A, AVPixelFormat::YA8);
        assert_eq!(AVPixelFormat::GBR24P, AVPixelFormat::GBRP);
        assert_eq!(AVPixelFormat::NB.as_raw(), ffi::AVPixelFormat_AV_PIX_FMT_NB);
    }

    #[test]
    fn unknown_values_round_trip() {
        let raw = ffi::AVPixelFormat_AV_PIX_FMT_NB + 17;
        assert_eq!(AVPixelFormat::from_raw(raw).as_raw(), raw);
        assert_eq!(ffi::AVPixelFormat::from(AVPixelFormat::from(raw)), raw);
    }
}
