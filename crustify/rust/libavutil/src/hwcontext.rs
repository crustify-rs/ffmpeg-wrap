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
///
/// Holding an unrecognized value is safe at every libavutil entry point that
/// takes this type: the name lookup range-checks its table, and the context
/// constructors and derivations search the backend table by equality. An
/// unknown value is therefore reported as "no such device" and never used as
/// an unchecked index, which is what makes [`from_raw`](Self::from_raw) safe.
///
/// Each constant below names the string libavutil uses for it in
/// [`av_hwdevice_get_type_name`] and [`av_hwdevice_find_type_by_name`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVHWDeviceType(ffi::AVHWDeviceType);

impl AVHWDeviceType {
    /// No device. This is the value returned for an unrecognized device name
    /// and the terminator of [`av_hwdevice_iterate_types`]; no context can be
    /// allocated for it.
    pub const NONE: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_NONE);
    /// VDPAU (`vdpau`).
    pub const VDPAU: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VDPAU);
    /// CUDA (`cuda`).
    pub const CUDA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA);
    /// VA-API (`vaapi`).
    pub const VAAPI: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VAAPI);
    /// DXVA2 (`dxva2`).
    pub const DXVA2: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DXVA2);
    /// Intel Quick Sync Video (`qsv`).
    pub const QSV: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_QSV);
    /// VideoToolbox (`videotoolbox`).
    pub const VIDEOTOOLBOX: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VIDEOTOOLBOX);
    /// Direct3D 11 video acceleration (`d3d11va`).
    pub const D3D11VA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D11VA);
    /// Linux DRM (`drm`).
    pub const DRM: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DRM);
    /// OpenCL (`opencl`).
    pub const OPENCL: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_OPENCL);
    /// Android MediaCodec (`mediacodec`).
    pub const MEDIACODEC: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_MEDIACODEC);
    /// Vulkan (`vulkan`).
    pub const VULKAN: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VULKAN);
    /// Direct3D 12 video acceleration (`d3d12va`).
    pub const D3D12VA: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D12VA);
    /// AMD Advanced Media Framework (`amf`).
    pub const AMF: Self = Self(ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_AMF);
    /// OpenHarmony codec (`ohcodec`).
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
///
/// The value reaches a hardware backend's format query, where every in-tree
/// backend either compares it against the two documented directions or
/// ignores it. No backend indexes with it, so an unrecognized value selects a
/// backend's default answer rather than reading out of bounds.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVHWFrameTransferDirection(ffi::AVHWFrameTransferDirection);

impl AVHWFrameTransferDirection {
    /// Query the formats data can be transferred *from* the hardware frame
    /// into.
    pub const FROM: Self = Self(ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_FROM);
    /// Query the formats data can be transferred *to* the hardware frame
    /// from.
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

    /// Every device-type constant, the binding it must equal, and the name
    /// libavutil publishes for it. The name table is compiled unconditionally,
    /// so these names hold whatever backends this build enables.
    const DEVICE_TYPES: &[(AVHWDeviceType, ffi::AVHWDeviceType, Option<&CStr>)] = &[
        (
            AVHWDeviceType::NONE,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_NONE,
            None,
        ),
        (
            AVHWDeviceType::VDPAU,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VDPAU,
            Some(c"vdpau"),
        ),
        (
            AVHWDeviceType::CUDA,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA,
            Some(c"cuda"),
        ),
        (
            AVHWDeviceType::VAAPI,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VAAPI,
            Some(c"vaapi"),
        ),
        (
            AVHWDeviceType::DXVA2,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DXVA2,
            Some(c"dxva2"),
        ),
        (
            AVHWDeviceType::QSV,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_QSV,
            Some(c"qsv"),
        ),
        (
            AVHWDeviceType::VIDEOTOOLBOX,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VIDEOTOOLBOX,
            Some(c"videotoolbox"),
        ),
        (
            AVHWDeviceType::D3D11VA,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D11VA,
            Some(c"d3d11va"),
        ),
        (
            AVHWDeviceType::DRM,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_DRM,
            Some(c"drm"),
        ),
        (
            AVHWDeviceType::OPENCL,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_OPENCL,
            Some(c"opencl"),
        ),
        (
            AVHWDeviceType::MEDIACODEC,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_MEDIACODEC,
            Some(c"mediacodec"),
        ),
        (
            AVHWDeviceType::VULKAN,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_VULKAN,
            Some(c"vulkan"),
        ),
        (
            AVHWDeviceType::D3D12VA,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_D3D12VA,
            Some(c"d3d12va"),
        ),
        (
            AVHWDeviceType::AMF,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_AMF,
            Some(c"amf"),
        ),
        (
            AVHWDeviceType::OHCODEC,
            ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_OHCODEC,
            Some(c"ohcodec"),
        ),
    ];

