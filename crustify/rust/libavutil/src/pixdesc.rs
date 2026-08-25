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
    /// Field: AVComponentDescriptor.offset
    #[must_use]
    pub fn offset(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).offset).read() }
    }

    /// Field: AVComponentDescriptor.shift
    #[must_use]
    pub fn shift(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).shift).read() }
    }

    /// Field: AVComponentDescriptor.plane
    #[must_use]
    pub fn plane(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).plane).read() }
    }

    /// Field: AVComponentDescriptor.depth
    #[must_use]
    pub fn depth(&self) -> i32 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).depth).read() }
    }

    /// Field: AVComponentDescriptor.step
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
        // The name table and the parser are inverses, which is what makes the
        // typed return the honest one.
        assert_eq!(
            av_chroma_location_name(
                av_chroma_location_from_name(c"topleft").expect("a known location")
            ),
            Some(c"topleft")
        );
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
    ///
    /// Describes how the bits of one pixel are laid out across up to four
    /// planes. Libavutil never allocates, frees or mutates a descriptor: every
    /// one it publishes is an entry of the `static const` table in
    /// `libavutil/pixdesc.c`, reached through [`av_pix_fmt_desc_get`] and
    /// [`av_pix_fmt_desc_next`], so the type has no lifecycle operation.
    ///
    /// # Invariant
    ///
    /// A handle over a descriptor asserts two things about its storage, both
    /// of which libavutil's own readers rely on without checking:
    ///
    /// - `name` and `alias` are null or address NUL-terminated strings that
    ///   outlive the handle's borrow. That is what makes the safe getters
    ///   [`AVPixFmtDescriptorRef::name`] and [`AVPixFmtDescriptorRef::alias`]
    ///   sound. The unsafe `from_ptr` constructors are where a caller
    ///   establishes it; [`AVPixFmtDescriptorMut::set_name`] and
    ///   [`AVPixFmtDescriptorMut::set_alias`] preserve it by accepting only a
    ///   `&'static CStr`; C upholds it with string literals or a null field.
    /// - `nb_components` is at most 4. `comp` has exactly four slots and every
    ///   libavutil reader — `av_get_bits_per_pixel`, `av_pix_fmt_count_planes`,
    ///   `av_image_fill_linesizes` — walks `comp[c]` for `c < nb_components`,
    ///   so a larger count is an out-of-bounds read inside C. `from_ptr`'s
    ///   caller establishes it and
    ///   [`AVPixFmtDescriptorMut::set_nb_components`] rejects anything larger.
    ///
    /// The remaining fields are unconstrained here, which is why the one
    /// wrapped C reader over this type, [`av_get_bits_per_pixel`], takes an
    /// [`AVPixFmtDescriptorEntry`] rather than a bare handle: it shifts an
    /// `int` by `log2_chroma_w + log2_chroma_h` and accumulates
    /// `comp[c].depth` into it, and no per-field bound makes that arithmetic
    /// defined for every combination a caller could set.
    AVPixFmtDescriptor,
    AVPixFmtDescriptorRef,
    AVPixFmtDescriptorMut,
    ffi::AVPixFmtDescriptor
);

