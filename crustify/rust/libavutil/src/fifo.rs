//! Wrappers for `libavutil/fifo.c`.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use ffibox::{CBox, CDropped, define_ctype};

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
/// Wraps: av_fifo_freep2
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

impl<'a> FifoReadBuffer<'a> {
    #[must_use]
    pub fn elements(&self) -> impl ExactSizeIterator<Item = &[u8]> {
        self.bytes.chunks_exact(self.element_size)
    }

    #[must_use]
    pub fn consume(&self, elements: usize) -> Option<FifoConsumed<'a>> {
        (elements <= self.bytes.len() / self.element_size)
            .then_some(FifoConsumed(elements, PhantomData))
    }
}

/// A checked element count returned by a consumer callback.
///
/// `'a` brands the count to the one [`FifoReadBuffer`] it was checked against.
/// The brand is invariant, so a count checked against a larger earlier segment
/// cannot be stored in the callback and returned for a smaller later one —
/// which C would take at face value, underflowing its own remaining-element
/// count:
///
/// ```compile_fail
/// use libavutil::fifo::{AVFifoWriteCallback, FifoConsumed, FifoReadBuffer};
///
/// struct Hoarder<'h>(Option<FifoConsumed<'h>>);
///
/// impl AVFifoWriteCallback for Hoarder<'_> {
///     fn write<'a>(&mut self, source: FifoReadBuffer<'a>) -> Result<FifoConsumed<'a>, i32> {
///         let count = source.elements().len();
///         self.0 = source.consume(count);
///         Err(-1)
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FifoConsumed<'a>(usize, PhantomData<fn(&'a ()) -> &'a ()>);

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
    fn write<'a>(&mut self, source: FifoReadBuffer<'a>) -> Result<FifoConsumed<'a>, i32>;
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
        assert_eq!(input.consume(2), Some(FifoConsumed(2, PhantomData)));
        assert_eq!(input.consume(3), None);
    }
}

/// Wraps: av_fifo_alloc2
#[must_use]
pub fn av_fifo_alloc2(elements: usize, element_size: usize, flags: u32) -> Option<CBox<AVFifo>> {
    // SAFETY: a non-null result is a newly allocated, fully initialized FIFO
    // whose matching destructor is registered on `AVFifo`.
    unsafe { CBox::from_raw(ffi::av_fifo_alloc2(elements, element_size, flags)) }
}

/// Wraps: av_fifo_auto_grow_limit
pub fn av_fifo_auto_grow_limit(fifo: &mut AVFifoMut<'_>, maximum_elements: usize) {
    // SAFETY: the exclusive handle identifies a live FIFO and C retains no
    // Rust borrow.
    unsafe { ffi::av_fifo_auto_grow_limit(fifo.as_mut_ptr(), maximum_elements) }
}

/// Wraps: av_fifo_can_read
#[must_use]
pub fn av_fifo_can_read(fifo: AVFifoRef<'_>) -> usize {
    // SAFETY: the shared handle identifies a live FIFO for this read-only call.
    unsafe { ffi::av_fifo_can_read(fifo.as_ptr()) }
}

/// Wraps: av_fifo_can_write
#[must_use]
pub fn av_fifo_can_write(fifo: AVFifoRef<'_>) -> usize {
    // SAFETY: the shared handle identifies a live FIFO for this read-only call.
    unsafe { ffi::av_fifo_can_write(fifo.as_ptr()) }
}

/// Wraps: av_fifo_drain2
pub fn av_fifo_drain2(fifo: &mut AVFifoMut<'_>, elements: usize) -> Result<(), i32> {
    if elements > av_fifo_can_read(fifo.as_ref()) {
        return Err(-22);
    }
    // SAFETY: the checked count does not exceed the readable element count,
    // and the exclusive handle permits the state update.
    unsafe { ffi::av_fifo_drain2(fifo.as_mut_ptr(), elements) }
    Ok(())
}

/// Wraps: av_fifo_elem_size
#[must_use]
pub fn av_fifo_elem_size(fifo: AVFifoRef<'_>) -> usize {
    // SAFETY: the shared handle identifies a live FIFO for this read-only call.
    unsafe { ffi::av_fifo_elem_size(fifo.as_ptr()) }
}

