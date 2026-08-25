//! Wrappers for libavutil string utilities.

use core::cmp::Ordering;
use core::ffi::{CStr, c_char};
use core::marker::PhantomData;

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

/// Wraps: av_escape
pub fn av_escape(
    source: &CStr,
    special_characters: Option<&CStr>,
    mode: AVEscapeMode,
    flags: i32,
) -> Result<CrustifyStr<AvFree>, i32> {
    let mut output = core::ptr::null_mut();
    // SAFETY: both non-null strings are live and terminated for this call;
    // `output` is a writable slot which receives a fresh av_malloc string on
    // success and neither input is retained.
    let status = unsafe {
        ffi::av_escape(
            &raw mut output,
            source.as_ptr(),
            special_characters.map_or(core::ptr::null(), CStr::as_ptr),
            mode.as_raw(),
            flags,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        // SAFETY: success transfers one non-null av_malloc-family string.
        Ok(unsafe { CrustifyStr::from_raw(output) }
            .expect("av_escape succeeded without returning a string"))
    }
}

#[cfg(test)]
mod escape_tests {
    use super::*;

    #[test]
    fn returns_an_owned_escaped_string() {
        let escaped = av_escape(c"a:b", Some(c":"), AVEscapeMode::BACKSLASH, 0).unwrap();
        assert_eq!(escaped.as_c_str(), c"a\\:b");
    }
}

/// A byte buffer that had to hold a C string but contains no NUL.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnterminatedBuffer;

/// Length of the C string a buffer starts with, which is also the offset of
/// its terminator. Every wrapper below that lets C read or extend a string
/// already in the buffer goes through this: C reaches the terminator with
/// `strlen`, so without one it walks off the end of the Rust slice.
fn terminated_length(buffer: &[u8]) -> Result<usize, UnterminatedBuffer> {
    buffer
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(UnterminatedBuffer)
}

/// Wraps: av_strstart
///
/// Returns the remainder of `string` after `prefix`, or `None` when it is not
/// a prefix. C's out-parameter is a pointer into `string`, so the result
/// borrows it rather than owning anything.
#[must_use]
pub fn av_strstart<'a>(string: &'a CStr, prefix: &CStr) -> Option<&'a CStr> {
    let mut rest = core::ptr::null();
    // SAFETY: both strings are live and terminated, C only reads them, and
    // `rest` is a writable slot C fills with a pointer into `string`.
    let matched = unsafe { ffi::av_strstart(string.as_ptr(), prefix.as_ptr(), &raw mut rest) != 0 };
    // SAFETY: on a match C set `rest` to a position inside `string`, so the
    // suffix at it is terminated and lives as long as the borrow.
    matched.then(|| unsafe { CStr::from_ptr(rest) })
}

/// Wraps: av_stristart
///
/// The ASCII case-insensitive form of [`av_strstart`].
#[must_use]
pub fn av_stristart<'a>(string: &'a CStr, prefix: &CStr) -> Option<&'a CStr> {
    let mut rest = core::ptr::null();
    // SAFETY: both strings are live and terminated, C only reads them, and
    // `rest` is a writable slot C fills with a pointer into `string`.
    let matched =
        unsafe { ffi::av_stristart(string.as_ptr(), prefix.as_ptr(), &raw mut rest) != 0 };
    // SAFETY: on a match C set `rest` to a position inside `string`.
    matched.then(|| unsafe { CStr::from_ptr(rest) })
}

/// Wraps: av_stristr
///
/// Returns the tail of `haystack` starting at the first ASCII case-insensitive
/// occurrence of `needle`. An empty needle matches at the start.
#[must_use]
pub fn av_stristr<'a>(haystack: &'a CStr, needle: &CStr) -> Option<&'a CStr> {
    // SAFETY: both strings are live and terminated and C only reads them. The
    // `char *` result is a position inside `haystack`, never a new allocation.
    let found = unsafe { ffi::av_stristr(haystack.as_ptr(), needle.as_ptr()) };
    if found.is_null() {
        None
    } else {
        // SAFETY: a match points inside `haystack`, so the suffix at it is
        // terminated and borrowed from the same string.
        Some(unsafe { CStr::from_ptr(found) })
    }
}

