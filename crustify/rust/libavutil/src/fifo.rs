//! Wrappers for `libavutil/fifo.c`.

use core::mem::MaybeUninit;
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

/// A FIFO segment C asks a producer callback to initialize.
pub struct FifoWriteBuffer<'a> {
    bytes: &'a mut [MaybeUninit<u8>],
    element_size: usize,
    initialized: usize,
}

impl FifoWriteBuffer<'_> {
    #[must_use]
    pub fn remaining_elements(&self) -> usize {
        self.bytes.len() / self.element_size - self.initialized
    }

    /// Initializes the next complete opaque element.
    pub fn write_next(&mut self, element: &[u8]) -> bool {
        if element.len() != self.element_size || self.remaining_elements() == 0 {
            return false;
        }
        let start = self.initialized * self.element_size;
        for (slot, byte) in self.bytes[start..start + self.element_size]
            .iter_mut()
            .zip(element)
        {
            slot.write(*byte);
        }
        self.initialized += 1;
        true
    }
}

/// A FIFO segment supplied to a consumer callback.
#[derive(Clone, Copy)]
pub struct FifoReadBuffer<'a> {
    bytes: &'a [u8],
    element_size: usize,
}

impl FifoReadBuffer<'_> {
    #[must_use]
    pub fn elements(&self) -> impl ExactSizeIterator<Item = &[u8]> {
        self.bytes.chunks_exact(self.element_size)
    }

    #[must_use]
    pub fn consume(&self, elements: usize) -> Option<FifoConsumed> {
        (elements <= self.bytes.len() / self.element_size).then_some(FifoConsumed(elements))
    }
}

/// A checked element count returned by a consumer callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FifoConsumed(usize);

/// Wraps: AVFifoCB
///
/// Producer interpretation used by `av_fifo_write_from_cb`. The callback sees
/// uninitialized opaque storage and can only report elements it initialized
/// through [`FifoWriteBuffer::write_next`].
pub trait AVFifoReadCallback {
    fn read(&mut self, destination: &mut FifoWriteBuffer<'_>) -> Result<(), i32>;
}

/// Wraps: AVFifoCB
///
/// Consumer interpretation used by `av_fifo_read_to_cb` and
/// `av_fifo_peek_to_cb`. [`FifoReadBuffer::consume`] prevents a safe callback
/// from claiming more elements than C supplied.
pub trait AVFifoWriteCallback {
    fn write(&mut self, source: FifoReadBuffer<'_>) -> Result<FifoConsumed, i32>;
}

#[cfg(test)]
mod callback_tests {
    use super::*;

    #[test]
    fn callback_views_track_whole_elements() {
        let mut storage = [MaybeUninit::uninit(); 4];
        let mut output = FifoWriteBuffer {
            bytes: &mut storage,
            element_size: 2,
            initialized: 0,
        };
        assert!(output.write_next(&[1, 2]));
        assert!(!output.write_next(&[3]));
        assert_eq!(output.remaining_elements(), 1);

        let input = FifoReadBuffer {
            bytes: &[1, 2, 3, 4],
            element_size: 2,
        };
        let mut elements = input.elements();
        assert_eq!(elements.next(), Some(&[1, 2][..]));
        assert_eq!(elements.next(), Some(&[3, 4][..]));
        assert_eq!(elements.next(), None);
        assert_eq!(input.consume(2), Some(FifoConsumed(2)));
        assert_eq!(input.consume(3), None);
    }
}
