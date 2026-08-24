//! Wrappers for libavutil reference-counted buffers.

use core::mem::MaybeUninit;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CBox, CCloned, CDropped, CSlice, CSliceMut, define_ctype};

use crate::ffi;

define_ctype!(
    /// Wraps: AVBuffer
    ///
    /// The public C API deliberately keeps this object opaque: callers hold it
    /// indirectly through the distinct C `AVBufferRef` structure. Consequently
    /// this wrapper exposes pointer identity and lifetime-carrying borrowed
    /// handles, but no fields or independent owner. In particular, an
    /// `AVBuffer` may be embedded in a pool entry — `BufferPoolEntry.buffer` in
    /// `libavutil/buffer_internal.h` holds one by value — so freeing its
    /// storage from a handle would be incorrect; the [`AVBufferReference`]
    /// lifecycle performs the refcounted release instead.
    ///
    /// Publishing no fields is this wrapper's choice, not a consequence of the
    /// bindings: `libavutil-sys` includes `buffer_internal.h`, so
    /// `ffi::AVBuffer` is a complete struct and the layout newtype has its
    /// full size. `zeroed()` therefore hands back a real all-zero `AVBuffer`,
    /// which is a valid C value but not a usable one — its `refcount` is 0 and
    /// its `free` callback null — so it is only ever a stand-in for a header's
    /// `buffer` slot in a test, never something to hand to libavutil.
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
    fn avbuffer_handles_stay_one_pointer_wide() {
        // Comparing the layout newtype against the type it is
        // `repr(transparent)` over cannot fail, so the claim worth pinning is
        // the handles': one pointer, with the null niche, so an
        // `Option<AVBufferRef>` still crosses the seam as a `*const AVBuffer`.
        assert_eq!(
            size_of::<AVBufferRef<'_>>(),
            size_of::<*const ffi::AVBuffer>()
        );
        assert_eq!(
            align_of::<AVBufferRef<'_>>(),
            align_of::<*const ffi::AVBuffer>()
        );
        assert_eq!(
            size_of::<AVBufferMut<'_>>(),
            size_of::<*mut ffi::AVBuffer>()
        );
        assert_eq!(
            size_of::<Option<AVBufferRef<'_>>>(),
            size_of::<*const ffi::AVBuffer>()
        );
    }

    #[test]
    fn avbuffer_identity_survives_a_window_move() {
        let first = av_buffer_allocz(4).expect("four-byte allocation");
        let second = av_buffer_allocz(4).expect("four-byte allocation");
        let mut shared = av_buffer_ref(first.as_ref()).expect("second reference");

        // The one property libavutil publishes for AVBuffer: two references
        // describe the same data buffer iff their `buffer` pointers are equal.
        assert_eq!(
            first.as_ref().buffer().as_ptr(),
            shared.as_ref().buffer().as_ptr()
        );
        assert_ne!(
            first.as_ref().buffer().as_ptr(),
            second.as_ref().buffer().as_ptr()
        );
        // Distinct headers over one buffer, so the count is shared.
        assert_ne!(first.as_ptr(), shared.as_ptr());
        assert_eq!(av_buffer_get_ref_count(first.as_ref()), 2);

        // Moving one window makes `data` differ while `buffer` must not. This
        // is what pins the projection to the header's first field rather than
        // its second: `av_buffer_ref` copies `data` too, so without the move
        // reading the wrong field would still compare equal.
        assert!(shared.as_mut().advance(1));
        assert_ne!(
            first.as_ref().data().unwrap().as_elem_ptr(),
            shared.as_ref().data().unwrap().as_elem_ptr()
        );
        assert_eq!(
            first.as_ref().buffer().as_ptr(),
            shared.as_ref().buffer().as_ptr()
        );
        assert_eq!(shared.as_ref().size(), 3);
    }

    #[test]
    fn avbuffer_handles_carry_pointer_identity_only() {
        let mut backing = AVBuffer::zeroed();
        let raw = addr_of_mut!(backing).cast::<ffi::AVBuffer>();

        {
            // SAFETY: `backing` is a live, initialized all-zero `ffi::AVBuffer`
            // that outlives this handle, which is the only one in use here.
            let mut exclusive = unsafe { AVBufferMut::from_ptr(raw) }.expect("non-null AVBuffer");
            assert_eq!(exclusive.as_mut_ptr(), raw);
            assert_eq!(exclusive.as_ref().as_ptr(), raw.cast_const());
        }

        // SAFETY: as above; the shared handle borrows the same live object.
        let shared = unsafe { AVBufferRef::from_ptr(raw) }.expect("non-null AVBuffer");
        assert_eq!(shared.as_ptr(), raw.cast_const());
        // The wrapper publishes no fields, so identity is the whole surface.
        assert_eq!(shared.as_void_ptr(), raw.cast_const().cast());

        // SAFETY: a null slot is the documented `None` case, not a violation.
        assert!(unsafe { AVBufferRef::from_ptr(core::ptr::null_mut()) }.is_none());
    }

    #[test]
    fn reference_fields_use_copying_and_handle_views() {
        // Declared first so it outlives every handle derived from `raw`.
        // `AVBufferRef::from_ptr` requires a live, initialized `ffi::AVBuffer`
        // behind the header's `buffer` slot, and an all-zero C struct is one;
        // "the wrapper only ever compares this pointer" would not discharge
        // that obligation, so a dangling address is not usable here.
        let mut backing = AVBuffer::zeroed();
        let mut bytes = [10_u8, 20, 30, 40];
        let buffer = addr_of_mut!(backing).cast::<ffi::AVBuffer>();
        let mut raw = ffi::AVBufferRef {
            buffer,
            data: bytes.as_mut_ptr(),
            size: bytes.len(),
        };

        // SAFETY: `raw`, its byte array and its backing AVBuffer are live and
        // initialized for the duration of the borrowed handle.
        let reference = unsafe { AVBufferReferenceRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(reference.size(), 4);
        let mut copied = [0_u8; 4];
        // SAFETY: the window is `bytes`, a fully initialized Rust array.
        let initialized = unsafe { reference.data_assume_init() }.unwrap();
        assert!(initialized.copy_to_slice(&mut copied));
        assert_eq!(copied, bytes);
        assert_eq!(reference.buffer().as_ptr(), buffer.cast_const());

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
        // SAFETY: `av_buffer_allocz` memsets the whole window, so every byte
        // of it has been written before this view is taken.
        let zeroed = unsafe { owner.as_ref().data_assume_init() }.unwrap();
        assert_eq!(zeroed.elems().sum::<u8>(), 0);

        assert!(owner.as_mut().write_all(&[1, 2, 3, 4]));
        // A partial write is refused rather than leaving the window ragged.
        assert!(!owner.as_mut().write_all(&[9, 9]));

        let clone = owner.try_clone().expect("reference header clone");
        assert!(owner.as_mut().data_mut().is_none());
        assert!(!owner.as_mut().write_all(&[5, 6, 7, 8]));
        // SAFETY: `write_all` initialized all four bytes above and the shared
        // clone views the same window.
        let cloned_window = unsafe { clone.as_ref().data_assume_init() }.unwrap();
        assert_eq!(cloned_window.elem(2), Some(3));
        drop(clone);
        assert!(owner.as_mut().data_mut().is_some());

        // Read-modify-write needs the initialized exclusive view, and it is
        // gated on the same sole-reference check as `data_mut`.
        {
            let mut reference = owner.as_mut();
            // SAFETY: `write_all` above initialized every byte of this window,
            // and nothing has grown or moved it since.
            let mut window =
                unsafe { reference.data_assume_init_mut() }.expect("sole reference is writable");
            assert_eq!(window.elem(0), Some(1));
            assert!(window.set_elem(0, 10));
        }
        // SAFETY: still fully initialized; only one byte changed value.
        let window = unsafe { owner.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.elem(0), Some(10));
    }

    #[test]
    fn a_zero_length_window_is_vacuously_initialized() {
        let mut empty = av_buffer_alloc(0).expect("zero-byte allocation");
        assert_eq!(empty.as_ref().size(), 0);
        // Nothing to write, so `write_all` succeeds and discharges the
        // obligation for the whole window.
        assert!(empty.as_mut().write_all(&[]));
        assert!(!empty.as_mut().write_all(&[1]));
        // SAFETY: an empty window has no byte that could be unwritten.
        let window = unsafe { empty.as_ref().data_assume_init() }.unwrap();
        assert!(window.is_empty());

        // `advance` to the end leaves the same vacuous case out of a window
        // whose bytes were never written.
        let mut unwritten = av_buffer_alloc(4).expect("four-byte allocation");
        assert!(unwritten.as_mut().advance(4));
        assert_eq!(unwritten.as_ref().size(), 0);
        // SAFETY: as above — the window is empty.
        let window = unsafe { unwritten.as_ref().data_assume_init() }.unwrap();
        assert!(window.is_empty());
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
    ///
    /// # Invariant
    ///
    /// `AVBufferReferenceRef::from_ptr` and `CBox::from_raw` promise only that
    /// the header itself is live and initialized. Every wrapped header
    /// additionally satisfies, and every unsafe constructor of one owes:
    ///
    /// - `buffer` is non-null and addresses an [`AVBuffer`] holding a count on
    ///   this header's behalf, so it outlives the header;
    /// - `data` is null, or addresses `size` **allocated** bytes owned by that
    ///   `AVBuffer` for as long as the header holds its count. Allocated is all
    ///   that is claimed — see [`AVBufferReferenceRef::data`] for why the
    ///   contents are not.
    ///
    /// [`buffer`](AVBufferReferenceRef::buffer), [`data`](AVBufferReferenceRef::data)
    /// and [`data_mut`](AVBufferReferenceMut::data_mut) are safe and rest on
    /// it. Every producer discharges it: `buffer_create` in
    /// `libavutil/buffer.c` is the only routine that fills a new header and
    /// writes all three fields together, and `av_buffer_ref` copies a header
    /// wholesale, so the wrappers over `av_buffer_alloc`, `av_buffer_allocz`,
    /// `av_buffer_ref`, `av_buffer_realloc` and `av_buffer_make_writable` all
    /// hold. The only safe writers in this crate are
    /// [`truncate`](AVBufferReferenceMut::truncate),
    /// [`advance`](AVBufferReferenceMut::advance) and
    /// [`write_all`](AVBufferReferenceMut::write_all): the first two replace
    /// the window with a sub-range of itself, the third writes inside it, and
    /// none touches `buffer`.
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

// SAFETY: `av_buffer_ref` copies the source header into a freshly allocated
// one and atomically increments the shared `AVBuffer.refcount`; it never writes
// the source header, and the underlying count it does write is an atomic the C
// implementation documents as safe to bump from several threads. The result is
// NULL or one independently releasable reference to the same underlying buffer.
unsafe impl CCloned for AVBufferReference {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: the trait contract supplies a live source reference and the
        // returned header, when non-null, is owned by the caller.
        let cloned = unsafe { ffi::av_buffer_ref(obj.as_ptr().cast::<ffi::AVBufferRef>()) };
        NonNull::new(cloned.cast::<Self>())
    }
}

impl<'a> AVBufferReferenceRef<'a> {
    /// Field: AVBufferRef.size
    ///
    /// Returns the length of this reference's current data window in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        // SAFETY: the handle points to a live initialized header. Copying the
        // scalar through a raw projection forms no reference to C storage.
        unsafe { addr_of!((*self.as_ptr()).size).read() }
    }

    /// Field: AVBufferRef.data
    ///
    /// Views the byte window without forming a Rust slice over memory that C
    /// may mutate, and without claiming its bytes have been written.
    /// `av_buffer_alloc` hands back raw `av_malloc` storage and
    /// `av_buffer_realloc` leaves the tail it grows into equally unwritten, so
    /// an initialized `CSlice<u8>` here would break
    /// [`CSlice::from_raw_parts`]'s precondition and let entirely safe code
    /// read an uninitialised `u8`. `None` represents a null data pointer, which
    /// is valid for an empty user-supplied buffer.
    ///
    /// Fill a window from safe code with
    /// [`write_all`](AVBufferReferenceMut::write_all), then read it through
    /// [`data_assume_init`](Self::data_assume_init).
    #[must_use]
    pub fn data(&self) -> Option<CSlice<'a, MaybeUninit<u8>>> {
        // SAFETY: both fields are copied through raw projections from a live
        // header; no reference to C storage is formed.
        let (data, size) = unsafe {
            (
                addr_of!((*self.as_ptr()).data).read(),
                addr_of!((*self.as_ptr()).size).read(),
            )
        };
        NonNull::new(data.cast::<MaybeUninit<u8>>()).map(|data| {
            // SAFETY: the type invariant gives `size` allocated bytes at a
            // non-null `data`, kept alive for `'a` by the header's count. Every
            // byte pattern — and the absence of one — is a valid
            // `MaybeUninit<u8>`, which is exactly what "allocated" licenses.
            unsafe { CSlice::from_raw_parts(data, size) }
        })
    }

    /// Field: AVBufferRef.data
    ///
    /// The same window, viewed as initialized bytes.
    ///
    /// # Safety
    ///
    /// All `size` bytes of the current window must already have been written:
    /// by `av_buffer_allocz`, by
    /// [`write_all`](AVBufferReferenceMut::write_all), or by C code outside
    /// this crate. `av_buffer_alloc` and the region `av_buffer_realloc` grows
    /// into do not satisfy this, and neither does a window narrowed by
    /// [`truncate`](AVBufferReferenceMut::truncate) or
    /// [`advance`](AVBufferReferenceMut::advance) out of an unwritten one.
    ///
    /// [`av_buffer_make_writable`] does not discharge it either, in itself:
    /// its C body `memcpy`s `buf->size` bytes out of the window it was given,
    /// so the copy carries whatever initialization the source had and no more.
    /// It preserves this obligation's status rather than establishing it.
    #[must_use]
    pub unsafe fn data_assume_init(&self) -> Option<CSlice<'a, u8>> {
        let window = self.data()?;
        let data = NonNull::new(window.as_elem_ptr().cast::<u8>())?;
        // SAFETY: the caller asserts every byte of the window is initialized;
        // `MaybeUninit<u8>` and `u8` share size and alignment, so the same
        // pointer and count describe the same run of `u8`, and `data()` has
        // already established that run under the type invariant.
        Some(unsafe { CSlice::from_raw_parts(data, window.len()) })
    }

    /// Field: AVBufferRef.buffer
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
    ///
    /// libavutil supports references that describe different parts of one
    /// buffer, and `av_buffer_realloc` explicitly re-checks
    /// `buf->data != buf->buffer->data` before reallocating in place, so a
    /// narrowed window stays usable. The result is a sub-range of the previous
    /// one, which is what keeps the type invariant true.
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
    ///
    /// As with [`truncate`](Self::truncate) the new window is a sub-range of
    /// the old one, so the type invariant continues to hold; `bytes == size`
    /// leaves an empty window at the one-past-the-end address.
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

    /// Field: AVBufferRef.data
    ///
    /// Exclusively views the data when this is the only reference and the
    /// underlying buffer is not marked read-only. As with
    /// [`AVBufferReferenceRef::data`], the elements are `MaybeUninit<u8>`
    /// because the type invariant claims the window is allocated, not written.
    #[must_use]
    pub fn data_mut(&mut self) -> Option<CSliceMut<'_, MaybeUninit<u8>>> {
        // SAFETY: the exclusive header handle is live. The C predicate checks
        // the underlying count and read-only flag and does not retain `buf`.
        if unsafe { ffi::av_buffer_is_writable(self.as_mut_ptr()) } == 0 {
            return None;
        }
        // SAFETY: fields are copied from the live exclusive header through raw
        // projections; no reference to C storage is formed.
        let (data, size) = unsafe {
            let header = self.as_mut_ptr();
            (
                addr_of!((*header).data).read(),
                addr_of!((*header).size).read(),
            )
        };
        NonNull::new(data.cast::<MaybeUninit<u8>>()).map(|data| {
            // SAFETY: the type invariant gives `size` allocated bytes at a
            // non-null `data`, every byte of which is a valid
            // `MaybeUninit<u8>`. The writability check proves no other
            // reference shares the underlying buffer, and the view is bound to
            // `&mut self`, so this is the only path to those bytes.
            unsafe { CSliceMut::from_raw_parts(data, size) }
        })
    }

    /// Field: AVBufferRef.data
    ///
    /// The same exclusive window, viewed as initialized bytes, for
    /// read-modify-write access.
    ///
    /// # Safety
    ///
    /// As [`AVBufferReferenceRef::data_assume_init`]: every byte of the current
    /// window must already have been written.
    #[must_use]
    pub unsafe fn data_assume_init_mut(&mut self) -> Option<CSliceMut<'_, u8>> {
        let mut window = self.data_mut()?;
        let len = window.len();
        let data = NonNull::new(window.as_mut_elem_ptr().cast::<u8>())?;
        // SAFETY: the caller asserts the window is initialized, and
        // `data_mut` has already proved exclusive access to `len` allocated
        // bytes at `data`. `MaybeUninit<u8>` and `u8` share size and
        // alignment, so both views describe the same run.
        Some(unsafe { CSliceMut::from_raw_parts(data, len) })
    }

    /// Field: AVBufferRef.data
    ///
    /// Writes `src` over the whole window, which is the safe way to make
    /// [`AVBufferReferenceRef::data_assume_init`] dischargeable for a buffer
    /// libavutil left unwritten. Returns `false` without writing anything when
    /// `src` does not cover the window exactly, or when this is not the only
    /// reference to a writable underlying buffer.
    pub fn write_all(&mut self, src: &[u8]) -> bool {
        if src.len() != self.as_ref().size() {
            return false;
        }
        let Some(mut window) = self.data_mut() else {
            // A null `data` slot has no bytes to write, so an empty `src`
            // leaves the window vacuously initialized; any other `None` means
            // the buffer is shared or read-only.
            return src.is_empty() && av_buffer_is_writable(self.as_ref());
        };
        // SAFETY: `window` exclusively covers exactly `src.len()` allocated
        // bytes of C storage, and `src` is a live Rust slice, which therefore
        // cannot overlap storage a C object owns. The copy initializes every
        // byte of the window.
        unsafe {
            core::ptr::copy_nonoverlapping(
                src.as_ptr(),
                window.as_mut_elem_ptr().cast::<u8>(),
                src.len(),
            );
        }
        true
    }
}

