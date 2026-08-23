//! Wrappers for libavutil dictionaries.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CDropped, define_ctype};

use crate::ffi;

define_ctype!(
    /// Wraps: AVDictionaryEntry
    ///
    /// A dictionary owns entries by value in its internal array. The layout type
    /// exists for embedding and FFI-compatible storage; access goes through
    /// [`AVDictionaryEntryRef`] and [`AVDictionaryEntryMut`] handles, never through
    /// a Rust reference to the C object.
    AVDictionaryEntry,
    AVDictionaryEntryRef,
    AVDictionaryEntryMut,
    ffi::AVDictionaryEntry
);

impl AVDictionaryEntryRef<'_> {
    /// Wraps: AVDictionaryEntry.value
    ///
    /// The returned string is borrowed from the entry. Although libavutil may
    /// replace or append to this allocation while mutating its dictionary, the
    /// public dictionary API forbids mutation while a borrowed entry is in use.
    #[must_use]
    pub fn value(&self) -> &CStr {
        // SAFETY: a live dictionary entry always owns a non-null,
        // NUL-terminated value. The result is tied to the borrow of this
        // handle, so it cannot outlive the entry borrow that made the handle.
        unsafe {
            let value = addr_of!((*self.as_ptr()).value).read();
            CStr::from_ptr(value.cast_const())
        }
    }

    /// Wraps: AVDictionaryEntry.key
    ///
    /// The returned string is borrowed from the entry and is read-only, as
    /// required by the public dictionary API.
    #[must_use]
    pub fn key(&self) -> &CStr {
        // SAFETY: a live dictionary entry always owns a non-null,
        // NUL-terminated key. The result is tied to the borrow of this handle,
        // so it cannot outlive the entry borrow that made the handle.
        unsafe {
            let key = addr_of!((*self.as_ptr()).key).read();
            CStr::from_ptr(key.cast_const())
        }
    }
}

define_ctype!(
    /// Wraps: AVDictionary
    AVDictionary,
    AVDictionaryRef,
    AVDictionaryMut,
    ffi::AVDictionary
);

/// Wraps: av_dict_free
// SAFETY: a `CBox<AVDictionary>` exclusively owns a fully constructed
// dictionary returned by libavutil. `c_drop` passes that one ownership unit to
// its public destructor, which frees the entries and header and nulls the local
// slot. No alias is exposed or retained.
unsafe impl CDropped for AVDictionary {
    unsafe fn c_drop(obj: NonNull<Self>) {
        let mut dictionary = obj.as_ptr().cast::<ffi::AVDictionary>();
        // SAFETY: the trait contract gives this call the unique, live
        // dictionary allocation. `av_dict_free` accepts a non-null pointer to
        // that local slot, consumes its inner pointer, and stores null in it.
        unsafe { ffi::av_dict_free(addr_of_mut!(dictionary)) }
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_int, c_void};
    use core::mem::{align_of, size_of};

    use ffibox::CBox;

    use super::*;

    #[repr(C)]
    struct EmptyDictionary {
        count: c_int,
        elems: *mut c_void,
    }

    #[test]
    fn entry_layout_and_borrowed_strings_match_c() {
        let mut raw = ffi::AVDictionaryEntry {
            key: c"key".as_ptr().cast_mut(),
            value: c"value".as_ptr().cast_mut(),
        };

        // SAFETY: `raw` is a live, fully initialised entry whose two pointers
        // address static NUL-terminated strings for the duration of `entry`.
        let entry =
            unsafe { AVDictionaryEntryRef::from_ptr(core::ptr::addr_of_mut!(raw)) }.unwrap();
        assert_eq!(entry.key(), c"key");
        assert_eq!(entry.value(), c"value");
        assert_eq!(
            size_of::<AVDictionaryEntry>(),
            size_of::<ffi::AVDictionaryEntry>()
        );
        assert_eq!(
            align_of::<AVDictionaryEntry>(),
            align_of::<ffi::AVDictionaryEntry>()
        );
    }

    #[test]
    fn dictionary_owner_uses_av_dict_free() {
        // SAFETY: `av_malloc` is asked for exactly the private C dictionary
        // layout established by dict.c. The subsequent raw write initialises
        // both fields before the allocation is adopted as an AVDictionary.
        let raw = unsafe { ffi::av_malloc(size_of::<EmptyDictionary>()) }.cast::<EmptyDictionary>();
        assert!(!raw.is_null());
        // SAFETY: `raw` denotes a suitably aligned, uniquely owned allocation
        // large enough for `EmptyDictionary`; both fields are valid C values.
        unsafe {
            raw.write(EmptyDictionary {
                count: 0,
                elems: core::ptr::null_mut(),
            });
        }

        // SAFETY: the initialised private layout is exactly AVDictionary's C
        // representation and came from the allocator its destructor expects.
        let dictionary = unsafe {
            CBox::<AVDictionary>::from_raw(raw.cast::<ffi::AVDictionary>())
                .expect("av_malloc returned a non-null dictionary")
        };
        assert_eq!(dictionary.as_ref().as_ptr(), raw.cast_const().cast());
        drop(dictionary);
    }
}

