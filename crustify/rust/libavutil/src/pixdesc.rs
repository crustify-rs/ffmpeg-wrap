//! Wrappers for libavutil pixel format descriptors.

use core::ffi::CStr;
use core::ptr::{addr_of, addr_of_mut};

use ffibox::define_ctype;

use crate::ffi;

/// Wraps: av_chroma_location_from_name
///
/// Returns the non-negative `AVChromaLocation` value, or libavutil's negative
/// error code when the name is unknown.
pub fn av_chroma_location_from_name(name: &CStr) -> Result<i32, i32> {
    // SAFETY: `name` is NUL-terminated and remains live for the read-only call.
    let value = unsafe { ffi::av_chroma_location_from_name(name.as_ptr()) };
    if value < 0 { Err(value) } else { Ok(value) }
}

define_ctype!(
    /// Wraps: AVComponentDescriptor
    AVComponentDescriptor,
    AVComponentDescriptorRef,
    AVComponentDescriptorMut,
    ffi::AVComponentDescriptor
);

impl AVComponentDescriptorRef<'_> {
    /// Wraps: AVComponentDescriptor.offset
    #[must_use]
    pub fn offset(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).offset).read() }
    }

    /// Wraps: AVComponentDescriptor.shift
    #[must_use]
    pub fn shift(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).shift).read() }
    }

    /// Wraps: AVComponentDescriptor.plane
    #[must_use]
    pub fn plane(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).plane).read() }
    }

    /// Wraps: AVComponentDescriptor.depth
    #[must_use]
    pub fn depth(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).depth).read() }
    }

    /// Wraps: AVComponentDescriptor.step
    #[must_use]
    pub fn step(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).step).read() }
    }
}

impl AVComponentDescriptorMut<'_> {
    /// Sets the number of elements before the first component.
    pub fn set_offset(&mut self, value: i32) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).offset).write(value) }
    }

    /// Sets the number of low bits discarded from the component.
    pub fn set_shift(&mut self, value: i32) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).shift).write(value) }
    }

    /// Sets the plane containing the component.
    pub fn set_plane(&mut self, value: i32) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).plane).write(value) }
    }

    /// Sets the component bit depth.
    pub fn set_depth(&mut self, value: i32) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).depth).write(value) }
    }

    /// Sets the distance between horizontally consecutive components.
    pub fn set_step(&mut self, value: i32) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).step).write(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_chroma_locations() {
        assert!(av_chroma_location_from_name(c"left").is_ok());
        assert!(av_chroma_location_from_name(c"not-a-location").is_err());
    }

    #[test]
    fn component_descriptor_fields_round_trip() {
        let mut raw = ffi::AVComponentDescriptor {
            plane: 1,
            step: 2,
            offset: 3,
            shift: 4,
            depth: 5,
        };

        // SAFETY: `raw` is live and initialised for the returned handle's
        // scope, and this is its only borrowed handle.
        let mut descriptor = unsafe {
            AVComponentDescriptorMut::from_ptr(addr_of_mut!(raw))
                .expect("stack descriptor is non-null")
        };
        assert_eq!(descriptor.as_ref().plane(), 1);
        assert_eq!(descriptor.as_ref().step(), 2);
        assert_eq!(descriptor.as_ref().offset(), 3);
        assert_eq!(descriptor.as_ref().shift(), 4);
        assert_eq!(descriptor.as_ref().depth(), 5);

        descriptor.set_plane(6);
        descriptor.set_step(7);
        descriptor.set_offset(8);
        descriptor.set_shift(9);
        descriptor.set_depth(10);

        let shared = descriptor.as_ref();
        assert_eq!(shared.plane(), 6);
        assert_eq!(shared.step(), 7);
        assert_eq!(shared.offset(), 8);
        assert_eq!(shared.shift(), 9);
        assert_eq!(shared.depth(), 10);
    }
}
