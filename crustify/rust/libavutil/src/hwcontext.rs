//! Wrappers for libavutil hardware contexts.

use core::ffi::CStr;
use core::ptr::{NonNull, addr_of_mut};

use ffibox::{CBox, CVec};

use crate::buffer::{AVBufferReference, AVBufferReferenceRef};
use crate::dict::AVDictionaryRef;
use crate::ffi;
use crate::mem::AvFree;
use crate::pixfmt::AVPixelFormat;

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

/// An allocated device context that is still in its configuration phase.
/// Initialization consumes this state, preventing safe code from initializing
/// the same context twice.
pub struct HWDeviceContextUninit(CBox<AVBufferReference>);

/// An initialized hardware device context.
pub struct HWDeviceContext(CBox<AVBufferReference>);

impl HWDeviceContext {
    /// Fallibly creates another independently releasable reference to the same
    /// initialized context.
    #[must_use]
    pub fn try_clone(&self) -> Option<Self> {
        self.0.try_clone().map(Self)
    }

    fn buffer_ref(&self) -> AVBufferReferenceRef<'_> {
        self.0.as_ref()
    }
}

/// An allocated frames context that is still being configured.
pub struct HWFramesContextUninit(CBox<AVBufferReference>);

/// An initialized hardware frames context.
pub struct HWFramesContext(CBox<AVBufferReference>);

impl HWFramesContext {
    /// Fallibly creates another independently releasable reference to the same
    /// initialized context.
    #[must_use]
    pub fn try_clone(&self) -> Option<Self> {
        self.0.try_clone().map(Self)
    }

    fn buffer_ref(&self) -> AVBufferReferenceRef<'_> {
        self.0.as_ref()
    }
}

/// Wraps: av_hwdevice_ctx_alloc
#[must_use]
pub fn av_hwdevice_ctx_alloc(device_type: AVHWDeviceType) -> Option<HWDeviceContextUninit> {
    // SAFETY: a non-null return is a fully constructed owned AVBufferRef. Its
    // data contains the construction-phase device context, while the header's
    // ordinary AVBufferReference lifecycle remains valid immediately.
    unsafe { CBox::from_raw(ffi::av_hwdevice_ctx_alloc(device_type.as_raw())) }
        .map(HWDeviceContextUninit)
}

/// Wraps: av_hwdevice_ctx_create
///
/// Creates and initializes a device context in one operation.
pub fn av_hwdevice_ctx_create(
    device_type: AVHWDeviceType,
    device: Option<&CStr>,
    options: Option<AVDictionaryRef<'_>>,
    flags: i32,
) -> Result<HWDeviceContext, i32> {
    let mut raw = core::ptr::null_mut();
    // SAFETY: the output slot is writable and initially null; strings and the
    // optional dictionary remain live for the call and are not retained.
    let status = unsafe {
        ffi::av_hwdevice_ctx_create(
            addr_of_mut!(raw),
            device_type.as_raw(),
            device.map_or(core::ptr::null(), CStr::as_ptr),
            options.map_or(core::ptr::null_mut(), |dictionary| {
                dictionary.as_ptr().cast_mut()
            }),
            flags,
        )
    };
    if status < 0 {
        return Err(status);
    }
    // SAFETY: success writes one fully constructed, initialized owned reference.
    let owner =
        unsafe { CBox::from_raw(raw) }.expect("successful device creation returned an owner");
    Ok(HWDeviceContext(owner))
}

/// Wraps: av_hwdevice_ctx_create_derived
pub fn av_hwdevice_ctx_create_derived(
    device_type: AVHWDeviceType,
    source: &HWDeviceContext,
    flags: i32,
) -> Result<HWDeviceContext, i32> {
    let mut raw = core::ptr::null_mut();
    // SAFETY: the source is a live initialized device context and the output
    // slot receives a new independently releasable reference or remains null.
    let status = unsafe {
        ffi::av_hwdevice_ctx_create_derived(
            addr_of_mut!(raw),
            device_type.as_raw(),
            source.buffer_ref().as_ptr().cast_mut(),
            flags,
        )
    };
    if status < 0 {
        return Err(status);
    }
    // SAFETY: successful derivation writes one initialized owned reference.
    let owner = unsafe { CBox::from_raw(raw) }.expect("successful derivation returned an owner");
    Ok(HWDeviceContext(owner))
}

/// Wraps: av_hwdevice_ctx_create_derived_opts
pub fn av_hwdevice_ctx_create_derived_opts(
    device_type: AVHWDeviceType,
    source: &HWDeviceContext,
    options: Option<AVDictionaryRef<'_>>,
    flags: i32,
) -> Result<HWDeviceContext, i32> {
    let mut raw = core::ptr::null_mut();
    // SAFETY: the source is initialized and borrowed for the call; the
    // dictionary is optional borrowed input, and the output slot is writable.
    let status = unsafe {
        ffi::av_hwdevice_ctx_create_derived_opts(
            addr_of_mut!(raw),
            device_type.as_raw(),
            source.buffer_ref().as_ptr().cast_mut(),
            options.map_or(core::ptr::null_mut(), |dictionary| {
                dictionary.as_ptr().cast_mut()
            }),
            flags,
        )
    };
    if status < 0 {
        return Err(status);
    }
    // SAFETY: successful derivation writes one initialized owned reference.
    let owner = unsafe { CBox::from_raw(raw) }.expect("successful derivation returned an owner");
    Ok(HWDeviceContext(owner))
}

