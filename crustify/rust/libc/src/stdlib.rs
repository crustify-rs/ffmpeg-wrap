//! Ownership strategies for the C standard allocator.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CDropped, CLenDropped};

use crate::ffi;

/// Wraps: free
///
/// Releases scalar allocations, buffers, and strings produced by the C
/// standard allocation family.
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
    use ffibox::{CVec, CVoidBox};

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
}
