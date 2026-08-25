//! Wrappers for `libavutil/common.h`.

use crate::ffi;

/// Wraps: av_ceil_log2_c
#[must_use]
pub fn av_ceil_log2_c(value: i32) -> i32 {
    // SAFETY: the shim takes and returns integers and the C implementation uses
    // unsigned arithmetic for the potentially wrapping intermediate value.
    unsafe { ffi::crustify_av_ceil_log2_c(value) }
}

/// Wraps: av_clip64_c
#[must_use]
pub fn av_clip64_c(value: i64, min: i64, max: i64) -> Option<i64> {
    (min <= max).then(|| {
        // SAFETY: the checked bounds satisfy the inline helper's precondition.
        unsafe { ffi::crustify_av_clip64_c(value, min, max) }
    })
}

/// Wraps: av_clip_c
#[must_use]
pub fn av_clip_c(value: i32, min: i32, max: i32) -> Option<i32> {
    (min <= max).then(|| {
        // SAFETY: the checked bounds satisfy the inline helper's precondition.
        unsafe { ffi::crustify_av_clip_c(value, min, max) }
    })
}

/// Wraps: av_clip_int16_c
#[must_use]
pub fn av_clip_int16_c(value: i32) -> i16 {
    // SAFETY: every `int` is accepted by the header helper.
    unsafe { ffi::crustify_av_clip_int16_c(value) }
}

/// Wraps: av_clip_int8_c
#[must_use]
pub fn av_clip_int8_c(value: i32) -> i8 {
    // SAFETY: every `int` is accepted by the header helper.
    unsafe { ffi::crustify_av_clip_int8_c(value) }
}

/// Wraps: av_clip_intp2_c
#[must_use]
pub fn av_clip_intp2_c(value: i32, bits: u32) -> Option<i32> {
    (bits <= 30).then(|| {
        // SAFETY: restricting the shift to 0..=30 keeps each signed and
        // unsigned left shift in the C helper defined.
        unsafe { ffi::crustify_av_clip_intp2_c(value, bits as i32) }
    })
}

/// Wraps: av_clip_uint16_c
#[must_use]
pub fn av_clip_uint16_c(value: i32) -> u16 {
    // SAFETY: every `int` is accepted by the header helper.
    unsafe { ffi::crustify_av_clip_uint16_c(value) }
}

/// Wraps: av_clip_uint8_c
#[must_use]
pub fn av_clip_uint8_c(value: i32) -> u8 {
    // SAFETY: every `int` is accepted by the header helper.
    unsafe { ffi::crustify_av_clip_uint8_c(value) }
}

/// Wraps: av_clip_uintp2_c
#[must_use]
pub fn av_clip_uintp2_c(value: i32, bits: u32) -> Option<u32> {
    (bits <= 31).then(|| {
        // SAFETY: the checked shift is within the width of C `unsigned`.
        unsafe { ffi::crustify_av_clip_uintp2_c(value, bits as i32) }
    })
}

/// Wraps: av_clipd_c
#[must_use]
pub fn av_clipd_c(value: f64, min: f64, max: f64) -> Option<f64> {
    (min <= max).then(|| {
        // SAFETY: the checked, ordered bounds satisfy the helper contract.
        unsafe { ffi::crustify_av_clipd_c(value, min, max) }
    })
}

/// Wraps: av_clipf_c
#[must_use]
pub fn av_clipf_c(value: f32, min: f32, max: f32) -> Option<f32> {
    (min <= max).then(|| {
        // SAFETY: the checked, ordered bounds satisfy the helper contract.
        unsafe { ffi::crustify_av_clipf_c(value, min, max) }
    })
}

/// Wraps: av_clipl_int32_c
#[must_use]
pub fn av_clipl_int32_c(value: i64) -> i32 {
    // SAFETY: every `int64_t` is accepted by the header helper.
    unsafe { ffi::crustify_av_clipl_int32_c(value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clips_integer_and_float_ranges() {
        assert_eq!(av_clip_c(12, 0, 10), Some(10));
        assert_eq!(av_clip64_c(-2, 0, 10), Some(0));
        assert_eq!(av_clip_c(1, 2, 0), None);
        assert_eq!(av_clip_uint8_c(300), 255);
        assert_eq!(av_clip_int8_c(-200), -128);
        assert_eq!(av_clipl_int32_c(i64::MAX), i32::MAX);
        assert_eq!(av_clipf_c(2.0, 0.0, 1.0), Some(1.0));
        assert_eq!(av_clipd_c(f64::NAN, 0.0, 1.0), Some(0.0));
    }

    #[test]
    fn checks_power_of_two_shift_domains() {
        assert_eq!(av_clip_intp2_c(100, 3), Some(7));
        assert_eq!(av_clip_intp2_c(1, 31), None);
        assert_eq!(av_clip_uintp2_c(-1, 8), Some(0));
        assert_eq!(av_clip_uintp2_c(1, 32), None);
        assert_eq!(av_ceil_log2_c(9), 4);
    }
}
