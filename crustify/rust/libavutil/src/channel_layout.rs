//! Wrappers for libavutil channel layouts.

use core::ffi::{c_char, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CSlice, CSliceMut, CValued, define_ctype};

use crate::ffi;

/// Wraps: AVChannelOrder
///
/// Describes how channels are ordered in an `AVChannelLayout`. The transparent
/// integer representation preserves values introduced by newer libavutil
/// versions instead of turning an unfamiliar C value into an invalid Rust
/// enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVChannelOrder(ffi::AVChannelOrder);

impl AVChannelOrder {
    /// Only the channel count is known.
    pub const UNSPECIFIED: Self = Self(ffi::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC);

    /// Channels follow the order of `AVChannel`.
    pub const NATIVE: Self = Self(ffi::AVChannelOrder_AV_CHANNEL_ORDER_NATIVE);

    /// Channels are described by an explicit map.
    pub const CUSTOM: Self = Self(ffi::AVChannelOrder_AV_CHANNEL_ORDER_CUSTOM);

    /// Channels contain ACN-ordered spherical-harmonic components.
    pub const AMBISONIC: Self = Self(ffi::AVChannelOrder_AV_CHANNEL_ORDER_AMBISONIC);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVChannelOrder) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVChannelOrder {
        self.0
    }
}

impl From<ffi::AVChannelOrder> for AVChannelOrder {
    fn from(raw: ffi::AVChannelOrder) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVChannelOrder> for ffi::AVChannelOrder {
    fn from(order: AVChannelOrder) -> Self {
        order.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn channel_order_is_layout_compatible_and_open() {
        assert_eq!(
            size_of::<AVChannelOrder>(),
            size_of::<ffi::AVChannelOrder>()
        );
        assert_eq!(
            align_of::<AVChannelOrder>(),
            align_of::<ffi::AVChannelOrder>()
        );
        assert_eq!(
            AVChannelOrder::AMBISONIC.as_raw(),
            ffi::AVChannelOrder_AV_CHANNEL_ORDER_AMBISONIC
        );

        let future = ffi::AVChannelOrder::MAX;
        assert_eq!(AVChannelOrder::from_raw(future).as_raw(), future);
    }
}
/// Wraps: AVChannel
///
/// A transparent integer newtype is used instead of a Rust enum because the C
/// API defines every value in the inclusive ambisonic range `0x400..=0x7ff`,
/// not merely the two named endpoints.  It also preserves unknown values from
/// newer libavutil versions without constructing an invalid Rust enum
/// discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVChannel(ffi::AVChannel);

impl AVChannel {
    pub const NONE: Self = Self(ffi::AVChannel_AV_CHAN_NONE);
    pub const FRONT_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_FRONT_LEFT);
    pub const FRONT_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_FRONT_RIGHT);
    pub const FRONT_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_FRONT_CENTER);
    pub const LOW_FREQUENCY: Self = Self(ffi::AVChannel_AV_CHAN_LOW_FREQUENCY);
    pub const BACK_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_BACK_LEFT);
    pub const BACK_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_BACK_RIGHT);
    pub const FRONT_LEFT_OF_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_FRONT_LEFT_OF_CENTER);
    pub const FRONT_RIGHT_OF_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_FRONT_RIGHT_OF_CENTER);
    pub const BACK_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_BACK_CENTER);
    pub const SIDE_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_SIDE_LEFT);
    pub const SIDE_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_SIDE_RIGHT);
    pub const TOP_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_TOP_CENTER);
    pub const TOP_FRONT_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_FRONT_LEFT);
    pub const TOP_FRONT_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_TOP_FRONT_CENTER);
    pub const TOP_FRONT_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_FRONT_RIGHT);
    pub const TOP_BACK_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_BACK_LEFT);
    pub const TOP_BACK_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_TOP_BACK_CENTER);
    pub const TOP_BACK_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_BACK_RIGHT);
    pub const STEREO_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_STEREO_LEFT);
    pub const STEREO_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_STEREO_RIGHT);
    pub const WIDE_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_WIDE_LEFT);
    pub const WIDE_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_WIDE_RIGHT);
    pub const SURROUND_DIRECT_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_SURROUND_DIRECT_LEFT);
    pub const SURROUND_DIRECT_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_SURROUND_DIRECT_RIGHT);
    pub const LOW_FREQUENCY_2: Self = Self(ffi::AVChannel_AV_CHAN_LOW_FREQUENCY_2);
    pub const TOP_SIDE_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_SIDE_LEFT);
    pub const TOP_SIDE_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_SIDE_RIGHT);
    pub const BOTTOM_FRONT_CENTER: Self = Self(ffi::AVChannel_AV_CHAN_BOTTOM_FRONT_CENTER);
    pub const BOTTOM_FRONT_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_BOTTOM_FRONT_LEFT);
    pub const BOTTOM_FRONT_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_BOTTOM_FRONT_RIGHT);
    pub const SIDE_SURROUND_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_SIDE_SURROUND_LEFT);
    pub const SIDE_SURROUND_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_SIDE_SURROUND_RIGHT);
    pub const TOP_SURROUND_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_SURROUND_LEFT);
    pub const TOP_SURROUND_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_TOP_SURROUND_RIGHT);
    pub const BINAURAL_LEFT: Self = Self(ffi::AVChannel_AV_CHAN_BINAURAL_LEFT);
    pub const BINAURAL_RIGHT: Self = Self(ffi::AVChannel_AV_CHAN_BINAURAL_RIGHT);
    pub const UNUSED: Self = Self(ffi::AVChannel_AV_CHAN_UNUSED);
    pub const UNKNOWN: Self = Self(ffi::AVChannel_AV_CHAN_UNKNOWN);
    pub const AMBISONIC_BASE: Self = Self(ffi::AVChannel_AV_CHAN_AMBISONIC_BASE);
    pub const AMBISONIC_END: Self = Self(ffi::AVChannel_AV_CHAN_AMBISONIC_END);

    /// Preserve a channel value received from the C ABI.
    #[inline]
    #[must_use]
    pub const fn from_raw(value: ffi::AVChannel) -> Self {
        Self(value)
    }

    /// Return the integer representation expected by the C ABI.
    #[inline]
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVChannel {
        self.0
    }

    /// Construct the channel for an Ambisonic Channel Number (ACN).
    #[must_use]
    pub const fn ambisonic(acn: u16) -> Option<Self> {
        let value = Self::AMBISONIC_BASE.0 + acn as ffi::AVChannel;
        if value <= Self::AMBISONIC_END.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Return this channel's Ambisonic Channel Number, when applicable.
    #[must_use]
    pub const fn ambisonic_index(self) -> Option<u16> {
        if self.0 >= Self::AMBISONIC_BASE.0 && self.0 <= Self::AMBISONIC_END.0 {
            Some((self.0 - Self::AMBISONIC_BASE.0) as u16)
        } else {
            None
        }
    }
}

