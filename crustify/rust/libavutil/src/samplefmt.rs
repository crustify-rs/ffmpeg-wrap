//! Wrappers for libavutil sample formats.

use crate::ffi;

/// Wraps: AVSampleFormat
///
/// ABI-compatible audio sample-format value. This is a transparent integer
/// newtype rather than a Rust enum because an application may link to a newer
/// libavutil that returns a format unknown to this crate. Such values remain
/// valid Rust values and round-trip through [`from_raw`](Self::from_raw) and
/// [`as_raw`](Self::as_raw).
///
/// [`from_raw`](Self::from_raw) is safe for every `c_int` because libavutil
/// treats the enum as an open integer: each entry point taking a sample
/// format range-checks it against `AV_SAMPLE_FMT_NB` before touching the
/// format table, and reports "unknown" (a null name, zero bytes per sample,
/// non-planar, [`NONE`](Self::NONE), or `EINVAL`) instead of indexing it.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVSampleFormat(ffi::AVSampleFormat);

impl AVSampleFormat {
    /// No recognized sample format.
    pub const NONE: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_NONE);
    /// Unsigned 8-bit packed samples.
    pub const U8: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_U8);
    /// Signed 16-bit packed samples.
    pub const S16: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S16);
    /// Signed 32-bit packed samples.
    pub const S32: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S32);
    /// 32-bit floating-point packed samples.
    pub const FLT: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_FLT);
    /// 64-bit floating-point packed samples.
    pub const DBL: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_DBL);
    /// Unsigned 8-bit planar samples.
    pub const U8P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_U8P);
    /// Signed 16-bit planar samples.
    pub const S16P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S16P);
    /// Signed 32-bit planar samples.
    pub const S32P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S32P);
    /// 32-bit floating-point planar samples.
    pub const FLTP: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_FLTP);
    /// 64-bit floating-point planar samples.
    pub const DBLP: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_DBLP);
    /// Signed 64-bit packed samples.
    pub const S64: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S64);
    /// Signed 64-bit planar samples.
    pub const S64P: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_S64P);
    /// Number of sample formats in the headers used to build this crate.
    ///
    /// This sentinel is not stable when dynamically linking libavutil.
    pub const NB: Self = Self(ffi::AVSampleFormat_AV_SAMPLE_FMT_NB);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVSampleFormat) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVSampleFormat {
        self.0
    }
}