/// Wraps: av_strnstr
///
/// Searches at most `haystack.len()` bytes, so the haystack is a byte slice
/// rather than a C string: C never reads past the supplied length and never
/// requires a terminator inside it. The result is the matching tail of that
/// slice.
#[must_use]
pub fn av_strnstr<'a>(haystack: &'a [u8], needle: &CStr) -> Option<&'a [u8]> {
    // SAFETY: `haystack` provides exactly the `hay_length` readable bytes C is
    // allowed to touch — its last comparison starts `needle` bytes before the
    // end — and `needle` is live and terminated. Both are read-only here.
    let found =
        unsafe { ffi::av_strnstr(haystack.as_ptr().cast(), needle.as_ptr(), haystack.len()) };
    if found.is_null() {
        return None;
    }
    let offset = found as usize - haystack.as_ptr() as usize;
    Some(&haystack[offset..])
}

/// Wraps: av_strlcpy
///
/// Copies `source` into `destination`, truncating to keep one terminator
/// inside it, and returns the length `source` would have needed. An empty
/// destination is written not at all, which is what C's `size == 0` case does.
pub fn av_strlcpy(destination: &mut [u8], source: &CStr) -> usize {
    // SAFETY: C writes at most `size` bytes at `dst`, terminator included, and
    // `size` is this slice's own length; `source` is live and terminated and is
    // only read. The two borrows cannot overlap.
    unsafe {
        ffi::av_strlcpy(
            destination.as_mut_ptr().cast(),
            source.as_ptr(),
            destination.len(),
        )
    }
}

/// Wraps: av_strlcat
///
/// Appends `source` to the C string already in `destination` and returns the
/// combined length that would have been needed. The buffer must already hold a
/// terminator, because C locates the append position with `strlen`.
pub fn av_strlcat(destination: &mut [u8], source: &CStr) -> Result<usize, UnterminatedBuffer> {
    terminated_length(destination)?;
    // SAFETY: the check above proves `strlen(dst) < size`, so C's `dst + len`
    // stays inside this slice and it writes at most `size` bytes in total.
    // `source` is live, terminated and only read.
    Ok(unsafe {
        ffi::av_strlcat(
            destination.as_mut_ptr().cast(),
            source.as_ptr(),
            destination.len(),
        )
    })
}

/// Wraps: av_strlcatf
///
/// Appends `text` to the C string already in `destination`. The variadic C
/// signature is projected onto its one safely expressible shape — a single
/// `%s` conversion supplied by this argument — because forwarding a caller's
/// format string would let it name conversions with no matching argument.
/// Unlike [`av_strlcat`], the return value is `vsnprintf`'s, so it counts what
/// would have been written even when nothing fitted.
pub fn av_strlcatf(destination: &mut [u8], text: &CStr) -> Result<usize, UnterminatedBuffer> {
    terminated_length(destination)?;
    // SAFETY: the check above proves `strlen(dst) < size`, so C's `dst + len`
    // stays inside the slice and `vsnprintf` receives the remaining capacity.
    // The format is a literal with exactly one `%s`, matched by the one live
    // terminated string passed for it.
    Ok(unsafe {
        ffi::av_strlcatf(
            destination.as_mut_ptr().cast(),
            destination.len(),
            c"%s".as_ptr(),
            text.as_ptr(),
        )
    })
}

/// Wraps: av_strnlen
///
/// Counts the leading non-zero bytes of `bytes`, stopping at its end. The
/// slice supplies both the pointer and C's `len` bound, so no terminator is
/// required.
#[must_use]
pub fn av_strnlen(bytes: &[u8]) -> usize {
    // SAFETY: the shim reads at most `len` bytes from `s`, and the slice
    // provides exactly that many readable bytes for the call.
    unsafe { ffi::crustify_av_strnlen(bytes.as_ptr().cast(), bytes.len()) }
}

/// Wraps: av_tolower
///
/// Locale-independent ASCII lowercasing of a C `int` character value; every
/// other value is returned unchanged.
#[must_use]
pub fn av_tolower(value: i32) -> i32 {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_tolower(value) }
}

/// Wraps: av_toupper
///
/// Locale-independent ASCII uppercasing of a C `int` character value; every
/// other value is returned unchanged.
#[must_use]
pub fn av_toupper(value: i32) -> i32 {
    // SAFETY: the inline helper accepts every `int`.
    unsafe { ffi::crustify_av_toupper(value) }
}

/// Wraps: av_strcasecmp
///
/// ASCII case-insensitive ordering. C's `int` is the difference of the first
/// differing bytes, so only its sign carries meaning and [`Ordering`] states
/// exactly that.
#[must_use]
pub fn av_strcasecmp(a: &CStr, b: &CStr) -> Ordering {
    // SAFETY: both strings are live and terminated; C stops at either
    // terminator and only reads.
    let difference = unsafe { ffi::av_strcasecmp(a.as_ptr(), b.as_ptr()) };
    difference.cmp(&0)
}