/// Wraps: av_fifo_grow2
pub fn av_fifo_grow2(fifo: &mut AVFifoMut<'_>, increment: usize) -> Result<(), i32> {
    // SAFETY: the exclusive handle permits C to resize the FIFO allocation.
    let status = unsafe { ffi::av_fifo_grow2(fifo.as_mut_ptr(), increment) };
    if status < 0 { Err(status) } else { Ok(()) }
}

fn element_count(bytes: usize, element_size: usize) -> Result<usize, i32> {
    if element_size == 0 || !bytes.is_multiple_of(element_size) {
        Err(-22)
    } else {
        Ok(bytes / element_size)
    }
}

/// Wraps: av_fifo_peek
///
/// An empty `output` is still forwarded to C, because `offset` is validated
/// against the readable element count even for a zero-element request.
pub fn av_fifo_peek(fifo: AVFifoRef<'_>, output: &mut [u8], offset: usize) -> Result<(), i32> {
    let count = element_count(output.len(), av_fifo_elem_size(fifo))?;
    // SAFETY: `output` provides exactly `count * element_size` writable bytes;
    // C copies into that pointer only while its remaining count is nonzero, so
    // the dangling pointer of an empty slice is never dereferenced. The FIFO
    // is shared because peeking does not modify it.
    let status =
        unsafe { ffi::av_fifo_peek(fifo.as_ptr(), output.as_mut_ptr().cast(), count, offset) };
    if status < 0 { Err(status) } else { Ok(()) }
}

/// Wraps: av_fifo_read
pub fn av_fifo_read(fifo: &mut AVFifoMut<'_>, output: &mut [u8]) -> Result<(), i32> {
    let count = element_count(output.len(), av_fifo_elem_size(fifo.as_ref()))?;
    if count == 0 {
        return Ok(());
    }
    // SAFETY: `output` provides exactly `count * element_size` writable bytes
    // and the exclusive handle permits consuming FIFO state.
    let status = unsafe { ffi::av_fifo_read(fifo.as_mut_ptr(), output.as_mut_ptr().cast(), count) };
    if status < 0 { Err(status) } else { Ok(()) }
}

/// Wraps: av_fifo_reset2
pub fn av_fifo_reset2(fifo: &mut AVFifoMut<'_>) {
    // SAFETY: the exclusive handle permits resetting the live FIFO.
    unsafe { ffi::av_fifo_reset2(fifo.as_mut_ptr()) }
}

/// Wraps: av_fifo_write
pub fn av_fifo_write(fifo: &mut AVFifoMut<'_>, input: &[u8]) -> Result<(), i32> {
    let count = element_count(input.len(), av_fifo_elem_size(fifo.as_ref()))?;
    if count == 0 {
        return Ok(());
    }
    // SAFETY: `input` provides exactly `count * element_size` readable bytes
    // and remains borrowed for the call.
    let status = unsafe { ffi::av_fifo_write(fifo.as_mut_ptr(), input.as_ptr().cast(), count) };
    if status < 0 { Err(status) } else { Ok(()) }
}

struct ReadCallbackState<'a, C> {
    callback: &'a mut C,
    element_size: usize,
}

unsafe extern "C" fn read_callback<C: AVFifoReadCallback>(
    opaque: *mut c_void,
    buffer: *mut c_void,
    elements: *mut usize,
) -> i32 {
    // SAFETY: the callback wrappers below pass this exact state type and C
    // invokes the trampoline synchronously while the state remains live.
    let state = unsafe { &mut *opaque.cast::<ReadCallbackState<'_, C>>() };
    // SAFETY: C supplies a writable segment of `*elements` complete FIFO
    // elements for the duration of this callback.
    let maximum = unsafe { *elements };
    if maximum == 0 {
        return 0;
    }
    let Some(length) = maximum.checked_mul(state.element_size) else {
        return -22;
    };
    // SAFETY: the callback contract supplies `length` writable bytes.
    let bytes =
        unsafe { core::slice::from_raw_parts_mut(buffer.cast::<MaybeUninit<u8>>(), length) };
    let mut destination = FifoWriteBuffer {
        bytes,
        element_size: state.element_size,
        initialized: 0,
    };
    let status = state.callback.read(&mut destination).err().unwrap_or(0);
    // SAFETY: the count slot is writable and `initialized` cannot exceed the
    // maximum because `write_next` checks each complete element.
    unsafe { *elements = destination.initialized };
    status
}

struct WriteCallbackState<'a, C> {
    callback: &'a mut C,
    element_size: usize,
}