use ffibox::{CBox, CrustifyStr};

use crate::mem::AvFree;

/// Optional dictionary ownership. An empty dictionary is represented by
/// `None`, matching libavutil's nullable `AVDictionary *` convention.
#[derive(Default)]
pub struct Dictionary {
    inner: Option<CBox<AVDictionary>>,
}

impl Dictionary {
    fn as_ptr(&self) -> *const ffi::AVDictionary {
        self.inner.as_ref().map_or(core::ptr::null(), |dictionary| {
            dictionary.as_ptr().cast_const()
        })
    }

    fn take_raw(&mut self) -> *mut ffi::AVDictionary {
        self.inner
            .take()
            .map_or(core::ptr::null_mut(), CBox::into_raw)
    }

    unsafe fn restore_raw(&mut self, pointer: *mut ffi::AVDictionary) {
        // SAFETY: callers pass back the unique nullable dictionary pointer
        // produced by the just-completed `AVDictionary **` operation.
        self.inner = unsafe { CBox::from_raw(pointer) };
    }
}

/// An entry cursor carrying the identity of the dictionary it belongs to.
/// This prevents safe code from passing an entry from one dictionary as the
/// `previous` cursor for another, which would make C subtract unrelated pointers.
#[derive(Clone, Copy)]
pub struct DictionaryEntry<'a> {
    entry: AVDictionaryEntryRef<'a>,
    dictionary: *const ffi::AVDictionary,
}

impl DictionaryEntry<'_> {
    #[must_use]
    pub fn key(&self) -> &CStr {
        self.entry.key()
    }

    #[must_use]
    pub fn value(&self) -> &CStr {
        self.entry.value()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DictionaryEntryMismatch;

fn previous_pointer<'a>(
    dictionary: &'a Dictionary,
    previous: Option<DictionaryEntry<'a>>,
) -> Result<*const ffi::AVDictionaryEntry, DictionaryEntryMismatch> {
    match previous {
        Some(previous) if previous.dictionary != dictionary.as_ptr() => {
            Err(DictionaryEntryMismatch)
        }
        Some(previous) => Ok(previous.entry.as_ptr()),
        None => Ok(core::ptr::null()),
    }
}

fn status_result(status: i32) -> Result<(), i32> {
    if status < 0 { Err(status) } else { Ok(()) }
}

const DONT_STRDUP_KEY: i32 = 4;
const DONT_STRDUP_VALUE: i32 = 8;

/// Wraps: av_dict_copy
pub fn av_dict_copy(
    destination: &mut Dictionary,
    source: &Dictionary,
    flags: i32,
) -> Result<(), i32> {
    let mut raw = destination.take_raw();
    // The ownership-transfer flags are incompatible with a borrowed source;
    // this safe variant always asks C to duplicate both strings.
    let flags = flags & !(DONT_STRDUP_KEY | DONT_STRDUP_VALUE);
    // SAFETY: `raw` is the nullable unique destination owner slot and `source`
    // remains shared and live. C does not retain the source pointer.
    let status = unsafe { ffi::av_dict_copy(&raw mut raw, source.as_ptr(), flags) };
    // SAFETY: C returns the destination slot as null or one unique live owner,
    // on both success and partial-copy failure.
    unsafe { destination.restore_raw(raw) };
    status_result(status)
}

/// Wraps: av_dict_count
#[must_use]
pub fn av_dict_count(dictionary: &Dictionary) -> usize {
    // SAFETY: the nullable shared pointer stays live and C only reads it.
    let count = unsafe { ffi::av_dict_count(dictionary.as_ptr()) };
    usize::try_from(count).expect("libavutil returned a negative dictionary count")
}

/// Wraps: av_dict_get
pub fn av_dict_get<'a>(
    dictionary: &'a Dictionary,
    key: &CStr,
    previous: Option<DictionaryEntry<'a>>,
    flags: i32,
) -> Result<Option<DictionaryEntry<'a>>, DictionaryEntryMismatch> {
    let previous = previous_pointer(dictionary, previous)?;
    // SAFETY: the dictionary and key remain live and unmodified for the call;
    // the identity check above proves `previous` belongs to this dictionary.
    let entry = unsafe { ffi::av_dict_get(dictionary.as_ptr(), key.as_ptr(), previous, flags) };
    // SAFETY: a non-null result is an entry borrowed from `dictionary`; the
    // returned handle is tied to that borrow and exposes no mutation.
    let entry = unsafe { AVDictionaryEntryRef::from_ptr(entry) };
    Ok(entry.map(|entry| DictionaryEntry {
        entry,
        dictionary: dictionary.as_ptr(),
    }))
}

