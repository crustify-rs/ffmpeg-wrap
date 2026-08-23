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
