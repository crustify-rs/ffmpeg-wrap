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
