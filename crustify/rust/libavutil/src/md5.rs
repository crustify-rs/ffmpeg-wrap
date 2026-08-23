//! Wrappers for libavutil MD5 utilities.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CCell, CDropped, CPtr, CType};

use crate::ffi;

/// Wraps: AVMD5
///
/// Opaque MD5 state allocated by [`ffi::av_md5_alloc`]. The public C header
/// deliberately withholds the layout, so this wrapper cannot be constructed
/// inline; an allocation is instead owned as [`ffibox::CBox<AVMD5>`]. Its drop
/// uses [`ffi::av_free`], matching every in-tree owner of an allocated MD5
/// context.
///
/// Access is carried by [`AVMD5Ref`] and [`AVMD5Mut`]. They hold a pointer and
/// a borrow lifetime without ever forming a Rust reference over storage that C
/// may mutate.
#[repr(transparent)]
pub struct AVMD5(CType<ffi::AVMD5>);

/// Shared borrowed handle to an [`AVMD5`].
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AVMD5Ref<'a>(CPtr<'a, AVMD5>);

impl<'a> AVMD5Ref<'a> {
    /// Borrow an MD5 context pointer, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live, initialized context that remains live and
    /// is not mutably accessed for `'a`.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut ffi::AVMD5) -> Option<Self> {
        NonNull::new(ptr.cast::<AVMD5>()).map(|ptr| {
            // SAFETY: the caller supplies the liveness and shared-borrow
            // guarantees required by `CPtr::new`.
            Self(unsafe { CPtr::new(ptr) })
        })
    }

    /// Return the borrowed context pointer for a read-only FFI call.
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const ffi::AVMD5 {
        self.0.as_non_null().as_ptr().cast::<ffi::AVMD5>()
    }
}

/// Exclusive borrowed handle to an [`AVMD5`].
#[repr(transparent)]
pub struct AVMD5Mut<'a>(AVMD5Ref<'a>);

impl<'a> AVMD5Mut<'a> {
    /// Borrow an MD5 context pointer exclusively, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live, initialized context for `'a`, and no other
    /// handle to the context may be used while the result lives.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut ffi::AVMD5) -> Option<Self> {
        NonNull::new(ptr.cast::<AVMD5>()).map(|ptr| {
            // SAFETY: the caller supplies the liveness and exclusive-borrow
            // guarantees required by `CPtr::new`.
            Self(AVMD5Ref(unsafe { CPtr::new(ptr) }))
        })
    }

    /// Return the context pointer for a mutating FFI call.
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::AVMD5 {
        self.0.0.as_non_null().as_ptr().cast::<ffi::AVMD5>()
    }

    /// Reborrow this exclusive handle as a shared handle.
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> AVMD5Ref<'_> {
        self.0
    }
}

// SAFETY: `AVMD5` is transparent over `CType<ffi::AVMD5>`, both handles are
// transparent over `CPtr<'a, AVMD5>`, and the shared handle cannot write.
unsafe impl CCell for AVMD5 {
    type C = ffi::AVMD5;
    type Ref<'a> = AVMD5Ref<'a>;
    type Mut<'a> = AVMD5Mut<'a>;

    #[inline]
    unsafe fn ref_from_raw<'a>(ptr: NonNull<Self>) -> Self::Ref<'a> {
        // SAFETY: the caller upholds `CCell::ref_from_raw`'s liveness and
        // shared-borrow contract.
        AVMD5Ref(unsafe { CPtr::new(ptr) })
    }

    #[inline]
    unsafe fn mut_from_raw<'a>(ptr: NonNull<Self>) -> Self::Mut<'a> {
        // SAFETY: the caller upholds `CCell::mut_from_raw`'s liveness and
        // exclusive-borrow contract.
        AVMD5Mut(AVMD5Ref(unsafe { CPtr::new(ptr) }))
    }
}

// SAFETY: `AVMD5` allocations come from the `av_malloc` family and contain no
// separately owned fields. `av_free` is their matching one-shot releaser.
unsafe impl CDropped for AVMD5 {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract transfers a live, uniquely owned AVMD5
        // allocation to its allocator-matched release routine exactly once.
        unsafe { ffi::av_free(obj.as_ptr().cast::<c_void>()) }
    }
}

#[cfg(test)]
mod tests {
    use ffibox::CBox;

    use super::*;

    #[test]
    fn allocated_context_supports_shared_and_exclusive_borrows() {
        // SAFETY: `av_md5_alloc` returns null or a fresh initialized allocation
        // whose unique ownership transfers to the CBox; AVMD5's drop matches
        // its allocator.
        let mut context = unsafe { CBox::<AVMD5>::from_raw(ffi::av_md5_alloc()) }
            .expect("allocate AVMD5 context");
        let ptr = context.as_ptr();

        assert_eq!(context.as_ref().as_ptr().cast_mut(), ptr);
        {
            let mut exclusive = context.as_mut();
            assert_eq!(exclusive.as_mut_ptr(), ptr);
            assert_eq!(exclusive.as_ref().as_ptr().cast_mut(), ptr);
        }

        // Dropping `context` exercises the allocator-matched `av_free` path.
    }
}