    const TRANSFER_DIRECTIONS: &[(AVHWFrameTransferDirection, ffi::AVHWFrameTransferDirection)] = &[
        (
            AVHWFrameTransferDirection::FROM,
            ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_FROM,
        ),
        (
            AVHWFrameTransferDirection::TO,
            ffi::AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_TO,
        ),
    ];

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

        let unknown = ffi::AVHWDeviceType::MAX;
        assert_eq!(AVHWDeviceType::from_raw(unknown).as_raw(), unknown);
    }

    /// A constant bound to the wrong sibling would still compile and still
    /// round-trip, so pin each one to its binding and to the name the linked
    /// libavutil reports for that value.
    #[test]
    fn device_type_constants_match_the_c_enum_and_its_name_table() {
        for (index, &(wrapped, raw, name)) in DEVICE_TYPES.iter().enumerate() {
            assert_eq!(wrapped.as_raw(), raw);
            assert_eq!(wrapped, AVHWDeviceType::from(raw));
            assert_eq!(ffi::AVHWDeviceType::from(wrapped), raw);
            assert_eq!(av_hwdevice_get_type_name(wrapped), name);
            if let Some(name) = name {
                assert_eq!(av_hwdevice_find_type_by_name(name), wrapped);
            }
            assert!(
                DEVICE_TYPES[..index]
                    .iter()
                    .all(|&(earlier, ..)| earlier != wrapped)
            );
        }
    }

    /// Values libavutil does not know are the reason this is a newtype rather
    /// than a Rust enum: they stay representable and stay nameless.
    #[test]
    fn unknown_device_type_has_no_name() {
        let unknown = AVHWDeviceType::from_raw(ffi::AVHWDeviceType::MAX);
        assert_eq!(av_hwdevice_get_type_name(unknown), None);
        assert!(av_hwdevice_ctx_alloc(unknown).is_none());
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

        let unknown = ffi::AVHWFrameTransferDirection::MAX;
        assert_eq!(
            AVHWFrameTransferDirection::from_raw(unknown).as_raw(),
            unknown
        );
    }

    #[test]
    fn transfer_direction_constants_match_the_c_enum() {
        for &(wrapped, raw) in TRANSFER_DIRECTIONS {
            assert_eq!(wrapped.as_raw(), raw);
            assert_eq!(wrapped, AVHWFrameTransferDirection::from(raw));
            assert_eq!(ffi::AVHWFrameTransferDirection::from(wrapped), raw);
        }
        assert_ne!(
            AVHWFrameTransferDirection::FROM,
            AVHWFrameTransferDirection::TO
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
        // Walk the whole iteration rather than its first step: each compiled-in
        // type must name itself and be found again under that name, and the
        // walk must be strictly increasing so it terminates. This campaign
        // configures every hardware backend off, so the loop covers nothing
        // here; the two assertions after it are what hold in this build.
        let mut previous = AVHWDeviceType::NONE;
        let mut seen = 0;
        loop {
            let next = av_hwdevice_iterate_types(previous);
            if next == AVHWDeviceType::NONE {
                break;
            }
            assert!(next.as_raw() > previous.as_raw());
            let name = av_hwdevice_get_type_name(next).expect("iterated type has a name");
            assert_eq!(av_hwdevice_find_type_by_name(name), next);
            previous = next;
            seen += 1;
            assert!(seen < 64, "iteration did not terminate");
        }

        assert_eq!(
            av_hwdevice_find_type_by_name(c"not-a-device"),
            AVHWDeviceType::NONE
        );
        // `NONE` is the iteration sentinel, so C answers it with a null name
        // rather than the zeroth table entry.
        assert_eq!(av_hwdevice_get_type_name(AVHWDeviceType::NONE), None);
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

/// An owned list of pixel formats returned by a hardware backend. A backend
/// may report none — VA-API does so when the driver exposes no image format —
/// so the list can be empty. The terminating sentinel is retained inside the
/// private allocation but omitted from the safe indexed view.
pub struct HWFrameTransferFormats {
    allocation: CVec<AVPixelFormat, AvFree>,
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
        self.allocation.as_slice().get(index).copied()
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
    let formats = formats.as_ptr().cast::<AVPixelFormat>();
    // SAFETY: success guarantees a readable AVPixelFormat array terminated by
    // AV_PIX_FMT_NONE, and the wrapper is `repr(transparent)` over that C enum,
    // so the cast above keeps the element type. Each step stays inside that
    // allocation through the guaranteed sentinel, and no other owner or writer
    // exists.
    while unsafe { formats.add(len).read() } != AVPixelFormat::NONE {
        len += 1;
    }
    // SAFETY: the pointer is a uniquely owned av_malloc-family allocation with
    // `len` initialized formats plus the initialized sentinel.
    let allocation =
        unsafe { CVec::from_raw_parts(formats, len + 1) }.expect("checked non-null format list");
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

/// Wraps: av_hwframe_get_buffer
///
/// Allocates a hardware surface from an initialized frames context into an
/// empty frame. The C API's currently-unused flags parameter is fixed to zero.
pub fn av_hwframe_get_buffer(
    context: &HWFramesContext,
    frame: &mut crate::frame::AVFrameMut<'_>,
) -> Result<(), i32> {
    // SAFETY: the type state proves the context is initialized, the frame is
    // exclusively borrowed, and zero is the only documented flags value. Any
    // installed buffer references become owned by the destination frame.
    let status = unsafe {
        ffi::av_hwframe_get_buffer(
            context.buffer_ref().as_ptr().cast_mut(),
            frame.as_mut_ptr(),
            0,
        )
    };
    if status < 0 { Err(status) } else { Ok(()) }
}

/// Wraps: av_hwframe_transfer_data
///
/// Transfers image data between a hardware frame and another compatible
/// frame. The C API's currently-unused flags parameter is fixed to zero.
pub fn av_hwframe_transfer_data(
    destination: &mut crate::frame::AVFrameMut<'_>,
    source: crate::frame::AVFrameRef<'_>,
) -> Result<(), i32> {
    // SAFETY: the destination is exclusively borrowed and the source is shared
    // for the call. C retains neither frame header and zero is the documented
    // flags value.
    let status =
        unsafe { ffi::av_hwframe_transfer_data(destination.as_mut_ptr(), source.as_ptr(), 0) };
    if status < 0 { Err(status) } else { Ok(()) }
}

#[cfg(test)]
mod scheduled_hwframe_function_tests {
    use super::*;
    use crate::frame::av_frame_alloc;

    #[test]
    fn transfer_rejects_frames_without_a_hardware_context() {
        let source = av_frame_alloc().expect("source frame");
        let mut destination = av_frame_alloc().expect("destination frame");
        assert!(av_hwframe_transfer_data(&mut destination.as_mut(), source.as_ref()).is_err());
    }
}
