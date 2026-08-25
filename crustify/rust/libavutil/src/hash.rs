//! Wrappers for `libavutil/hash.c`.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of_mut};

use ffibox::{CBox, CCell, CDropped, CPtr, CType};

use crate::ffi;

/// Wraps: AVHashContext
///
/// Opaque hash state allocated by `av_hash_alloc`. The public header withholds
/// its layout, so it cannot be constructed inline. An owning pointer is an
/// [`ffibox::CBox<AVHashContext>`], whose drop calls `av_hash_freep`; that
/// routine releases the context's optional algorithm state and its header.
///
/// This representation is written out instead of using
/// [`ffibox::define_ctype!`], because that macro would expose `zeroed()` for
/// bindgen's zero-sized incomplete declaration rather than a real context.
#[repr(transparent)]
pub struct AVHashContext(CType<ffi::AVHashContext>);

/// Shared borrowed handle to an [`AVHashContext`].
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AVHashContextRef<'a>(CPtr<'a, AVHashContext>);

impl<'a> AVHashContextRef<'a> {
    /// Borrows a context pointer, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live initialized context that remains live and is
    /// not mutably accessed for `'a`.
    pub unsafe fn from_ptr(ptr: *mut ffi::AVHashContext) -> Option<Self> {
        NonNull::new(ptr.cast::<AVHashContext>()).map(|ptr| {
            // SAFETY: the caller supplies the lifetime and shared-access
            // guarantees required by `CPtr::new`.
            Self(unsafe { CPtr::new(ptr) })
        })
    }

    /// Returns the borrowed pointer for a read-only FFI call.
    #[must_use]
    pub fn as_ptr(&self) -> *const ffi::AVHashContext {
        self.0.as_non_null().as_ptr().cast::<ffi::AVHashContext>()
    }
}

/// Exclusive borrowed handle to an [`AVHashContext`].
#[repr(transparent)]
pub struct AVHashContextMut<'a>(AVHashContextRef<'a>);

impl<'a> AVHashContextMut<'a> {
    /// Borrows a context pointer exclusively, returning `None` for null.
    ///
    /// # Safety
    ///
    /// `ptr` must address a live initialized context for `'a`, and no other
    /// handle may be used while the result lives.
    pub unsafe fn from_ptr(ptr: *mut ffi::AVHashContext) -> Option<Self> {
        NonNull::new(ptr.cast::<AVHashContext>()).map(|ptr| {
            // SAFETY: the caller supplies the lifetime and exclusive-access
            // guarantees required by `CPtr::new`.
            Self(AVHashContextRef(unsafe { CPtr::new(ptr) }))
        })
    }

    /// Returns the context pointer for a mutating FFI call.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::AVHashContext {
        self.0.0.as_non_null().as_ptr().cast::<ffi::AVHashContext>()
    }

    /// Reborrows this exclusive handle as a shared handle.
    #[must_use]
    pub fn as_ref(&self) -> AVHashContextRef<'_> {
        self.0
    }
}

// SAFETY: `AVHashContext` is transparent over `CType<ffi::AVHashContext>`;
// both handles are transparent over `CPtr<'a, AVHashContext>`, and the shared
// handle exposes no mutating operation.
unsafe impl CCell for AVHashContext {
    type C = ffi::AVHashContext;
    type Ref<'a> = AVHashContextRef<'a>;
    type Mut<'a> = AVHashContextMut<'a>;

    unsafe fn ref_from_raw<'a>(ptr: NonNull<Self>) -> Self::Ref<'a> {
        // SAFETY: the caller upholds the `CCell` liveness and shared-access
        // contract for the constructed handle.
        AVHashContextRef(unsafe { CPtr::new(ptr) })
    }

    unsafe fn mut_from_raw<'a>(ptr: NonNull<Self>) -> Self::Mut<'a> {
        // SAFETY: the caller upholds the `CCell` liveness and exclusive-access
        // contract for the constructed handle.
        AVHashContextMut(AVHashContextRef(unsafe { CPtr::new(ptr) }))
    }
}

