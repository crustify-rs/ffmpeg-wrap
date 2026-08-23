//! Wrappers for libavutil pixel format descriptors.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CSlice, CSliceMut, define_ctype};

use crate::ffi;

/// Wraps: av_chroma_location_from_name
///
/// Returns the named chroma location, or libavutil's negative error code when
/// the name is unknown. C returns the index into its name table, which is the
/// `AVChromaLocation` value itself, or `AVERROR(EINVAL)`.
pub fn av_chroma_location_from_name(name: &CStr) -> Result<crate::pixfmt::AVChromaLocation, i32> {
    // SAFETY: `name` is NUL-terminated and remains live for the read-only call.
    let value = unsafe { ffi::av_chroma_location_from_name(name.as_ptr()) };
    if value < 0 {
        return Err(value);
    }
    // The non-negative branch is a name-table index below `AVCHROMA_LOC_NB`,
    // so the cast to the unsigned ABI representation preserves it.
    Ok(crate::pixfmt::AVChromaLocation::from_raw(
        value as ffi::AVChromaLocation,
    ))
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
        assert_eq!(
            av_chroma_location_from_name(c"left"),
            Ok(crate::pixfmt::AVChromaLocation::LEFT)
        );
        assert_eq!(
            av_chroma_location_from_name(c"bottomleft"),
            Ok(crate::pixfmt::AVChromaLocation::BOTTOM_LEFT)
        );
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

    #[test]
    fn component_descriptor_wrapper_preserves_layout() {
        // The wrapper is reached as an element of `AVPixFmtDescriptor.comp`
        // through `CSlice`, which strides by `size_of::<AVComponentDescriptor>()`
        // over storage C laid out; the two must agree.
        assert_eq!(
            core::mem::size_of::<AVComponentDescriptor>(),
            core::mem::size_of::<ffi::AVComponentDescriptor>()
        );
        assert_eq!(
            core::mem::align_of::<AVComponentDescriptor>(),
            core::mem::align_of::<ffi::AVComponentDescriptor>()
        );
    }
}

define_ctype!(
    /// Wraps: AVPixFmtDescriptor
    AVPixFmtDescriptor,
    AVPixFmtDescriptorRef,
    AVPixFmtDescriptorMut,
    ffi::AVPixFmtDescriptor
);

impl<'a> AVPixFmtDescriptorRef<'a> {
    /// Wraps: AVPixFmtDescriptor.flags
    #[must_use]
    pub fn flags(&self) -> u64 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).flags).read() }
    }

    /// Wraps: AVPixFmtDescriptor.name
    #[must_use]
    pub fn name(&self) -> Option<&CStr> {
        // SAFETY: `as_ptr` addresses a live descriptor; raw-place projection
        // and `read` form no reference to its storage.
        let pointer = unsafe { addr_of!((*self.as_ptr()).name).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: a non-null `name` in a valid descriptor addresses an
            // immutable NUL-terminated string which remains live while used.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Wraps: AVPixFmtDescriptor.comp
    ///
    /// The view borrows for the handle's own lifetime, not for the borrow of
    /// the handle: the descriptors live inside the C object the handle already
    /// addresses for `'a`, and the only route to a table descriptor's
    /// components is through a temporary handle from
    /// [`AVPixFmtDescriptorEntry::as_ref`].
    ///
    /// The array always has four elements; `nb_components` says how many of
    /// them the pixel format uses.
    #[must_use]
    pub fn components(&self) -> CSlice<'a, AVComponentDescriptor> {
        // SAFETY: `as_ptr` addresses a live descriptor; `addr_of!` performs
        // raw-place projection without forming a reference to the array.
        let pointer = unsafe {
            addr_of!((*self.as_ptr()).comp)
                .cast::<AVComponentDescriptor>()
                .cast_mut()
        };
        let pointer = NonNull::new(pointer).expect("an embedded field is non-null");
        // SAFETY: `comp` is an inline array of four initialized component
        // descriptors embedded in a descriptor live for `'a`, and the returned
        // view is shared, so it cannot outlive or alias-violate the handle.
        unsafe { CSlice::from_raw_parts(pointer, 4) }
    }

    /// Wraps: AVPixFmtDescriptor.nb_components
    #[must_use]
    pub fn nb_components(&self) -> u8 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).nb_components).read() }
    }

    /// Wraps: AVPixFmtDescriptor.alias
    #[must_use]
    pub fn alias(&self) -> Option<&CStr> {
        // SAFETY: `as_ptr` addresses a live descriptor; raw-place projection
        // and `read` form no reference to its storage.
        let pointer = unsafe { addr_of!((*self.as_ptr()).alias).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: a non-null `alias` in a valid descriptor addresses an
            // immutable NUL-terminated string which remains live while used.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Wraps: AVPixFmtDescriptor.log2_chroma_h
    #[must_use]
    pub fn log2_chroma_h(&self) -> u8 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).log2_chroma_h).read() }
    }

    /// Wraps: AVPixFmtDescriptor.log2_chroma_w
    #[must_use]
    pub fn log2_chroma_w(&self) -> u8 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).log2_chroma_w).read() }
    }
}

