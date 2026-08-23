//! Wrappers for libavutil reference-counted buffers.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CCloned, CDropped, CSlice, CSliceMut, define_ctype};

use crate::ffi;

define_ctype!(
    /// Wraps: AVBuffer
    ///
    /// The public C API deliberately keeps this object opaque: callers hold it
    /// indirectly through the distinct C `AVBufferRef` structure. Consequently
    /// this wrapper exposes pointer identity and lifetime-carrying borrowed
    /// handles, but no fields or independent owner. In particular, an
    /// `AVBuffer` may be embedded in a pool entry, so freeing its storage from a
    /// handle would be incorrect; the `AVBufferRef` lifecycle performs the
    /// refcounted release instead.
    ///
    /// [`AVBufferRef`] is the shared borrowed handle for this object. The safe
    /// wrapper for C's same-spelled `AVBufferRef` structure therefore uses a
    /// distinct descriptive Rust name when that dependent type is translated.
    AVBuffer,
    AVBufferRef,
    AVBufferMut,
    ffi::AVBuffer
);

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use ffibox::CBox;

    use super::*;

    #[test]
    fn avbuffer_preserves_the_c_layout() {
        assert_eq!(size_of::<AVBuffer>(), size_of::<ffi::AVBuffer>());
        assert_eq!(align_of::<AVBuffer>(), align_of::<ffi::AVBuffer>());
        assert_eq!(
            size_of::<AVBufferRef<'_>>(),
            size_of::<*const ffi::AVBuffer>()
        );
        assert_eq!(
            size_of::<AVBufferMut<'_>>(),
            size_of::<*mut ffi::AVBuffer>()
        );
    }

    #[test]
    fn reference_fields_use_copying_and_handle_views() {
        let mut bytes = [10_u8, 20, 30, 40];
        let opaque = NonNull::<ffi::AVBuffer>::dangling().as_ptr();
        let mut raw = ffi::AVBufferRef {
            buffer: opaque,
            data: bytes.as_mut_ptr(),
            size: bytes.len(),
        };

        // SAFETY: `raw` and its byte array are live and initialized for the
        // duration of the borrowed handle; the opaque buffer pointer is only
        // compared and never dereferenced.
        let reference = unsafe { AVBufferReferenceRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(reference.size(), 4);
        let mut copied = [0_u8; 4];
        assert!(reference.data().unwrap().copy_to_slice(&mut copied));
        assert_eq!(copied, bytes);
        assert_eq!(reference.buffer().as_ptr(), opaque.cast_const());

        {
            // SAFETY: `raw` is still live and no shared handle is used during
            // this exclusive borrow.
            let mut reference =
                unsafe { AVBufferReferenceMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
            assert!(reference.advance(1));
            assert!(reference.truncate(2));
            assert!(!reference.advance(3));
            assert_eq!(reference.as_ref().size(), 2);
        }
        assert_eq!(raw.data, bytes[1..].as_mut_ptr());
        assert_eq!(raw.size, 2);
    }

    #[test]
    fn owned_reference_clones_and_gates_mutable_data() {
        // SAFETY: the returned fully constructed C reference is immediately
        // adopted by the matching CBox lifecycle.
        let mut owner = unsafe {
            CBox::<AVBufferReference>::from_raw(ffi::av_buffer_allocz(4))
                .expect("four-byte AVBuffer allocation")
        };
        assert_eq!(owner.as_ref().data().unwrap().elems().sum::<u8>(), 0);

        {
            let mut reference = owner.as_mut();
            let mut data = reference.data_mut().expect("sole reference is writable");
            assert!(data.copy_from_slice(&[1, 2, 3, 4]));
        }

        let clone = owner.try_clone().expect("reference header clone");
        assert!(owner.as_mut().data_mut().is_none());
        assert_eq!(clone.as_ref().data().unwrap().elem(2), Some(3));
        drop(clone);
        assert!(owner.as_mut().data_mut().is_some());
    }
}

define_ctype!(
    /// Wraps: AVBufferRef
    ///
    /// Owns one independently releasable reference to an underlying
    /// [`AVBuffer`]. The descriptive Rust name avoids colliding with
    /// [`AVBufferRef`], the borrowed handle for that opaque underlying object.
    /// Owned values use `CBox<AVBufferReference>`; cloning allocates a new C
    /// reference header and increments the underlying buffer's reference count.
    AVBufferReference,
    AVBufferReferenceRef,
    AVBufferReferenceMut,
    ffi::AVBufferRef
);

// SAFETY: `av_buffer_unref` consumes one live AVBufferRef header, releases its
// underlying count, frees the header, and nulls the pointer slot. A
// `CBox<AVBufferReference>` owns exactly that independently releasable unit.
unsafe impl CDropped for AVBufferReference {
    unsafe fn c_drop(obj: NonNull<Self>) {
        let mut reference = obj.as_ptr().cast::<ffi::AVBufferRef>();
        // SAFETY: the trait contract transfers the one live owned reference to
        // its matching public destructor; the local slot is writable.
        unsafe { ffi::av_buffer_unref(addr_of_mut!(reference)) }
    }
}

// SAFETY: `av_buffer_ref` leaves its source unchanged and returns either NULL
// or a freshly allocated AVBufferRef header carrying one independently
// releasable reference to the same underlying buffer.
unsafe impl CCloned for AVBufferReference {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: the trait contract supplies a live source reference and the
        // returned header, when non-null, is owned by the caller.
        let cloned = unsafe { ffi::av_buffer_ref(obj.as_ptr().cast::<ffi::AVBufferRef>()) };
        NonNull::new(cloned.cast::<Self>())
    }
}