/// Wraps: av_strncasecmp
///
/// [`av_strcasecmp`] over at most `count` bytes. C stops at a terminator too,
/// so a count past the end of either string is harmless.
#[must_use]
pub fn av_strncasecmp(a: &CStr, b: &CStr, count: usize) -> Ordering {
    // SAFETY: both strings are live and terminated; C stops at the first
    // terminator or after `count` bytes, whichever comes first, and only reads.
    let difference = unsafe { ffi::av_strncasecmp(a.as_ptr(), b.as_ptr(), count) };
    difference.cmp(&0)
}

/// A failed [`av_strireplace`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringReplaceError {
    /// C searches for `from` with `av_stristr`, which reports an empty pattern
    /// as a match at the current position without consuming anything. Its
    /// replacement loop then never advances and appends `to` to an unbounded
    /// buffer forever, so the wrapper refuses that input instead of hanging.
    EmptyPattern,
    /// The growing buffer could not be allocated or finalized.
    AllocationFailed,
}

/// Wraps: av_strireplace
///
/// Replaces every ASCII case-insensitive occurrence of `from` in `string` with
/// `to`, returning a new owned string.
pub fn av_strireplace(
    string: &CStr,
    from: &CStr,
    to: &CStr,
) -> Result<CrustifyStr<AvFree>, StringReplaceError> {
    if from.is_empty() {
        return Err(StringReplaceError::EmptyPattern);
    }
    // SAFETY: all three strings are live and terminated and C only reads them;
    // the non-empty pattern makes the scan advance, so the loop terminates. A
    // non-null result is a finalized av_malloc-family string owned by us.
    unsafe {
        CrustifyStr::from_raw(ffi::av_strireplace(
            string.as_ptr(),
            from.as_ptr(),
            to.as_ptr(),
        ))
    }
    .ok_or(StringReplaceError::AllocationFailed)
}

/// Wraps: av_strtok
///
/// A destructive tokenizer over a caller-owned buffer. C writes a terminator
/// over each delimiter it consumes, which is why the buffer is borrowed
/// exclusively, and it keeps its position in a `saveptr` this type owns.
///
/// A token borrows the tokenizer, so it must be finished with before the next
/// one is taken: the following call may write a terminator further along the
/// same buffer.
pub struct AvStrTok<'a> {
    /// C's `saveptr`: null once the buffer is exhausted, otherwise a position
    /// inside the terminated string this tokenizer borrows.
    save: *mut c_char,
    buffer: PhantomData<&'a mut [u8]>,
}

impl<'a> AvStrTok<'a> {
    /// Starts tokenizing the C string at the start of `buffer`, which must
    /// contain a terminator.
    pub fn new(buffer: &'a mut [u8]) -> Result<Self, UnterminatedBuffer> {
        terminated_length(buffer)?;
        Ok(Self {
            save: buffer.as_mut_ptr().cast(),
            buffer: PhantomData,
        })
    }

    /// Returns the next token, or `None` once the buffer holds no more.
    ///
    /// `delimiters` is a set of single bytes, not a separator string.
    pub fn next_token(&mut self, delimiters: &CStr) -> Option<&CStr> {
        // Passing NULL is C's "continue where you left off"; the first call
        // continues from the buffer start this type seeded `save` with, which
        // is exactly what passing the buffer itself would have done.
        //
        // SAFETY: `save` is null or a position inside the terminated string in
        // the exclusively borrowed buffer, so C reads to its terminator and
        // writes at most one terminator over a delimiter inside it. The slot
        // itself is live for the call and C does not retain its address.
        let token = unsafe {
            ffi::av_strtok(
                core::ptr::null_mut(),
                delimiters.as_ptr(),
                &raw mut self.save,
            )
        };
        if token.is_null() {
            None
        } else {
            // SAFETY: C terminated the token inside the borrowed buffer, which
            // outlives the borrow this reference is tied to.
            Some(unsafe { CStr::from_ptr(token) })
        }
    }
}

/// A rejected UTF-8 sequence. In both cases the cursor has already advanced
/// past the offending input, matching C's resynchronizing behaviour.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Utf8DecodeError {
    /// The bytes were not a well-formed UTF-8 sequence, or encoded a value C
    /// refuses outright.
    InvalidSequence(i32),
    /// A sequence decoded, and then `flags` rejected the code point it named.
    RejectedCodePoint { status: i32, code_point: u32 },
}