unsafe extern "C" fn write_callback<C: AVFifoWriteCallback>(
    opaque: *mut c_void,
    buffer: *mut c_void,
    elements: *mut usize,
) -> i32 {
    // SAFETY: the wrapper passes this exact state and C calls synchronously.
    let state = unsafe { &mut *opaque.cast::<WriteCallbackState<'_, C>>() };
    // SAFETY: C supplies a readable segment of `*elements` complete elements.
    let maximum = unsafe { *elements };
    if maximum == 0 {
        return 0;
    }
    let Some(length) = maximum.checked_mul(state.element_size) else {
        return -22;
    };
    // SAFETY: the callback contract supplies `length` initialized bytes.
    let bytes = unsafe { core::slice::from_raw_parts(buffer.cast_const().cast::<u8>(), length) };
    match state.callback.write(FifoReadBuffer {
        bytes,
        element_size: state.element_size,
    }) {
        Ok(consumed) => {
            // SAFETY: the count slot is writable, and `FifoConsumed`'s brand
            // ties this count to the segment built just above, whose checked
            // maximum is `maximum`. C therefore never sees more elements than
            // it offered.
            unsafe { *elements = consumed.0 };
            0
        }
        Err(error) => error,
    }
}

/// Wraps: av_fifo_write_from_cb
pub fn av_fifo_write_from_cb<C: AVFifoReadCallback>(
    fifo: &mut AVFifoMut<'_>,
    callback: &mut C,
    maximum_elements: usize,
) -> Result<usize, i32> {
    let mut elements = maximum_elements;
    let mut state = ReadCallbackState {
        callback,
        element_size: av_fifo_elem_size(fifo.as_ref()),
    };
    // SAFETY: the erased pointer identifies `state` for synchronous callbacks;
    // the trampoline initializes only complete elements and bounds its count.
    let status = unsafe {
        ffi::av_fifo_write_from_cb(
            fifo.as_mut_ptr(),
            Some(read_callback::<C>),
            (&raw mut state).cast(),
            &raw mut elements,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        Ok(elements)
    }
}

/// Wraps: av_fifo_read_to_cb
pub fn av_fifo_read_to_cb<C: AVFifoWriteCallback>(
    fifo: &mut AVFifoMut<'_>,
    callback: &mut C,
    maximum_elements: usize,
) -> Result<usize, i32> {
    let mut elements = maximum_elements;
    let mut state = WriteCallbackState {
        callback,
        element_size: av_fifo_elem_size(fifo.as_ref()),
    };
    // SAFETY: the state remains live for every synchronous callback and the
    // trampoline cannot report more than the supplied segment.
    let status = unsafe {
        ffi::av_fifo_read_to_cb(
            fifo.as_mut_ptr(),
            Some(write_callback::<C>),
            (&raw mut state).cast(),
            &raw mut elements,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        Ok(elements)
    }
}

/// Wraps: av_fifo_peek_to_cb
pub fn av_fifo_peek_to_cb<C: AVFifoWriteCallback>(
    fifo: AVFifoRef<'_>,
    callback: &mut C,
    maximum_elements: usize,
    offset: usize,
) -> Result<usize, i32> {
    let mut elements = maximum_elements;
    let mut state = WriteCallbackState {
        callback,
        element_size: av_fifo_elem_size(fifo),
    };
    // SAFETY: the state remains live for every synchronous callback; peeking
    // only reads FIFO storage and callback buffers are exposed shared.
    let status = unsafe {
        ffi::av_fifo_peek_to_cb(
            fifo.as_ptr(),
            Some(write_callback::<C>),
            (&raw mut state).cast(),
            &raw mut elements,
            offset,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        Ok(elements)
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    struct Producer {
        next: u8,
    }

    impl AVFifoReadCallback for Producer {
        fn read(&mut self, destination: &mut FifoWriteBuffer<'_>) -> Result<(), i32> {
            while destination.remaining_elements() != 0 {
                let value = self.next;
                self.next += 1;
                assert!(destination.write_next(&[value]));
            }
            Ok(())
        }
    }

    #[derive(Default)]
    struct Consumer {
        bytes: [u8; 8],
        length: usize,
    }

    impl AVFifoWriteCallback for Consumer {
        fn write<'a>(&mut self, source: FifoReadBuffer<'a>) -> Result<FifoConsumed<'a>, i32> {
            let count = source.elements().len();
            for element in source.elements() {
                let end = self.length + element.len();
                self.bytes[self.length..end].copy_from_slice(element);
                self.length = end;
            }
            Ok(source.consume(count).unwrap())
        }
    }

    #[test]
    fn direct_and_callback_operations_preserve_elements() {
        let mut fifo = av_fifo_alloc2(2, 1, 1).unwrap();
        av_fifo_auto_grow_limit(&mut fifo.as_mut(), 16);
        av_fifo_write(&mut fifo.as_mut(), &[1, 2]).unwrap();
        av_fifo_grow2(&mut fifo.as_mut(), 2).unwrap();
        assert_eq!(av_fifo_can_read(fifo.as_ref()), 2);
        let mut peeked = [0; 2];
        av_fifo_peek(fifo.as_ref(), &mut peeked, 0).unwrap();
        assert_eq!(peeked, [1, 2]);

        let mut producer = Producer { next: 3 };
        assert_eq!(
            av_fifo_write_from_cb(&mut fifo.as_mut(), &mut producer, 2),
            Ok(2)
        );
        let mut consumer = Consumer::default();
        assert_eq!(
            av_fifo_peek_to_cb(fifo.as_ref(), &mut consumer, 4, 0),
            Ok(4)
        );
        assert_eq!(&consumer.bytes[..consumer.length], [1, 2, 3, 4]);

        let mut output = [0; 2];
        av_fifo_read(&mut fifo.as_mut(), &mut output).unwrap();
        assert_eq!(output, [1, 2]);
        let mut consumer = Consumer::default();
        assert_eq!(
            av_fifo_read_to_cb(&mut fifo.as_mut(), &mut consumer, 2),
            Ok(2)
        );
        assert_eq!(&consumer.bytes[..consumer.length], [3, 4]);
        av_fifo_reset2(&mut fifo.as_mut());
        assert_eq!(av_fifo_can_read(fifo.as_ref()), 0);
    }

    #[test]
    fn empty_peek_still_reports_an_out_of_range_offset() {
        let mut fifo = av_fifo_alloc2(4, 1, 0).unwrap();
        av_fifo_write(&mut fifo.as_mut(), &[1, 2]).unwrap();

        // C validates `offset` against the readable element count before it
        // looks at the request size, so a zero-element peek is not a no-op.
        assert_eq!(av_fifo_peek(fifo.as_ref(), &mut [], 2), Ok(()));
        assert_eq!(av_fifo_peek(fifo.as_ref(), &mut [], 3), Err(-22));
    }

    /// A consumer that reports every element it was shown, which is the
    /// largest count `FifoConsumed`'s brand allows it to report.
    struct GreedyConsumer;

    impl AVFifoWriteCallback for GreedyConsumer {
        fn write<'a>(&mut self, source: FifoReadBuffer<'a>) -> Result<FifoConsumed<'a>, i32> {
            let count = source.elements().len();
            Ok(source
                .consume(count)
                .expect("the whole segment is consumable"))
        }
    }

    #[test]
    fn a_consumer_cannot_claim_more_than_the_segment_it_was_shown() {
        let mut fifo = av_fifo_alloc2(8, 1, 0).unwrap();
        av_fifo_write(&mut fifo.as_mut(), &[1, 2, 3, 4, 5, 6]).unwrap();

        // The first call sees six elements, the second only one. A count
        // carried over from the first would underflow C's remaining count;
        // the brand makes carrying one over a compile error, so each call
        // reports at most what it was offered.
        assert_eq!(
            av_fifo_read_to_cb(&mut fifo.as_mut(), &mut GreedyConsumer, 6),
            Ok(6)
        );
        av_fifo_write(&mut fifo.as_mut(), &[7]).unwrap();
        assert_eq!(
            av_fifo_read_to_cb(&mut fifo.as_mut(), &mut GreedyConsumer, 1),
            Ok(1)
        );
        assert_eq!(av_fifo_can_read(fifo.as_ref()), 0);
    }
}
