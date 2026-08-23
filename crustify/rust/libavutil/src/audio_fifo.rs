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

/// Wraps: av_audio_fifo_free
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

extern crate alloc;

use alloc::vec::Vec;
use core::ffi::c_void;

use ffibox::CBox;

use crate::samplefmt::{AVSampleFormat, av_get_bytes_per_sample, av_get_planar_sample_fmt};

/// An owning audio FIFO together with the layout facts needed to validate safe
/// sample-buffer slices before they cross the C boundary.
pub struct AudioFifo {
    inner: CBox<AVAudioFifo>,
    buffers: usize,
    bytes_per_buffer_sample: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFifoError {
    CountOverflow,
    BufferLayout,
    Ffmpeg(i32),
}

fn sample_count(count: usize) -> Result<i32, AudioFifoError> {
    i32::try_from(count).map_err(|_| AudioFifoError::CountOverflow)
}

fn result_count(status: i32) -> Result<usize, AudioFifoError> {
    if status < 0 {
        Err(AudioFifoError::Ffmpeg(status))
    } else {
        Ok(status as usize)
    }
}

impl AudioFifo {
    fn required_bytes(&self, samples: usize) -> Result<usize, AudioFifoError> {
        self.bytes_per_buffer_sample
            .checked_mul(samples)
            .ok_or(AudioFifoError::CountOverflow)
    }

    fn validate_input(&self, data: &[&[u8]], samples: usize) -> Result<(), AudioFifoError> {
        let bytes = self.required_bytes(samples)?;
        if data.len() != self.buffers || data.iter().any(|buffer| buffer.len() < bytes) {
            Err(AudioFifoError::BufferLayout)
        } else {
            Ok(())
        }
    }