/// Wraps: av_utf8_decode
///
/// Decodes one code point from the front of `input` and advances it past the
/// bytes consumed. `Ok(None)` is C's empty-input result, which consumes
/// nothing.
///
/// `flags` is a set of `AV_UTF8_FLAG_*` values.
pub fn av_utf8_decode(input: &mut &[u8], flags: u32) -> Result<Option<u32>, Utf8DecodeError> {
    // C writes a value in `0..=0x7FFFFFFF` here and writes nothing at all on
    // the paths that reject a sequence before decoding it, so a negative
    // sentinel distinguishes the two error shapes without ambiguity.
    let mut code: i32 = -1;
    let range = input.as_ptr_range();
    let mut cursor: *const u8 = range.start;
    // SAFETY: `code` and `cursor` are live writable slots; `cursor` and
    // `range.end` delimit exactly the readable bytes of `input`, which is what
    // bounds every read C makes. C retains neither slot.
    let status = unsafe { ffi::av_utf8_decode(&raw mut code, &raw mut cursor, range.end, flags) };
    let consumed = cursor as usize - range.start as usize;
    *input = &input[consumed..];
    if status >= 0 {
        return Ok(u32::try_from(code).ok());
    }
    Err(match u32::try_from(code) {
        Ok(code_point) => Utf8DecodeError::RejectedCodePoint { status, code_point },
        Err(_) => Utf8DecodeError::InvalidSequence(status),
    })
}

#[cfg(test)]
mod scheduled_string_tests {
    use super::*;

    #[test]
    fn prefix_and_substring_searches_borrow_their_haystack() {
        assert_eq!(av_strstart(c"file:/tmp", c"file:"), Some(c"/tmp"));
        assert_eq!(av_strstart(c"file:/tmp", c"FILE:"), None);
        assert_eq!(av_stristart(c"file:/tmp", c"FILE:"), Some(c"/tmp"));
        assert_eq!(av_stristart(c"file:/tmp", c"http:"), None);
        // An empty prefix always matches, and leaves the whole string.
        assert_eq!(av_strstart(c"abc", c""), Some(c"abc"));

        assert_eq!(av_stristr(c"Hello World", c"WORLD"), Some(c"World"));
        assert_eq!(av_stristr(c"Hello", c"z"), None);
        assert_eq!(av_stristr(c"Hello", c""), Some(c"Hello"));
    }

    #[test]
    fn length_bounded_search_needs_no_terminator() {
        // No NUL anywhere in the haystack: the length is the only bound, and
        // the sanitiser run is what proves C respects it.
        let haystack = *b"abcdef";
        assert_eq!(av_strnstr(&haystack, c"cd"), Some(&b"cdef"[..]));
        assert_eq!(av_strnstr(&haystack, c"ef"), Some(&b"ef"[..]));
        assert_eq!(av_strnstr(&haystack, c"fg"), None);
        assert_eq!(av_strnstr(&haystack, c""), Some(&haystack[..]));
        assert_eq!(av_strnstr(&[], c"a"), None);
    }

    #[test]
    fn bounded_copies_truncate_and_report_the_full_length() {
        let mut buffer = [0xAA_u8; 8];
        assert_eq!(av_strlcpy(&mut buffer, c"abc"), 3);
        assert_eq!(&buffer[..4], b"abc\0");

        let mut small = [0xAA_u8; 4];
        assert_eq!(av_strlcpy(&mut small, c"abcdef"), 6);
        assert_eq!(&small, b"abc\0");

        // C's `size == 0` case writes nothing at all.
        let mut empty: [u8; 0] = [];
        assert_eq!(av_strlcpy(&mut empty, c"abc"), 3);

        let mut buffer = [0_u8; 8];
        assert_eq!(av_strlcpy(&mut buffer, c"ab"), 2);
        assert_eq!(av_strlcat(&mut buffer, c"cd"), Ok(4));
        assert_eq!(&buffer[..5], b"abcd\0");
        assert_eq!(av_strlcatf(&mut buffer, c"ef"), Ok(6));
        assert_eq!(&buffer[..7], b"abcdef\0");

        // A buffer with no terminator is refused before C runs `strlen` on it.
        let mut unterminated = *b"abcd";
        assert_eq!(av_strlcat(&mut unterminated, c"e"), Err(UnterminatedBuffer));
        assert_eq!(
            av_strlcatf(&mut unterminated, c"e"),
            Err(UnterminatedBuffer)
        );
        assert_eq!(&unterminated, b"abcd");
    }

