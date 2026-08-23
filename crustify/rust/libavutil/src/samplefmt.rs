//! Wrappers for libavutil sample formats.

use crate::ffi;

/// Wraps: AVSampleFormat
///
/// ABI-compatible audio sample-format value. This is a transparent integer
/// newtype rather than a Rust enum because an application may link to a newer
/// libavutil that returns a format unknown to this crate. Such values remain
/// valid Rust values and round-trip through [`from_raw`](Self::from_raw) and
/// [`as_raw`](Self::as_raw).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVSampleFormat(ffi::AVSampleFormat);

impl AVSampleFormat {
    /// No recognized sample format.
    pub const NONE: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_NONE);
    /// Unsigned 8-bit packed samples.
    pub const U8: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_U8);
    /// Signed 16-bit packed samples.
    pub const S16: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S16);
    /// Signed 32-bit packed samples.
    pub const S32: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S32);
    /// 32-bit floating-point packed samples.
    pub const FLT: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_FLT);
    /// 64-bit floating-point packed samples.
    pub const DBL: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_DBL);
    /// Unsigned 8-bit planar samples.
    pub const U8P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_U8P);
    /// Signed 16-bit planar samples.
    pub const S16P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S16P);
    /// Signed 32-bit planar samples.
    pub const S32P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S32P);
    /// 32-bit floating-point planar samples.
    pub const FLTP: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_FLTP);
    /// 64-bit floating-point planar samples.
    pub const DBLP: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_DBLP);
    /// Signed 64-bit packed samples.
    pub const S64: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S64);
    /// Signed 64-bit planar samples.
    pub const S64P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S64P);
    /// Number of sample formats in the headers used to build this crate.
    ///
    /// This sentinel is not stable when dynamically linking libavutil.
    pub const NB: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_NB);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVSampleFormat) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVSampleFormat {
        self.0
    }
}

impl From<ffi::AVSampleFormat> for AVSampleFormat {
    fn from(raw: ffi::AVSampleFormat) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVSampleFormat> for ffi::AVSampleFormat {
    fn from(format: AVSampleFormat) -> Self {
        format.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn declared_values_match_the_c_enum() {
        let formats = [
            (AVSampleFormat::NONE, ffi::AVSampleFormat_AV_SAMPLE_FMT_NONE),
            (AVSampleFormat::U8, ffi::AVSampleFormat_AV_SAMPLE_FMT_U8),
            (AVSampleFormat::S16, ffi::AVSampleFormat_AV_SAMPLE_FMT_S16),
            (AVSampleFormat::S32, ffi::AVSampleFormat_AV_SAMPLE_FMT_S32),
            (AVSampleFormat::FLT, ffi::AVSampleFormat_AV_SAMPLE_FMT_FLT),
            (AVSampleFormat::DBL, ffi::AVSampleFormat_AV_SAMPLE_FMT_DBL),
            (AVSampleFormat::U8P, ffi::AVSampleFormat_AV_SAMPLE_FMT_U8P),
            (AVSampleFormat::S16P, ffi::AVSampleFormat_AV_SAMPLE_FMT_S16P),
            (AVSampleFormat::S32P, ffi::AVSampleFormat_AV_SAMPLE_FMT_S32P),
            (AVSampleFormat::FLTP, ffi::AVSampleFormat_AV_SAMPLE_FMT_FLTP),
            (AVSampleFormat::DBLP, ffi::AVSampleFormat_AV_SAMPLE_FMT_DBLP),
            (AVSampleFormat::S64, ffi::AVSampleFormat_AV_SAMPLE_FMT_S64),
            (AVSampleFormat::S64P, ffi::AVSampleFormat_AV_SAMPLE_FMT_S64P),
            (AVSampleFormat::NB, ffi::AVSampleFormat_AV_SAMPLE_FMT_NB),
        ];

        for (format, raw) in formats {
            assert_eq!(format.as_raw(), raw);
            assert_eq!(AVSampleFormat::from(raw), format);
        }
    }

    #[test]
    fn layout_matches_raw_enum_and_unknown_values_round_trip() {
        assert_eq!(
            size_of::<AVSampleFormat>(),
            size_of::<ffi::AVSampleFormat>()
        );
        assert_eq!(
            align_of::<AVSampleFormat>(),
            align_of::<ffi::AVSampleFormat>()
        );

        let unknown = ffi::AVSampleFormat_AV_SAMPLE_FMT_NB + 1;
        assert_eq!(AVSampleFormat::from_raw(unknown).as_raw(), unknown);
    }
}
