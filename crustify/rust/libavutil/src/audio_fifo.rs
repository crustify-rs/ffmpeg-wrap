//! Wrappers for libavutil audio FIFOs.

use core::ptr::NonNull;

use ffibox::{CDropped, define_ctype};

use crate::ffi;

define_ctype!(
    /// Wraps: AVAudioFifo
    ///
    /// An opaque audio sample FIFO. Owning pointers use
    /// [`CBox<AVAudioFifo>`](ffibox::CBox); shared and exclusive borrows use
    /// [`AVAudioFifoRef`] and [`AVAudioFifoMut`] without ever forming a Rust
    /// reference over storage that libavutil may mutate.
    AVAudioFifo,
    AVAudioFifoRef,
    AVAudioFifoMut,
    ffi::AVAudioFifo
);

// SAFETY: a fully constructed `AVAudioFifo` is uniquely released by
// `av_audio_fifo_free`, including all AVFifo elements and the pointer table it
// owns. `CBox` calls this operation exactly once for each adopted pointer.
unsafe impl CDropped for AVAudioFifo {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract transfers a live, uniquely owned,
        // fully constructed `AVAudioFifo` to its matching public destructor.
        unsafe { ffi::av_audio_fifo_free(obj.as_ptr().cast()) }
    }
}

#[cfg(test)]
mod tests {
    use ffibox::CBox;

    use super::*;

    #[test]
    fn owned_fifo_produces_shared_and_exclusive_handles_and_drops() {
        // SAFETY: these arguments describe two channels of interleaved signed
        // 16-bit audio and a positive initial capacity. The returned pointer
        // is NULL or a fresh, fully constructed FIFO owned by the caller.
        let raw = unsafe { ffi::av_audio_fifo_alloc(ffi::AVSampleFormat_AV_SAMPLE_FMT_S16, 2, 8) };
        // SAFETY: `raw` is the fresh allocation just returned above and has
        // not been adopted or freed. `CBox` uses its matching destructor.
        let mut fifo =
            unsafe { CBox::<AVAudioFifo>::from_raw(raw) }.expect("av_audio_fifo_alloc failed");

        let shared = fifo.as_ref();
        assert_eq!(shared.as_ptr(), raw.cast_const());

        let mut exclusive = fifo.as_mut();
        assert_eq!(exclusive.as_mut_ptr(), raw);
        assert_eq!(exclusive.as_ref().as_ptr(), raw.cast_const());

        drop(fifo);
    }
}