    #[test]
    fn counts_bounded_lengths_and_folds_ascii_case() {
        assert_eq!(av_strnlen(b"abc\0def"), 3);
        assert_eq!(av_strnlen(b"abcdef"), 6);
        assert_eq!(av_strnlen(&[]), 0);

        assert_eq!(av_tolower(i32::from(b'A')), i32::from(b'a'));
        assert_eq!(av_toupper(i32::from(b'a')), i32::from(b'A'));
        assert_eq!(av_tolower(i32::from(b'-')), i32::from(b'-'));
        assert_eq!(av_toupper(-1), -1);

        assert_eq!(av_strcasecmp(c"AbC", c"aBc"), Ordering::Equal);
        assert_eq!(av_strcasecmp(c"abc", c"abd"), Ordering::Less);
        assert_eq!(av_strcasecmp(c"abd", c"abc"), Ordering::Greater);
        assert_eq!(av_strncasecmp(c"abc", c"ABD", 2), Ordering::Equal);
        assert_eq!(av_strncasecmp(c"abc", c"ABD", 3), Ordering::Less);
        assert_eq!(av_strncasecmp(c"abc", c"xyz", 0), Ordering::Equal);
    }

    #[test]
    fn replaces_every_case_insensitive_occurrence() {
        let replaced = av_strireplace(c"aXbXc", c"x", c"--").expect("replace");
        assert_eq!(replaced.as_c_str(), c"a--b--c");

        let unchanged = av_strireplace(c"abc", c"z", c"!").expect("replace");
        assert_eq!(unchanged.as_c_str(), c"abc");

        // The input C cannot survive: `av_stristr` matches an empty pattern
        // without consuming it, so C's replacement loop would never advance.
        assert_eq!(
            av_strireplace(c"abc", c"", c"!").err(),
            Some(StringReplaceError::EmptyPattern)
        );
    }

    #[test]
    fn tokenizes_a_buffer_in_place() {
        let mut buffer = *b"  one,two,,three \0";
        let mut tokens = AvStrTok::new(&mut buffer).expect("a terminated buffer");
        assert_eq!(tokens.next_token(c" ,"), Some(c"one"));
        assert_eq!(tokens.next_token(c" ,"), Some(c"two"));
        // Empty fields are delimiters, not tokens.
        assert_eq!(tokens.next_token(c" ,"), Some(c"three"));
        assert_eq!(tokens.next_token(c" ,"), None);
        assert_eq!(tokens.next_token(c" ,"), None);

        let mut unterminated = *b"abc";
        assert!(AvStrTok::new(&mut unterminated).is_err());
    }

    #[test]
    fn decodes_code_points_and_resynchronizes() {
        let mut input: &[u8] = "aé€".as_bytes();
        assert_eq!(av_utf8_decode(&mut input, 0), Ok(Some(u32::from('a'))));
        assert_eq!(av_utf8_decode(&mut input, 0), Ok(Some(u32::from('é'))));
        assert_eq!(av_utf8_decode(&mut input, 0), Ok(Some(u32::from('€'))));
        assert_eq!(av_utf8_decode(&mut input, 0), Ok(None));
        assert!(input.is_empty());

        // A continuation byte on its own is rejected before anything decodes,
        // and the cursor still advances so the caller can carry on.
        let mut input: &[u8] = &[0x80, b'z'];
        assert!(matches!(
            av_utf8_decode(&mut input, 0),
            Err(Utf8DecodeError::InvalidSequence(_))
        ));
        assert_eq!(input, b"z");

        // A surrogate is well-formed as bytes: C decodes it, then the flags
        // decide, so the rejected code point is still reported.
        let mut input: &[u8] = &[0xED, 0xA0, 0x80];
        assert!(matches!(
            av_utf8_decode(&mut input, 0),
            Err(Utf8DecodeError::RejectedCodePoint {
                code_point: 0xD800,
                ..
            })
        ));
        assert!(input.is_empty());
        let mut input: &[u8] = &[0xED, 0xA0, 0x80];
        assert_eq!(
            av_utf8_decode(&mut input, ffi::AV_UTF8_FLAG_ACCEPT_SURROGATES),
            Ok(Some(0xD800))
        );
    }
}
