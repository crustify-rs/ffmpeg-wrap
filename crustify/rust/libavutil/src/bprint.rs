//! Wrappers for `libavutil/bprint.c`.

use core::ffi::{CStr, c_char};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CBox, CDropped, CSlice, CSliceMut, CrustifyStr};

use crate::avstring::AVEscapeMode;
use crate::ffi;
use crate::mem::AvFree;

ffibox::define_ctype!(
    /// Wraps: AVBPrint
    ///
    /// ABI-compatible view of FFmpeg's progressive print buffer.
    ///
    /// An initialized value is address-sensitive: `str` may point into the
    /// value's own internal buffer. Consequently this module exposes borrowed
    /// handles but no movable inline owner.
    AVBPrint,
    AVBPrintRef,
    AVBPrintMut,
    ffi::AVBPrint
);

impl AVBPrintRef<'_> {
    /// Field: AVBPrint.len
    ///
    /// Returns the total requested content length, which can exceed the
    /// initialized prefix available from [`str`](Self::str).
    #[must_use]
    pub fn len(&self) -> u32 {
        // SAFETY: the handle addresses a live initialized print-buffer header;
        // raw-place projection copies one integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).len).read() }
    }

    /// Returns whether no content has been requested.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Field: AVBPrint.size
    ///
    /// Returns the current string-buffer capacity, including its terminator.
    #[must_use]
    pub fn size(&self) -> u32 {
        // SAFETY: as `len`, for the adjacent scalar field.
        unsafe { addr_of!((*self.as_ptr()).size).read() }
    }

    /// Field: AVBPrint.reserved_padding
    ///
    /// Views the ABI-reserved bytes without assuming that C initialized them.
    #[must_use]
    pub fn reserved_padding(&self) -> CSlice<'_, MaybeUninit<c_char>> {
        // SAFETY: raw-place projection locates the 1000 inline slots without
        // reading them. `MaybeUninit<c_char>` permits their indeterminate
        // contents, and the view is bounded by this borrow of the handle.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).reserved_padding)
                .cast::<MaybeUninit<c_char>>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 1000)
        }
    }

    /// Field: AVBPrint.reserved_internal_buffer
    ///
    /// Views the one-byte public member without assuming that an uninitialized
    /// header has assigned it a value. The internal allocation extends through
    /// reserved padding in C, but this accessor deliberately describes only
    /// the declared array member.
    #[must_use]
    pub fn reserved_internal_buffer(&self) -> CSlice<'_, MaybeUninit<c_char>> {
        // SAFETY: as `reserved_padding`, for the one inline declared slot.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).reserved_internal_buffer)
                .cast::<MaybeUninit<c_char>>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 1)
        }
    }

    /// Field: AVBPrint.size_max
    ///
    /// Returns the configured maximum allocation size.
    #[must_use]
    pub fn size_max(&self) -> u32 {
        // SAFETY: as `len`, for the adjacent scalar field.
        unsafe { addr_of!((*self.as_ptr()).size_max).read() }
    }

    /// Field: AVBPrint.str
    ///
    /// Returns the initialized content prefix, excluding its trailing NUL.
    /// `None` denotes a finalized buffer whose allocation was released or
    /// transferred. The view is limited to `min(len, size - 1)`, because
    /// `len` continues increasing after truncation.
    #[must_use]
    pub fn str(&self) -> Option<CSlice<'_, u8>> {
        // SAFETY: raw-place projection copies the pointer without forming a
        // reference to the header or pointer field.
        let pointer = unsafe { addr_of!((*self.as_ptr()).str_).read() }.cast::<u8>();
        let pointer = NonNull::new(pointer)?;
        let initialized = self.len().min(self.size().saturating_sub(1)) as usize;
        // SAFETY: the AVBPrint invariant makes the first `min(len, size - 1)`
        // bytes initialized and keeps their storage alive for the header's
        // usable lifetime. This view is bound to the current shared borrow and
        // forms no Rust reference over storage C may later mutate.
        Some(unsafe { CSlice::from_raw_parts(pointer, initialized) })
    }
}