    fn validate_output(&self, data: &[&mut [u8]], samples: usize) -> Result<(), AudioFifoError> {
        let bytes = self.required_bytes(samples)?;
        if data.len() != self.buffers || data.iter().any(|buffer| buffer.len() < bytes) {
            Err(AudioFifoError::BufferLayout)
        } else {
            Ok(())
        }
    }
}

/// Wraps: av_audio_fifo_alloc
#[must_use]
pub fn av_audio_fifo_alloc(
    format: AVSampleFormat,
    channels: usize,
    initial_samples: usize,
) -> Option<AudioFifo> {
    let channels_i32 = i32::try_from(channels).ok()?;
    let samples_i32 = i32::try_from(initial_samples).ok()?;
    // SAFETY: the arguments are values. A non-null result is a fresh fully
    // constructed FIFO transferred to the caller.
    let raw = unsafe { ffi::av_audio_fifo_alloc(format.as_raw(), channels_i32, samples_i32) };
    // SAFETY: `raw` is null or the unique allocation just returned, whose
    // matching destructor is registered on `AVAudioFifo`.
    let inner = unsafe { CBox::from_raw(raw) }?;

    let bytes = usize::try_from(av_get_bytes_per_sample(format)).ok()?;
    if bytes == 0 {
        return None;
    }
    let planar = av_get_planar_sample_fmt(format) == format;
    Some(AudioFifo {
        inner,
        buffers: if planar { channels } else { 1 },
        bytes_per_buffer_sample: if planar {
            bytes
        } else {
            bytes.checked_mul(channels)?
        },
    })
}

/// Wraps: av_audio_fifo_drain
pub fn av_audio_fifo_drain(fifo: &mut AudioFifo, samples: usize) -> Result<(), AudioFifoError> {
    let samples = sample_count(samples)?;
    // SAFETY: the exclusive handle supplies a live FIFO and C retains nothing.
    let status = unsafe { ffi::av_audio_fifo_drain(fifo.inner.as_mut().as_mut_ptr(), samples) };
    result_count(status).map(|_| ())
}

/// Wraps: av_audio_fifo_peek
pub fn av_audio_fifo_peek(
    fifo: &AudioFifo,
    data: &mut [&mut [u8]],
    samples: usize,
) -> Result<usize, AudioFifoError> {
    fifo.validate_output(data, samples)?;
    let samples = sample_count(samples)?;
    let mut pointers: Vec<*mut c_void> = data
        .iter_mut()
        .map(|buffer| buffer.as_mut_ptr().cast())
        .collect();
    // SAFETY: validation proves the table has exactly the FIFO's buffer count
    // and every pointee has the byte extent C may write. The shared FIFO call
    // only observes FIFO state and retains no pointer.
    let status = unsafe {
        ffi::av_audio_fifo_peek(fifo.inner.as_ref().as_ptr(), pointers.as_mut_ptr(), samples)
    };
    result_count(status)
}

/// Wraps: av_audio_fifo_peek_at
pub fn av_audio_fifo_peek_at(
    fifo: &AudioFifo,
    data: &mut [&mut [u8]],
    samples: usize,
    offset: usize,
) -> Result<usize, AudioFifoError> {
    fifo.validate_output(data, samples)?;
    let samples = sample_count(samples)?;
    let offset = sample_count(offset)?;
    let mut pointers: Vec<*mut c_void> = data
        .iter_mut()
        .map(|buffer| buffer.as_mut_ptr().cast())
        .collect();
    // SAFETY: the validated output table remains live and exclusive for the
    // call; the FIFO is only read and C retains neither table nor buffers.
    let status = unsafe {
        ffi::av_audio_fifo_peek_at(
            fifo.inner.as_ref().as_ptr(),
            pointers.as_mut_ptr(),
            samples,
            offset,
        )
    };
    result_count(status)
}

/// Wraps: av_audio_fifo_read
pub fn av_audio_fifo_read(
    fifo: &mut AudioFifo,
    data: &mut [&mut [u8]],
    samples: usize,
) -> Result<usize, AudioFifoError> {
    fifo.validate_output(data, samples)?;
    let samples = sample_count(samples)?;
    let mut pointers: Vec<*mut c_void> = data
        .iter_mut()
        .map(|buffer| buffer.as_mut_ptr().cast())
        .collect();
    // SAFETY: validation proves all output extents; the exclusive FIFO borrow
    // covers C's state mutation and no pointer is retained.
    let status = unsafe {
        ffi::av_audio_fifo_read(
            fifo.inner.as_mut().as_mut_ptr(),
            pointers.as_mut_ptr(),
            samples,
        )
    };
    result_count(status)
}

/// Wraps: av_audio_fifo_realloc
pub fn av_audio_fifo_realloc(fifo: &mut AudioFifo, samples: usize) -> Result<(), AudioFifoError> {
    let samples = sample_count(samples)?;
    // SAFETY: the exclusive handle supplies the live FIFO C mutates in place.
    let status = unsafe { ffi::av_audio_fifo_realloc(fifo.inner.as_mut().as_mut_ptr(), samples) };
    result_count(status).map(|_| ())
}

/// Wraps: av_audio_fifo_reset
pub fn av_audio_fifo_reset(fifo: &mut AudioFifo) {
    // SAFETY: the exclusive handle supplies a live FIFO; C resets it in place.
    unsafe { ffi::av_audio_fifo_reset(fifo.inner.as_mut().as_mut_ptr()) }
}

/// Wraps: av_audio_fifo_size
#[must_use]
pub fn av_audio_fifo_size(fifo: &AudioFifo) -> usize {
    // SAFETY: source inspection establishes that this nominally mutable C
    // parameter is only read. The shared handle stays live for the call.
    let size = unsafe { ffi::av_audio_fifo_size(fifo.inner.as_ref().as_ptr().cast_mut()) };
    usize::try_from(size).expect("libavutil returned a negative FIFO size")
}

/// Wraps: av_audio_fifo_space
#[must_use]
pub fn av_audio_fifo_space(fifo: &AudioFifo) -> usize {
    // SAFETY: source inspection establishes that this nominally mutable C
    // parameter is only read. The shared handle stays live for the call.
    let space = unsafe { ffi::av_audio_fifo_space(fifo.inner.as_ref().as_ptr().cast_mut()) };
    usize::try_from(space).expect("libavutil returned negative FIFO space")
}

/// Wraps: av_audio_fifo_write
pub fn av_audio_fifo_write(
    fifo: &mut AudioFifo,
    data: &[&[u8]],
    samples: usize,
) -> Result<usize, AudioFifoError> {
    fifo.validate_input(data, samples)?;
    let samples = sample_count(samples)?;
    let mut pointers: Vec<*mut c_void> = data
        .iter()
        .map(|buffer| buffer.as_ptr().cast_mut().cast())
        .collect();
    // SAFETY: validation proves each source extent and table cardinality. C
    // only reads the sample bytes, mutates the exclusively borrowed FIFO, and
    // retains no pointer.
    let status = unsafe {
        ffi::av_audio_fifo_write(
            fifo.inner.as_mut().as_mut_ptr(),
            pointers.as_mut_ptr(),
            samples,
        )
    };
    result_count(status)
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn packed_fifo_round_trip_and_management() {
        let mut fifo = av_audio_fifo_alloc(AVSampleFormat::S16, 2, 2).expect("allocate FIFO");
        let packed = [1_u8, 0, 2, 0, 3, 0, 4, 0];
        assert_eq!(av_audio_fifo_write(&mut fifo, &[&packed], 2), Ok(2));
        assert_eq!(av_audio_fifo_size(&fifo), 2);

        let mut peeked = [0_u8; 8];
        assert_eq!(av_audio_fifo_peek(&fifo, &mut [&mut peeked], 2), Ok(2));
        assert_eq!(peeked, packed);

        let mut read = [0_u8; 8];
        assert_eq!(av_audio_fifo_read(&mut fifo, &mut [&mut read], 2), Ok(2));
        assert_eq!(read, packed);
        assert_eq!(av_audio_fifo_size(&fifo), 0);

        av_audio_fifo_realloc(&mut fifo, 8).unwrap();
        assert!(av_audio_fifo_space(&fifo) >= 8);
        av_audio_fifo_reset(&mut fifo);
        av_audio_fifo_drain(&mut fifo, 0).unwrap();
    }

    #[test]
    fn rejects_wrong_buffer_layout_before_ffi() {
        let mut fifo = av_audio_fifo_alloc(AVSampleFormat::S16P, 2, 2).unwrap();
        assert_eq!(
            av_audio_fifo_write(&mut fifo, &[&[0_u8; 4]], 2),
            Err(AudioFifoError::BufferLayout)
        );
    }
}
