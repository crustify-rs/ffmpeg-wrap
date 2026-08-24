//! Wrappers for libavutil file mapping.
//!
//! [`av_file_map`] is `unsafe` and [`AvFileUnmap`] is sound on its own; the
//! two halves carry different obligations and only the first one needs the
//! caller. See each item for why.

use core::ffi::{CStr, c_void};

use ffibox::{CLenDropped, CVec};

use crate::{ffi, log::LogContextRef};

/// Wraps: av_file_unmap
///
/// Release strategy for a byte mapping produced by [`av_file_map`], and the
/// drop half of the [`CVec<u8, AvFileUnmap>`](ffibox::CVec) that wrapper hands
/// back.
///
/// # The obligation this impl adds
///
/// [`CLenDropped::c_drop_len`] asks its caller for a valid allocation of *at
/// least* `byte_len` bytes from the allocator the strategy targets, and `Self`
/// is a zero-sized name for a release class rather than a description of the
/// bytes. Both halves of that are weaker than what this release needs.
/// `av_file_unmap` resolves to `munmap`, which releases exactly the range it
/// is handed: a `byte_len` below the mapped size leaves the tail mapped, and
/// one above it tears down whatever the kernel placed after the mapping. So
/// this impl narrows the obligation to an exact pair — `ptr` must be the base
/// address `av_file_map` wrote and `byte_len` the size it reported beside it.
///
/// No safe path can produce a mismatched pair, and the ffibox bounds are what
/// rule that out rather than a convention. [`ffibox::CVec`] is the only
/// owner that reaches [`CLenDropped`], its sole constructor
/// ([`from_raw_parts`](ffibox::CVec::from_raw_parts)) is `unsafe`, and no safe
/// method on it changes the element count that `byte_len` is derived from —
/// cloning would, but that runs through [`CLenCloned`](ffibox::CLenCloned),
/// which this strategy deliberately does not implement, since a `memdup` of a
/// mapping is a heap allocation `munmap` must never see. Within this crate the
/// one owner is built by [`av_file_map`] from the two out-parameters of a
/// single successful call.
pub struct AvFileUnmap;

// SAFETY: given that obligation, the release is well formed: `av_file_unmap`
// unmaps exactly the range described by the pair, touches nothing else, and is
// reached once per owner because `CVec::Drop` consumes it.
unsafe impl CLenDropped for AvFileUnmap {
    unsafe fn c_drop_len(ptr: *mut u8, byte_len: usize) {
        // SAFETY: the caller owes the base address and exact size of one live
        // `av_file_map` mapping, which is what `av_file_unmap` consumes. It
        // also tolerates the zero length this crate never constructs — an
        // empty file maps to no pointer at all — so the `munmap` EINVAL that a
        // zero-length range would raise is unreachable from either side.
        unsafe { ffi::av_file_unmap(ptr, byte_len) }
    }
}