// SAFETY: every owned AVHashContext comes from `av_hash_alloc`, and
// `av_hash_freep` is its one-shot matching releaser. It disposes the optional
// owned algorithm context before freeing the header.
/// Wraps: av_hash_freep
unsafe impl CDropped for AVHashContext {
    unsafe fn c_drop(context: NonNull<Self>) {
        let mut raw = context.as_ptr().cast::<ffi::AVHashContext>();
        // SAFETY: the trait contract transfers a live uniquely owned context
        // exactly once; `raw` is a writable local slot C may null after
        // consuming the pointee.
        unsafe { ffi::av_hash_freep(addr_of_mut!(raw)) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use ffibox::CBox;

    use super::*;

    #[test]
    fn opaque_handles_and_owner_are_pointer_sized() {
        assert_eq!(size_of::<AVHashContextRef<'_>>(), size_of::<*mut ()>());
        assert_eq!(size_of::<AVHashContextMut<'_>>(), size_of::<*mut ()>());
        assert_eq!(size_of::<CBox<AVHashContext>>(), size_of::<*mut ()>());
    }

    #[test]
    fn null_pointer_cannot_be_adopted_as_an_owner() {
        // SAFETY: null transfers no allocation, and `from_raw` represents it
        // as `None` without invoking the destructor.
        let owner = unsafe { CBox::<AVHashContext>::from_raw(core::ptr::null_mut()) };
        assert!(owner.is_none());
    }
}

/// Wraps: av_hash_names
#[must_use]
pub fn av_hash_names(index: i32) -> Option<&'static CStr> {
    // SAFETY: C returns null or a pointer into its immutable static hash table.
    let pointer = unsafe { ffi::av_hash_names(index) };
    if pointer.is_null() {
        None
    } else {
        // SAFETY: a non-null table entry is a static NUL-terminated name.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

#[cfg(test)]
mod scheduled_name_tests {
    use super::*;

    #[test]
    fn indexes_static_hash_names() {
        assert_eq!(av_hash_names(0), Some(c"MD5"));
        assert_eq!(av_hash_names(-1), None);
        assert_eq!(av_hash_names(i32::MAX), None);
    }
}

/// Wraps: av_hash_alloc
///
/// The wrapper also performs the initialization required before the first
/// update, so every returned owner is ready for hashing.
pub fn av_hash_alloc(name: &CStr) -> Result<CBox<AVHashContext>, i32> {
    let mut pointer = core::ptr::null_mut();
    // SAFETY: `pointer` is a writable output slot and `name` is a live
    // terminated string which C does not retain.
    let status = unsafe { ffi::av_hash_alloc(&raw mut pointer, name.as_ptr()) };
    if status < 0 {
        return Err(status);
    }
    // SAFETY: success transfers a fresh fully allocated context.
    let mut context =
        unsafe { CBox::from_raw(pointer) }.expect("av_hash_alloc succeeded with a null context");
    av_hash_init(&mut context.as_mut());
    Ok(context)
}

/// Wraps: av_hash_get_name
#[must_use]
pub fn av_hash_get_name(context: AVHashContextRef<'_>) -> &'static CStr {
    // SAFETY: the live context carries a valid algorithm index and C returns
    // its immutable process-lifetime table name.
    unsafe { CStr::from_ptr(ffi::av_hash_get_name(context.as_ptr())) }
}

/// Wraps: av_hash_get_size
#[must_use]
pub fn av_hash_get_size(context: AVHashContextRef<'_>) -> usize {
    // SAFETY: the live initialized context carries a valid algorithm index.
    unsafe { ffi::av_hash_get_size(context.as_ptr()) as usize }
}

/// Wraps: av_hash_init
pub fn av_hash_init(context: &mut AVHashContextMut<'_>) {
    // SAFETY: exclusive access permits resetting the live context state.
    unsafe { ffi::av_hash_init(context.as_mut_ptr()) }
}

/// Wraps: av_hash_update
pub fn av_hash_update(context: &mut AVHashContextMut<'_>, source: &[u8]) {
    // SAFETY: source supplies exactly its reported readable length and is not
    // retained; exclusive access permits updating the context.
    unsafe { ffi::av_hash_update(context.as_mut_ptr(), source.as_ptr(), source.len()) }
}

fn output_size(output: &[u8]) -> Result<i32, i32> {
    i32::try_from(output.len()).map_err(|_| -22)
}

/// Wraps: av_hash_final
///
/// The context is borrowed, not consumed: C finalizes the algorithm state in
/// place and leaves the allocation to its owner, so the same context serves
/// another message after [`av_hash_init`] resets it.
pub fn av_hash_final(context: &mut AVHashContextMut<'_>, output: &mut [u8]) -> Result<usize, i32> {
    let required = av_hash_get_size(context.as_ref());
    if output.len() < required {
        return Err(-22);
    }
    // SAFETY: output has at least the algorithm's full digest size, which is
    // what C writes, and the exclusive handle permits finalizing in place.
    unsafe { ffi::av_hash_final(context.as_mut_ptr(), output.as_mut_ptr()) }
    Ok(required)
}

/// Wraps: av_hash_final_bin
///
/// Borrows the context, as [`av_hash_final`] does.
pub fn av_hash_final_bin(context: &mut AVHashContextMut<'_>, output: &mut [u8]) -> Result<(), i32> {
    let size = output_size(output)?;
    // SAFETY: output supplies the `size` writable bytes C fills, truncating or
    // zero-padding the digest to exactly that width.
    unsafe { ffi::av_hash_final_bin(context.as_mut_ptr(), output.as_mut_ptr(), size) }
    Ok(())
}

/// Wraps: av_hash_final_hex
///
/// Borrows the context, as [`av_hash_final`] does.
pub fn av_hash_final_hex(context: &mut AVHashContextMut<'_>, output: &mut [u8]) -> Result<(), i32> {
    let size = output_size(output)?;
    // SAFETY: output supplies `size` writable bytes, which bound every
    // `snprintf` C writes into it.
    unsafe { ffi::av_hash_final_hex(context.as_mut_ptr(), output.as_mut_ptr(), size) }
    Ok(())
}

/// Wraps: av_hash_final_b64
///
/// Borrows the context, as [`av_hash_final`] does.
pub fn av_hash_final_b64(context: &mut AVHashContextMut<'_>, output: &mut [u8]) -> Result<(), i32> {
    let size = output_size(output)?;
    if size == 0 {
        return Err(-22);
    }
    // SAFETY: output supplies `size` writable bytes, and `size >= 1` keeps C
    // away from the `dst[size - 1] = 0` truncation it performs for a buffer
    // shorter than the encoded digest.
    unsafe { ffi::av_hash_final_b64(context.as_mut_ptr(), output.as_mut_ptr(), size) }
    Ok(())
}

#[cfg(test)]
mod hashing_tests {
    use super::*;

    const ABC_MD5: [u8; 16] = [
        0x90, 0x01, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0, 0xd6, 0x96, 0x3f, 0x7d, 0x28, 0xe1, 0x7f,
        0x72,
    ];

    #[test]
    fn hashes_and_finalizes_in_place() {
        let mut context = av_hash_alloc(c"MD5").unwrap();
        assert_eq!(av_hash_get_name(context.as_ref()), c"MD5");
        assert_eq!(av_hash_get_size(context.as_ref()), 16);
        av_hash_update(&mut context.as_mut(), b"abc");
        let mut digest = [0; 16];
        assert_eq!(av_hash_final(&mut context.as_mut(), &mut digest), Ok(16));
        assert_eq!(digest, ABC_MD5);
    }

    #[test]
    fn a_reset_context_hashes_another_message() {
        // C borrows the context for the whole hash lifecycle, so one owner
        // covers any number of messages.
        let mut context = av_hash_alloc(c"MD5").unwrap();
        let mut first = [0; 16];
        av_hash_update(&mut context.as_mut(), b"abc");
        assert_eq!(av_hash_final(&mut context.as_mut(), &mut first), Ok(16));

        av_hash_init(&mut context.as_mut());
        let mut second = [0; 16];
        av_hash_update(&mut context.as_mut(), b"a");
        av_hash_update(&mut context.as_mut(), b"bc");
        assert_eq!(av_hash_final(&mut context.as_mut(), &mut second), Ok(16));

        assert_eq!(first, ABC_MD5);
        assert_eq!(second, ABC_MD5);
    }

    #[test]
    fn shorter_and_longer_outputs_are_written_to_their_exact_width() {
        let mut context = av_hash_alloc(c"MD5").unwrap();
        av_hash_update(&mut context.as_mut(), b"abc");

        // A digest wider than the algorithm's is zero-padded, a narrower one
        // truncated; both stay inside the slice.
        let mut wide = [0xff; 20];
        assert_eq!(av_hash_final_bin(&mut context.as_mut(), &mut wide), Ok(()));
        assert_eq!(wide[..16], ABC_MD5);
        assert_eq!(wide[16..], [0; 4]);

        av_hash_init(&mut context.as_mut());
        av_hash_update(&mut context.as_mut(), b"abc");
        let mut narrow = [0xff; 4];
        assert_eq!(
            av_hash_final_bin(&mut context.as_mut(), &mut narrow),
            Ok(())
        );
        assert_eq!(narrow, ABC_MD5[..4]);

        av_hash_init(&mut context.as_mut());
        av_hash_update(&mut context.as_mut(), b"abc");
        let mut hex = [0; 33];
        assert_eq!(av_hash_final_hex(&mut context.as_mut(), &mut hex), Ok(()));
        assert_eq!(&hex[..6], b"900150");

        av_hash_init(&mut context.as_mut());
        av_hash_update(&mut context.as_mut(), b"abc");
        let mut base64 = [0; 25];
        assert_eq!(
            av_hash_final_b64(&mut context.as_mut(), &mut base64),
            Ok(())
        );
        assert_eq!(&base64[..6], b"kAFQmD");
    }

    #[test]
    fn full_digest_output_must_fit() {
        let mut context = av_hash_alloc(c"MD5").unwrap();
        // `av_hash_final` writes the algorithm's full width and takes no size.
        assert_eq!(av_hash_final(&mut context.as_mut(), &mut [0; 15]), Err(-22));
    }

    #[test]
    fn base64_requires_space_for_its_terminator() {
        let mut context = av_hash_alloc(c"MD5").unwrap();
        // C truncates with `dst[size - 1] = 0`, which underflows at size 0.
        assert_eq!(av_hash_final_b64(&mut context.as_mut(), &mut []), Err(-22));
    }
}