/// Wraps: av_hwdevice_ctx_init
///
/// Promotes a construction-phase context only after libavutil reports that its
/// device-specific initialization succeeded. On failure, ownership and the
/// uninitialized state are returned for inspection or retry.
pub fn av_hwdevice_ctx_init(
    context: HWDeviceContextUninit,
) -> Result<HWDeviceContext, (i32, HWDeviceContextUninit)> {
    // SAFETY: the type state proves this is a live construction-phase device
    // context, exclusively owned for the duration of initialization.
    let status = unsafe { ffi::av_hwdevice_ctx_init(context.0.as_ptr()) };
    if status < 0 {
        Err((status, context))
    } else {
        Ok(HWDeviceContext(context.0))
    }
}

/// Wraps: av_hwframe_ctx_alloc
#[must_use]
pub fn av_hwframe_ctx_alloc(device: &HWDeviceContext) -> Option<HWFramesContextUninit> {
    // SAFETY: the typed source is a live initialized device context. A
    // non-null result transfers one fresh frames-context reference to Rust.
    unsafe {
        CBox::from_raw(ffi::av_hwframe_ctx_alloc(
            device.buffer_ref().as_ptr().cast_mut(),
        ))
    }
    .map(HWFramesContextUninit)
}

/// Wraps: av_hwframe_ctx_init
pub fn av_hwframe_ctx_init(
    context: HWFramesContextUninit,
) -> Result<HWFramesContext, (i32, HWFramesContextUninit)> {
    // SAFETY: the type state proves this is a live construction-phase frames
    // context, exclusively owned during the call.
    let status = unsafe { ffi::av_hwframe_ctx_init(context.0.as_ptr()) };
    if status < 0 {
        Err((status, context))
    } else {
        Ok(HWFramesContext(context.0))
    }
}

/// An owned, nonempty list of pixel formats returned by a hardware backend.
/// The sentinel is retained inside the private allocation but omitted from the
/// safe indexed view.
pub struct HWFrameTransferFormats {
    allocation: CVec<ffi::AVPixelFormat, AvFree>,
    len: usize,
}

impl HWFrameTransferFormats {
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<AVPixelFormat> {
        self.allocation
            .as_slice()
            .get(index)
            .copied()
            .map(AVPixelFormat::from_raw)
    }

    pub fn iter(&self) -> HWFrameTransferFormatIter<'_> {
        HWFrameTransferFormatIter {
            formats: self,
            index: 0,
        }
    }
}

pub struct HWFrameTransferFormatIter<'a> {
    formats: &'a HWFrameTransferFormats,
    index: usize,
}

impl Iterator for HWFrameTransferFormatIter<'_> {
    type Item = AVPixelFormat;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.formats.get(self.index)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.formats.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for HWFrameTransferFormatIter<'_> {}

/// Wraps: av_hwframe_transfer_get_formats
///
/// Queries the backend with the API-mandated zero flags and adopts the
/// sentinel-terminated result into an allocator-matched owner.
pub fn av_hwframe_transfer_get_formats(
    context: &HWFramesContext,
    direction: AVHWFrameTransferDirection,
) -> Result<HWFrameTransferFormats, i32> {
    let mut formats = core::ptr::null_mut();
    // SAFETY: the typed context is initialized, the output slot is writable,
    // and zero is the only documented flags value.
    let status = unsafe {
        ffi::av_hwframe_transfer_get_formats(
            context.buffer_ref().as_ptr().cast_mut(),
            direction.as_raw(),
            addr_of_mut!(formats),
            0,
        )
    };
    if status < 0 {
        return Err(status);
    }
    let formats = NonNull::new(formats).expect("successful format query returned a list");
    let mut len = 0usize;
    // SAFETY: success guarantees a readable AVPixelFormat array terminated by
    // AV_PIX_FMT_NONE. Each step stays inside that allocation through the
    // guaranteed sentinel, and no other owner or writer exists.
    while unsafe { formats.as_ptr().add(len).read() } != ffi::AVPixelFormat_AV_PIX_FMT_NONE {
        len += 1;
    }
    // SAFETY: the pointer is a uniquely owned av_malloc-family allocation with
    // `len` initialized formats plus the initialized sentinel.
    let allocation = unsafe { CVec::from_raw_parts(formats.as_ptr(), len + 1) }
        .expect("checked non-null format list");
    Ok(HWFrameTransferFormats { allocation, len })
}

#[cfg(test)]
mod scheduled_context_tests {
    use super::*;
    use crate::buffer::av_buffer_get_ref_count;

    #[test]
    fn allocation_uses_typed_construction_state_and_refcounting() {
        assert!(av_hwdevice_ctx_alloc(AVHWDeviceType::NONE).is_none());
        let device_type = av_hwdevice_iterate_types(AVHWDeviceType::NONE);
        if device_type == AVHWDeviceType::NONE {
            return;
        }
        let context = av_hwdevice_ctx_alloc(device_type).expect("compiled-in device allocates");
        assert_eq!(av_buffer_get_ref_count(context.0.as_ref()), 1);
        let clone = context.0.try_clone().expect("context reference clone");
        assert_eq!(av_buffer_get_ref_count(context.0.as_ref()), 2);
        drop(clone);
    }
}