impl From<ffi::AVSampleFormat> for AVSampleFormat {
    fn from(raw: ffi::AVSampleFormat) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVSampleFormat> for ffi::AVSampleFormat {
    fn from(format: AVSampleFormat) -> Self {
        format.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::{AVSampleFormat as Fmt, *};

    /// Every format this crate declares, paired with what libavutil itself
    /// reports for it: `(packed, planar, packed name, planar name, bytes per
    /// sample)`. Checking a constant against the library rather than against
    /// the same generated binding is what catches one bound to the wrong
    /// enumerator, which an `X == ffi::X` assertion cannot.
    const PAIRS: [(Fmt, Fmt, &CStr, &CStr, i32); 6] = [
        (Fmt::U8, Fmt::U8P, c"u8", c"u8p", 1),
        (Fmt::S16, Fmt::S16P, c"s16", c"s16p", 2),
        (Fmt::S32, Fmt::S32P, c"s32", c"s32p", 4),
        (Fmt::FLT, Fmt::FLTP, c"flt", c"fltp", 4),
        (Fmt::DBL, Fmt::DBLP, c"dbl", c"dblp", 8),
        (Fmt::S64, Fmt::S64P, c"s64", c"s64p", 8),
    ];

    #[test]
    fn each_constant_denotes_the_format_its_name_claims() {
        for (packed, planar, packed_name, planar_name, bytes) in PAIRS {
            assert_eq!(av_get_sample_fmt_name(packed), Some(packed_name));
            assert_eq!(av_get_sample_fmt_name(planar), Some(planar_name));
            assert_eq!(av_get_sample_fmt(packed_name), packed);
            assert_eq!(av_get_sample_fmt(planar_name), planar);
            assert_eq!(av_get_bytes_per_sample(packed), bytes);
            assert_eq!(av_get_bytes_per_sample(planar), bytes);
            assert!(!av_sample_fmt_is_planar(packed));
            assert!(av_sample_fmt_is_planar(planar));
            assert_eq!(av_get_planar_sample_fmt(packed), planar);
            assert_eq!(av_get_packed_sample_fmt(planar), packed);
        }
    }

    #[test]
    fn sentinels_sit_outside_the_named_format_table() {
        assert_eq!(av_get_sample_fmt_name(AVSampleFormat::NONE), None);
        assert_eq!(av_get_bytes_per_sample(AVSampleFormat::NONE), 0);
        assert_eq!(
            av_get_sample_fmt(c"not-a-sample-format"),
            AVSampleFormat::NONE
        );

        // `NB` counts the formats the headers of this build declare, so it is
        // one past the last named one and is itself unnamed.
        assert_eq!(av_get_sample_fmt_name(AVSampleFormat::NB), None);
        assert_eq!(
            AVSampleFormat::NB.as_raw(),
            AVSampleFormat::S64P.as_raw() + 1
        );
        assert_eq!(
            usize::try_from(AVSampleFormat::NB.as_raw()).unwrap(),
            2 * PAIRS.len()
        );
    }

    #[test]
    fn layout_matches_raw_enum_and_unknown_values_round_trip() {
        assert_eq!(
            size_of::<AVSampleFormat>(),
            size_of::<ffi::AVSampleFormat>()
        );
        assert_eq!(
            align_of::<AVSampleFormat>(),
            align_of::<ffi::AVSampleFormat>()
        );

        // A value a newer libavutil could return, and the far ends of the C
        // integer range: all stay valid Rust values, round-trip unchanged, and
        // are answered rather than indexed by the library.
        for raw in [
            ffi::AVSampleFormat_AV_SAMPLE_FMT_NB + 1,
            ffi::AVSampleFormat::MAX,
            ffi::AVSampleFormat::MIN,
        ] {
            let unknown = AVSampleFormat::from_raw(raw);
            assert_eq!(unknown.as_raw(), raw);
            assert_eq!(AVSampleFormat::from(raw), unknown);
            assert_eq!(ffi::AVSampleFormat::from(unknown), raw);

            assert_eq!(av_get_sample_fmt_name(unknown), None);
            assert_eq!(av_get_bytes_per_sample(unknown), 0);
            assert!(!av_sample_fmt_is_planar(unknown));
            assert_eq!(av_get_packed_sample_fmt(unknown), AVSampleFormat::NONE);
            assert_eq!(av_get_planar_sample_fmt(unknown), AVSampleFormat::NONE);
        }
    }
}

use crate::mem::AvFree;
use core::marker::PhantomData;
use ffibox::CVec;

const MAX_PLANES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SamplesError {
    TooManyChannels,
    RangeOverflow,
    Library(i32),
}

fn sample_size(status: i32) -> Result<usize, SamplesError> {
    if status < 0 {
        Err(SamplesError::Library(status))
    } else {
        Ok(status as usize)
    }
}

/// Calculated byte extent and per-plane stride for an audio layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SamplesLayout {
    pub size: usize,
    pub linesize: i32,
}

/// Non-owning plane table tied to one contiguous sample buffer.
pub struct AudioPlanes<'a> {
    pointers: [*mut u8; MAX_PLANES],
    plane_count: usize,
    linesize: i32,
    _buffer: PhantomData<&'a [u8]>,
}
impl AudioPlanes<'_> {
    #[must_use]
    pub fn plane_count(&self) -> usize {
        self.plane_count
    }
    #[must_use]
    pub fn linesize(&self) -> i32 {
        self.linesize
    }
}

struct AudioPlanesMut<'a> {
    pointers: [*mut u8; MAX_PLANES],
    _buffer: PhantomData<&'a mut [u8]>,
}

/// An av_malloc-owned, initialized sample buffer.
pub struct SamplesBuffer {
    storage: CVec<u8, AvFree>,
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
    linesize: i32,
}
impl SamplesBuffer {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.storage.as_slice()
    }
    #[must_use]
    pub fn channels(&self) -> i32 {
        self.channels
    }
    #[must_use]
    pub fn samples(&self) -> i32 {
        self.samples
    }
    #[must_use]
    pub fn format(&self) -> AVSampleFormat {
        self.format
    }
    #[must_use]
    pub fn linesize(&self) -> i32 {
        self.linesize
    }
}