impl AVPixFmtDescriptorMut<'_> {
    /// Sets the pixel-format flags.
    pub fn set_flags(&mut self, value: u64) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).flags).write(value) }
    }

    /// Sets the optional static pixel-format name.
    pub fn set_name(&mut self, value: Option<&'static CStr>) {
        let pointer = value.map_or(core::ptr::null(), CStr::as_ptr);
        // SAFETY: the exclusive handle supplies write provenance; the
        // `'static` string, when present, remains live for every later read.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).name).write(pointer) }
    }

    /// Exclusively borrows the four inline component descriptors.
    #[must_use]
    pub fn components_mut(&mut self) -> CSliceMut<'_, AVComponentDescriptor> {
        // SAFETY: the exclusive handle addresses a live descriptor;
        // `addr_of_mut!` projects the array without forming a reference.
        let pointer =
            unsafe { addr_of_mut!((*self.as_mut_ptr()).comp).cast::<AVComponentDescriptor>() };
        let pointer = NonNull::new(pointer).expect("an embedded field is non-null");
        // SAFETY: `comp` is an inline array of four initialized component
        // descriptors, and `&mut self` provides exclusive access to the array
        // for the returned view's lifetime.
        unsafe { CSliceMut::from_raw_parts(pointer, 4) }
    }

    /// Sets the number of components in each pixel.
    pub fn set_nb_components(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).nb_components).write(value) }
    }

    /// Sets the optional static comma-separated alias list.
    pub fn set_alias(&mut self, value: Option<&'static CStr>) {
        let pointer = value.map_or(core::ptr::null(), CStr::as_ptr);
        // SAFETY: the exclusive handle supplies write provenance; the
        // `'static` string, when present, remains live for every later read.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).alias).write(pointer) }
    }

    /// Sets the vertical chroma subsampling exponent.
    pub fn set_log2_chroma_h(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).log2_chroma_h).write(value) }
    }

    /// Sets the horizontal chroma subsampling exponent.
    pub fn set_log2_chroma_w(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance and prevents
        // any other handle from being used for the duration of this call.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).log2_chroma_w).write(value) }
    }
}

#[cfg(test)]
mod pix_fmt_descriptor_tests {
    use super::*;

    fn component(plane: i32, depth: i32) -> ffi::AVComponentDescriptor {
        ffi::AVComponentDescriptor {
            plane,
            step: 1,
            offset: 0,
            shift: 0,
            depth,
        }
    }