impl AVBPrintMut<'_> {
    /// Returns exclusive access to the initialized string content.
    #[must_use]
    pub fn str_mut(&mut self) -> Option<CSliceMut<'_, u8>> {
        let initialized = {
            let view = self.as_ref();
            view.len().min(view.size().saturating_sub(1)) as usize
        };
        // SAFETY: the exclusive handle supplies write provenance; raw-place
        // projection copies the pointer without forming a reference.
        let pointer = unsafe { addr_of!((*self.as_mut_ptr()).str_).read() }.cast::<u8>();
        let pointer = NonNull::new(pointer)?;
        // SAFETY: the same initialized-prefix invariant as `str` holds, and
        // `&mut self` prevents another Rust handle from being used while the
        // exclusive view lives.
        Some(unsafe { CSliceMut::from_raw_parts(pointer, initialized) })
    }

    /// Returns exclusive access to the ABI-reserved padding bytes.
    #[must_use]
    pub fn reserved_padding_mut(&mut self) -> CSliceMut<'_, MaybeUninit<c_char>> {
        // SAFETY: the exclusive handle supplies write provenance to all 1000
        // inline slots; `MaybeUninit` does not read their prior contents.
        unsafe {
            let pointer =
                addr_of_mut!((*self.as_mut_ptr()).reserved_padding).cast::<MaybeUninit<c_char>>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 1000)
        }
    }

    /// Returns exclusive access to the declared internal-buffer byte.
    #[must_use]
    pub fn reserved_internal_buffer_mut(&mut self) -> CSliceMut<'_, MaybeUninit<c_char>> {
        // SAFETY: as `reserved_padding_mut`, for the one declared inline slot.
        unsafe {
            let pointer = addr_of_mut!((*self.as_mut_ptr()).reserved_internal_buffer)
                .cast::<MaybeUninit<c_char>>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_bindgen() {
        assert_eq!(size_of::<AVBPrint>(), size_of::<ffi::AVBPrint>());
        assert_eq!(align_of::<AVBPrint>(), align_of::<ffi::AVBPrint>());
    }

    #[test]
    fn borrowed_views_cover_initialized_content_and_reserved_storage() {
        let mut raw: ffi::AVBPrint = unsafe {
            // SAFETY: all-zero is a valid starting bit pattern for the raw C
            // layout, and every field required by the handle invariant is set
            // below before a wrapper handle is created.
            core::mem::zeroed()
        };
        let internal = addr_of_mut!(raw.reserved_internal_buffer).cast::<c_char>();
        // SAFETY: `internal` addresses the first byte of the 1001-byte tail;
        // the next byte is the first padding byte, so both writes are in-bounds.
        unsafe {
            internal.write(b'A' as c_char);
            internal.add(1).write(0);
        }
        raw.str_ = internal;
        raw.len = 1;
        raw.size = 1001;
        raw.size_max = 1001;

        // SAFETY: `raw` now satisfies AVBPrint's initialized-header invariant
        // and stays live without mutation while the shared handle is used.
        let view = unsafe { AVBPrintRef::from_ptr(addr_of!(raw).cast_mut()) }.unwrap();
        assert_eq!(view.len(), 1);
        assert_eq!(view.size(), 1001);
        assert_eq!(view.size_max(), 1001);
        assert_eq!(view.str().unwrap().elem(0), Some(b'A'));
        assert_eq!(view.reserved_internal_buffer().len(), 1);
        assert_eq!(view.reserved_padding().len(), 1000);
    }
}

// SAFETY: this strategy is used only for heap headers created by
// `av_bprint_init` below. Finalization releases the optional dynamic string;
// `av_free` then releases the separately allocated header.
unsafe impl CDropped for AVBPrint {
    unsafe fn c_drop(object: NonNull<Self>) {
        let raw = object.as_ptr().cast::<ffi::AVBPrint>();
        // SAFETY: `raw` uniquely owns a live initialized print buffer and its
        // header was allocated by `av_mallocz`.
        unsafe {
            ffi::av_bprint_finalize(raw, core::ptr::null_mut());
            ffi::av_free(raw.cast());
        }
    }
}

/// Address-stable print buffer backed by caller-owned storage.
pub struct BorrowedAVBPrint<'a> {
    header: NonNull<ffi::AVBPrint>,
    _storage: PhantomData<&'a mut [MaybeUninit<u8>]>,
}

