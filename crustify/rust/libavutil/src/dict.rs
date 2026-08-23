//! Wrappers for libavutil dictionaries.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CBox, CDropped, CrustifyStr, define_ctype};

use crate::ffi;
use crate::mem::AvFree;

define_ctype!(
    /// Wraps: AVDictionaryEntry
    ///
    /// A dictionary owns entries by value in its internal array. The layout type
    /// exists for embedding and FFI-compatible storage; access goes through
    /// [`AVDictionaryEntryRef`] and [`AVDictionaryEntryMut`] handles, never through
    /// a Rust reference to the C object.
    ///
    /// # Handle invariant
    ///
    /// Both borrowed handles carry more than `from_ptr`'s generic "live and
    /// initialised" contract: the entry must be one an `AVDictionary` holds, so
    /// that `key` and `value` each address an av_malloc-family NUL-terminated
    /// string. `av_dict_set` inserts an entry only once it has both, and
    /// `av_dict_free` clears both when it drops one, so every entry published by
    /// `av_dict_get` or `av_dict_iterate` satisfies this. A zeroed or otherwise
    /// hand-built [`AVDictionaryEntry`] does not, and its handles' safe getters
    /// would then read through a null pointer.
    ///
    /// No setters exist: the public dictionary API declares a published entry's
    /// key and value read-only, and the dictionary is the only writer.
    AVDictionaryEntry,
    AVDictionaryEntryRef,
    AVDictionaryEntryMut,
    ffi::AVDictionaryEntry
);

impl<'a> AVDictionaryEntryRef<'a> {
    /// Wraps: AVDictionaryEntry.value
    ///
    /// The returned string is borrowed from the dictionary that owns the entry,
    /// not from this handle. Although libavutil may replace or append to the
    /// allocation while mutating its dictionary, the public dictionary API
    /// forbids mutation while a borrowed entry is in use.
    #[must_use]
    pub fn value(&self) -> &'a CStr {
        // SAFETY: the handle addresses a live initialised entry; raw-place
        // projection copies the pointer field without forming a reference to
        // the C object.
        let value = unsafe { addr_of!((*self.as_ptr()).value).read() };
        // SAFETY: by the handle invariant this entry belongs to a dictionary,
        // so `value` is a non-null NUL-terminated string that dictionary keeps
        // live for `'a` — the lifetime for which the entry itself is borrowed.
        unsafe { CStr::from_ptr(value.cast_const()) }
    }

    /// Wraps: AVDictionaryEntry.key
    ///
    /// The returned string is borrowed from the dictionary that owns the entry
    /// and is read-only, as required by the public dictionary API.
    #[must_use]
    pub fn key(&self) -> &'a CStr {
        // SAFETY: the handle addresses a live initialised entry; raw-place
        // projection copies the pointer field without forming a reference to
        // the C object.
        let key = unsafe { addr_of!((*self.as_ptr()).key).read() };
        // SAFETY: by the handle invariant this entry belongs to a dictionary,
        // so `key` is a non-null NUL-terminated string that dictionary keeps
        // live for `'a` — the lifetime for which the entry itself is borrowed.
        unsafe { CStr::from_ptr(key.cast_const()) }
    }
}