impl<'a> AVBufferReferenceRef<'a> {
    /// Wraps: AVBufferRef.size
    ///
    /// Returns the length of this reference's current data window in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        // SAFETY: the handle points to a live initialized header. Copying the
        // scalar through a raw projection forms no reference to C storage.
        unsafe { addr_of!((*self.as_ptr()).size).read() }
    }

    /// Wraps: AVBufferRef.data
    ///
    /// Views the byte window without forming a Rust slice over memory that C
    /// may mutate. `None` represents a null data pointer, which is valid for an
    /// empty user-supplied buffer.
    #[must_use]
    pub fn data(&self) -> Option<CSlice<'a, u8>> {
        // SAFETY: both fields are copied through raw projections from a live
        // header; the underlying buffer keeps `size` bytes alive for `'a`.
        let (data, size) = unsafe {
            (
                addr_of!((*self.as_ptr()).data).read(),
                addr_of!((*self.as_ptr()).size).read(),
            )
        };
        NonNull::new(data).map(|data| {
            // SAFETY: a non-null AVBufferRef data field addresses its `size`
            // byte window, kept alive by the header's underlying count.
            unsafe { CSlice::from_raw_parts(data, size) }
        })
    }

    /// Wraps: AVBufferRef.buffer
    ///
    /// Borrows the opaque underlying buffer that keeps the data window alive.
    #[must_use]
    pub fn buffer(&self) -> AVBufferRef<'a> {
        // SAFETY: every initialized AVBufferRef owns a non-null underlying
        // AVBuffer count. The returned handle is tied to the header borrow.
        unsafe {
            AVBufferRef::from_ptr(addr_of!((*self.as_ptr()).buffer).read())
                .expect("a live AVBufferRef has a non-null AVBuffer")
        }
    }
}

impl AVBufferReferenceMut<'_> {
    /// Shortens the current data window without changing its start.
    pub fn truncate(&mut self, new_size: usize) -> bool {
        let size = self.as_ref().size();
        if new_size > size {
            return false;
        }
        // SAFETY: the exclusive handle permits updating the scalar header
        // field, and shrinking keeps the data window inside its prior bounds.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).size).write(new_size) }
        true
    }

    /// Removes `bytes` from the start of the current data window.
    pub fn advance(&mut self, bytes: usize) -> bool {
        let size = self.as_ref().size();
        if bytes > size {
            return false;
        }
        // SAFETY: the two fields are read from the live header through raw
        // projections. A positive offset implies a non-null data pointer.
        let data = unsafe { addr_of!((*self.as_ref().as_ptr()).data).read() };
        if bytes != 0 && data.is_null() {
            return false;
        }
        let advanced = if bytes == 0 {
            data
        } else {
            // SAFETY: `bytes <= size`, so this remains within or one past the
            // current initialized byte window.
            unsafe { data.add(bytes) }
        };
        // SAFETY: the exclusive handle permits updating both header fields;
        // the new window is a suffix of the old one.
        unsafe {
            let header = self.as_mut_ptr();
            addr_of_mut!((*header).data).write(advanced);
            addr_of_mut!((*header).size).write(size - bytes);
        }
        true
    }

    /// Exclusively views the data when this is the only reference and the
    /// underlying buffer is not marked read-only.
    #[must_use]
    pub fn data_mut(&mut self) -> Option<CSliceMut<'_, u8>> {
        // SAFETY: the exclusive header handle is live. The C predicate checks
        // the underlying count and read-only flag and does not retain `buf`.
        if unsafe { ffi::av_buffer_is_writable(self.as_mut_ptr()) } == 0 {
            return None;
        }
        // SAFETY: fields are copied from the live exclusive header. A positive
        // writability result licenses mutation of this `size`-byte window.
        let (data, size) = unsafe {
            let header = self.as_mut_ptr();
            (
                addr_of!((*header).data).read(),
                addr_of!((*header).size).read(),
            )
        };
        NonNull::new(data).map(|data| {
            // SAFETY: the writability check established exclusive access to
            // the initialized window, and the view is bound to `&mut self`.
            unsafe { CSliceMut::from_raw_parts(data, size) }
        })
    }
}