impl BorrowedAVBPrint<'_> {
    #[must_use]
    pub fn as_ref(&self) -> AVBPrintRef<'_> {
        // SAFETY: the heap header stays live for `self` and remains initialized.
        unsafe { AVBPrintRef::from_ptr(self.header.as_ptr()) }.expect("stored non-null header")
    }

    #[must_use]
    pub fn as_mut(&mut self) -> AVBPrintMut<'_> {
        // SAFETY: `&mut self` supplies exclusive access to the live header and
        // to the external storage retained by the lifetime marker.
        unsafe { AVBPrintMut::from_ptr(self.header.as_ptr()) }.expect("stored non-null header")
    }
}

impl Drop for BorrowedAVBPrint<'_> {
    fn drop(&mut self) {
        // The fixed-buffer initializer never allocates or adopts the external
        // byte slice. Only the separately allocated header is ours to release.
        // SAFETY: `header` came from `av_mallocz` and has not been freed.
        unsafe { ffi::av_free(self.header.as_ptr().cast()) }
    }
}

/// Wraps: av_bprint_init
#[must_use]
pub fn av_bprint_init(size_init: u32, size_max: u32) -> Option<CBox<AVBPrint>> {
    // SAFETY: allocation requests exactly one raw header; null is handled.
    let raw: *mut ffi::AVBPrint =
        unsafe { ffi::av_mallocz(core::mem::size_of::<ffi::AVBPrint>()).cast() };
    if raw.is_null() {
        return None;
    }
    // SAFETY: C initializes every public field and the self-relative internal
    // pointer while the allocation is already at its permanent address.
    unsafe { ffi::av_bprint_init(raw, size_init, size_max) };
    // SAFETY: the initialized allocation is uniquely transferred to CBox.
    unsafe { CBox::from_raw(raw) }
}

/// Wraps: av_bprint_init_for_buffer
#[must_use]
pub fn av_bprint_init_for_buffer(storage: &mut [MaybeUninit<u8>]) -> Option<BorrowedAVBPrint<'_>> {
    let size = u32::try_from(storage.len()).ok()?;
    // SAFETY: allocation requests exactly one header; null is handled.
    let raw: *mut ffi::AVBPrint =
        unsafe { ffi::av_mallocz(core::mem::size_of::<ffi::AVBPrint>()).cast() };
    let header = NonNull::new(raw)?;
    // SAFETY: a nonempty slice supplies `size` writable bytes; for an empty
    // slice C ignores the pointer and selects count-only internal storage.
    unsafe { ffi::av_bprint_init_for_buffer(raw, storage.as_mut_ptr().cast(), size) };
    Some(BorrowedAVBPrint {
        header,
        _storage: PhantomData,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BPrintLengthOverflow;

/// Wraps: av_bprint_append_data
pub fn av_bprint_append_data(
    buffer: &mut AVBPrintMut<'_>,
    data: &[u8],
) -> Result<(), BPrintLengthOverflow> {
    let size = u32::try_from(data.len()).map_err(|_| BPrintLengthOverflow)?;
    // SAFETY: both borrows stay live for the call and C reads exactly `size` bytes.
    unsafe { ffi::av_bprint_append_data(buffer.as_mut_ptr(), data.as_ptr().cast(), size) };
    Ok(())
}

/// Wraps: av_bprint_chars
pub fn av_bprint_chars(buffer: &mut AVBPrintMut<'_>, byte: u8, count: u32) {
    // SAFETY: the exclusive initialized handle is live for the call.
    unsafe { ffi::av_bprint_chars(buffer.as_mut_ptr(), byte as c_char, count) }
}

/// Wraps: av_bprint_clear
pub fn av_bprint_clear(buffer: &mut AVBPrintMut<'_>) {
    // SAFETY: the exclusive initialized handle is live for the call.
    unsafe { ffi::av_bprint_clear(buffer.as_mut_ptr()) }
}

/// Wraps: av_bprint_escape
pub fn av_bprint_escape(
    buffer: &mut AVBPrintMut<'_>,
    source: &CStr,
    special: Option<&CStr>,
    mode: AVEscapeMode,
    flags: i32,
) {
    // SAFETY: strings and the exclusive buffer remain live; C retains none.
    unsafe {
        ffi::av_bprint_escape(
            buffer.as_mut_ptr(),
            source.as_ptr(),
            special.map_or(core::ptr::null(), CStr::as_ptr),
            mode.as_raw(),
            flags,
        )
    }
}

/// Wraps: av_bprint_get_buffer
pub fn av_bprint_get_buffer<'a>(
    buffer: &'a mut AVBPrintMut<'_>,
    requested: u32,
) -> Option<CSliceMut<'a, MaybeUninit<u8>>> {
    let mut memory = core::ptr::null_mut();
    let mut actual = 0;
    // SAFETY: both output slots are writable and the exclusive buffer handle
    // keeps the returned tail storage alive for the resulting reborrow.
    unsafe {
        ffi::av_bprint_get_buffer(
            buffer.as_mut_ptr(),
            requested,
            &raw mut memory,
            &raw mut actual,
        )
    };
    let memory = NonNull::new(memory.cast::<MaybeUninit<u8>>())?;
    // SAFETY: C reports exactly `actual` writable bytes at the returned pointer.
    Some(unsafe { CSliceMut::from_raw_parts(memory, actual as usize) })
}