/// Wraps: av_dict_get_string
pub fn av_dict_get_string(
    dictionary: &Dictionary,
    key_value_separator: u8,
    pair_separator: u8,
) -> Result<CrustifyStr<AvFree>, i32> {
    let mut string = core::ptr::null_mut();
    // SAFETY: the shared dictionary remains live, while `string` is a distinct
    // writable output slot. On success C transfers a fresh av_malloc string.
    let status = unsafe {
        ffi::av_dict_get_string(
            dictionary.as_ptr(),
            &raw mut string,
            key_value_separator as core::ffi::c_char,
            pair_separator as core::ffi::c_char,
        )
    };
    // SAFETY: any non-null output is a uniquely owned NUL-terminated
    // av_malloc-family allocation, even on a defensive failure path.
    let owner = unsafe { CrustifyStr::<AvFree>::from_raw(string) };
    if status < 0 {
        drop(owner);
        Err(status)
    } else {
        owner.ok_or(status)
    }
}

/// Wraps: av_dict_iterate
pub fn av_dict_iterate<'a>(
    dictionary: &'a Dictionary,
    previous: Option<DictionaryEntry<'a>>,
) -> Result<Option<DictionaryEntry<'a>>, DictionaryEntryMismatch> {
    let previous = previous_pointer(dictionary, previous)?;
    // SAFETY: the identity check proves `previous` is null or borrowed from
    // this dictionary; no mutation occurs and C retains nothing.
    let entry = unsafe { ffi::av_dict_iterate(dictionary.as_ptr(), previous) };
    // SAFETY: a non-null entry is borrowed from `dictionary` for `'a`.
    let entry = unsafe { AVDictionaryEntryRef::from_ptr(entry.cast_mut()) };
    Ok(entry.map(|entry| DictionaryEntry {
        entry,
        dictionary: dictionary.as_ptr(),
    }))
}

/// Wraps: av_dict_parse_string
pub fn av_dict_parse_string(
    dictionary: &mut Dictionary,
    input: Option<&CStr>,
    key_value_separators: &CStr,
    pair_separators: &CStr,
    flags: i32,
) -> Result<(), i32> {
    let mut raw = dictionary.take_raw();
    // SAFETY: `raw` is a unique nullable owner slot. All strings are live and
    // read-only for the call; this C function explicitly clears transfer flags.
    let status = unsafe {
        ffi::av_dict_parse_string(
            &raw mut raw,
            input.map_or(core::ptr::null(), CStr::as_ptr),
            key_value_separators.as_ptr(),
            pair_separators.as_ptr(),
            flags,
        )
    };
    // SAFETY: the returned slot is null or the unique live dictionary owner.
    unsafe { dictionary.restore_raw(raw) };
    status_result(status)
}

/// Wraps: av_dict_set
pub fn av_dict_set(
    dictionary: &mut Dictionary,
    key: &CStr,
    value: Option<&CStr>,
    flags: i32,
) -> Result<(), i32> {
    let mut raw = dictionary.take_raw();
    // A borrowed Rust string cannot be transferred. This variant preserves all
    // other flags while forcing C to duplicate key and value.
    let flags = flags & !(DONT_STRDUP_KEY | DONT_STRDUP_VALUE);
    // SAFETY: `raw` is a unique nullable owner slot and the borrowed strings
    // stay live for the call. With transfer flags cleared, C retains only copies.
    let status = unsafe {
        ffi::av_dict_set(
            &raw mut raw,
            key.as_ptr(),
            value.map_or(core::ptr::null(), CStr::as_ptr),
            flags,
        )
    };
    // SAFETY: the returned slot is null or the unique live dictionary owner.
    unsafe { dictionary.restore_raw(raw) };
    status_result(status)
}

/// Wraps: av_dict_set_int
pub fn av_dict_set_int(
    dictionary: &mut Dictionary,
    key: &CStr,
    value: i64,
    flags: i32,
) -> Result<(), i32> {
    let mut raw = dictionary.take_raw();
    let flags = flags & !DONT_STRDUP_KEY;
    // SAFETY: `raw` is the unique nullable owner slot and transfer of the
    // borrowed key is disabled, so C retains only its own duplicate.
    let status = unsafe { ffi::av_dict_set_int(&raw mut raw, key.as_ptr(), value, flags) };
    // SAFETY: the returned slot is null or the unique live dictionary owner.
    unsafe { dictionary.restore_raw(raw) };
    status_result(status)
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn dictionary_mutation_iteration_copy_and_string_ownership() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"artist", Some(c"Crustify"), 0).unwrap();
        av_dict_set_int(&mut dictionary, c"year", 2026, 0).unwrap();
        assert_eq!(av_dict_count(&dictionary), 2);

        let artist = av_dict_get(&dictionary, c"artist", None, 0)
            .unwrap()
            .unwrap();
        assert_eq!(artist.value(), c"Crustify");
        assert!(av_dict_iterate(&dictionary, None).unwrap().is_some());

        let encoded = av_dict_get_string(&dictionary, b'=', b',').unwrap();
        assert!(
            encoded
                .as_bytes()
                .windows(15)
                .any(|s| s == b"artist=Crustify")
        );

        let mut copied = Dictionary::default();
        av_dict_copy(&mut copied, &dictionary, 0).unwrap();
        assert_eq!(av_dict_count(&copied), 2);

        av_dict_parse_string(&mut copied, Some(c"answer=42"), c"=", c",", 0).unwrap();
        assert!(av_dict_get(&copied, c"answer", None, 0).unwrap().is_some());
    }
}