define_ctype!(
    /// Wraps: AVDictionary
    ///
    /// libavutil publishes the dictionary as an opaque header owning a
    /// growable array of by-value [`AVDictionaryEntry`] elements, each holding
    /// its own key and value allocation; `av_dict_free` tears down that whole
    /// graph.
    ///
    /// [`AVDictionaryRef`] carries the read-only operations, so a dictionary
    /// borrowed out of another C object — an `AVFrame`'s metadata, for example
    /// — can be counted, searched and iterated. [`AVDictionaryMut`] carries
    /// none: every mutating entry point takes an `AVDictionary **` owner slot
    /// rather than the header, because it may reallocate or release the header
    /// itself. Mutation therefore goes through [`Dictionary`], which owns that
    /// slot.
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

/// Optional dictionary ownership. An empty dictionary is represented by
/// `None`, matching libavutil's nullable `AVDictionary *` convention: C creates
/// the header on the first insertion and frees it again when the last entry is
/// removed, so an owner that always held a pointer could not model the type.
#[derive(Default)]
pub struct Dictionary {
    inner: Option<CBox<AVDictionary>>,
}

impl Dictionary {
    /// Adopts an owned dictionary, as handed out by
    /// [`AVFrameMut::replace_metadata`](crate::frame::AVFrameMut::replace_metadata)
    /// or left behind by
    /// [`av_opt_set_dict`](crate::opt::av_opt_set_dict).
    #[must_use]
    pub fn from_owner(owner: Option<CBox<AVDictionary>>) -> Self {
        Self { inner: owner }
    }

    /// Surrenders the owned dictionary to a consumer that stores an
    /// `AVDictionary *` of its own.
    #[must_use]
    pub fn into_owner(self) -> Option<CBox<AVDictionary>> {
        self.inner
    }

    /// Borrows the dictionary header, or `None` while it holds no entries.
    ///
    /// The read-only operations accept exactly this, so they apply equally to
    /// a dictionary owned here and to one borrowed out of another C object.
    /// `Option` rather than a bare handle because C represents the empty
    /// dictionary as a null `AVDictionary *`, with no header to borrow.
    #[must_use]
    pub fn as_ref(&self) -> Option<AVDictionaryRef<'_>> {
        self.inner.as_ref().map(CBox::as_ref)
    }

    /// Runs one libavutil `AVDictionary **` operation over this owner's slot.
    ///
    /// The dictionary is moved out of `self` into a local slot for the call and
    /// re-adopted from that slot afterwards. That is what makes the mutating
    /// operations expressible at all: `av_dict_set` allocates the header on the
    /// first insertion and calls `av_freep(pm)` when its last entry is removed,
    /// so the pointer the owner holds is not stable across a call. Re-adoption
    /// is unconditional here rather than repeated at each call site, so no
    /// early return can drop the moved-out dictionary on the floor.
    ///
    /// # Safety
    ///
    /// `operation` must invoke a libavutil `AVDictionary **` entry point on the
    /// slot it is handed and must not retain the slot's address. On every path,
    /// including its failure paths, it must leave the slot null or holding
    /// exactly one uniquely owned, fully constructed dictionary releasable by
    /// `av_dict_free`.
    unsafe fn with_owner_slot<R>(
        &mut self,
        operation: impl FnOnce(&mut *mut ffi::AVDictionary) -> R,
    ) -> R {
        let mut slot = self
            .inner
            .take()
            .map_or(core::ptr::null_mut(), CBox::into_raw);
        let result = operation(&mut slot);
        // SAFETY: by the caller's contract the slot now holds null or exactly
        // one uniquely owned dictionary, which is what `CBox` adopts. The old
        // owner was moved out above, so nothing else can release it.
        self.inner = unsafe { CBox::from_raw(slot) };
        result
    }
}

/// An entry cursor carrying the identity of the dictionary it belongs to.
/// This prevents safe code from passing an entry from one dictionary as the
/// `previous` cursor for another, which would make C subtract unrelated pointers.
#[derive(Clone, Copy)]
pub struct DictionaryEntry<'a> {
    entry: AVDictionaryEntryRef<'a>,
    dictionary: AVDictionaryRef<'a>,
}

impl<'a> DictionaryEntry<'a> {
    /// The entry key, borrowed from the dictionary rather than from this
    /// cursor, which is only a `Copy` handle to it.
    #[must_use]
    pub fn key(&self) -> &'a CStr {
        self.entry.key()
    }

    /// The entry value, borrowed from the dictionary for as long as the shared
    /// borrow that produced this cursor.
    #[must_use]
    pub fn value(&self) -> &'a CStr {
        self.entry.value()
    }
}

/// Returned when a cursor entry does not belong to the dictionary it is used
/// with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DictionaryEntryMismatch;