fn plane_slots(channels: i32, format: AVSampleFormat) -> Result<usize, SamplesError> {
    if channels < 0 {
        return Err(SamplesError::Library(-22));
    }
    let count = if av_sample_fmt_is_planar(format) {
        channels as usize
    } else {
        1
    };
    if count > MAX_PLANES {
        Err(SamplesError::TooManyChannels)
    } else {
        Ok(count)
    }
}

/// Wraps: av_sample_fmt_is_planar
#[must_use]
pub fn av_sample_fmt_is_planar(format: AVSampleFormat) -> bool {
    // SAFETY: the open integer value is accepted by C and no state is retained.
    unsafe { ffi::av_sample_fmt_is_planar(format.as_raw()) != 0 }
}

/// Wraps: av_samples_get_buffer_size
pub fn av_samples_get_buffer_size(
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
) -> Result<SamplesLayout, SamplesError> {
    let mut linesize = 0;
    // SAFETY: the out-slot is writable and all other arguments are values.
    let size = sample_size(unsafe {
        ffi::av_samples_get_buffer_size(
            &raw mut linesize,
            channels,
            samples,
            format.as_raw(),
            align,
        )
    })?;
    Ok(SamplesLayout { size, linesize })
}

/// Wraps: av_samples_fill_arrays
pub fn av_samples_fill_arrays<'a>(
    buffer: &'a [u8],
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
) -> Result<AudioPlanes<'a>, SamplesError> {
    let plane_count = plane_slots(channels, format)?;
    let layout = av_samples_get_buffer_size(channels, samples, format, align)?;
    if layout.size > buffer.len() {
        return Err(SamplesError::Library(-22));
    }
    let mut pointers = [core::ptr::null_mut(); MAX_PLANES];
    let mut linesize = 0;
    // SAFETY: the pointer table has enough slots for every plane. C computes
    // pointers but does not read the buffer, and the result remains private.
    let required = sample_size(unsafe {
        ffi::av_samples_fill_arrays(
            pointers.as_mut_ptr(),
            &raw mut linesize,
            buffer.as_ptr(),
            channels,
            samples,
            format.as_raw(),
            align,
        )
    })?;
    debug_assert_eq!(required, layout.size);
    Ok(AudioPlanes {
        pointers,
        plane_count,
        linesize,
        _buffer: PhantomData,
    })
}

fn make_buffer(
    pointer: *mut u8,
    size: usize,
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
    linesize: i32,
) -> Result<SamplesBuffer, SamplesError> {
    // SAFETY: successful allocation routines return one initialized contiguous
    // av_malloc-family block containing exactly `size` bytes.
    let storage = unsafe { CVec::<u8, AvFree>::from_raw_parts(pointer, size) }
        .ok_or(SamplesError::Library(-12))?;
    Ok(SamplesBuffer {
        storage,
        channels,
        samples,
        format,
        align,
        linesize,
    })
}

/// Wraps: av_samples_alloc
pub fn av_samples_alloc(
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
) -> Result<SamplesBuffer, SamplesError> {
    plane_slots(channels, format)?;
    let mut pointers = [core::ptr::null_mut(); MAX_PLANES];
    let mut linesize = 0;
    // SAFETY: the pointer table has the required channel slots and the line
    // size out-slot is writable. On success ownership of pointers[0] transfers.
    let size = sample_size(unsafe {
        ffi::av_samples_alloc(
            pointers.as_mut_ptr(),
            &raw mut linesize,
            channels,
            samples,
            format.as_raw(),
            align,
        )
    })?;
    make_buffer(
        pointers[0],
        size,
        channels,
        samples,
        format,
        align,
        linesize,
    )
}

