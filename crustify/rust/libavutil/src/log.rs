//! Wrappers for libavutil logging.

use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::ffi;

/// The raw C logging callback shape. Its raw arguments are confined to the
/// callback boundary because a `va_list` cannot be represented as a safe Rust
/// iterator and the context's concrete AVClass-bearing type is erased.
pub type LogCallback = unsafe extern "C" fn(
    context: *mut c_void,
    level: c_int,
    format: *const c_char,
    arguments: *mut ffi::__va_list_tag,
);

/// Shared borrowed handle to an AVClass-bearing logging object whose concrete
/// wrapper has not yet been translated.
#[derive(Clone, Copy)]
pub struct LogContextRef<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

impl<'a> LogContextRef<'a> {
    /// Construct a temporary erased logging-context handle.
    ///
    /// # Safety
    ///
    /// `pointer` must remain live and unmodified for `'a` and identify an
    /// object whose first field is a valid `AVClass *`.
    pub unsafe fn from_raw(pointer: NonNull<c_void>) -> Self {
        Self {
            pointer,
            _borrow: PhantomData,
        }
    }

    pub(crate) fn as_ptr(self) -> *mut c_void {
        self.pointer.as_ptr()
    }
}

/// Wraps: av_log_get_flags
#[must_use]
pub fn av_log_get_flags() -> i32 {
    // SAFETY: the C implementation performs an atomic load and has no caller
    // obligations.
    unsafe { ffi::av_log_get_flags() }
}

/// Wraps: av_log_get_level
#[must_use]
pub fn av_log_get_level() -> i32 {
    // SAFETY: the C implementation performs an atomic load and has no caller
    // obligations.
    unsafe { ffi::av_log_get_level() }
}

/// Wraps: av_log_set_callback
///
/// # Safety
///
/// An installed callback is retained process-globally, may be called from any
/// thread, and must remain valid and thread-safe until replaced. Every callback
/// invocation must also honor the raw context, format and `va_list` contracts.
pub unsafe fn av_log_set_callback(callback: Option<LogCallback>) {
    // SAFETY: the caller accepts the global lifetime and concurrency contract;
    // the function atomically stores the supplied code pointer.
    unsafe { ffi::av_log_set_callback(callback) }
}

/// Wraps: av_log_set_flags
pub fn av_log_set_flags(flags: i32) {
    // SAFETY: the C implementation performs an atomic store and takes no
    // pointers.
    unsafe { ffi::av_log_set_flags(flags) }
}

/// Wraps: av_log_set_level
pub fn av_log_set_level(level: i32) {
    // SAFETY: the C implementation performs an atomic store and takes no
    // pointers.
    unsafe { ffi::av_log_set_level(level) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_level_and_flags() {
        let old_level = av_log_get_level();
        let old_flags = av_log_get_flags();
        av_log_set_level(17);
        av_log_set_flags(23);
        assert_eq!(av_log_get_level(), 17);
        assert_eq!(av_log_get_flags(), 23);
        av_log_set_level(old_level);
        av_log_set_flags(old_flags);
    }
}

/// Wraps: AVClassCategory
///
/// Classifies the component represented by an [`AVClass`](crate::ffi::AVClass).
/// The transparent integer representation preserves values introduced by a
/// newer libavutil instead of creating an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVClassCategory(ffi::AVClassCategory);

impl AVClassCategory {
    pub const NA: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_NA);
    pub const INPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_INPUT);
    pub const OUTPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_OUTPUT);
    pub const MUXER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_MUXER);
    pub const DEMUXER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEMUXER);
    pub const ENCODER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_ENCODER);
    pub const DECODER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DECODER);
    pub const FILTER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_FILTER);
    pub const BITSTREAM_FILTER: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_BITSTREAM_FILTER);
    pub const SWSCALER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_SWSCALER);
    pub const SWRESAMPLER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_SWRESAMPLER);
    pub const HWDEVICE: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_HWDEVICE);
    pub const DEVICE_VIDEO_OUTPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT);
    pub const DEVICE_VIDEO_INPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT);
    pub const DEVICE_AUDIO_OUTPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT);
    pub const DEVICE_AUDIO_INPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT);
    pub const DEVICE_OUTPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_OUTPUT);
    pub const DEVICE_INPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_INPUT);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVClassCategory) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVClassCategory {
        self.0
    }
}

impl From<ffi::AVClassCategory> for AVClassCategory {
    fn from(raw: ffi::AVClassCategory) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVClassCategory> for ffi::AVClassCategory {
    fn from(category: AVClassCategory) -> Self {
        category.as_raw()
    }
}

#[cfg(test)]
mod category_tests {
    use super::*;

    #[test]
    fn known_and_future_categories_round_trip() {
        assert_eq!(AVClassCategory::DEVICE_INPUT.as_raw(), 45);
        let future = AVClassCategory::from_raw(10_000);
        assert_eq!(future.as_raw(), 10_000);
    }
}
