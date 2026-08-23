//! Ownership strategies for the C standard allocator.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CDropped, CLenDropped};

use crate::ffi;

/// Wraps: free
///
/// Releases an allocation from the C standard allocation family. The recorded
/// contract covers all three byte-level shapes the pointer may hold, and each
/// reaches `free` through the owner that matches it:
///
/// | shape | owner |
/// |---|---|
/// | single value | [`CVoidBox<LibcFree>`](ffibox::CVoidBox) |
/// | counted buffer | [`CVec<T, LibcFree>`](ffibox::CVec) |
/// | NUL-terminated string | [`CrustifyStr<LibcFree>`](ffibox::CrustifyStr) |
///
/// `free` never needs the extent, so one strategy serves all three: the
/// [`CDropped`] impl carries the pointer-only owners and the [`CLenDropped`]
/// impl the counted one.
///
/// This is deliberately distinct from libavutil's `AvFree`. The two allocation
/// families are not interchangeable — `av_free` resolves to `_aligned_free`
/// wherever `HAVE_ALIGNED_MALLOC` holds, and to a prefixed allocator under
/// `MALLOC_PREFIX` — so an `av_malloc` allocation must never reach this
/// strategy, nor a `malloc` allocation reach that one.
pub struct LibcFree;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `free`.
unsafe impl CDropped for LibcFree {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract requires `obj` to denote a uniquely owned
        // C-standard-library allocation, which `free` accepts.
        unsafe { ffi::free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: `free` does not need the allocation length and releases any buffer
// produced by the C standard allocation family.
unsafe impl CLenDropped for LibcFree {
    unsafe fn c_drop_len(ptr: *mut u8, _byte_len: usize) {
        // SAFETY: the trait contract transfers one C-standard-library
        // allocation to this call, so it may be released exactly once.
        unsafe { ffi::free(ptr.cast::<c_void>()) }
    }
}

#[cfg(test)]
mod tests {
    use ffibox::{CVec, CVoidBox, CrustifyStr};

    use super::*;

    #[test]
    fn drops_scalar_and_counted_allocations() {
        // SAFETY: each `malloc` result is null or a fresh allocation compatible
        // with `LibcFree`, and each owner below adopts its result exactly once.
        unsafe {
            let scalar = CVoidBox::<LibcFree>::from_raw(ffi::malloc(1)).expect("malloc failed");
            drop(scalar);

            let buffer = CVec::<u8, LibcFree>::from_raw_parts(ffi::malloc(4).cast(), 4)
                .expect("malloc failed");
            drop(buffer);
        }
    }

    #[test]
    fn drops_terminated_string() {
        const TEXT: &[u8] = b"crustify\0";

        // SAFETY: the checked `malloc` result is a fresh allocation of exactly
        // `TEXT.len()` bytes owned by nobody else, and `TEXT` is a distinct,
        // fully initialised source of that many bytes, so the copy leaves a
        // uniquely owned NUL-terminated `malloc` string — which is what
        // `CrustifyStr` adopts, with `LibcFree` as its matching destructor.
        let string = unsafe {
            let raw = ffi::malloc(TEXT.len() as _).cast::<u8>();
            assert!(!raw.is_null(), "malloc failed");
            core::ptr::copy_nonoverlapping(TEXT.as_ptr(), raw, TEXT.len());
            CrustifyStr::<LibcFree>::from_raw(raw.cast()).expect("malloc failed")
        };

        assert_eq!(string.as_bytes(), b"crustify");
        assert_eq!(string.len(), 8);
        drop(string);
    }
}