    #[test]
    fn pixel_format_descriptor_fields_round_trip() {
        let mut raw = ffi::AVPixFmtDescriptor {
            name: c"initial".as_ptr(),
            nb_components: 3,
            log2_chroma_w: 1,
            log2_chroma_h: 2,
            flags: 0x10,
            comp: [
                component(0, 8),
                component(1, 9),
                component(2, 10),
                component(3, 11),
            ],
            alias: core::ptr::null(),
        };

        // SAFETY: `raw` is live and initialized for the returned handle's
        // scope, and this is its only borrowed handle.
        let mut descriptor = unsafe {
            AVPixFmtDescriptorMut::from_ptr(addr_of_mut!(raw))
                .expect("stack descriptor is non-null")
        };

        let shared = descriptor.as_ref();
        assert_eq!(shared.name(), Some(c"initial"));
        assert_eq!(shared.alias(), None);
        assert_eq!(shared.nb_components(), 3);
        assert_eq!(shared.log2_chroma_w(), 1);
        assert_eq!(shared.log2_chroma_h(), 2);
        assert_eq!(shared.flags(), 0x10);
        let components = shared.components();
        assert_eq!(components.len(), 4);
        assert_eq!(components.get(2).unwrap().plane(), 2);
        assert_eq!(components.get(2).unwrap().depth(), 10);

        descriptor.set_name(Some(c"updated"));
        descriptor.set_alias(Some(c"updated-alias"));
        descriptor.set_nb_components(4);
        descriptor.set_log2_chroma_w(3);
        descriptor.set_log2_chroma_h(4);
        descriptor.set_flags(0x20);
        descriptor
            .components_mut()
            .get_mut(2)
            .unwrap()
            .set_depth(12);

        let shared = descriptor.as_ref();
        assert_eq!(shared.name(), Some(c"updated"));
        assert_eq!(shared.alias(), Some(c"updated-alias"));
        assert_eq!(shared.nb_components(), 4);
        assert_eq!(shared.log2_chroma_w(), 3);
        assert_eq!(shared.log2_chroma_h(), 4);
        assert_eq!(shared.flags(), 0x20);
        assert_eq!(shared.components().get(2).unwrap().depth(), 12);

        descriptor.set_name(None);
        descriptor.set_alias(None);
        let shared = descriptor.as_ref();
        assert_eq!(shared.name(), None);
        assert_eq!(shared.alias(), None);
    }

    #[test]
    fn pixel_format_descriptor_wrapper_preserves_layout() {
        assert_eq!(
            core::mem::size_of::<AVPixFmtDescriptor>(),
            core::mem::size_of::<ffi::AVPixFmtDescriptor>()
        );
        assert_eq!(
            core::mem::align_of::<AVPixFmtDescriptor>(),
            core::mem::align_of::<ffi::AVPixFmtDescriptor>()
        );
    }
}

fn static_name(pointer: *const core::ffi::c_char) -> Option<&'static CStr> {
    if pointer.is_null() {
        None
    } else {
        // SAFETY: all callers pass pointers returned from libavutil's immutable
        // process-lifetime name tables; the null case was handled above.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

/// Wraps: av_alpha_mode_from_name
#[must_use]
pub fn av_alpha_mode_from_name(name: &CStr) -> crate::pixfmt::AVAlphaMode {
    // SAFETY: `name` is a live NUL-terminated read-only string and is not retained.
    crate::pixfmt::AVAlphaMode::from_raw(unsafe { ffi::av_alpha_mode_from_name(name.as_ptr()) })
}

/// Wraps: av_alpha_mode_name
#[must_use]
pub fn av_alpha_mode_name(mode: crate::pixfmt::AVAlphaMode) -> Option<&'static CStr> {
    // SAFETY: the argument is an ABI-compatible value; C returns null or a
    // pointer into its immutable process-lifetime name table.
    static_name(unsafe { ffi::av_alpha_mode_name(mode.as_raw()) })
}

/// Wraps: av_chroma_location_enum_to_pos
pub fn av_chroma_location_enum_to_pos(
    location: crate::pixfmt::AVChromaLocation,
) -> Result<(i32, i32), i32> {
    let mut x = 0;
    let mut y = 0;
    // SAFETY: the two output pointers address distinct live integer slots.
    let status =
        unsafe { ffi::av_chroma_location_enum_to_pos(&raw mut x, &raw mut y, location.as_raw()) };
    if status < 0 { Err(status) } else { Ok((x, y)) }
}

