//! Wrappers for `libavutil/fifo.c`.

use core::ptr::NonNull;

use ffibox::{CDropped, define_ctype};

use crate::ffi;

define_ctype!(
    /// Wraps: AVFifo
    ///
    /// Opaque, uniquely owned generic FIFO storage. An owning value is a
    /// `CBox<AVFifo>` adopted from a successful C constructor; dropping it
    /// releases both the private byte buffer and the FIFO header through
    /// `av_fifo_freep2`. The public header exposes no fields, so borrowed
    /// handles intentionally provide identity only.
    AVFifo,
    AVFifoRef,
    AVFifoMut,
    ffi::AVFifo
);

// SAFETY: a `CBox<AVFifo>` may only be adopted from a fully initialized,
// uniquely owned C allocation. `av_fifo_freep2` is its published destructor:
// it releases the owned buffer and header and accepts the temporary pointer
// slot used here.
unsafe impl CDropped for AVFifo {
    unsafe fn c_drop(obj: NonNull<Self>) {
        let mut raw = obj.as_ptr().cast::<ffi::AVFifo>();
        // SAFETY: the trait contract gives this call the one owned, fully
        // constructed FIFO allocation. `raw` is a live local slot and the C
        // destructor consumes its pointee before writing NULL to the slot.
        unsafe { ffi::av_fifo_freep2(&raw mut raw) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use ffibox::CBox;

    use super::*;

    #[test]
    fn handles_have_pointer_layout() {
        assert_eq!(size_of::<AVFifoRef<'_>>(), size_of::<*const ffi::AVFifo>());
        assert_eq!(
            align_of::<AVFifoRef<'_>>(),
            align_of::<*const ffi::AVFifo>()
        );
        assert_eq!(size_of::<AVFifoMut<'_>>(), size_of::<*mut ffi::AVFifo>());
        assert_eq!(
            size_of::<Option<AVFifoRef<'_>>>(),
            size_of::<*const ffi::AVFifo>()
        );
    }

    #[test]
    fn owning_fifo_uses_the_published_destructor() {
        // SAFETY: `av_fifo_alloc2` returns either NULL or one newly allocated,
        // fully initialized FIFO. The matching `CBox` immediately adopts it.
        let fifo = unsafe { CBox::<AVFifo>::from_raw(ffi::av_fifo_alloc2(8, 4, 0)) }
            .expect("FIFO allocation");
        assert!(!fifo.as_ptr().is_null());
        drop(fifo);
    }
}
