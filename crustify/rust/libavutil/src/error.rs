//! Wrappers for libavutil error utilities.

use core::ffi::{CStr, c_char};

use crate::ffi;

/// A description written by [`av_strerror`]. A negative status means the
/// number was unknown, but libavutil still supplied the generic text.
#[derive(Clone, Copy, Debug)]
pub struct ErrorDescription<'a> {
    pub status: i32,
    pub text: &'a CStr,
}

/// Wraps: av_strerror
///
/// Returns `None` only when the caller supplies an empty buffer, which cannot
/// hold even a NUL terminator. For every nonempty buffer libavutil writes a
/// terminated (possibly truncated) description.
pub fn av_strerror(errnum: i32, buffer: &mut [u8]) -> Option<ErrorDescription<'_>> {
    if buffer.is_empty() {
        return None;
    }

    // SAFETY: `buffer` supplies `buffer.len()` writable bytes for the call and
    // is not otherwise accessed or retained while C fills it.
    let status =
        unsafe { ffi::av_strerror(errnum, buffer.as_mut_ptr().cast::<c_char>(), buffer.len()) };
    let text = CStr::from_bytes_until_nul(buffer).ok()?;
    Some(ErrorDescription { status, text })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_known_and_unknown_codes() {
        let mut known = [0_u8; 64];
        let known = av_strerror(-22, &mut known).expect("nonempty output");
        assert!(!known.text.to_bytes().is_empty());

        let mut unknown = [0_u8; 64];
        let unknown = av_strerror(i32::MIN, &mut unknown).expect("generic output");
        assert!(unknown.status < 0);
        assert!(!unknown.text.to_bytes().is_empty());

        assert!(av_strerror(0, &mut []).is_none());
    }
}
