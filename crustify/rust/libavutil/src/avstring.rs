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