/// Wraps: av_buffer_alloc
///
/// Allocates an uninitialized byte buffer and adopts its independently
/// releasable reference header. The window is `av_malloc` storage nobody has
/// written, so it reads back only through
/// [`AVBufferReferenceRef::data`]; use
/// [`AVBufferReferenceMut::write_all`] first if you need the initialized view.
#[must_use]
pub fn av_buffer_alloc(size: usize) -> Option<CBox<AVBufferReference>> {
    // SAFETY: a non-null return is a fully constructed AVBufferRef carrying
    // one ownership count, released by AVBufferReference's CDropped impl.
    unsafe { CBox::from_raw(ffi::av_buffer_alloc(size)) }
}

/// Wraps: av_buffer_allocz
///
/// Allocates a zero-filled byte buffer. Unlike [`av_buffer_alloc`] the C
/// implementation `memset`s the whole window, so
/// [`AVBufferReferenceRef::data_assume_init`] is immediately dischargeable for
/// the result — until [`av_buffer_realloc`] grows it.
#[must_use]
pub fn av_buffer_allocz(size: usize) -> Option<CBox<AVBufferReference>> {
    // SAFETY: the ownership contract is identical to `av_buffer_alloc`.
    unsafe { CBox::from_raw(ffi::av_buffer_allocz(size)) }
}