impl<'a> AVPixFmtDescriptorRef<'a> {
    /// Field: AVPixFmtDescriptor.flags
    #[must_use]
    pub fn flags(&self) -> u64 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).flags).read() }
    }

    /// Field: AVPixFmtDescriptor.name
    ///
    /// Returns the canonical format name, or `None` for a table slot no
    /// configuration filled in. The string borrows for the handle's own
    /// lifetime, not for the borrow of the handle.
    #[must_use]
    pub fn name(&self) -> Option<&'a CStr> {
        // SAFETY: `as_ptr` addresses a live descriptor; raw-place projection
        // and `read` form no reference to its storage.
        let pointer = unsafe { addr_of!((*self.as_ptr()).name).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: `AVPixFmtDescriptor`'s handle invariant makes a non-null
            // `name` a NUL-terminated string outliving this borrow. Every
            // producer carries that obligation: `from_ptr`'s caller asserts it,
            // and `set_name` cannot break it because it accepts only
            // `&'static CStr`. The result is narrowed to `'a`, which the
            // descriptor itself already outlives.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVPixFmtDescriptor.comp
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

    /// Field: AVPixFmtDescriptor.nb_components
    #[must_use]
    pub fn nb_components(&self) -> u8 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).nb_components).read() }
    }

    /// Field: AVPixFmtDescriptor.alias
    ///
    /// Returns the comma-separated alternative names — the spellings
    /// `av_get_pix_fmt` also accepts for this format — or `None` when the
    /// descriptor declares none. The string borrows for the handle's own
    /// lifetime, not for the borrow of the handle.
    #[must_use]
    pub fn alias(&self) -> Option<&'a CStr> {
        // SAFETY: `as_ptr` addresses a live descriptor; raw-place projection
        // and `read` form no reference to its storage.
        let pointer = unsafe { addr_of!((*self.as_ptr()).alias).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: `AVPixFmtDescriptor`'s handle invariant makes a non-null
            // `alias` a NUL-terminated string outliving this borrow, upheld by
            // `from_ptr`'s caller and preserved by `set_alias`, which accepts
            // only `&'static CStr`. The result is narrowed to `'a`, which the
            // descriptor itself already outlives.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVPixFmtDescriptor.log2_chroma_h
    #[must_use]
    pub fn log2_chroma_h(&self) -> u8 {
        // SAFETY: `as_ptr` addresses a live descriptor for this handle's
        // lifetime; raw-place projection and `read` form no reference to it.
        unsafe { addr_of!((*self.as_ptr()).log2_chroma_h).read() }
    }

    /// Field: AVPixFmtDescriptor.log2_chroma_w
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
    ///
    /// # Panics
    ///
    /// Panics when `value` exceeds 4. `comp` holds four descriptors and
    /// libavutil reads `comp[c]` for every `c < nb_components`, so a larger
    /// count would break the [type's invariant](AVPixFmtDescriptor) and read
    /// past the array inside C.
    pub fn set_nb_components(&mut self, value: u8) {
        assert!(
            value <= 4,
            "a pixel format has at most 4 components, got {value}"
        );
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
    #[should_panic = "a pixel format has at most 4 components"]
    fn a_component_count_past_the_array_is_rejected() {
        let mut raw = ffi::AVPixFmtDescriptor {
            name: c"probe".as_ptr(),
            nb_components: 1,
            log2_chroma_w: 0,
            log2_chroma_h: 0,
            flags: 0,
            comp: [component(0, 8); 4],
            alias: core::ptr::null(),
        };

        // SAFETY: `raw` is live and initialized for the handle's scope, its
        // `nb_components` is within the four-slot `comp` array, its `name` is a
        // `'static` literal, and this is its only borrowed handle.
        let mut descriptor = unsafe {
            AVPixFmtDescriptorMut::from_ptr(addr_of_mut!(raw))
                .expect("stack descriptor is non-null")
        };

        // Without the bound, a descriptor built here reaches C claiming more
        // components than `comp` can hold, and the reader walks past the array:
        // the probe behind this test set the count to 200 and called
        // `av_get_bits_per_pixel`, which UBSan reported as an out-of-bounds
        // index at pixdesc.c:3419 with no `unsafe` anywhere in the caller.
        descriptor.set_nb_components(5);
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
///
/// Returns the named alpha mode, or libavutil's negative error code when the
/// name is unknown, exactly as [`av_chroma_location_from_name`] does for the
/// sibling table. C declares the result `enum AVAlphaMode` but returns either a
/// name-table index below `AVALPHA_MODE_NB` or `AVERROR(EINVAL)`; since the
/// enumerators are all non-negative the compiler gives that enum an unsigned
/// type, so the error arrives as a very large value. Handing it back as an
/// [`AVAlphaMode`](crate::pixfmt::AVAlphaMode) would leave every caller to
/// recognise that one bit pattern as failure.
///
/// Sign, not table membership, separates the two: a mode added by a newer
/// libavutil is still a small index and comes back as `Ok`.
pub fn av_alpha_mode_from_name(name: &CStr) -> Result<crate::pixfmt::AVAlphaMode, i32> {
    // SAFETY: `name` is a live NUL-terminated read-only string and is not retained.
    let value = unsafe { ffi::av_alpha_mode_from_name(name.as_ptr()) };
    let status = value as i32;
    if status < 0 {
        return Err(status);
    }
    Ok(crate::pixfmt::AVAlphaMode::from_raw(value))
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
    use crate::pixfmt::{
        AVAlphaMode, AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic, AVPixelFormat,
    };

    #[test]
    fn names_and_reverse_lookups_round_trip() {
        assert_eq!(
            av_alpha_mode_from_name(c"premultiplied"),
            Ok(AVAlphaMode::PREMULTIPLIED)
        );
        // The unknown-name path is an error rather than an alpha mode holding
        // `AVERROR(EINVAL)` reinterpreted through an unsigned enum.
        assert_eq!(av_alpha_mode_from_name(c"not-an-alpha-mode"), Err(-22));
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
        // Every location C assigns a position to must survive the round trip,
        // and each must occupy a distinct one — otherwise `pos_to_enum` would
        // answer with whichever came first and the pairing above would still
        // pass for the one location it checked.
        let locations = [
            AVChromaLocation::LEFT,
            AVChromaLocation::CENTER,
            AVChromaLocation::TOP_LEFT,
            AVChromaLocation::TOP,
            AVChromaLocation::BOTTOM_LEFT,
            AVChromaLocation::BOTTOM,
        ];
        let mut positions = [(0_i32, 0_i32); 6];
        for (slot, location) in positions.iter_mut().zip(locations) {
            let (x, y) = av_chroma_location_enum_to_pos(location).unwrap();
            assert_eq!(av_chroma_location_pos_to_enum(x, y), location);
            *slot = (x, y);
        }
        for (index, position) in positions.iter().enumerate() {
            assert!(!positions[..index].contains(position));
        }

        // The sentinel has no position, and an unmatched position falls back
        // to it instead of naming a location.
        assert_eq!(
            av_chroma_location_enum_to_pos(AVChromaLocation::UNSPECIFIED),
            Err(-22)
        );
        assert_eq!(
            av_chroma_location_pos_to_enum(1, 1),
            AVChromaLocation::UNSPECIFIED
        );
    }

    #[test]
    fn each_colour_property_table_names_its_own_constants() {
        // The four `*_name` wrappers read four different static tables. Each
        // pairing is the string libavutil stores at that index, so a constant
        // bound to the wrong enumerator fails here.
        assert_eq!(av_color_range_name(AVColorRange::MPEG), Some(c"tv"));
        assert_eq!(av_color_range_name(AVColorRange::JPEG), Some(c"pc"));
        assert_eq!(av_color_space_name(AVColorSpace::BT709), Some(c"bt709"));
        assert_eq!(av_color_space_name(AVColorSpace::RGB), Some(c"gbr"));
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::LINEAR),
            Some(c"linear")
        );
        assert_eq!(
            av_color_primaries_name(AVColorPrimaries::BT470BG),
            Some(c"bt470bg")
        );

        // Each table is bounds-checked rather than indexed blindly, so a value
        // past its end has no name.
        assert_eq!(
            av_color_range_name(AVColorRange::from_raw(ffi::AVColorRange::MAX)),
            None
        );
        assert_eq!(
            av_color_space_name(AVColorSpace::from_raw(ffi::AVColorSpace::MAX)),
            None
        );
        assert_eq!(
            av_color_transfer_name(AVColorTransferCharacteristic::from_raw(
                ffi::AVColorTransferCharacteristic::MAX
            )),
            None
        );
        assert_eq!(
            av_color_primaries_name(AVColorPrimaries::from_raw(ffi::AVColorPrimaries::MAX)),
            None
        );
        assert_eq!(
            av_get_pix_fmt_name(AVPixelFormat::from_raw(ffi::AVPixelFormat::MAX)),
            None
        );
    }
}

/// An immutable entry in libavutil's process-lifetime pixel-format table.
///
/// The table is the `static const` array in `libavutil/pixdesc.c`; nothing
/// writes to it after load, so an entry is only ever read. Holding one is what
/// distinguishes a descriptor libavutil itself vouches for from a descriptor a
/// caller built, which is the distinction
/// [`av_pix_fmt_desc_get_id`] and [`av_get_bits_per_pixel`] need.
#[derive(Clone, Copy)]
pub struct AVPixFmtDescriptorEntry(AVPixFmtDescriptorRef<'static>);

impl AVPixFmtDescriptorEntry {
    /// Borrows the immutable descriptor metadata in this table entry.
    ///
    /// The handle borrows for `'static`, not for this borrow of the entry:
    /// the descriptor it addresses lives as long as the loaded library.
    #[must_use]
    pub fn as_ref(&self) -> AVPixFmtDescriptorRef<'static> {
        self.0
    }
}

/// Wraps: av_get_bits_per_pixel
///
/// Returns the average number of bits each pixel of the format occupies,
/// counting chroma subsampling but not padding.
///
/// Takes a table entry rather than a bare handle. C sums
/// `comp[c].depth << (log2_chroma_w + log2_chroma_h)` over `c` below
/// `nb_components` and shifts the `int` total back down, checking none of the
/// three fields; the descriptors libavutil publishes are the ones whose values
/// that arithmetic is defined for. See the
/// [type documentation](AVPixFmtDescriptor).
#[must_use]
pub fn av_get_bits_per_pixel(descriptor: AVPixFmtDescriptorEntry) -> i32 {
    // SAFETY: the entry type can only be produced by the table lookup or the
    // iterator, so the pointer addresses a live immutable table descriptor for
    // this read-only call, and C does not retain it.
    unsafe { ffi::av_get_bits_per_pixel(descriptor.0.as_ptr()) }
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
        assert_eq!(av_get_bits_per_pixel(descriptor), 12);
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
    fn the_wrapper_agrees_with_the_c_table_layout() {
        // C's descriptors are one contiguous array indexed by pixel format, so
        // the distance between two adjacent entries is what C compiled as
        // `sizeof(AVPixFmtDescriptor)`. That is the stride `CSlice`, `addr_of!`
        // and every projection in this module are laid out against.
        let first = av_pix_fmt_desc_get(AVPixelFormat::YUV420P).expect("known format");
        let second = av_pix_fmt_desc_get(AVPixelFormat::YUYV422).expect("known format");
        assert_eq!(
            second.as_ref().as_ptr() as usize - first.as_ref().as_ptr() as usize,
            core::mem::size_of::<AVPixFmtDescriptor>()
        );

        // Every field offset, read back against pixdesc.c's own initializer for
        // AV_PIX_FMT_GRAY8. A field projected at the wrong offset lands in a
        // neighbouring one, which these values distinguish.
        let gray = av_pix_fmt_desc_get(AVPixelFormat::GRAY8).expect("known format");
        let gray = gray.as_ref();
        assert_eq!(gray.name(), Some(c"gray"));
        assert_eq!(gray.alias(), Some(c"gray8,y8"));
        assert_eq!(gray.nb_components(), 1);
        assert_eq!(gray.log2_chroma_w(), 0);
        assert_eq!(gray.log2_chroma_h(), 0);
        assert_eq!(gray.flags(), 0);
        let luma = gray.components().get(0).expect("a first component");
        assert_eq!(
            (
                luma.plane(),
                luma.step(),
                luma.offset(),
                luma.shift(),
                luma.depth()
            ),
            (0, 1, 0, 0, 8)
        );

        // yuv420p pins what gray8 leaves at zero: pixdesc.h numbers the flags
        // BE, PAL, BITSTREAM, HWACCEL, PLANAR from bit 0, and this format is
        // planar and subsampled by two in each direction.
        let planar = first.as_ref();
        assert_eq!(planar.flags(), 1 << 4);
        assert_eq!((planar.log2_chroma_w(), planar.log2_chroma_h()), (1, 1));
        // The `comp` stride is `size_of::<AVComponentDescriptor>()`; C gives
        // the three planes distinct indices, so a wrong stride shows up here.
        let components = planar.components();
        assert_eq!(components.get(0).expect("a luma plane").plane(), 0);
        assert_eq!(components.get(1).expect("a chroma-U plane").plane(), 1);
        assert_eq!(components.get(2).expect("a chroma-V plane").plane(), 2);
    }

    #[test]
    fn table_strings_outlive_the_entry_that_produced_them() {
        // Both strings borrow from the table, not from the handle or the entry
        // the handle came from -- this block would not compile otherwise.
        let (name, alias) = {
            let entry = av_pix_fmt_desc_get(AVPixelFormat::GRAY8).expect("known format");
            (entry.as_ref().name(), entry.as_ref().alias())
        };
        assert_eq!(name, Some(c"gray"));
        assert_eq!(alias, Some(c"gray8,y8"));
        // The alias is not decoration: C accepts each comma-separated spelling
        // in it as a name for the same format.
        assert_eq!(av_get_pix_fmt(c"y8"), AVPixelFormat::GRAY8);
    }

    #[test]
    fn descriptor_iteration_starts_at_a_named_entry() {
        let first = av_pix_fmt_desc_next(None).expect("descriptor table is nonempty");
        assert!(first.as_ref().name().is_some());
        assert!(av_pix_fmt_desc_next(Some(first)).is_some());
    }
}

fn color_from_name<T>(
    name: &CStr,
    parse: unsafe extern "C" fn(*const core::ffi::c_char) -> i32,
    convert: impl FnOnce(i32) -> T,
) -> Result<T, i32> {
    // SAFETY: `name` is a live terminated string and the selected parser only
    // reads it for this call.
    let value = unsafe { parse(name.as_ptr()) };
    if value < 0 {
        Err(value)
    } else {
        Ok(convert(value))
    }
}

/// Wraps: av_color_primaries_from_name
pub fn av_color_primaries_from_name(name: &CStr) -> Result<crate::pixfmt::AVColorPrimaries, i32> {
    color_from_name(name, ffi::av_color_primaries_from_name, |value| {
        crate::pixfmt::AVColorPrimaries::from_raw(value as ffi::AVColorPrimaries)
    })
}

/// Wraps: av_color_range_from_name
pub fn av_color_range_from_name(name: &CStr) -> Result<crate::pixfmt::AVColorRange, i32> {
    color_from_name(name, ffi::av_color_range_from_name, |value| {
        crate::pixfmt::AVColorRange::from_raw(value as ffi::AVColorRange)
    })
}

/// Wraps: av_color_space_from_name
pub fn av_color_space_from_name(name: &CStr) -> Result<crate::pixfmt::AVColorSpace, i32> {
    color_from_name(name, ffi::av_color_space_from_name, |value| {
        crate::pixfmt::AVColorSpace::from_raw(value as ffi::AVColorSpace)
    })
}

/// Wraps: av_color_transfer_from_name
pub fn av_color_transfer_from_name(
    name: &CStr,
) -> Result<crate::pixfmt::AVColorTransferCharacteristic, i32> {
    color_from_name(name, ffi::av_color_transfer_from_name, |value| {
        crate::pixfmt::AVColorTransferCharacteristic::from_raw(
            value as ffi::AVColorTransferCharacteristic,
        )
    })
}

#[cfg(test)]
mod scheduled_name_tests {
    use super::*;
    use crate::pixfmt::{
        AVColorPrimaries, AVColorRange, AVColorSpace, AVColorTransferCharacteristic,
    };

    #[test]
    fn parses_color_property_names() {
        assert_eq!(
            av_color_primaries_from_name(c"bt709"),
            Ok(AVColorPrimaries::BT709)
        );
        assert_eq!(av_color_range_from_name(c"pc"), Ok(AVColorRange::JPEG));
        assert_eq!(
            av_color_space_from_name(c"bt2020nc"),
            Ok(AVColorSpace::BT2020_NCL)
        );
        assert_eq!(
            av_color_transfer_from_name(c"smpte2084"),
            Ok(AVColorTransferCharacteristic::SMPTE2084)
        );
        assert!(av_color_range_from_name(c"missing").is_err());
    }
}