/// Checks that `previous` came from `dictionary` before it is handed to C as an
/// iteration cursor, since `av_dict_iterate` recovers the cursor's index by
/// subtracting the dictionary's entry array from it.
fn checked_previous<'a>(
    dictionary: Option<AVDictionaryRef<'a>>,
    previous: Option<DictionaryEntry<'a>>,
) -> Result<Option<AVDictionaryEntryRef<'a>>, DictionaryEntryMismatch> {
    match (dictionary, previous) {
        (_, None) => Ok(None),
        (Some(dictionary), Some(previous))
            if core::ptr::eq(previous.dictionary.as_ptr(), dictionary.as_ptr()) =>
        {
            Ok(Some(previous.entry))
        }
        _ => Err(DictionaryEntryMismatch),
    }
}

fn status_result(status: i32) -> Result<(), i32> {
    if status < 0 { Err(status) } else { Ok(()) }
}

/// The two flags that hand C ownership of the caller's key and value strings.
/// A borrowed `&CStr` can never be transferred, so every wrapper below clears
/// whichever of them its C entry point still honours.
const DONT_STRDUP_KEY: i32 = ffi::AV_DICT_DONT_STRDUP_KEY as i32;
const DONT_STRDUP_VAL: i32 = ffi::AV_DICT_DONT_STRDUP_VAL as i32;

/// Wraps: av_dict_copy
///
/// `source` is read-only, so it may be a dictionary borrowed out of any C
/// object. Copying a dictionary into itself is ruled out by `destination`'s
/// exclusive borrow.
pub fn av_dict_copy(
    destination: &mut Dictionary,
    source: Option<AVDictionaryRef<'_>>,
    flags: i32,
) -> Result<(), i32> {
    // `av_dict_copy` forwards `flags` verbatim to `av_dict_set` for every
    // entry, so the transfer flags would make C free the source's own strings.
    let flags = flags & !(DONT_STRDUP_KEY | DONT_STRDUP_VAL);
    let source = source.map_or(core::ptr::null(), |source| source.as_ptr());
    let copy = |slot: &mut *mut ffi::AVDictionary| {
        // SAFETY: `source` stays live and shared for the call, and C only reads
        // it; the transfer flags are cleared, so C duplicates every string.
        unsafe { ffi::av_dict_copy(slot, source, flags) }
    };
    // SAFETY: `av_dict_copy` is an `AVDictionary **` entry point; it leaves the
    // destination slot null or holding one unique live owner on success and on
    // a partial-copy failure alike, and retains neither slot nor source.
    let status = unsafe { destination.with_owner_slot(copy) };
    status_result(status)
}

/// Wraps: av_dict_count
#[must_use]
pub fn av_dict_count(dictionary: Option<AVDictionaryRef<'_>>) -> usize {
    // SAFETY: the nullable shared pointer stays live and C only reads it.
    let count = unsafe {
        ffi::av_dict_count(dictionary.map_or(core::ptr::null(), |dictionary| dictionary.as_ptr()))
    };
    usize::try_from(count).expect("libavutil returned a negative dictionary count")
}

/// Wraps: av_dict_get
pub fn av_dict_get<'a>(
    dictionary: Option<AVDictionaryRef<'a>>,
    key: &CStr,
    previous: Option<DictionaryEntry<'a>>,
    flags: i32,
) -> Result<Option<DictionaryEntry<'a>>, DictionaryEntryMismatch> {
    let previous = checked_previous(dictionary, previous)?;
    let previous = previous.map_or(core::ptr::null(), |previous| previous.as_ptr());
    // SAFETY: the dictionary and key remain live and unmodified for the call;
    // the identity check above proves `previous` belongs to this dictionary.
    let entry = unsafe {
        ffi::av_dict_get(
            dictionary.map_or(core::ptr::null(), |dictionary| dictionary.as_ptr()),
            key.as_ptr(),
            previous,
            flags,
        )
    };
    // SAFETY: a non-null result is an entry borrowed from `dictionary` for
    // `'a`; the shared handle exposes no mutation of it.
    let entry = unsafe { AVDictionaryEntryRef::from_ptr(entry) };
    Ok(entry
        .zip(dictionary)
        .map(|(entry, dictionary)| DictionaryEntry { entry, dictionary }))
}