/// Wraps: av_chroma_location_name
#[must_use]
pub fn av_chroma_location_name(location: crate::pixfmt::AVChromaLocation) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static table entry.
    static_name(unsafe { ffi::av_chroma_location_name(location.as_raw()) })
}

/// Wraps: av_chroma_location_pos_to_enum
#[must_use]
pub fn av_chroma_location_pos_to_enum(x: i32, y: i32) -> crate::pixfmt::AVChromaLocation {
    // SAFETY: the function accepts both coordinates by value.
    crate::pixfmt::AVChromaLocation::from_raw(unsafe { ffi::av_chroma_location_pos_to_enum(x, y) })
}

/// Wraps: av_color_primaries_name
#[must_use]
pub fn av_color_primaries_name(value: crate::pixfmt::AVColorPrimaries) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static table entry.
    static_name(unsafe { ffi::av_color_primaries_name(value.as_raw()) })
}

/// Wraps: av_color_range_name
#[must_use]
pub fn av_color_range_name(value: crate::pixfmt::AVColorRange) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static table entry.
    static_name(unsafe { ffi::av_color_range_name(value.as_raw()) })
}

/// Wraps: av_color_space_name
#[must_use]
pub fn av_color_space_name(value: crate::pixfmt::AVColorSpace) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static table entry.
    static_name(unsafe { ffi::av_color_space_name(value.as_raw()) })
}

/// Wraps: av_color_transfer_name
#[must_use]
pub fn av_color_transfer_name(
    value: crate::pixfmt::AVColorTransferCharacteristic,
) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static table entry.
    static_name(unsafe { ffi::av_color_transfer_name(value.as_raw()) })
}

/// Wraps: av_get_pix_fmt
#[must_use]
pub fn av_get_pix_fmt(name: &CStr) -> crate::pixfmt::AVPixelFormat {
    // SAFETY: `name` is NUL-terminated, readable, and not retained.
    crate::pixfmt::AVPixelFormat::from_raw(unsafe { ffi::av_get_pix_fmt(name.as_ptr()) })
}

/// Wraps: av_get_pix_fmt_name
#[must_use]
pub fn av_get_pix_fmt_name(format: crate::pixfmt::AVPixelFormat) -> Option<&'static CStr> {
    // SAFETY: C returns null or a static descriptor name.
    static_name(unsafe { ffi::av_get_pix_fmt_name(format.as_raw()) })
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;
    use crate::pixfmt::{AVAlphaMode, AVChromaLocation, AVColorPrimaries, AVPixelFormat};

    #[test]
    fn names_and_reverse_lookups_round_trip() {
        assert_eq!(
            av_alpha_mode_from_name(c"premultiplied"),
            AVAlphaMode::PREMULTIPLIED
        );
        assert_eq!(av_alpha_mode_name(AVAlphaMode::STRAIGHT), Some(c"straight"));
        assert_eq!(
            av_chroma_location_name(AVChromaLocation::LEFT),
            Some(c"left")
        );
        assert_eq!(
            av_color_primaries_name(AVColorPrimaries::BT709),
            Some(c"bt709")
        );
        assert_eq!(av_get_pix_fmt(c"yuv420p"), AVPixelFormat::YUV420P);
        assert_eq!(
            av_get_pix_fmt_name(AVPixelFormat::YUV420P),
            Some(c"yuv420p")
        );
    }

    #[test]
    fn chroma_positions_convert_in_both_directions() {
        let position = av_chroma_location_enum_to_pos(AVChromaLocation::LEFT).unwrap();
        assert_eq!(
            av_chroma_location_pos_to_enum(position.0, position.1),
            AVChromaLocation::LEFT
        );
    }
}

/// Wraps: av_get_bits_per_pixel
#[must_use]
pub fn av_get_bits_per_pixel(descriptor: AVPixFmtDescriptorRef<'_>) -> i32 {
    // SAFETY: the handle supplies a live immutable descriptor for this
    // read-only call and C does not retain it.
    unsafe { ffi::av_get_bits_per_pixel(descriptor.as_ptr()) }
}