/// Wraps: av_buffer_get_ref_count
#[must_use]
pub fn av_buffer_get_ref_count(buffer: AVBufferReferenceRef<'_>) -> usize {
    // SAFETY: the borrowed handle supplies a live reference header for this
    // read-only call and C does not retain it.
    unsafe { ffi::av_buffer_get_ref_count(buffer.as_ptr()) as usize }
}

/// Wraps: av_buffer_is_writable
#[must_use]
pub fn av_buffer_is_writable(buffer: AVBufferReferenceRef<'_>) -> bool {
    // SAFETY: the borrowed handle supplies a live reference header for this
    // read-only call and C does not retain it.
    unsafe { ffi::av_buffer_is_writable(buffer.as_ptr()) != 0 }
}

/// Wraps: av_buffer_make_writable
///
/// Consumes a reference and returns a writable reference, which may identify
/// a copied buffer. On allocation failure the original owner is returned with
/// the negative libavutil error code.
///
/// The window's contents survive: C either keeps the reference untouched when
/// it is already writable, or `memcpy`s all `size` bytes into a fresh
/// `av_buffer_alloc` window. Because that copy is the only thing that writes
/// the new window, it carries exactly the initialization the old one had — so
/// this operation preserves whether
/// [`AVBufferReferenceRef::data_assume_init`] is dischargeable, and never
/// establishes it.
pub fn av_buffer_make_writable(
    buffer: CBox<AVBufferReference>,
) -> Result<CBox<AVBufferReference>, (i32, CBox<AVBufferReference>)> {
    let mut raw = buffer.into_raw();
    // SAFETY: `raw` is a writable local owner slot containing one live
    // reference. C either leaves it unchanged or consumes and replaces it with
    // another independently owned live reference.
    let status = unsafe { ffi::av_buffer_make_writable(addr_of_mut!(raw)) };
    // SAFETY: both success and failure contracts leave the slot non-null and
    // owning exactly one fully constructed reference.
    let buffer = unsafe { CBox::from_raw(raw) }.expect("av_buffer_make_writable kept an owner");
    if status < 0 {
        Err((status, buffer))
    } else {
        Ok(buffer)
    }
}