impl From<ffi::AVChannel> for AVChannel {
    fn from(value: ffi::AVChannel) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVChannel> for ffi::AVChannel {
    fn from(value: AVChannel) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod avchannel_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn avchannel_is_abi_transparent() {
        assert_eq!(size_of::<AVChannel>(), size_of::<ffi::AVChannel>());
        assert_eq!(align_of::<AVChannel>(), align_of::<ffi::AVChannel>());
        assert_eq!(AVChannel::NONE.as_raw(), -1);
        assert_eq!(AVChannel::FRONT_LEFT.as_raw(), 0);
        assert_eq!(AVChannel::BINAURAL_RIGHT.as_raw(), 62);
    }

    #[test]
    fn every_ambisonic_channel_is_representable() {
        assert_eq!(AVChannel::ambisonic(0), Some(AVChannel::AMBISONIC_BASE));
        assert_eq!(AVChannel::ambisonic(1023), Some(AVChannel::AMBISONIC_END));
        assert_eq!(AVChannel::ambisonic(1024), None);
        assert_eq!(
            AVChannel::ambisonic(511).unwrap().ambisonic_index(),
            Some(511)
        );
        assert_eq!(AVChannel::FRONT_LEFT.ambisonic_index(), None);
    }

    #[test]
    fn unknown_c_values_round_trip_without_invalid_discriminants() {
        let value = AVChannel::from_raw(0x123);
        assert_eq!(value.as_raw(), 0x123);
    }

    #[test]
    fn custom_channel_fields_round_trip_without_c_references() {
        let mut raw = ffi::AVChannelCustom {
            id: ffi::AVChannel_AV_CHAN_UNKNOWN,
            name: [0; 16],
            opaque: core::ptr::null_mut(),
        };
        // SAFETY: `raw` is live and initialized, and this is its only active
        // handle for the duration of the test.
        let mut custom = unsafe { AVChannelCustomMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert!(custom.set_name(b"dialogue"));
        assert!(!custom.set_name(b"sixteen-byte-name"));
        custom.set_id(AVChannel::FRONT_CENTER);
        let cookie = NonNull::<u8>::dangling().cast::<c_void>();
        // SAFETY: the non-null identity is never dereferenced, and is cleared
        // before the temporary backing contract ends.
        unsafe { custom.set_opaque(Some(cookie)) };

        assert_eq!(custom.as_ref().id(), AVChannel::FRONT_CENTER);
        assert_eq!(&custom.as_ref().name()[..9], b"dialogue\0");
        assert!(custom.as_ref().opaque().is_some());
        custom.clear_opaque();
        assert!(custom.as_ref().opaque().is_none());

        assert_eq!(
            size_of::<AVChannelCustom>(),
            size_of::<ffi::AVChannelCustom>()
        );
        assert_eq!(
            align_of::<AVChannelCustom>(),
            align_of::<ffi::AVChannelCustom>()
        );
    }
}

define_ctype!(
    /// Wraps: AVChannelCustom
    ///
    /// One by-value element of an `AV_CHANNEL_ORDER_CUSTOM` channel map. It
    /// owns no allocation: the enclosing layout owns the element array, while
    /// `opaque` remains application-managed even when libavutil copies a map.
    AVChannelCustom,
    AVChannelCustomRef,
    AVChannelCustomMut,
    ffi::AVChannelCustom
);

/// A lifetime-bound identity token for application-managed channel metadata.
///
/// It intentionally exposes no dereference operation: C records and copies the
/// erased address but neither knows nor manages the pointee type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVChannelCustomOpaque<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

impl AVChannelCustomRef<'_> {
    /// Wraps: AVChannelCustom.opaque
    ///
    /// Returns an identity-only token for the application-managed cookie.
    #[must_use]
    pub fn opaque(&self) -> Option<AVChannelCustomOpaque<'_>> {
        // SAFETY: the pointer value is copied through a raw projection; no
        // reference to either the C object or the erased pointee is formed.
        NonNull::new(unsafe { addr_of!((*self.as_ptr()).opaque).read() }).map(|pointer| {
            AVChannelCustomOpaque {
                pointer,
                _borrow: PhantomData,
            }
        })
    }

