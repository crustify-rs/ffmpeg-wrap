//! Ownership strategies for memory allocated by libavutil.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CDropped, CLenCloned, CLenDropped};

use crate::ffi;

/// Wraps: av_free
///
/// Releases scalar allocations, buffers, and strings produced by the
/// `av_malloc` family. For counted buffers it also enables bytewise cloning
/// through [`av_memdup`](ffi::av_memdup).
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
    use ffibox::{CVec, CVoidBox};

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
}