/// Wraps: av_buffer_realloc
///
/// Resizes an owned reference, or allocates one when `buffer` is `None`. On
/// failure the original nullable owner is returned unchanged.
pub fn av_buffer_realloc(
    buffer: Option<CBox<AVBufferReference>>,
    size: usize,
) -> Result<CBox<AVBufferReference>, (i32, Option<CBox<AVBufferReference>>)> {
    let mut raw = buffer.map_or(core::ptr::null_mut(), CBox::into_raw);
    // SAFETY: `raw` is a writable nullable owner slot. C adopts no ownership on
    // failure and leaves one newly allocated or resized owner on success.
    let status = unsafe { ffi::av_buffer_realloc(addr_of_mut!(raw), size) };
    // SAFETY: the call leaves either null or one fully constructed owned
    // reference in the local slot, whose ownership is transferred here.
    let buffer = unsafe { CBox::from_raw(raw) };
    if status < 0 {
        Err((status, buffer))
    } else {
        Ok(buffer.expect("successful av_buffer_realloc returned an owner"))
    }
}

/// Wraps: av_buffer_ref
///
/// Creates another independently releasable header for the same underlying
/// byte buffer.
#[must_use]
pub fn av_buffer_ref(buffer: AVBufferReferenceRef<'_>) -> Option<CBox<AVBufferReference>> {
    // SAFETY: the source handle is live and read-only for the call. A non-null
    // result transfers a newly allocated header and one count to Rust.
    unsafe { CBox::from_raw(ffi::av_buffer_ref(buffer.as_ptr())) }
}