    /// Wraps: AVChannelCustom.name
    ///
    /// Copies all 16 bytes of the fixed-size, NUL-terminated-or-zero name.
    #[must_use]
    pub fn name(&self) -> [u8; 16] {
        let mut name = [0_u8; 16];
        // SAFETY: the handle addresses a live initialized element; the field
        // contains exactly 16 bytes and the destination is disjoint Rust
        // storage. Copying forms no reference to the C object.
        unsafe {
            core::ptr::copy_nonoverlapping(
                addr_of!((*self.as_ptr()).name).cast::<u8>(),
                name.as_mut_ptr(),
                name.len(),
            );
        }
        name
    }

    /// Wraps: AVChannelCustom.id
    #[must_use]
    pub fn id(&self) -> AVChannel {
        // SAFETY: the integer-backed C enum is copied through a raw projection
        // and AVChannel preserves every possible ABI value.
        AVChannel::from_raw(unsafe { addr_of!((*self.as_ptr()).id).read() })
    }
}

impl AVChannelCustomMut<'_> {
    /// Clears the application-managed metadata cookie.
    pub fn clear_opaque(&mut self) {
        // SAFETY: the exclusive handle permits replacing the pointer value;
        // libavutil never owns or frees its pointee.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).opaque).write(core::ptr::null_mut()) }
    }

    /// Stores an application-managed, type-erased metadata address.
    ///
    /// # Safety
    ///
    /// The pointee must remain alive and permit any external access made
    /// through this address until this element and every layout copy containing
    /// it have either been cleared or destroyed. Libavutil copies but never
    /// dereferences or frees the address.
    pub unsafe fn set_opaque(&mut self, pointer: Option<NonNull<c_void>>) {
        // SAFETY: the caller supplies the erased-pointee lifetime contract and
        // the exclusive handle permits replacing this pointer field.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).opaque)
                .write(pointer.map_or(core::ptr::null_mut(), NonNull::as_ptr));
        }
    }

    /// Sets the channel name, rejecting embedded NULs and payloads longer than
    /// the 15 bytes available before the terminator.
    pub fn set_name(&mut self, name: &[u8]) -> bool {
        if name.len() >= 16 || name.contains(&0) {
            return false;
        }
        let mut stored = [0 as c_char; 16];
        for (dst, src) in stored.iter_mut().zip(name.iter().copied()) {
            *dst = src as c_char;
        }
        // SAFETY: the exclusive handle permits replacing the complete array;
        // `stored` is zero-filled after the payload and is therefore terminated.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).name).write(stored) }
        true
    }

    /// Sets the channel identifier.
    pub fn set_id(&mut self, id: AVChannel) {
        // SAFETY: the exclusive handle permits replacing this integer-backed
        // enum field, and AVChannel is ABI-transparent.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).id).write(id.as_raw()) }
    }
}

