//! Wrappers for libavutil hardware contexts.

use crate::ffi;

/// Wraps: AVHWDeviceType
///
/// Identifies the hardware API backing a device context. The transparent
/// representation also preserves values introduced by newer libavutil
/// versions rather than turning an unknown C value into an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVHWDeviceType(ffi::AVHWDeviceType);

impl AVHWDeviceType {
    pub const NONE: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_NONE);
    pub const VDPAU: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VDPAU);
    pub const CUDA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA);
    pub const VAAPI: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VAAPI);
    pub const DXVA2: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DXVA2);
    pub const QSV: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_QSV);
    pub const VIDEOTOOLBOX: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VIDEOTOOLBOX);
    pub const D3D11VA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D11VA);
    pub const DRM: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DRM);
    pub const OPENCL: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_OPENCL);
    pub const MEDIACODEC: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_MEDIACODEC);
    pub const VULKAN: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VULKAN);
    pub const D3D12VA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D12VA);
    pub const AMF: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_AMF);
    pub const OHCODEC: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_OHCODEC);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVHWDeviceType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVHWDeviceType {
        self.0
    }
}

impl From<ffi::AVHWDeviceType> for AVHWDeviceType {
    fn from(raw: ffi::AVHWDeviceType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVHWDeviceType> for ffi::AVHWDeviceType {
    fn from(value: AVHWDeviceType) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVHWFrameTransferDirection
///
/// Selects whether formats are queried as sources of, or targets for, a
/// hardware-frame transfer. Unknown C values remain representable without
/// creating an invalid Rust enum.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVHWFrameTransferDirection(ffi::AVHWFrameTransferDirection);

impl AVHWFrameTransferDirection {
    pub const FROM: Self = Self(ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_FROM);
    pub const TO: Self = Self(ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_TO);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVHWFrameTransferDirection) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVHWFrameTransferDirection {
        self.0
    }
}

impl From<ffi::AVHWFrameTransferDirection> for AVHWFrameTransferDirection {
    fn from(raw: ffi::AVHWFrameTransferDirection) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVHWFrameTransferDirection> for ffi::AVHWFrameTransferDirection {
    fn from(value: AVHWFrameTransferDirection) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn device_type_is_layout_compatible_and_round_trips() {
        assert_eq!(
            size_of::<AVHWDeviceType>(),
            size_of::<ffi::AVHWDeviceType>()
        );
        assert_eq!(
            align_of::<AVHWDeviceType>(),
            align_of::<ffi::AVHWDeviceType>()
        );
        assert_eq!(
            AVHWDeviceType::VULKAN.as_raw(),
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VULKAN
        );

        let unknown = ffi::AVHWDeviceType::MAX;
        assert_eq!(AVHWDeviceType::from_raw(unknown).as_raw(), unknown);
    }

    #[test]
    fn transfer_direction_is_layout_compatible_and_round_trips() {
        assert_eq!(
            size_of::<AVHWFrameTransferDirection>(),
            size_of::<ffi::AVHWFrameTransferDirection>()
        );
        assert_eq!(
            align_of::<AVHWFrameTransferDirection>(),
            align_of::<ffi::AVHWFrameTransferDirection>()
        );
        assert_eq!(
            AVHWFrameTransferDirection::FROM.as_raw(),
            ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_FROM
        );

        let unknown = ffi::AVHWFrameTransferDirection::MAX;
        assert_eq!(
            AVHWFrameTransferDirection::from_raw(unknown).as_raw(),
            unknown
        );
    }
}

use core::ffi::CStr;

/// Wraps: av_hwdevice_find_type_by_name
#[must_use]
pub fn av_hwdevice_find_type_by_name(name: &CStr) -> AVHWDeviceType {
    // SAFETY: `name` is a readable NUL-terminated string and is not retained.
    AVHWDeviceType::from_raw(unsafe { ffi::av_hwdevice_find_type_by_name(name.as_ptr()) })
}

/// Wraps: av_hwdevice_get_type_name
#[must_use]
pub fn av_hwdevice_get_type_name(device_type: AVHWDeviceType) -> Option<&'static CStr> {
    // SAFETY: C returns null or an immutable process-lifetime table string.
    let pointer = unsafe { ffi::av_hwdevice_get_type_name(device_type.as_raw()) };
    if pointer.is_null() {
        None
    } else {
        // SAFETY: the checked pointer is a static NUL-terminated name.
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

/// Wraps: av_hwdevice_iterate_types
#[must_use]
pub fn av_hwdevice_iterate_types(previous: AVHWDeviceType) -> AVHWDeviceType {
    // SAFETY: the device type is passed and returned by value.
    AVHWDeviceType::from_raw(unsafe { ffi::av_hwdevice_iterate_types(previous.as_raw()) })
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn device_type_names_round_trip_when_compiled_in() {
        let first = av_hwdevice_iterate_types(AVHWDeviceType::NONE);
        if first != AVHWDeviceType::NONE {
            let name = av_hwdevice_get_type_name(first).expect("iterated type has a name");
            assert_eq!(av_hwdevice_find_type_by_name(name), first);
        }
        assert_eq!(
            av_hwdevice_find_type_by_name(c"not-a-device"),
            AVHWDeviceType::NONE
        );
    }
}