/// An immutable entry in libavutil's process-lifetime pixel-format table.
#[derive(Clone, Copy)]
pub struct AVPixFmtDescriptorEntry(AVPixFmtDescriptorRef<'static>);

impl AVPixFmtDescriptorEntry {
    /// Borrows the immutable descriptor metadata in this table entry.
    #[must_use]
    pub fn as_ref(&self) -> AVPixFmtDescriptorRef<'_> {
        self.0
    }
}

/// Wraps: av_pix_fmt_desc_get
///
/// Returns an immutable descriptor from libavutil's process-lifetime table.
#[must_use]
pub fn av_pix_fmt_desc_get(
    format: crate::pixfmt::AVPixelFormat,
) -> Option<AVPixFmtDescriptorEntry> {
    // SAFETY: C returns null or a pointer into its immutable static descriptor
    // table; the handle never forms a Rust reference to those bytes.
    unsafe { AVPixFmtDescriptorRef::from_ptr(ffi::av_pix_fmt_desc_get(format.as_raw()).cast_mut()) }
        .map(AVPixFmtDescriptorEntry)
}

/// Wraps: av_pix_fmt_desc_get_id
#[must_use]
pub fn av_pix_fmt_desc_get_id(descriptor: AVPixFmtDescriptorEntry) -> crate::pixfmt::AVPixelFormat {
    // SAFETY: the entry type can only be produced by the table lookup or
    // iterator, so C's pointer-range operation stays within its static array.
    crate::pixfmt::AVPixelFormat::from_raw(unsafe {
        ffi::av_pix_fmt_desc_get_id(descriptor.0.as_ptr())
    })
}

/// Wraps: av_pix_fmt_desc_next
///
/// Pass `None` to start iteration. Both the input and returned handles refer
/// to libavutil's immutable process-lifetime descriptor table.
#[must_use]
pub fn av_pix_fmt_desc_next(
    previous: Option<AVPixFmtDescriptorEntry>,
) -> Option<AVPixFmtDescriptorEntry> {
    let previous = previous.map_or(core::ptr::null(), |descriptor| descriptor.0.as_ptr());
    // SAFETY: `previous` is null or came from this static table. C returns null
    // or another static table entry and retains nothing.
    unsafe { AVPixFmtDescriptorRef::from_ptr(ffi::av_pix_fmt_desc_next(previous).cast_mut()) }
        .map(AVPixFmtDescriptorEntry)
}

#[cfg(test)]
mod scheduled_descriptor_tests {
    use super::*;
    use crate::pixfmt::AVPixelFormat;

    #[test]
    fn descriptor_lookup_identity_and_bit_count_round_trip() {
        let descriptor = av_pix_fmt_desc_get(AVPixelFormat::YUV420P).expect("known format");
        assert_eq!(av_pix_fmt_desc_get_id(descriptor), AVPixelFormat::YUV420P);
        assert_eq!(av_get_bits_per_pixel(descriptor.as_ref()), 12);
    }

    #[test]
    fn table_components_outlive_the_temporary_handle() {
        let entry = av_pix_fmt_desc_get(AVPixelFormat::YUV420P).expect("known format");
        // The handle from `as_ref()` is a temporary; the view must borrow the
        // descriptor itself, not that temporary.
        let components = entry.as_ref().components();
        assert_eq!(components.len(), 4);
        let luma = components.get(0).expect("a first component");
        assert_eq!(luma.plane(), 0);
        assert_eq!(luma.depth(), 8);
        assert_eq!(luma.step(), 1);
        assert_eq!(
            components
                .iter()
                .take(usize::from(entry.as_ref().nb_components()))
                .map(|component| component.depth())
                .sum::<i32>(),
            24
        );
    }

    #[test]
    fn descriptor_iteration_starts_at_a_named_entry() {
        let first = av_pix_fmt_desc_next(None).expect("descriptor table is nonempty");
        assert!(first.as_ref().name().is_some());
        assert!(av_pix_fmt_desc_next(Some(first)).is_some());
    }
}