use core::ffi::CStr;

#[derive(Clone, Copy, Debug)]
pub struct ChannelText<'a> {
    pub required: usize,
    pub text: Option<&'a CStr>,
}

fn channel_text<'a>(
    buffer: &'a mut [u8],
    channel: AVChannel,
    describe: bool,
) -> Result<ChannelText<'a>, i32> {
    // SAFETY: `buffer` supplies exactly its length writable bytes and remains
    // exclusively borrowed. C writes at most that extent and retains nothing.
    let status = unsafe {
        if describe {
            ffi::av_channel_description(
                buffer.as_mut_ptr().cast::<c_char>(),
                buffer.len(),
                channel.as_raw(),
            )
        } else {
            ffi::av_channel_name(
                buffer.as_mut_ptr().cast::<c_char>(),
                buffer.len(),
                channel.as_raw(),
            )
        }
    };
    if status < 0 {
        return Err(status);
    }
    let text = CStr::from_bytes_until_nul(buffer).ok();
    Ok(ChannelText {
        required: status as usize,
        text,
    })
}

/// Wraps: av_channel_description
pub fn av_channel_description(
    buffer: &mut [u8],
    channel: AVChannel,
) -> Result<ChannelText<'_>, i32> {
    channel_text(buffer, channel, true)
}

/// Wraps: av_channel_from_string
#[must_use]
pub fn av_channel_from_string(name: &CStr) -> AVChannel {
    // SAFETY: `name` is a live NUL-terminated string and is not retained.
    AVChannel::from_raw(unsafe { ffi::av_channel_from_string(name.as_ptr()) })
}

/// Wraps: av_channel_name
pub fn av_channel_name(buffer: &mut [u8], channel: AVChannel) -> Result<ChannelText<'_>, i32> {
    channel_text(buffer, channel, false)
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn channel_names_and_descriptions_are_bounded() {
        assert_eq!(av_channel_from_string(c"FL"), AVChannel::FRONT_LEFT);

        let mut name = [0_u8; 16];
        let result = av_channel_name(&mut name, AVChannel::FRONT_LEFT).unwrap();
        assert_eq!(result.text, Some(c"FL"));

        let mut description = [0_u8; 64];
        let result = av_channel_description(&mut description, AVChannel::FRONT_LEFT).unwrap();
        assert_eq!(result.text, Some(c"front left"));
        assert!(result.required <= description.len());
    }
}

define_ctype!(
    /// Wraps: AVChannelLayout
    ///
    /// A public, by-value channel layout. Owned inline values use
    /// `CVal<AVChannelLayout>` so a custom channel map is always released with
    /// `av_channel_layout_uninit`; borrowed values use the generated handles.
    AVChannelLayout,
    AVChannelLayoutRef,
    AVChannelLayoutMut,
    ffi::AVChannelLayout
);

// SAFETY: `av_channel_layout_uninit` releases only resources owned by the
// by-value layout (the CUSTOM map), retains the header storage, and resets the
// complete header. `CVal` invokes it exactly once before releasing its inline
// Rust storage.
unsafe impl CValued for AVChannelLayout {
    unsafe fn c_dispose(this: NonNull<Self>) {
        // SAFETY: the trait contract supplies one live initialized layout; the
        // transparent wrapper has the exact layout expected by C.
        unsafe { ffi::av_channel_layout_uninit(this.as_ptr().cast()) }
    }
}

/// A lifetime-bound identity token for an application-managed layout cookie.
///
/// Libavutil preserves the address but neither dereferences nor frees it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVChannelLayoutOpaque<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

