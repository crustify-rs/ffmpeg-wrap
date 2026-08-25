//! Wrappers for `libavutil/bprint.c`.

use core::ffi::c_char;
use core::mem::MaybeUninit;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CSlice, CSliceMut};

use crate::ffi;

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
