//! Wrappers for libavutil file mapping.

use core::ffi::{CStr, c_void};

use ffibox::{CLenDropped, CVec};

use crate::{ffi, log::LogContextRef};

/// Wraps: av_file_unmap
///
/// Drop strategy for a byte mapping returned by [`av_file_map`]. The byte
/// length carried by `CVec` is the exact length the C unmapper requires.
pub struct AvFileUnmap;

// SAFETY: this strategy is used only for mappings returned by `av_file_map`,
// and `byte_len` preserves the exact size paired with that pointer.
unsafe impl CLenDropped for AvFileUnmap {
    unsafe fn c_drop_len(ptr: *mut u8, byte_len: usize) {
        // SAFETY: the trait contract and this strategy's construction site
        // guarantee an `av_file_map` pointer with its matching byte length.
        unsafe { ffi::av_file_unmap(ptr, byte_len) }
    }
}

/// Wraps: av_file_map
///
/// Maps a file. An empty file is represented by `Ok(None)`, matching C's
/// null pointer and zero size.
pub fn av_file_map(
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

    #[test]
    fn maps_an_empty_file_and_reports_missing_files() {
        assert!(matches!(av_file_map(c"/dev/null", 0, None), Ok(None)));
        assert!(av_file_map(c"/definitely/not/a/crustify/file", 0, None).is_err());
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
