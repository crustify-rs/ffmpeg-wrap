//! Wrappers for libavutil time utilities.

use core::time::Duration;

use crate::ffi;

/// Wraps: av_gettime
#[must_use]
pub fn av_gettime() -> i64 {
    // SAFETY: the function has no pointer arguments or caller obligations.
    unsafe { ffi::av_gettime() }
}

/// Wraps: av_gettime_relative
#[must_use]
pub fn av_gettime_relative() -> i64 {
    // SAFETY: the function has no pointer arguments or caller obligations.
    unsafe { ffi::av_gettime_relative() }
}

/// Wraps: av_gettime_relative_is_monotonic
#[must_use]
pub fn av_gettime_relative_is_monotonic() -> bool {
    // SAFETY: the function has no pointer arguments or caller obligations.
    unsafe { ffi::av_gettime_relative_is_monotonic() != 0 }
}

/// Wraps: av_usleep
///
/// Returns an error without sleeping if the duration does not fit the C API's
/// unsigned-microsecond argument.
pub fn av_usleep(duration: Duration) -> Result<(), SleepError> {
    let micros = u32::try_from(duration.as_micros()).map_err(|_| SleepError::DurationOverflow)?;
    // SAFETY: the function takes its duration by value and retains nothing.
    let status = unsafe { ffi::av_usleep(micros) };
    if status < 0 {
        Err(SleepError::Library(status))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepError {
    DurationOverflow,
    Library(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clocks_and_zero_sleep_are_callable() {
        let _ = av_gettime();
        let _ = av_gettime_relative();
        let _ = av_gettime_relative_is_monotonic();
        assert_eq!(av_usleep(Duration::ZERO), Ok(()));
    }

    #[test]
    fn rejects_unrepresentable_sleep() {
        assert_eq!(
            av_usleep(Duration::from_secs(u64::from(u32::MAX))),
            Err(SleepError::DurationOverflow)
        );
    }
}