/// Wraps: av_dict_get_string
///
/// Fails with `AVERROR(EINVAL)` when the two separators are equal, NUL, or a
/// backslash, which is C's own validation.
pub fn av_dict_get_string(
    dictionary: Option<AVDictionaryRef<'_>>,
    key_value_separator: u8,
    pair_separator: u8,
) -> Result<CrustifyStr<AvFree>, i32> {
    let mut string = core::ptr::null_mut();
    // SAFETY: the shared dictionary remains live, while `string` is a distinct
    // writable output slot. On success C transfers a fresh av_malloc string.
    let status = unsafe {
        ffi::av_dict_get_string(
            dictionary.map_or(core::ptr::null(), |dictionary| dictionary.as_ptr()),
            &raw mut string,
            key_value_separator as core::ffi::c_char,
            pair_separator as core::ffi::c_char,
        )
    };
    // SAFETY: any non-null output is a uniquely owned NUL-terminated
    // av_malloc-family allocation, even on a defensive failure path. The
    // argument-validation path returns before writing, which the null
    // initialisation above covers.
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
    dictionary: Option<AVDictionaryRef<'a>>,
    previous: Option<DictionaryEntry<'a>>,
) -> Result<Option<DictionaryEntry<'a>>, DictionaryEntryMismatch> {
    let previous = checked_previous(dictionary, previous)?;
    let previous = previous.map_or(core::ptr::null(), |previous| previous.as_ptr());
    // SAFETY: the identity check proves `previous` is null or borrowed from
    // this dictionary; no mutation occurs and C retains nothing.
    let entry = unsafe {
        ffi::av_dict_iterate(
            dictionary.map_or(core::ptr::null(), |dictionary| dictionary.as_ptr()),
            previous,
        )
    };
    // SAFETY: a non-null entry is borrowed from `dictionary` for `'a`.
    let entry = unsafe { AVDictionaryEntryRef::from_ptr(entry.cast_mut()) };
    Ok(entry
        .zip(dictionary)
        .map(|(entry, dictionary)| DictionaryEntry { entry, dictionary }))
}

/// Wraps: av_dict_parse_string
pub fn av_dict_parse_string(
    dictionary: &mut Dictionary,
    input: Option<&CStr>,
    key_value_separators: &CStr,
    pair_separators: &CStr,
    flags: i32,
) -> Result<(), i32> {
    let input = input.map_or(core::ptr::null(), CStr::as_ptr);
    let parse = |slot: &mut *mut ffi::AVDictionary| {
        // SAFETY: all strings are live and read-only for the call; this C
        // function clears the transfer flags itself.
        unsafe {
            ffi::av_dict_parse_string(
                slot,
                input,
                key_value_separators.as_ptr(),
                pair_separators.as_ptr(),
                flags,
            )
        }
    };
    // SAFETY: `av_dict_parse_string` is an `AVDictionary **` entry point which
    // leaves the slot null or holding one unique live owner on every path, and
    // retains nothing.
    let status = unsafe { dictionary.with_owner_slot(parse) };
    status_result(status)
}

/// Wraps: av_dict_set
///
/// A `None` value removes the entry, and removing the last entry releases the
/// dictionary header, leaving `dictionary` empty again.
pub fn av_dict_set(
    dictionary: &mut Dictionary,
    key: &CStr,
    value: Option<&CStr>,
    flags: i32,
) -> Result<(), i32> {
    // A borrowed Rust string cannot be transferred. This variant preserves all
    // other flags while forcing C to duplicate key and value.
    let flags = flags & !(DONT_STRDUP_KEY | DONT_STRDUP_VAL);
    let value = value.map_or(core::ptr::null(), CStr::as_ptr);
    let set = |slot: &mut *mut ffi::AVDictionary| {
        // SAFETY: the borrowed strings stay live for the call; with the
        // transfer flags cleared C retains only its own duplicates.
        unsafe { ffi::av_dict_set(slot, key.as_ptr(), value, flags) }
    };
    // SAFETY: `av_dict_set` is an `AVDictionary **` entry point. It allocates
    // the header, frees it and stores null when the last entry goes, and leaves
    // one unique live owner otherwise, including on its ENOMEM paths.
    let status = unsafe { dictionary.with_owner_slot(set) };
    status_result(status)
}

