//! Wrappers for libavutil dictionaries.

use core::ffi::CStr;
use core::ptr::addr_of;

use ffibox::define_ctype;

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

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

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
}