/// Wraps: av_buffer_unref
///
/// Releases a nullable owned reference. This consumes the Rust owner so no
/// usable handle remains after C decrements the count.
pub fn av_buffer_unref(buffer: Option<CBox<AVBufferReference>>) {
    drop(buffer);
}

#[cfg(test)]
mod scheduled_function_tests {
    use super::*;

    #[test]
    fn allocation_clone_writability_and_release_are_owned() {
        let mut buffer = av_buffer_allocz(8).expect("buffer allocation");
        assert_eq!(av_buffer_get_ref_count(buffer.as_ref()), 1);
        assert!(av_buffer_is_writable(buffer.as_ref()));

        let clone = av_buffer_ref(buffer.as_ref()).expect("reference clone");
        assert_eq!(av_buffer_get_ref_count(buffer.as_ref()), 2);
        assert!(!av_buffer_is_writable(buffer.as_ref()));
        av_buffer_unref(Some(clone));

        buffer = av_buffer_make_writable(buffer).expect("already writable");
        assert!(buffer.as_mut().data_mut().is_some());
        // `av_buffer_allocz` wrote the whole window, and `make_writable`
        // either kept it or `memcpy`d it, so it is still fully initialized.
        // SAFETY: as stated above.
        let window = unsafe { buffer.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.len(), 8);
        assert!(window.elems().all(|byte| byte == 0));
    }

    #[test]
    fn realloc_allocates_and_preserves_the_prefix() {
        let mut buffer = av_buffer_alloc(2).expect("buffer allocation");
        assert!(buffer.as_mut().write_all(&[7, 9]));
        let buffer = av_buffer_realloc(Some(buffer), 4).expect("buffer growth");
        assert_eq!(buffer.as_ref().size(), 4);

        // Only the copied prefix has been written; the two bytes the growth
        // added are `av_realloc` storage, so the window as a whole is not
        // `data_assume_init`-able and each surviving byte is read on its own.
        let window = buffer.as_ref().data().expect("non-null window");
        // SAFETY: `write_all` initialized both bytes before the growth, and
        // `av_buffer_realloc` preserves `FFMIN(size, buf->size)` of them.
        assert_eq!(unsafe { window.elem(0).unwrap().assume_init() }, 7);
        // SAFETY: as above.
        assert_eq!(unsafe { window.elem(1).unwrap().assume_init() }, 9);

        let allocated = av_buffer_realloc(None, 3).expect("null-slot allocation");
        assert_eq!(allocated.as_ref().size(), 3);
    }
}