/// Wraps: av_bprint_strftime
pub fn av_bprint_strftime(
    buffer: &mut AVBPrintMut<'_>,
    format: &CStr,
    time: libc::struct_tm::TmRef<'_>,
) {
    // SAFETY: all handles remain live and C only reads `format` and `time`.
    unsafe {
        ffi::av_bprint_strftime(
            buffer.as_mut_ptr(),
            format.as_ptr(),
            time.as_ptr().cast::<ffi::tm>(),
        )
    }
}

/// Wraps: av_bprintf
///
/// Safe specialization of the variadic API that formats one string through
/// the fixed C format `"%s"`.
pub fn av_bprintf(buffer: &mut AVBPrintMut<'_>, text: &CStr) {
    // SAFETY: the shim fixes the variadic argument types and retains nothing.
    unsafe { ffi::crustify_av_bprintf_string(buffer.as_mut_ptr(), text.as_ptr()) }
}

/// Wraps: av_vbprintf
///
/// Safe specialization of the `va_list` API that formats one string.
pub fn av_vbprintf(buffer: &mut AVBPrintMut<'_>, text: &CStr) {
    // SAFETY: the shim constructs the matching `va_list` and retains nothing.
    unsafe { ffi::crustify_av_vbprintf_string(buffer.as_mut_ptr(), text.as_ptr()) }
}

/// Wraps: av_bprint_finalize
pub fn av_bprint_finalize(buffer: CBox<AVBPrint>) -> Result<Option<CrustifyStr<AvFree>>, i32> {
    let raw = CBox::into_raw(buffer);
    let mut string = core::ptr::null_mut();
    // SAFETY: ownership of the live buffer was surrendered above; C transfers
    // its string through the output slot and leaves the header disposable.
    let status = unsafe { ffi::av_bprint_finalize(raw, &raw mut string) };
    // SAFETY: the header itself was allocated by `av_mallocz` and finalization
    // has released or transferred every allocation it referenced.
    unsafe { ffi::av_free(raw.cast()) };
    // SAFETY: a non-null output is a unique av_malloc-family C string.
    let string = unsafe { CrustifyStr::<AvFree>::from_raw(string) };
    if status < 0 { Err(status) } else { Ok(string) }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    #[test]
    fn owned_buffer_appends_and_transfers_its_string() {
        let mut buffer = av_bprint_init(8, u32::MAX).expect("header allocation");
        av_bprint_append_data(&mut buffer.as_mut(), b"ab").unwrap();
        av_bprint_chars(&mut buffer.as_mut(), b'c', 2);
        av_bprintf(&mut buffer.as_mut(), c"!");
        let string = av_bprint_finalize(buffer)
            .unwrap()
            .expect("string allocation");
        assert_eq!(string.as_c_str(), c"abcc!");
    }

    #[test]
    fn fixed_buffer_borrows_caller_storage() {
        let mut storage = [MaybeUninit::uninit(); 8];
        let mut buffer = av_bprint_init_for_buffer(&mut storage).expect("header allocation");
        av_bprint_append_data(&mut buffer.as_mut(), b"hello").unwrap();
        assert_eq!(buffer.as_ref().str().unwrap().elem(4), Some(b'o'));
    }
}
