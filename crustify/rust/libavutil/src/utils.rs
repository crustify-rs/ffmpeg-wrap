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
    fn every_named_media_type_gets_the_string_c_switches_on() {
        // One pairing per arm of `av_get_media_type_string`'s switch. Checking
        // the constants against the library rather than against the same
        // generated binding is what catches one bound to the wrong enumerator.
        for (media_type, name) in [
            (AVMediaType::VIDEO, c"video"),
            (AVMediaType::AUDIO, c"audio"),
            (AVMediaType::DATA, c"data"),
            (AVMediaType::SUBTITLE, c"subtitle"),
            (AVMediaType::ATTACHMENT, c"attachment"),
        ] {
            assert_eq!(av_get_media_type_string(media_type), Some(name));
        }
    }

    #[test]
    fn types_outside_the_switch_have_no_name() {
        // `UNKNOWN`, the `NB` count and a value from a newer library all reach
        // the same `default` arm: C switches on the type rather than indexing
        // a table, so none of them can produce a string.
        assert_eq!(av_get_media_type_string(AVMediaType::UNKNOWN), None);
        assert_eq!(av_get_media_type_string(AVMediaType::NB), None);
        assert_eq!(
            av_get_media_type_string(AVMediaType::from_raw(ffi::AVMediaType::MAX)),
            None
        );
    }
}
