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

/// Wraps: av_parity_c
///
/// Returns the parity bit of `value`: one when it has an odd number of set
/// bits, zero otherwise.
#[must_use]
pub fn av_parity_c(value: u32) -> i32 {
    // SAFETY: the shim accepts every `uint32_t` and performs pure arithmetic.
    unsafe { ffi::crustify_av_parity_c(value) }
}

/// Wraps: av_popcount_c
#[must_use]
pub fn av_popcount_c(value: u32) -> i32 {
    // SAFETY: the shim accepts every `uint32_t` and performs pure arithmetic.
    unsafe { ffi::crustify_av_popcount_c(value) }
}

/// Wraps: av_popcount64_c
#[must_use]
pub fn av_popcount64_c(value: u64) -> i32 {
    // SAFETY: the shim accepts every `uint64_t` and performs pure arithmetic.
    unsafe { ffi::crustify_av_popcount64_c(value) }
}

/// Wraps: av_zero_extend_c
///
/// Keeps the low `bits` bits of `value`. `None` reports the one input the C
/// helper cannot accept: it forms `1U << bits`, so a shift count at or past
/// the width of C `unsigned` is undefined rather than a saturating no-op.
#[must_use]
pub fn av_zero_extend_c(value: u32, bits: u32) -> Option<u32> {
    (bits <= 31).then(|| {
        // SAFETY: the checked shift count is within the width of C `unsigned`,
        // which is the helper's documented 0..=31 precondition.
        unsafe { ffi::crustify_av_zero_extend_c(value, bits) }
    })
}

/// Wraps: av_sat_add32_c
#[must_use]
pub fn av_sat_add32_c(a: i32, b: i32) -> i32 {
    // SAFETY: the sum is formed in `int64_t`, so every `int` pair is accepted.
    unsafe { ffi::crustify_av_sat_add32_c(a, b) }
}

/// Wraps: av_sat_sub32_c
#[must_use]
pub fn av_sat_sub32_c(a: i32, b: i32) -> i32 {
    // SAFETY: the difference is formed in `int64_t`, so every `int` pair is
    // accepted.
    unsafe { ffi::crustify_av_sat_sub32_c(a, b) }
}

/// Wraps: av_sat_dadd32_c
///
/// Computes `sat(a + sat(2 * b))`, saturating at both stages.
#[must_use]
pub fn av_sat_dadd32_c(a: i32, b: i32) -> i32 {
    // SAFETY: both stages saturate through the 64-bit clip, so every `int`
    // pair is accepted.
    unsafe { ffi::crustify_av_sat_dadd32_c(a, b) }
}

/// Wraps: av_sat_dsub32_c
///
/// Computes `sat(a - sat(2 * b))`, saturating at both stages.
#[must_use]
pub fn av_sat_dsub32_c(a: i32, b: i32) -> i32 {
    // SAFETY: both stages saturate through the 64-bit clip, so every `int`
    // pair is accepted.
    unsafe { ffi::crustify_av_sat_dsub32_c(a, b) }
}

/// Wraps: av_sat_add64_c
#[must_use]
pub fn av_sat_add64_c(a: i64, b: i64) -> i64 {
    // SAFETY: the helper detects the overflow itself with a checked builtin,
    // so every `int64_t` pair is accepted.
    unsafe { ffi::crustify_av_sat_add64_c(a, b) }
}

/// Wraps: av_sat_sub64_c
#[must_use]
pub fn av_sat_sub64_c(a: i64, b: i64) -> i64 {
    // SAFETY: the helper detects the overflow itself with a checked builtin,
    // so every `int64_t` pair is accepted.
    unsafe { ffi::crustify_av_sat_sub64_c(a, b) }
}

#[cfg(test)]
mod bit_and_saturation_tests {
    use super::*;

    #[test]
    fn counts_bits_and_parity() {
        assert_eq!(av_popcount_c(0), 0);
        assert_eq!(av_popcount_c(u32::MAX), 32);
        assert_eq!(av_popcount_c(0b1011), 3);
        assert_eq!(av_popcount64_c(u64::MAX), 64);
        assert_eq!(av_popcount64_c(1 << 63), 1);
        assert_eq!(av_parity_c(0b1011), 1);
        assert_eq!(av_parity_c(0b1010), 0);
    }

    #[test]
    fn zero_extend_checks_its_shift_domain() {
        assert_eq!(av_zero_extend_c(0xFFFF_FFFF, 8), Some(0xFF));
        assert_eq!(av_zero_extend_c(0xFFFF_FFFF, 0), Some(0));
        assert_eq!(av_zero_extend_c(0xFFFF_FFFF, 31), Some(0x7FFF_FFFF));
        assert_eq!(av_zero_extend_c(1, 32), None);
    }

    #[test]
    fn saturating_arithmetic_clamps_at_both_widths() {
        assert_eq!(av_sat_add32_c(i32::MAX, 1), i32::MAX);
        assert_eq!(av_sat_sub32_c(i32::MIN, 1), i32::MIN);
        assert_eq!(av_sat_add32_c(2, 3), 5);
        assert_eq!(av_sat_sub32_c(2, 3), -1);

        // The doubled stage saturates first, so the outer one sees `INT_MAX`
        // rather than the `2 * INT_MAX` that would have wrapped.
        assert_eq!(av_sat_dadd32_c(0, i32::MAX), i32::MAX);
        assert_eq!(av_sat_dsub32_c(0, i32::MAX), -i32::MAX);
        assert_eq!(av_sat_dadd32_c(1, 2), 5);
        assert_eq!(av_sat_dsub32_c(1, 2), -3);

        assert_eq!(av_sat_add64_c(i64::MAX, 1), i64::MAX);
        assert_eq!(av_sat_add64_c(i64::MIN, -1), i64::MIN);
        assert_eq!(av_sat_sub64_c(i64::MIN, 1), i64::MIN);
        assert_eq!(av_sat_sub64_c(i64::MAX, -1), i64::MAX);
        assert_eq!(av_sat_add64_c(2, 3), 5);
        assert_eq!(av_sat_sub64_c(2, 3), -1);
    }
}
