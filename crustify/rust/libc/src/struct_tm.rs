//! Wrappers for `/usr/include/x86_64-linux-gnu/bits/types/struct_tm.h`.

use core::ffi::{CStr, c_long};
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::CValued;

use crate::ffi;

ffibox::define_ctype!(
    /// Wraps: tm
    ///
    /// ABI-compatible storage and borrowed handles for C's broken-down time
    /// structure. A `tm` has no teardown operation; its optional timezone
    /// abbreviation is borrowed external storage and is not owned by this
    /// wrapper.
    Tm,
    TmRef,
    TmMut,
    ffi::tm
);

// SAFETY: `tm` owns no resources and has no teardown operation. In particular,
// libc does not transfer ownership of the string addressed by `tm_zone` to the
// structure, so disposing an inline value is always a no-op.
unsafe impl CValued for Tm {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

macro_rules! calendar_field {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl TmRef<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the handle addresses a live initialized `tm`;
                // raw-place projection copies one scalar and forms no Rust
                // reference to storage libc may write through.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl TmMut<'_> {
            #[doc = concat!("Sets `", stringify!($field), "`.")]
            ///
            /// The value is stored unchanged. `mktime` and `timegm`
            /// deliberately accept out-of-range members and normalize them,
            /// so this setter imposes no range of its own.
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance for
                // this live scalar field, and every `$ty` bit pattern is a
                // valid value of the C field.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

calendar_field!(
    /// Field: tm.tm_sec
    ///
    /// Seconds after the minute, `0..=60` for a value produced by libc.
    sec,
    set_sec,
    tm_sec,
    i32
);
calendar_field!(
    /// Field: tm.tm_min
    ///
    /// Minutes after the hour, `0..=59` for a value produced by libc.
    min,
    set_min,
    tm_min,
    i32
);
calendar_field!(
    /// Field: tm.tm_hour
    ///
    /// Hours since midnight, `0..=23` for a value produced by libc.
    hour,
    set_hour,
    tm_hour,
    i32
);
calendar_field!(
    /// Field: tm.tm_mday
    ///
    /// Day of the month, `1..=31` for a value produced by libc.
    mday,
    set_mday,
    tm_mday,
    i32
);
calendar_field!(
    /// Field: tm.tm_mon
    ///
    /// Months since January, `0..=11` for a value produced by libc.
    mon,
    set_mon,
    tm_mon,
    i32
);
calendar_field!(
    /// Field: tm.tm_year
    ///
    /// Years since 1900.
    year,
    set_year,
    tm_year,
    i32
);
calendar_field!(
    /// Field: tm.tm_wday
    ///
    /// Days since Sunday, `0..=6` for a value produced by libc.
    wday,
    set_wday,
    tm_wday,
    i32
);
calendar_field!(
    /// Field: tm.tm_yday
    ///
    /// Days since January 1st, `0..=365` for a value produced by libc.
    yday,
    set_yday,
    tm_yday,
    i32
);
calendar_field!(
    /// Field: tm.tm_isdst
    ///
    /// Positive when daylight saving time is in effect, zero when it is not
    /// and negative when the information is unavailable.
    isdst,
    set_isdst,
    tm_isdst,
    i32
);
calendar_field!(
    /// Field: tm.__tm_gmtoff
    ///
    /// Seconds east of UTC. The glibc header spells this member `tm_gmtoff`
    /// under `__USE_MISC` and `__tm_gmtoff` otherwise; both name one field.
    gmtoff,
    set_gmtoff,
    tm_gmtoff,
    c_long
);

impl<'a> TmRef<'a> {
    /// Field: tm.__tm_zone
    ///
    /// Returns the timezone abbreviation, or `None` when the field is null.
    /// The glibc header spells this member `tm_zone` under `__USE_MISC` and
    /// `__tm_zone` otherwise; both name one field.
    ///
    /// # Safety
    ///
    /// A `tm` does not own this string, and no Rust lifetime describes how
    /// long it stays valid: for a value libc produced it addresses timezone
    /// state that `tzset`, `localtime` and their relatives may replace, and
    /// for a caller-initialized value it addresses whatever that caller
    /// stored. The caller must establish that the string is still live and
    /// NUL-terminated, and that nothing invalidates it for `'a`.
    #[must_use]
    pub unsafe fn zone(&self) -> Option<&'a CStr> {
        // SAFETY: the handle addresses a live initialized `tm`; raw-place
        // projection copies the pointer without forming a Rust reference.
        let pointer = unsafe { addr_of!((*self.as_ptr()).tm_zone).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: the caller established that a non-null value addresses a
            // live NUL-terminated string that stays valid for `'a`.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Reports whether a timezone abbreviation is present.
    #[must_use]
    pub fn has_zone(&self) -> bool {
        // SAFETY: the handle addresses a live initialized `tm` and this
        // raw-place projection only copies the pointer value out.
        !unsafe { addr_of!((*self.as_ptr()).tm_zone).read() }.is_null()
    }
}

impl TmMut<'_> {
    /// Stores a timezone abbreviation borrowed from external storage.
    ///
    /// # Safety
    ///
    /// The `tm` does not take ownership, and its own storage may outlive any
    /// Rust borrow this handle carries. The caller must guarantee that `zone`
    /// stays live and NUL-terminated for as long as the field is readable,
    /// including through every later reader of the underlying `tm`.
    pub unsafe fn set_zone(&mut self, zone: &CStr) {
        // SAFETY: the exclusive handle supplies write provenance for the live
        // pointer field, and the caller guarantees the referent outlives it.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).tm_zone).write(zone.as_ptr()) }
    }

    /// Clears the timezone abbreviation, leaving the field null.
    ///
    /// A `tm` never owns that string, so dropping the pointer releases
    /// nothing.
    pub fn clear_zone(&mut self) {
        // SAFETY: the exclusive handle supplies write provenance for the live
        // pointer field, and null is the documented "absent" value.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).tm_zone).write(core::ptr::null()) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn tm_is_layout_compatible() {
        assert_eq!(size_of::<Tm>(), size_of::<ffi::tm>());
        assert_eq!(align_of::<Tm>(), align_of::<ffi::tm>());
    }

    #[test]
    fn zeroed_tm_supports_shared_and_exclusive_handles() {
        let mut value = CVal::new(Tm::zeroed());
        let shared = value.as_ref();
        assert!(!shared.as_ptr().is_null());

        let mut exclusive = value.as_mut();
        assert!(!exclusive.as_mut_ptr().is_null());
        assert_eq!(exclusive.as_ref().as_ptr(), exclusive.as_mut_ptr());
    }

    #[test]
    fn every_calendar_field_round_trips_through_the_handles() {
        let mut value = CVal::new(Tm::zeroed());
        let mut time = value.as_mut();
        time.set_sec(60);
        time.set_min(59);
        time.set_hour(23);
        time.set_mday(31);
        time.set_mon(11);
        time.set_year(125);
        time.set_wday(6);
        time.set_yday(365);
        time.set_isdst(-1);
        time.set_gmtoff(-28_800);

        let time = value.as_ref();
        assert_eq!(time.sec(), 60);
        assert_eq!(time.min(), 59);
        assert_eq!(time.hour(), 23);
        assert_eq!(time.mday(), 31);
        assert_eq!(time.mon(), 11);
        assert_eq!(time.year(), 125);
        assert_eq!(time.wday(), 6);
        assert_eq!(time.yday(), 365);
        assert_eq!(time.isdst(), -1);
        assert_eq!(time.gmtoff(), -28_800);
    }

    #[test]
    fn zone_is_absent_until_a_borrowed_string_is_stored() {
        let mut value = CVal::new(Tm::zeroed());
        assert!(!value.as_ref().has_zone());
        // SAFETY: `value` holds a zeroed `tm`, so the field is null and the
        // getter reads nothing.
        assert!(unsafe { value.as_ref().zone() }.is_none());

        const ZONE: &CStr = c"UTC";
        // SAFETY: `ZONE` is a `'static` string, so it outlives `value` and
        // every reader of the stored pointer.
        unsafe { value.as_mut().set_zone(ZONE) };
        assert!(value.as_ref().has_zone());
        // SAFETY: the stored pointer is the `'static` string above.
        assert_eq!(unsafe { value.as_ref().zone() }, Some(ZONE));

        value.as_mut().clear_zone();
        assert!(!value.as_ref().has_zone());
    }

    #[test]
    fn setters_write_the_fields_c_reads_at_their_declared_offsets() {
        let mut value = CVal::new(Tm::zeroed());
        let mut time = value.as_mut();
        time.set_sec(1);
        time.set_min(2);
        time.set_hour(3);
        time.set_mday(4);
        time.set_mon(5);
        time.set_year(6);
        time.set_wday(0);
        time.set_yday(7);
        time.set_isdst(0);
        time.set_gmtoff(8);

        // SAFETY: the handle addresses one live initialized `ffi::tm`, and the
        // read copies it out by value without forming a reference to it.
        let raw = unsafe { value.as_ref().as_ptr().read() };
        assert_eq!(raw.tm_sec, 1);
        assert_eq!(raw.tm_min, 2);
        assert_eq!(raw.tm_hour, 3);
        assert_eq!(raw.tm_mday, 4);
        assert_eq!(raw.tm_mon, 5);
        assert_eq!(raw.tm_year, 6);
        assert_eq!(raw.tm_wday, 0);
        assert_eq!(raw.tm_yday, 7);
        assert_eq!(raw.tm_isdst, 0);
        assert_eq!(raw.tm_gmtoff, 8);
        assert!(raw.tm_zone.is_null());
    }
}
