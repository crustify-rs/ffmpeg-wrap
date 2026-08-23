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

    #[test]
    fn named_constants_agree_with_the_c_descriptor_table() {
        // Each constant is checked against the name libavutil stores at that
        // index, so a constant bound to the wrong enumerator cannot pass.
        for (kind, name) in [
            (AVFrameSideDataType::PANSCAN, c"AVPanScan"),
            (
                AVFrameSideDataType::A53_CC,
                c"ATSC A53 Part 4 Closed Captions",
            ),
            (AVFrameSideDataType::STEREO3D, c"Stereo 3D"),
            (AVFrameSideDataType::S12M_TIMECODE, c"SMPTE 12-1 timecode"),
            (
                AVFrameSideDataType::REFERENCE_DISPLAYS_3D,
                c"3D Reference Displays Information",
            ),
            (AVFrameSideDataType::DOWNMIX_MATRIX, c"Downmix Matrix"),
        ] {
            assert_eq!(av_frame_side_data_name(kind), Some(name));
        }
    }

    #[test]
    fn a_type_outside_the_descriptor_table_has_no_name() {
        // The open newtype carries a value libavutil does not describe instead
        // of forming an invalid discriminant; C answers with a null name.
        let unknown = AVFrameSideDataType::from_raw(ffi::AVFrameSideDataType::MAX);
        assert!(av_frame_side_data_name(unknown).is_none());
    }
}
