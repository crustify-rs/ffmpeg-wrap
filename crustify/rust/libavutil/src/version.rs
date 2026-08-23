//! Wrappers for libavutil version information.

use core::ffi::CStr;

use crate::ffi;

/// Wraps: av_version_info
#[must_use]
pub fn av_version_info() -> &'static CStr {
    // SAFETY: libavutil returns a non-null NUL-terminated build-time string
    // whose storage is static and immutable.
    unsafe { CStr::from_ptr(ffi::av_version_info()) }
}

/// Wraps: avutil_configuration
#[must_use]
pub fn avutil_configuration() -> &'static CStr {
    // SAFETY: libavutil returns a non-null NUL-terminated build-time string
    // whose storage is static and immutable.
    unsafe { CStr::from_ptr(ffi::avutil_configuration()) }
}

/// Wraps: avutil_license
#[must_use]
pub fn avutil_license() -> &'static CStr {
    // SAFETY: libavutil returns a non-null NUL-terminated build-time string
    // whose storage is static and immutable.
    unsafe { CStr::from_ptr(ffi::avutil_license()) }
}

/// Wraps: avutil_version
#[must_use]
pub fn avutil_version() -> u32 {
    // SAFETY: the function has no pointer arguments or caller obligations.
    unsafe { ffi::avutil_version() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_nonempty_build_information() {
        assert!(!av_version_info().to_bytes().is_empty());
        assert!(!avutil_configuration().to_bytes().is_empty());
        assert!(!avutil_license().to_bytes().is_empty());
        assert_ne!(avutil_version(), 0);
    }
}
