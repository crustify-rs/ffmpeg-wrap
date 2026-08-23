//! Wrappers for libavutil frame side data.

use core::ffi::CStr;

use crate::ffi;
use crate::frame::AVFrameSideDataType;

/// Wraps: av_frame_side_data_name
#[must_use]
pub fn av_frame_side_data_name(kind: AVFrameSideDataType) -> Option<&'static CStr> {
    // SAFETY: C returns null or an immutable process-lifetime descriptor name.
    let pointer = unsafe { ffi::av_frame_side_data_name(kind.as_raw()) };
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
    fn known_side_data_has_a_static_name() {
        assert!(av_frame_side_data_name(AVFrameSideDataType::PANSCAN).is_some());
    }
}