/// Wraps: av_dict_set_int
pub fn av_dict_set_int(
    dictionary: &mut Dictionary,
    key: &CStr,
    value: i64,
    flags: i32,
) -> Result<(), i32> {
    // Only the key can be transferred here: `av_dict_set_int` formats the value
    // into a stack buffer and clears the value transfer flag itself.
    let flags = flags & !DONT_STRDUP_KEY;
    let set_int = |slot: &mut *mut ffi::AVDictionary| {
        // SAFETY: the borrowed key stays live for the call and its transfer is
        // disabled, so C retains only its own duplicate.
        unsafe { ffi::av_dict_set_int(slot, key.as_ptr(), value, flags) }
    };
    // SAFETY: `av_dict_set_int` forwards to `av_dict_set` on the same slot and
    // inherits its owner-slot contract.
    let status = unsafe { dictionary.with_owner_slot(set_int) };
    status_result(status)
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_int, c_void};
    use core::mem::{align_of, size_of};

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

    #[test]
    fn transfer_flags_are_cleared_from_the_generated_constants() {
        assert_eq!(DONT_STRDUP_KEY, 4);
        assert_eq!(DONT_STRDUP_VAL, 8);
    }
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn dictionary_mutation_iteration_copy_and_string_ownership() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"artist", Some(c"Crustify"), 0).unwrap();
        av_dict_set_int(&mut dictionary, c"year", 2026, 0).unwrap();
        assert_eq!(av_dict_count(dictionary.as_ref()), 2);

        let artist = av_dict_get(dictionary.as_ref(), c"artist", None, 0)
            .unwrap()
            .unwrap();
        assert_eq!(artist.value(), c"Crustify");
        assert!(
            av_dict_iterate(dictionary.as_ref(), None)
                .unwrap()
                .is_some()
        );

        let encoded = av_dict_get_string(dictionary.as_ref(), b'=', b',').unwrap();
        assert!(
            encoded
                .as_bytes()
                .windows(15)
                .any(|s| s == b"artist=Crustify")
        );

        let mut copied = Dictionary::default();
        av_dict_copy(&mut copied, dictionary.as_ref(), 0).unwrap();
        assert_eq!(av_dict_count(copied.as_ref()), 2);

        av_dict_parse_string(&mut copied, Some(c"answer=42"), c"=", c",", 0).unwrap();
        assert!(
            av_dict_get(copied.as_ref(), c"answer", None, 0)
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn an_empty_dictionary_reads_as_a_null_borrow() {
        let dictionary = Dictionary::default();
        assert!(dictionary.as_ref().is_none());
        assert_eq!(av_dict_count(None), 0);
        assert!(av_dict_get(None, c"absent", None, 0).unwrap().is_none());
        assert!(av_dict_iterate(None, None).unwrap().is_none());
        assert_eq!(
            av_dict_get_string(None, b'=', b',').unwrap().as_bytes(),
            b""
        );
    }

    #[test]
    fn removing_the_last_entry_releases_the_header() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"only", Some(c"1"), 0).unwrap();
        assert!(dictionary.as_ref().is_some());

        // C frees the header and stores null in the owner slot here; the
        // wrapper has to adopt that null rather than keep the stale pointer.
        av_dict_set(&mut dictionary, c"only", None, 0).unwrap();
        assert!(dictionary.as_ref().is_none());
        assert_eq!(av_dict_count(dictionary.as_ref()), 0);
    }

    #[test]
    fn entry_strings_outlive_the_cursor_that_produced_them() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"artist", Some(c"Crustify"), 0).unwrap();

        // The strings belong to the dictionary, so they stay borrowable after
        // the `Copy` cursor and its entry handle are gone.
        let (key, value) = {
            let entry = av_dict_get(dictionary.as_ref(), c"artist", None, 0)
                .unwrap()
                .unwrap();
            (entry.key(), entry.value())
        };

        assert_eq!(key, c"artist");
        assert_eq!(value, c"Crustify");
        assert_eq!(av_dict_count(dictionary.as_ref()), 1);
    }

    #[test]
    fn a_cursor_from_another_dictionary_is_refused() {
        let mut first = Dictionary::default();
        av_dict_set(&mut first, c"k", Some(c"v"), 0).unwrap();
        let mut second = Dictionary::default();
        av_dict_set(&mut second, c"k", Some(c"v"), 0).unwrap();

        let cursor = av_dict_iterate(first.as_ref(), None).unwrap().unwrap();
        assert_eq!(
            av_dict_iterate(second.as_ref(), Some(cursor)).err(),
            Some(DictionaryEntryMismatch)
        );
        assert_eq!(
            av_dict_get(second.as_ref(), c"k", Some(cursor), 0).err(),
            Some(DictionaryEntryMismatch)
        );
        assert_eq!(
            av_dict_iterate(None, Some(cursor)).err(),
            Some(DictionaryEntryMismatch)
        );
        // The same cursor is accepted by its own dictionary, which has one
        // entry, so iteration ends there.
        assert!(
            av_dict_iterate(first.as_ref(), Some(cursor))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn transfer_flags_are_ignored_rather_than_honoured() {
        let mut dictionary = Dictionary::default();
        // Were these forwarded, C would take ownership of the static strings
        // and `av_free` them.
        av_dict_set(
            &mut dictionary,
            c"k",
            Some(c"v"),
            DONT_STRDUP_KEY | DONT_STRDUP_VAL,
        )
        .unwrap();
        av_dict_set_int(&mut dictionary, c"n", 7, DONT_STRDUP_KEY).unwrap();

        let mut copied = Dictionary::default();
        av_dict_copy(
            &mut copied,
            dictionary.as_ref(),
            DONT_STRDUP_KEY | DONT_STRDUP_VAL,
        )
        .unwrap();
        assert_eq!(av_dict_count(copied.as_ref()), 2);
        assert_eq!(
            av_dict_get(dictionary.as_ref(), c"k", None, 0)
                .unwrap()
                .unwrap()
                .value(),
            c"v"
        );
    }

    #[test]
    fn separator_validation_reports_c_s_error() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"k", Some(c"v"), 0).unwrap();
        assert!(av_dict_get_string(dictionary.as_ref(), b'=', b'=').is_err());
        assert!(av_dict_get_string(dictionary.as_ref(), b'\\', b',').is_err());
    }

    #[test]
    fn ownership_round_trips_through_the_boxed_owner() {
        let mut dictionary = Dictionary::default();
        av_dict_set(&mut dictionary, c"k", Some(c"v"), 0).unwrap();

        let owner = dictionary.into_owner().expect("a populated dictionary");
        let dictionary = Dictionary::from_owner(Some(owner));
        assert_eq!(av_dict_count(dictionary.as_ref()), 1);

        assert!(Dictionary::from_owner(None).as_ref().is_none());
    }

    #[test]
    fn a_borrowed_dictionary_is_readable_without_owning_it() {
        let mut owner = Dictionary::default();
        av_dict_set(&mut owner, c"a", Some(c"1"), 0).unwrap();
        av_dict_set(&mut owner, c"b", Some(c"2"), 0).unwrap();

        // The shape a dictionary reached through another C object arrives in.
        let borrowed: Option<AVDictionaryRef<'_>> = owner.as_ref();
        let mut seen = 0;
        let mut cursor = None;
        while let Some(entry) = av_dict_iterate(borrowed, cursor).unwrap() {
            assert!(!entry.key().to_bytes().is_empty());
            seen += 1;
            cursor = Some(entry);
        }
        assert_eq!(seen, av_dict_count(borrowed));
    }
}
