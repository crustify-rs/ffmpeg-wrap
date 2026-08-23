//! Ownership strategy for POSIX memory mappings.

use ffibox::CLenDropped;

use crate::ffi;

/// Wraps: munmap
///
/// Releases a complete mapping established by `mmap`. `munmap` takes the extent
/// as a separate argument, so an owner must carry the byte length: pair this
/// strategy with [`CVec`](ffibox::CVec), never with a length-free
/// [`CVoidBox`](ffibox::CVoidBox).
///
/// The release is page-granular — `munmap` unmaps every page the range touches
/// — so the owner must span one whole mapping and never a sub-range of one.
/// A zero-byte range is not a mapping at all (`mmap` rejects a zero length), so
/// this strategy treats it as a no-op instead of issuing the `EINVAL` call that
/// a literal forward would make from a destructor. That mirrors
/// `av_file_unmap`, which returns early unless both the pointer and the size
/// are non-zero.
pub struct Munmap;

// SAFETY: `c_drop_len` forwards the exact mapping address and byte length
// required by `munmap`; the trait contract excludes partial or foreign ranges.
unsafe impl CLenDropped for Munmap {
    unsafe fn c_drop_len(ptr: *mut u8, byte_len: usize) {
        if byte_len == 0 {
            // No mapping is zero bytes wide, so there is nothing to release
            // and `munmap` would only fail with `EINVAL`.
            return;
        }
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

    /// Map one anonymous page the way `av_file_map` maps a file — writable and
    /// `MAP_PRIVATE`, so no other writer aliases it and slice views are sound.
    fn map_private_page(len: usize) -> *mut u8 {
        // SAFETY: this requests a fresh anonymous private mapping; a null
        // address lets the kernel choose one, and `-1`/`0` are the ignored
        // file arguments an anonymous mapping takes.
        let mapping = unsafe {
            ffi::mmap(
                ptr::null_mut(),
                len,
                (ffi::PROT_READ | ffi::PROT_WRITE) as _,
                (ffi::MAP_PRIVATE | ffi::MAP_ANONYMOUS) as _,
                -1,
                0,
            )
        };
        assert_ne!(mapping as isize, -1, "mmap failed");
        mapping.cast()
    }

    #[test]
    fn drops_complete_mapping() {
        const LEN: usize = 4096;

        // SAFETY: `map_private_page` returns a successful, uniquely owned
        // LEN-byte mapping, which this `CVec` adopts exactly once with the
        // exact extent `Munmap` hands back to `munmap`.
        let mut owner = unsafe { CVec::<u8, Munmap>::from_raw_parts(map_private_page(LEN), LEN) }
            .expect("mmap unexpectedly returned null");
        assert_eq!(owner.byte_len(), LEN);

        // The recorded contract is a writable private byte range, so exercise
        // both directions before releasing it.
        owner.as_mut_slice()[..4].copy_from_slice(&[1, 2, 3, 4]);
        assert_eq!(&owner.as_slice()[..4], &[1, 2, 3, 4]);

        drop(owner);
    }

    #[test]
    fn empty_range_releases_nothing() {
        // A `CVec` drops unconditionally, so a zero-length one would reach
        // `munmap` with a length POSIX rejects. The strategy must short-circuit
        // before the call rather than fail inside a destructor.
        //
        // SAFETY: `byte_len` is zero, so `c_drop_len` returns without reading
        // or unmapping anything and never dereferences the dangling pointer.
        unsafe { Munmap::c_drop_len(ptr::NonNull::<u8>::dangling().as_ptr(), 0) };
    }
}
