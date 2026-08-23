//! Wrappers for libavutil channel layouts.

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
}
