//! Wrappers for `libavutil/hash.c`.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of_mut};

use ffibox::{CCell, CDropped, CPtr, CType};

use crate::ffi;

/// Wraps: AVHashContext
///
/// Opaque hash state allocated by `av_hash_alloc`. The public header withholds
/// its layout, so it cannot be constructed inline. An owning pointer is an
/// [`ffibox::CBox<AVHashContext>`], whose drop calls `av_hash_freep`; that
/// routine releases the context's optional algorithm state and its header.
///
/// This representation is written out instead of using
/// [`ffibox::define_ctype!`], because that macro would expose `zeroed()` for
/// bindgen's zero-sized incomplete declaration rather than a real context.
#[repr(transparent)]
pub struct AVHashContext(CType<ffi::AVHashContext>);

/// Shared borrowed handle to an [`AVHashContext`].
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AVHashContextRef<'a>(CPtr<'a, AVHashContext>);

impl<'a> AVHashContextRef<'a> {
    /// Borrows a context pointer, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live initialized context that remains live and is
    /// not mutably accessed for `'a`.
    pub unsafe fn from_ptr(ptr: *mut ffi::AVHashContext) -> Option<Self> {
        NonNull::new(ptr.cast::<AVHashContext>()).map(|ptr| {
            // SAFETY: the caller supplies the lifetime and shared-access
            // guarantees required by `CPtr::new`.
            Self(unsafe { CPtr::new(ptr) })
        })
    }

    /// Returns the borrowed pointer for a read-only FFI call.
    #[must_use]
    pub fn as_ptr(&self) -> *const ffi::AVHashContext {
        self.0.as_non_null().as_ptr().cast::<ffi::AVHashContext>()
    }
}

/// Exclusive borrowed handle to an [`AVHashContext`].
#[repr(transparent)]
pub struct AVHashContextMut<'a>(AVHashContextRef<'a>);

impl<'a> AVHashContextMut<'a> {
    /// Borrows a context pointer exclusively, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live initialized context for `'a`, and no other
    /// handle may be used while the result lives.
    pub unsafe fn from_ptr(ptr: *mut ffi::AVHashContext) -> Option<Self> {
        NonNull::new(ptr.cast::<AVHashContext>()).map(|ptr| {
            // SAFETY: the caller supplies the lifetime and exclusive-access
            // guarantees required by `CPtr::new`.
            Self(AVHashContextRef(unsafe { CPtr::new(ptr) }))
        })
    }

    /// Returns the context pointer for a mutating FFI call.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::AVHashContext {
        self.0.0.as_non_null().as_ptr().cast::<ffi::AVHashContext>()
    }

    /// Reborrows this exclusive handle as a shared handle.
    #[must_use]
    pub fn as_ref(&self) -> AVHashContextRef<'_> {
        self.0
    }
}

// SAFETY: `AVHashContext` is transparent over `CType<ffi::AVHashContext>`;
// both handles are transparent over `CPtr<'a, AVHashContext>`, and the shared
// handle exposes no mutating operation.
unsafe impl CCell for AVHashContext {
    type C = ffi::AVHashContext;
    type Ref<'a> = AVHashContextRef<'a>;
    type Mut<'a> = AVHashContextMut<'a>;

    unsafe fn ref_from_raw<'a>(ptr: NonNull<Self>) -> Self::Ref<'a> {
        // SAFETY: the caller upholds the `CCell` liveness and shared-access
        // contract for the constructed handle.
        AVHashContextRef(unsafe { CPtr::new(ptr) })
    }

    unsafe fn mut_from_raw<'a>(ptr: NonNull<Self>) -> Self::Mut<'a> {
        // SAFETY: the caller upholds the `CCell` liveness and exclusive-access
        // contract for the constructed handle.
        AVHashContextMut(AVHashContextRef(unsafe { CPtr::new(ptr) }))
    }
}

// SAFETY: every owned AVHashContext comes from `av_hash_alloc`, and
// `av_hash_freep` is its one-shot matching releaser. It disposes the optional
// owned algorithm context before freeing the header.
unsafe impl CDropped for AVHashContext {
    unsafe fn c_drop(context: NonNull<Self>) {
        let mut raw = context.as_ptr().cast::<ffi::AVHashContext>();
        // SAFETY: the trait contract transfers a live uniquely owned context
        // exactly once; `raw` is a writable local slot C may null after
        // consuming the pointee.
        unsafe { ffi::av_hash_freep(addr_of_mut!(raw)) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use ffibox::CBox;

    use super::*;

    #[test]
    fn opaque_handles_and_owner_are_pointer_sized() {
        assert_eq!(size_of::<AVHashContextRef<'_>>(), size_of::<*mut ()>());
        assert_eq!(size_of::<AVHashContextMut<'_>>(), size_of::<*mut ()>());
        assert_eq!(size_of::<CBox<AVHashContext>>(), size_of::<*mut ()>());
    }

    #[test]
    fn null_pointer_cannot_be_adopted_as_an_owner() {
        // SAFETY: null transfers no allocation, and `from_raw` represents it
        // as `None` without invoking the destructor.
        let owner = unsafe { CBox::<AVHashContext>::from_raw(core::ptr::null_mut()) };
        assert!(owner.is_none());
    }
}

/// Wraps: av_hash_names
#[must_use]
pub fn av_hash_names(index: i32) -> Option<&'static CStr> {
    // SAFETY: C returns null or a pointer into its immutable static hash table.
    let pointer = unsafe { ffi::av_hash_names(index) };
    if pointer.is_null() {
        None
    } else {
        // SAFETY: a non-null table entry is a static NUL-terminated name.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

#[cfg(test)]
mod scheduled_name_tests {
    use super::*;

    #[test]
    fn indexes_static_hash_names() {
        assert_eq!(av_hash_names(0), Some(c"MD5"));
        assert_eq!(av_hash_names(-1), None);
        assert_eq!(av_hash_names(i32::MAX), None);
    }
}
