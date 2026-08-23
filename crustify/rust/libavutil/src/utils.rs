//! Wrappers for core libavutil utilities.

use core::ffi::CStr;

use crate::avutil::AVMediaType;
use crate::ffi;

/// Wraps: av_get_media_type_string
#[must_use]
pub fn av_get_media_type_string(media_type: AVMediaType) -> Option<&'static CStr> {
    // SAFETY: C returns null or an immutable process-lifetime name.
    let pointer = unsafe { ffi::av_get_media_type_string(media_type.as_raw()) };
    if pointer.is_null() {
        None
    } else {
        // SAFETY: the checked result is a NUL-terminated static string.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_media_type_has_a_name() {
        assert_eq!(av_get_media_type_string(AVMediaType::VIDEO), Some(c"video"));
    }
}
