//! Wrappers for libavutil string utilities.

use core::ffi::CStr;

use crate::ffi;

/// Wraps: av_match_name
///
/// Tests a name against libavutil's comma-separated, case-insensitive match
/// syntax. `None` preserves the C API's null-means-no-match behavior.
#[must_use]
pub fn av_match_name(name: Option<&CStr>, names: Option<&CStr>) -> bool {
    let name = name.map_or(core::ptr::null(), CStr::as_ptr);
    let names = names.map_or(core::ptr::null(), CStr::as_ptr);
    // SAFETY: each non-null pointer comes from a live `CStr` and is borrowed
    // only for this call; the C function only reads through both pointers.
    unsafe { ffi::av_match_name(name, names) != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_lists_and_nulls() {
        assert!(av_match_name(Some(c"H264"), Some(c"vp9,h264,av1")));
        assert!(!av_match_name(Some(c"h264"), Some(c"-h264,ALL")));
        assert!(!av_match_name(None, Some(c"ALL")));
        assert!(!av_match_name(Some(c"h264"), None));
    }
}

/// Wraps: AVEscapeMode
///
/// Selects the escaping syntax used by `av_escape`. The transparent integer
/// representation keeps every C ABI value valid, including a mode introduced
/// by a newer linked libavutil, rather than manufacturing an invalid Rust enum
/// discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVEscapeMode(ffi::AVEscapeMode);

impl AVEscapeMode {
    /// Let libavutil select the escaping syntax.
    pub const AUTO: Self = Self(ffi::AVEscapeMode_AV_ESCAPE_MODE_AUTO);
    /// Escape special characters with backslashes.
    pub const BACKSLASH: Self = Self(ffi::AVEscapeMode_AV_ESCAPE_MODE_BACKSLASH);
    /// Escape with single-quoted segments.
    pub const QUOTE: Self = Self(ffi::AVEscapeMode_AV_ESCAPE_MODE_QUOTE);
    /// Escape XML non-markup character data.
    pub const XML: Self = Self(ffi::AVEscapeMode_AV_ESCAPE_MODE_XML);

    /// Preserves a raw C enum value, including one unknown to this crate.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVEscapeMode) -> Self {
        Self(raw)
    }

    /// Returns the integer representation expected by the C ABI.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVEscapeMode {
        self.0
    }
}

impl From<ffi::AVEscapeMode> for AVEscapeMode {
    fn from(raw: ffi::AVEscapeMode) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVEscapeMode> for ffi::AVEscapeMode {
    fn from(mode: AVEscapeMode) -> Self {
        mode.as_raw()
    }
}

#[cfg(test)]
mod escape_mode_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn escape_modes_are_abi_transparent_and_open() {
        assert_eq!(size_of::<AVEscapeMode>(), size_of::<ffi::AVEscapeMode>());
        assert_eq!(align_of::<AVEscapeMode>(), align_of::<ffi::AVEscapeMode>());

        for (mode, raw) in [
            (AVEscapeMode::AUTO, ffi::AVEscapeMode_AV_ESCAPE_MODE_AUTO),
            (
                AVEscapeMode::BACKSLASH,
                ffi::AVEscapeMode_AV_ESCAPE_MODE_BACKSLASH,
            ),
            (AVEscapeMode::QUOTE, ffi::AVEscapeMode_AV_ESCAPE_MODE_QUOTE),
            (AVEscapeMode::XML, ffi::AVEscapeMode_AV_ESCAPE_MODE_XML),
        ] {
            assert_eq!(mode.as_raw(), raw);
            assert_eq!(AVEscapeMode::from(raw), mode);
        }

        let future = ffi::AVEscapeMode::MAX;
        assert_eq!(AVEscapeMode::from_raw(future).as_raw(), future);
    }
}
