//! Wrappers for libavutil pixel format descriptors.

use core::ffi::CStr;

use crate::ffi;

/// Wraps: av_chroma_location_from_name
///
/// Returns the non-negative `AVChromaLocation` value, or libavutil's negative
/// error code when the name is unknown.
pub fn av_chroma_location_from_name(name: &CStr) -> Result<i32, i32> {
    // SAFETY: `name` is NUL-terminated and remains live for the read-only call.
    let value = unsafe { ffi::av_chroma_location_from_name(name.as_ptr()) };
    if value < 0 { Err(value) } else { Ok(value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_chroma_locations() {
        assert!(av_chroma_location_from_name(c"left").is_ok());
        assert!(av_chroma_location_from_name(c"not-a-location").is_err());
    }
}