/// The union member selected by an `AVChannelLayout`'s channel order.
#[derive(Clone, Copy)]
pub enum AVChannelLayoutChannels<'a> {
    /// `AV_CHANNEL_ORDER_UNSPEC`; the union bytes are intentionally unread.
    Unspecified,
    /// A native or ambisonic order using the union's mask member.
    Mask { order: AVChannelOrder, mask: u64 },
    /// A custom order owning `nb_channels` by-value map entries.
    Custom(CSlice<'a, AVChannelCustom>),
    /// An unknown order or an internally inconsistent custom layout.
    Invalid,
}

impl<'a> AVChannelLayoutRef<'a> {
    /// Wraps: AVChannelLayout.order
    #[must_use]
    pub fn order(&self) -> AVChannelOrder {
        // SAFETY: the integer-backed enum is copied through a raw projection;
        // AVChannelOrder preserves every possible ABI value.
        AVChannelOrder::from_raw(unsafe { addr_of!((*self.as_ptr()).order).read() })
    }

    /// Wraps: AVChannelLayout.nb_channels
    #[must_use]
    pub fn nb_channels(&self) -> i32 {
        // SAFETY: the scalar is copied through a raw field projection from the
        // live layout; no reference to C storage is formed.
        unsafe { addr_of!((*self.as_ptr()).nb_channels).read() }
    }

    /// Wraps: AVChannelLayout.opaque
    ///
    /// Returns the application-managed cookie as an identity-only token.
    #[must_use]
    pub fn opaque(&self) -> Option<AVChannelLayoutOpaque<'a>> {
        // SAFETY: only the pointer value is copied. Libavutil never accesses
        // the erased pointee and the returned token is tied to the layout view.
        NonNull::new(unsafe { addr_of!((*self.as_ptr()).opaque).read() }).map(|pointer| {
            AVChannelLayoutOpaque {
                pointer,
                _borrow: PhantomData,
            }
        })
    }

    /// Wraps: AVChannelLayout.u.mask
    ///
    /// Reads the union member only for orders that define it.
    #[must_use]
    pub fn mask(&self) -> Option<u64> {
        let order = self.order();
        if order != AVChannelOrder::NATIVE && order != AVChannelOrder::AMBISONIC {
            return None;
        }
        // SAFETY: NATIVE and AMBISONIC layouts define the mask union member;
        // copying it through a raw projection forms no C-memory reference.
        Some(unsafe { addr_of!((*self.as_ptr()).u.mask).read() })
    }

    /// Wraps: AVChannelLayout.u.map
    ///
    /// Borrows the owned custom map when the discriminator and length are
    /// valid. The view yields per-element handles rather than `&[T]`.
    #[must_use]
    pub fn custom_map(&self) -> Option<CSlice<'a, AVChannelCustom>> {
        if self.order() != AVChannelOrder::CUSTOM {
            return None;
        }
        let len = usize::try_from(self.nb_channels()).ok()?;
        // SAFETY: CUSTOM selects the map union member. The layout owns a live
        // `nb_channels`-element allocation for the duration of this borrow.
        let map = unsafe { addr_of!((*self.as_ptr()).u.map).read() };
        NonNull::new(map.cast::<AVChannelCustom>()).map(|map| {
            // SAFETY: the CUSTOM layout invariant establishes `len`
            // contiguous initialized entries kept alive by this layout.
            unsafe { CSlice::from_raw_parts(map, len) }
        })
    }

    /// Wraps: AVChannelLayout.u
    ///
    /// Interprets the union only through the active member selected by order.
    #[must_use]
    pub fn channels(&self) -> AVChannelLayoutChannels<'a> {
        let order = self.order();
        if order == AVChannelOrder::UNSPECIFIED {
            AVChannelLayoutChannels::Unspecified
        } else if order == AVChannelOrder::NATIVE || order == AVChannelOrder::AMBISONIC {
            AVChannelLayoutChannels::Mask {
                order,
                // The order check above proves the union member is active.
                mask: self.mask().expect("mask order has a mask member"),
            }
        } else if order == AVChannelOrder::CUSTOM {
            self.custom_map().map_or(
                AVChannelLayoutChannels::Invalid,
                AVChannelLayoutChannels::Custom,
            )
        } else {
            AVChannelLayoutChannels::Invalid
        }
    }
}