/// Wraps: av_samples_alloc_array_and_samples
pub fn av_samples_alloc_array_and_samples(
    channels: i32,
    samples: i32,
    format: AVSampleFormat,
    align: i32,
) -> Result<SamplesBuffer, SamplesError> {
    plane_slots(channels, format)?;
    let mut table: *mut *mut u8 = core::ptr::null_mut();
    let mut linesize = 0;
    // SAFETY: both out-slots are writable. On success C returns two distinct
    // av_malloc-family allocations: the table and its first data pointer.
    let size = sample_size(unsafe {
        ffi::av_samples_alloc_array_and_samples(
            &raw mut table,
            &raw mut linesize,
            channels,
            samples,
            format.as_raw(),
            align,
        )
    })?;
    // SAFETY: success guarantees a non-null table with at least one initialized
    // pointer. Reading it does not form a reference to C-owned storage.
    let data = unsafe { table.read() };
    // SAFETY: the table allocation is no longer needed and av_free matches it;
    // the separate sample allocation remains owned through `data`.
    unsafe { ffi::av_free(table.cast()) };
    make_buffer(data, size, channels, samples, format, align, linesize)
}

fn buffer_planes(buffer: &SamplesBuffer) -> Result<AudioPlanes<'_>, SamplesError> {
    av_samples_fill_arrays(
        buffer.storage.as_slice(),
        buffer.channels,
        buffer.samples,
        buffer.format,
        buffer.align,
    )
}

fn buffer_planes_mut(buffer: &mut SamplesBuffer) -> Result<AudioPlanesMut<'_>, SamplesError> {
    let plane_count = plane_slots(buffer.channels, buffer.format)?;
    let layout =
        av_samples_get_buffer_size(buffer.channels, buffer.samples, buffer.format, buffer.align)?;
    if layout.size > buffer.storage.count() {
        return Err(SamplesError::Library(-22));
    }
    let mut pointers = [core::ptr::null_mut(); MAX_PLANES];
    let mut linesize = 0;
    let bytes = buffer.storage.as_mut_slice();
    // SAFETY: the table has `plane_count` slots (bounded above by its capacity)
    // and is derived from the exclusive byte borrow retained by the result.
    let required = sample_size(unsafe {
        ffi::av_samples_fill_arrays(
            pointers.as_mut_ptr(),
            &raw mut linesize,
            bytes.as_mut_ptr(),
            buffer.channels,
            buffer.samples,
            buffer.format.as_raw(),
            buffer.align,
        )
    })?;
    if required > bytes.len() {
        return Err(SamplesError::Library(-22));
    }
    let _ = plane_count;
    Ok(AudioPlanesMut {
        pointers,
        _buffer: PhantomData,
    })
}

/// Wraps: av_samples_copy
pub fn av_samples_copy(
    destination: &mut SamplesBuffer,
    source: &SamplesBuffer,
    destination_offset: i32,
    source_offset: i32,
    samples: i32,
) -> Result<(), SamplesError> {
    if destination.channels != source.channels
        || destination.format != source.format
        || destination_offset < 0
        || source_offset < 0
        || samples < 0
        || destination_offset
            .checked_add(samples)
            .is_none_or(|n| n > destination.samples)
        || source_offset
            .checked_add(samples)
            .is_none_or(|n| n > source.samples)
    {
        return Err(SamplesError::RangeOverflow);
    }
    let src = buffer_planes(source)?;
    let dst = buffer_planes_mut(destination)?;
    // SAFETY: both private pointer tables were derived from their live buffers;
    // range checks above keep every copied sample inside both allocations.
    let status = unsafe {
        ffi::av_samples_copy(
            dst.pointers.as_ptr(),
            src.pointers.as_ptr(),
            destination_offset,
            source_offset,
            samples,
            destination.channels,
            destination.format.as_raw(),
        )
    };
    if status < 0 {
        Err(SamplesError::Library(status))
    } else {
        Ok(())
    }
}

