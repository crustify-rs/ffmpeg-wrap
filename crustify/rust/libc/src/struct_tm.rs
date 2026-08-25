//! Wrappers for `/usr/include/x86_64-linux-gnu/bits/types/struct_tm.h`.

use core::ptr::NonNull;

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
}