impl AVChannelLayoutMut<'_> {
    /// Exclusively borrows the custom map when the layout is valid and custom.
    #[must_use]
    pub fn custom_map_mut(&mut self) -> Option<CSliceMut<'_, AVChannelCustom>> {
        let shared = self.as_ref();
        if shared.order() != AVChannelOrder::CUSTOM {
            return None;
        }
        let len = usize::try_from(shared.nb_channels()).ok()?;
        // SAFETY: the exclusive layout handle licenses reading the active map
        // pointer and grants exclusive access to its owned entry array.
        let map = unsafe { addr_of!((*self.as_mut_ptr()).u.map).read() };
        NonNull::new(map.cast::<AVChannelCustom>()).map(|map| {
            // SAFETY: CUSTOM guarantees `len` initialized entries and the view
            // is bound to the exclusive borrow of this layout handle.
            unsafe { CSliceMut::from_raw_parts(map, len) }
        })
    }

    /// Clears the application-managed metadata cookie.
    pub fn clear_opaque(&mut self) {
        // SAFETY: the exclusive handle permits replacing the pointer value and
        // neither this wrapper nor libavutil owns its pointee.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).opaque).write(core::ptr::null_mut()) }
    }

    /// Stores an application-managed, type-erased cookie address.
    ///
    /// # Safety
    ///
    /// The pointee must remain alive until this layout and every deep layout
    /// copy retaining the cookie have been cleared or destroyed. Libavutil
    /// copies the address but does not manage the pointee.
    pub unsafe fn set_opaque(&mut self, pointer: Option<NonNull<c_void>>) {
        // SAFETY: the caller supplies the external lifetime contract and the
        // exclusive handle permits replacing this field.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).opaque)
                .write(pointer.map_or(core::ptr::null_mut(), NonNull::as_ptr));
        }
    }
}

#[cfg(test)]
mod channel_layout_type_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn layout_union_is_discriminator_checked() {
        let mut raw = ffi::AVChannelLayout {
            order: ffi::AVChannelOrder_AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2,
            u: ffi::AVChannelLayout__bindgen_ty_1 { mask: 3 },
            opaque: core::ptr::null_mut(),
        };
        // SAFETY: `raw` is live and initialized and this shared handle is the
        // only access path used for its duration.
        let layout = unsafe { AVChannelLayoutRef::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(layout.order(), AVChannelOrder::NATIVE);
        assert_eq!(layout.nb_channels(), 2);
        assert_eq!(layout.mask(), Some(3));
        assert!(layout.custom_map().is_none());
        assert!(matches!(
            layout.channels(),
            AVChannelLayoutChannels::Mask { mask: 3, .. }
        ));
        assert_eq!(
            size_of::<AVChannelLayout>(),
            size_of::<ffi::AVChannelLayout>()
        );
        assert_eq!(
            align_of::<AVChannelLayout>(),
            align_of::<ffi::AVChannelLayout>()
        );
    }

    #[test]
    fn custom_map_uses_element_handles() {
        let mut entries = [
            ffi::AVChannelCustom {
                id: ffi::AVChannel_AV_CHAN_FRONT_LEFT,
                name: [0; 16],
                opaque: core::ptr::null_mut(),
            },
            ffi::AVChannelCustom {
                id: ffi::AVChannel_AV_CHAN_FRONT_RIGHT,
                name: [0; 16],
                opaque: core::ptr::null_mut(),
            },
        ];
        let mut raw = ffi::AVChannelLayout {
            order: ffi::AVChannelOrder_AV_CHANNEL_ORDER_CUSTOM,
            nb_channels: 2,
            u: ffi::AVChannelLayout__bindgen_ty_1 {
                map: entries.as_mut_ptr(),
            },
            opaque: core::ptr::null_mut(),
        };
        // SAFETY: the layout and its two-entry stack map remain live, and the
        // exclusive handle is the only access path during this scope.
        let mut layout = unsafe { AVChannelLayoutMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(layout.as_ref().custom_map().unwrap().len(), 2);
        let mut map = layout.custom_map_mut().unwrap();
        map.get_mut(1).unwrap().set_id(AVChannel::FRONT_CENTER);
        assert_eq!(map.as_ref().get(1).unwrap().id(), AVChannel::FRONT_CENTER);
    }

    #[test]
    fn zeroed_inline_layout_is_safely_disposed() {
        let layout = CVal::new(AVChannelLayout::zeroed());
        assert_eq!(layout.as_ref().order(), AVChannelOrder::UNSPECIFIED);
        assert_eq!(layout.as_ref().nb_channels(), 0);
        drop(layout);
    }
}
