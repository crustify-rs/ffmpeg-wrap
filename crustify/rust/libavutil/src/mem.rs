//! Ownership strategies for memory allocated by libavutil.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CDropped, CLenCloned, CLenDropped};

use crate::ffi;

/// Wraps: av_free
///
/// Releases an allocation from the `av_malloc` family — `av_malloc`,
/// `av_mallocz`, `av_calloc`, `av_realloc`, `av_memdup`, `av_strdup` and the
/// dynarray helpers built on them. The recorded contract covers all three
/// byte-level shapes the pointer may hold, and each reaches `av_free` through
/// the owner that matches it:
///
/// | shape | owner | callsite it models |
/// |---|---|---|
/// | single value | [`CVoidBox<AvFree>`](ffibox::CVoidBox) | `av_free(info)` in `av_encryption_info_free` |
/// | counted buffer | [`CVec<T, AvFree>`](ffibox::CVec) | `av_free(e->threads)` in `executor_free` |
/// | NUL-terminated string | [`CrustifyStr<AvFree>`](ffibox::CrustifyStr) | `av_free(copy_key)` in `av_dict_set` |
///
/// `av_free` never needs the extent, so one strategy serves all three: the
/// [`CDropped`] impl carries the pointer-only owners and the [`CLenDropped`]
/// impl the counted one. For a counted buffer it also enables bytewise cloning
/// through [`av_memdup`](ffi::av_memdup).
///
/// Unlike `munmap`, the length-aware release must not short-circuit on a zero
/// byte length: `av_malloc(0)` retries with one byte, so a zero-length owner
/// still holds a live allocation that has to be freed.
///
/// `av_free` accepts NULL, but no owner reaching this strategy can hold one —
/// every ffibox owner is non-null by construction, so the null case is absorbed
/// at `from_raw` and never becomes a drop.
///
/// This is deliberately distinct from `libc`'s `LibcFree` strategy. The two
/// allocation families are not interchangeable: `av_free` resolves to
/// `_aligned_free` wherever `HAVE_ALIGNED_MALLOC` holds and to a prefixed
/// allocator under `MALLOC_PREFIX`, matching an `av_malloc` that allocates
/// through `posix_memalign`, `_aligned_malloc` or `memalign`. An `av_malloc`
/// allocation must never reach `LibcFree`, nor a `malloc` allocation this one.
pub struct AvFree;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `av_free`.
unsafe impl CDropped for AvFree {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract requires `obj` to denote a uniquely owned
        // allocation from the `av_malloc` family, which `av_free` accepts.
        unsafe { ffi::av_free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: `av_free` does not need the buffer length and accepts every
// allocation produced by the `av_malloc` family.
unsafe impl CLenDropped for AvFree {
    unsafe fn c_drop_len(ptr: *mut u8, _byte_len: usize) {
        // SAFETY: the trait contract requires an `av_malloc`-family allocation
        // and transfers its ownership to this call.
        unsafe { ffi::av_free(ptr.cast::<c_void>()) }
    }
}

/// Wraps: av_memdup
// SAFETY: `av_memdup` returns an independent `av_malloc` allocation containing
// exactly the requested byte copy, and `AvFree` releases that allocation.
unsafe impl CLenCloned for AvFree {
    unsafe fn c_clone_len(ptr: *mut u8, byte_len: usize) -> Option<NonNull<u8>> {
        // SAFETY: the trait contract guarantees `byte_len` readable bytes at
        // `ptr`; `av_memdup` only reads that range and preserves the source.
        NonNull::new(unsafe { ffi::av_memdup(ptr.cast_const().cast(), byte_len) }.cast())
    }
}

#[cfg(test)]
mod tests {
    use ffibox::{CVec, CVoidBox, CrustifyStr};

    use super::*;

    #[test]
    fn drops_scalar_allocation() {
        // SAFETY: `av_malloc(1)` returns null or a uniquely owned allocation
        // compatible with `AvFree`; `CVoidBox` adopts it exactly once.
        let allocation =
            unsafe { CVoidBox::<AvFree>::from_raw(ffi::av_malloc(1)) }.expect("av_malloc failed");
        drop(allocation);
    }

    #[test]
    fn clones_and_drops_counted_buffer() {
        const LEN: usize = 4;

        // SAFETY: `av_malloc(LEN)` returns null or a uniquely owned allocation
        // of at least LEN bytes, which this `CVec` adopts exactly once.
        let mut original =
            unsafe { CVec::<u8, AvFree>::from_raw_parts(ffi::av_malloc(LEN).cast(), LEN) }
                .expect("av_malloc failed");
        original.as_mut_slice().copy_from_slice(&[1, 2, 3, 4]);

        let cloned = original.try_clone().expect("av_memdup failed");
        assert_eq!(cloned.as_slice(), original.as_slice());
        assert_ne!(cloned.as_ptr(), original.as_ptr());
    }

    #[test]
    fn drops_empty_counted_buffer() {
        // `av_malloc(0)` allocates one byte rather than returning null, so a
        // zero-length owner still owns storage. `c_drop_len` must therefore
        // reach `av_free` for it — a `munmap`-style zero-length short-circuit
        // would leak here, which the sanitiser run turns into a failure.
        //
        // SAFETY: the checked `av_malloc(0)` result is a uniquely owned
        // allocation this `CVec` adopts exactly once; the element count is
        // zero, so no byte of it is ever read through the slice views.
        let empty = unsafe { CVec::<u8, AvFree>::from_raw_parts(ffi::av_malloc(0).cast(), 0) }
            .expect("av_malloc failed");
        assert!(empty.is_empty());
        drop(empty);
    }

    #[test]
    fn drops_terminated_string() {
        const TEXT: &[u8] = b"crustify\0";

        // SAFETY: `TEXT` is a distinct, fully initialised source of
        // `TEXT.len()` bytes ending in a NUL, so `av_memdup` copies the
        // terminator along with the text; the result is null or a uniquely
        // owned NUL-terminated `av_malloc` string, which is what `CrustifyStr`
        // adopts with `AvFree` as its matching destructor.
        let string = unsafe {
            CrustifyStr::<AvFree>::from_raw(ffi::av_memdup(TEXT.as_ptr().cast(), TEXT.len()).cast())
        }
        .expect("av_memdup failed");

        assert_eq!(string.as_bytes(), b"crustify");
        assert_eq!(string.len(), 8);
        drop(string);
    }
}