/// Wraps: av_samples_set_silence
pub fn av_samples_set_silence(
    buffer: &mut SamplesBuffer,
    offset: i32,
    samples: i32,
) -> Result<(), SamplesError> {
    if offset < 0
        || samples < 0
        || offset
            .checked_add(samples)
            .is_none_or(|n| n > buffer.samples)
    {
        return Err(SamplesError::RangeOverflow);
    }
    let planes = buffer_planes_mut(buffer)?;
    // SAFETY: the exclusive buffer borrow and checked range cover every write;
    // the pointer table was derived from this same live allocation.
    let status = unsafe {
        ffi::av_samples_set_silence(
            planes.pointers.as_ptr(),
            offset,
            samples,
            buffer.channels,
            buffer.format.as_raw(),
        )
    };
    if status < 0 {
        Err(SamplesError::Library(status))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod scheduled_tests {
    use super::*;

    #[test]
    fn allocated_samples_copy_and_silence_with_bounds() {
        let mut source = av_samples_alloc(2, 8, AVSampleFormat::U8, 1).unwrap();
        let mut destination =
            av_samples_alloc_array_and_samples(2, 8, AVSampleFormat::U8, 1).unwrap();
        assert_eq!(source.as_bytes().len(), 16);
        av_samples_set_silence(&mut source, 0, 8).unwrap();
        av_samples_copy(&mut destination, &source, 0, 0, 8).unwrap();
        assert_eq!(destination.as_bytes(), &[0x80; 16]);
        assert_eq!(
            av_samples_set_silence(&mut destination, 8, 1),
            Err(SamplesError::RangeOverflow)
        );
    }

    #[test]
    fn layout_and_plane_table_match() {
        let layout = av_samples_get_buffer_size(2, 4, AVSampleFormat::S16P, 1).unwrap();
        assert_eq!(layout.size, 16);
        let bytes = [0_u8; 16];
        let planes = av_samples_fill_arrays(&bytes, 2, 4, AVSampleFormat::S16P, 1).unwrap();
        assert_eq!(planes.plane_count(), 2);
        assert_eq!(planes.linesize(), 8);
    }
}

use core::ffi::CStr;

/// Wraps: av_get_bytes_per_sample
#[must_use]
pub fn av_get_bytes_per_sample(format: AVSampleFormat) -> i32 {
    // SAFETY: the ABI-compatible format value is passed by value.
    unsafe { ffi::av_get_bytes_per_sample(format.as_raw()) }
}

/// Wraps: av_get_packed_sample_fmt
#[must_use]
pub fn av_get_packed_sample_fmt(format: AVSampleFormat) -> AVSampleFormat {
    // SAFETY: the format is passed and returned by value.
    AVSampleFormat::from_raw(unsafe { ffi::av_get_packed_sample_fmt(format.as_raw()) })
}

/// Wraps: av_get_planar_sample_fmt
#[must_use]
pub fn av_get_planar_sample_fmt(format: AVSampleFormat) -> AVSampleFormat {
    // SAFETY: the format is passed and returned by value.
    AVSampleFormat::from_raw(unsafe { ffi::av_get_planar_sample_fmt(format.as_raw()) })
}

/// Wraps: av_get_sample_fmt
#[must_use]
pub fn av_get_sample_fmt(name: &CStr) -> AVSampleFormat {
    // SAFETY: `name` is a readable NUL-terminated string and is not retained.
    AVSampleFormat::from_raw(unsafe { ffi::av_get_sample_fmt(name.as_ptr()) })
}

/// Wraps: av_get_sample_fmt_name
#[must_use]
pub fn av_get_sample_fmt_name(format: AVSampleFormat) -> Option<&'static CStr> {
    // SAFETY: C returns null or a pointer into its immutable process-lifetime
    // sample-format table.
    let pointer = unsafe { ffi::av_get_sample_fmt_name(format.as_raw()) };
    if pointer.is_null() {
        None
    } else {
        // SAFETY: the non-null result is a NUL-terminated static table entry.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn sample_format_queries_round_trip() {
        assert_eq!(av_get_bytes_per_sample(AVSampleFormat::S16), 2);
        assert_eq!(av_get_sample_fmt(c"s16"), AVSampleFormat::S16);
        assert_eq!(av_get_sample_fmt_name(AVSampleFormat::S16P), Some(c"s16p"));
        assert_eq!(
            av_get_planar_sample_fmt(AVSampleFormat::S16),
            AVSampleFormat::S16P
        );
        assert_eq!(
            av_get_packed_sample_fmt(AVSampleFormat::S16P),
            AVSampleFormat::S16
        );
    }
}
