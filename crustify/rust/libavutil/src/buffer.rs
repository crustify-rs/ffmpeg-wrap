//! Wrappers for libavutil reference-counted buffers.

use ffibox::define_ctype;

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
}
