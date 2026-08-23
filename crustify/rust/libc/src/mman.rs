//! Ownership strategy for POSIX memory mappings.

use ffibox::CLenDropped;

use crate::ffi;

/// Wraps: munmap
///
/// Releases a complete byte range returned by `mmap`.
pub struct Munmap;

// SAFETY: `c_drop_len` forwards the exact mapping address and byte length
// required by `munmap`; the trait contract excludes partial or foreign ranges.
unsafe impl CLenDropped for Munmap {
    unsafe fn c_drop_len(ptr: *mut u8, byte_len: usize) {
        // SAFETY: the trait contract requires `ptr..ptr + byte_len` to be one
        // live mapping whose ownership is transferred to this call.
        let result = unsafe { ffi::munmap(ptr.cast(), byte_len) };
        debug_assert_eq!(result, 0, "munmap rejected a strategy-owned mapping");
    }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use ffibox::CVec;

    use super::*;

    #[test]
    fn drops_complete_mapping() {
        const LEN: usize = 4096;

        // SAFETY: this requests a fresh anonymous private mapping. On success,
        // `CVec` adopts the complete mapping with the exact length for Munmap.
        let mapping = unsafe {
            ffi::mmap(
                ptr::null_mut(),
                LEN,
                ffi::PROT_READ as _,
                (ffi::MAP_PRIVATE | ffi::MAP_ANONYMOUS) as _,
                -1,
                0,
            )
        };
        assert_ne!(mapping as isize, -1, "mmap failed");
        // SAFETY: `mapping` is a successful, uniquely owned LEN-byte mapping.
        let owner = unsafe { CVec::<u8, Munmap>::from_raw_parts(mapping.cast(), LEN) }
            .expect("mmap unexpectedly returned null");
        drop(owner);
    }
}