/// Wraps: av_file_map
///
/// Maps a file. An empty file is represented by `Ok(None)`, matching C's
/// null pointer and zero size.
///
/// # Safety
///
/// For as long as the returned mapping lives, no process may change the length
/// or the contents of the file named by `filename`.
///
/// This is the one obligation in this module that no bound in the Rust type
/// system can carry, which is why the constructor is `unsafe` while every
/// other wrapper here is not. C hands back a `uint8_t *` and a size and
/// promises nothing about them; the owner returned here is a
/// [`CVec<u8, AvFileUnmap>`](ffibox::CVec), and
/// [`CVec::as_slice`](ffibox::CVec::as_slice) turns that into a `&[u8]`, which
/// promises two things `mmap` cannot keep:
///
/// * **Dereferenceable for its whole extent.** `libavutil/file.c` maps
///   `st_size` bytes of the file. Shrinking the file afterwards leaves the
///   tail of the mapping with no page behind it, and reading it raises
///   `SIGBUS` — a fault inside the *caller's* frame, on a reference the
///   caller never wrote `unsafe` to obtain.
/// * **Immutable for the borrow.** The mapping is `MAP_PRIVATE`, so it is
///   copy-on-write, not a snapshot: a page that has only ever been read still
///   reflects later writes to the file. Rewriting the file changes the bytes
///   a live `&[u8]` yields, with no `&mut` anywhere in sight.
///
/// Neither requires an adversary and neither is specific to this crate; it is
/// the same reason `memmap2::Mmap::map` is an `unsafe fn`. A caller discharges
/// the obligation by mapping a file only it can reach — one it created with
/// `O_EXCL` in a private directory, or a read-only file on a filesystem
/// nothing is writing.
///
/// Releasing the mapping is *not* part of this obligation; see
/// [`AvFileUnmap`], which the ffibox bounds keep sound on their own.
///
/// A caller that cannot make that guarantee should read the file instead of
/// mapping it. `std::fs::read` copies, so a concurrent truncation costs it
/// bytes rather than soundness.
///
/// ```compile_fail,E0133
/// // Calling it without an `unsafe` block does not compile: the gate below
/// // is what this obligation is carried by.
/// let _ = libavutil::file::av_file_map(c"/etc/hostname", 0, None);
/// ```
pub unsafe fn av_file_map(
    filename: &CStr,
    log_offset: i32,
    log_context: Option<LogContextRef<'_>>,
) -> Result<Option<CVec<u8, AvFileUnmap>>, i32> {
    let mut pointer = core::ptr::null_mut();
    let mut size = 0_usize;
    // SAFETY: `filename` is a live NUL-terminated string; both out-slots are
    // distinct and writable; the optional context handle proves any non-null
    // logging object remains live for the call.
    let status = unsafe {
        ffi::av_file_map(
            filename.as_ptr(),
            &raw mut pointer,
            &raw mut size,
            log_offset,
            log_context.map_or(core::ptr::null_mut::<c_void>(), LogContextRef::as_ptr),
        )
    };
    if status < 0 {
        return Err(status);
    }
    if pointer.is_null() {
        debug_assert_eq!(size, 0);
        return Ok(None);
    }
    // SAFETY: success returned a non-null uniquely owned mapping containing
    // exactly `size` initialized bytes, paired with `av_file_unmap`.
    Ok(unsafe { CVec::from_raw_parts(pointer, size) })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This source file, as an absolute NUL-terminated path fixed at compile
    /// time, so the test does not depend on the working directory a test
    /// binary happens to be launched from.
    const THIS_FILE: &CStr = match CStr::from_bytes_with_nul(
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/file.rs\0").as_bytes(),
    ) {
        Ok(path) => path,
        Err(_) => panic!("the concatenated path ends in exactly one NUL"),
    };

    /// The mapping obligation these tests discharge.
    ///
    /// Every call below maps this crate's own source file, read-only, from a
    /// test binary that does not write it. Nothing in the process changes its
    /// length or its contents while a mapping is live, which is exactly what
    /// [`av_file_map`]'s `# Safety` section asks for. It is an assumption
    /// about the test environment rather than something the type system
    /// carries, which is the whole reason the function is `unsafe`.
    ///
    /// # Safety
    ///
    /// The caller must be one of the tests in this module, running against an
    /// unmodified checkout.
    unsafe fn map_this_file() -> CVec<u8, AvFileUnmap> {
        // SAFETY: forwarded from this helper's own contract above.
        unsafe { av_file_map(THIS_FILE, 0, None) }
            .expect("mapping this source file")
            .expect("this source file is not empty")
    }

    #[test]
    fn maps_an_empty_file_and_reports_missing_files() {
        // SAFETY: `/dev/null` has zero length, so C returns before it maps
        // anything and there is no mapping for a writer to invalidate; the
        // missing path never reaches `mmap` either.
        unsafe {
            assert!(matches!(av_file_map(c"/dev/null", 0, None), Ok(None)));
            assert!(av_file_map(c"/definitely/not/a/crustify/file", 0, None).is_err());
        }
    }

    #[test]
    fn maps_a_real_file_and_unmaps_it_on_drop() {
        // The path the empty and missing cases never reach: a mapping that is
        // actually established, read through the owner, and torn down by
        // `AvFileUnmap` with the size C reported. Under the sanitiser run an
        // unmap of the wrong extent, or none at all, shows up here.
        //
        // SAFETY: as `map_this_file`.
        let mapping = unsafe { map_this_file() };

        assert_eq!(mapping.byte_len(), mapping.count());
        assert!(
            mapping
                .as_slice()
                .windows(b"av_file_map".len())
                .any(|window| window == b"av_file_map"),
            "the mapped bytes are this file's contents"
        );

        drop(mapping);
    }

    #[test]
    fn mapped_bytes_are_privately_writable() {
        // `av_file_map` asks for `PROT_READ|PROT_WRITE` over a `MAP_PRIVATE`
        // mapping, so the exclusive owner really may write and the file on
        // disk does not change. Handing this back as a read-only view would
        // understate what C established.
        //
        // The same `MAP_PRIVATE` is why the mapping is not a snapshot, and so
        // why the constructor is `unsafe`: a page this test has not yet
        // written still tracks the file. Writing through the owner is what
        // detaches it, and that only helps the pages actually written.
        //
        // SAFETY: as `map_this_file`.
        let mut mapping = unsafe { map_this_file() };

        let first = mapping.as_slice()[0];
        mapping.as_mut_slice()[0] = first ^ 0xff;
        assert_eq!(mapping.as_slice()[0], first ^ 0xff);

        // SAFETY: as `map_this_file`.
        let again = unsafe { map_this_file() };
        assert_eq!(again.as_slice()[0], first);
    }

    #[test]
    fn strategy_is_zero_sized() {
        assert_eq!(core::mem::size_of::<AvFileUnmap>(), 0);
        assert_eq!(
            core::mem::size_of::<core::ptr::NonNull<AvFileUnmap>>(),
            core::mem::size_of::<usize>()
        );
    }
}
