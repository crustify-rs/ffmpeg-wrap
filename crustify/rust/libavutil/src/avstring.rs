//! Wrappers for libavutil string utilities.

use core::ffi::CStr;

use ffibox::CrustifyStr;

use crate::mem::AvFree;

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

/// Wraps: av_append_path_component
#[must_use]
pub fn av_append_path_component(
    path: Option<&CStr>,
    component: Option<&CStr>,
) -> Option<CrustifyStr<AvFree>> {
    // SAFETY: non-null arguments are live terminated strings, and C neither
    // retains nor mutates them. A non-null result is a fresh av_malloc-family
    // string, which `AvFree` releases.
    unsafe {
        CrustifyStr::from_raw(ffi::av_append_path_component(
            path.map_or(core::ptr::null(), CStr::as_ptr),
            component.map_or(core::ptr::null(), CStr::as_ptr),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AsprintfError {
    UnsupportedFormatSpecifier,
    AllocationFailed,
}

/// Wraps: av_asprintf
///
/// This safe variant accepts the no-varargs subset of the variadic C API:
/// every percent sign must be escaped as `%%`. Forwarding a conversion without
/// its matching variadic argument would make C read a nonexistent value.
pub fn av_asprintf(format: &CStr) -> Result<CrustifyStr<AvFree>, AsprintfError> {
    let bytes = format.to_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if bytes.get(index + 1) != Some(&b'%') {
                return Err(AsprintfError::UnsupportedFormatSpecifier);
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    // SAFETY: validation proved that the format consumes no variadic argument;
    // C reads only the live terminated format. A non-null result is owned.
    unsafe { CrustifyStr::from_raw(ffi::av_asprintf(format.as_ptr())) }
        .ok_or(AsprintfError::AllocationFailed)
}

/// Wraps: av_basename
#[must_use]
pub fn av_basename(path: Option<&CStr>) -> &CStr {
    // SAFETY: C returns either a pointer into `path` or its static `"."`
    // literal, and never mutates or retains the argument.
    unsafe {
        CStr::from_ptr(ffi::av_basename(
            path.map_or(core::ptr::null(), CStr::as_ptr),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidPathBuffer;

/// Wraps: av_dirname
pub fn av_dirname(path: Option<&mut [u8]>) -> Result<&CStr, InvalidPathBuffer> {
    let pointer = match path {
        Some(bytes) => {
            CStr::from_bytes_with_nul(bytes).map_err(|_| InvalidPathBuffer)?;
            bytes.as_mut_ptr().cast()
        }
        None => core::ptr::null_mut(),
    };
    // SAFETY: a non-null pointer is a live writable buffer containing exactly
    // one trailing NUL. C only replaces one separator with NUL and returns a
    // prefix of that same borrow; null returns the static `"."` literal.
    Ok(unsafe { CStr::from_ptr(ffi::av_dirname(pointer)) })
}

/// Wraps: av_get_token
///
/// Advances `cursor` within its original string and returns an independently
/// owned token.
pub fn av_get_token(cursor: &mut &CStr, terminators: &CStr) -> Option<CrustifyStr<AvFree>> {
    let mut pointer = cursor.as_ptr();
    // SAFETY: the pointer slot is writable, its value and `terminators` are
    // live terminated strings, and C advances the value only within the first
    // string. A non-null result is a new av_malloc-family string.
    let token = unsafe { ffi::av_get_token(&raw mut pointer, terminators.as_ptr()) };
    // SAFETY: C leaves `pointer` at a position within the original terminated
    // string, so the suffix remains terminated and lives for `'a`.
    *cursor = unsafe { CStr::from_ptr(pointer) };
    // SAFETY: ownership of a non-null av_malloc-family result transfers here.
    unsafe { CrustifyStr::from_raw(token) }
}

/// Wraps: av_isdigit
#[must_use]
pub fn av_isdigit(value: i32) -> bool {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_isdigit(value) != 0 }
}

/// Wraps: av_isgraph
#[must_use]
pub fn av_isgraph(value: i32) -> bool {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_isgraph(value) != 0 }
}

/// Wraps: av_isspace
#[must_use]
pub fn av_isspace(value: i32) -> bool {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_isspace(value) != 0 }
}

/// Wraps: av_isxdigit
#[must_use]
pub fn av_isxdigit(value: i32) -> bool {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_isxdigit(value) != 0 }
}

/// Wraps: av_match_list
#[must_use]
pub fn av_match_list(name: &CStr, list: &CStr, separator: u8) -> bool {
    // SAFETY: both strings are terminated and read-only for the call; a byte
    // has the same value in C `char` after the explicit cast.
    unsafe { ffi::av_match_list(name.as_ptr(), list.as_ptr(), separator as core::ffi::c_char) != 0 }
}

#[cfg(test)]
mod scheduled_tests {
    use super::*;

    #[test]
    fn owns_created_paths_and_tokens() {
        let path =
            av_append_path_component(Some(c"/tmp/"), Some(c"/file")).expect("path allocation");
        assert_eq!(path.as_c_str(), c"/tmp/file");

        let mut cursor = c"  one, two";
        let token = av_get_token(&mut cursor, c",").expect("token allocation");
        assert_eq!(token.as_c_str(), c"one");
        assert_eq!(cursor, c", two");
    }

    #[test]
    fn borrows_path_components() {
        assert_eq!(av_basename(Some(c"/a/b")), c"b");
        assert_eq!(av_basename(None), c".");
        let mut path = *b"/a/b\0";
        assert_eq!(av_dirname(Some(&mut path)), Ok(c"/a"));
        assert_eq!(av_dirname(None), Ok(c"."));
    }

    #[test]
    fn validates_formats_and_ascii_classes() {
        assert_eq!(
            av_asprintf(c"100%% ready").unwrap().as_c_str(),
            c"100% ready"
        );
        assert_eq!(
            av_asprintf(c"%s").unwrap_err(),
            AsprintfError::UnsupportedFormatSpecifier
        );
        assert!(av_isdigit(b'7' as i32));
        assert!(av_isxdigit(b'F' as i32));
        assert!(av_isspace(b'\n' as i32));
        assert!(av_isgraph(b'!' as i32));
        assert!(av_match_list(c"a:b", c"x:b", b':'));
    }
}
