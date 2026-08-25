//! Wrappers for `libavutil/stereo3d.c`.

use core::ffi::c_void;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::CDropped;

use crate::ffi;

/// Wraps: AVStereo3DType
///
/// Describes how the two views of stereoscopic video are arranged. The
/// transparent integer representation preserves values introduced by newer
/// libavutil versions instead of creating an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVStereo3DType(ffi::AVStereo3DType);

impl AVStereo3DType {
    /// The video is not stereoscopic.
    pub const TWO_DIMENSIONAL: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_2D);
    /// The left and right views are next to each other.
    pub const SIDE_BY_SIDE: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_SIDEBYSIDE);
    /// The left and right views are above and below each other.
    pub const TOP_BOTTOM: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_TOPBOTTOM);
    /// The views alternate between consecutive frames.
    pub const FRAME_SEQUENCE: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_FRAMESEQUENCE);
    /// The views alternate in a checkerboard pattern.
    pub const CHECKERBOARD: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_CHECKERBOARD);
    /// Side-by-side views requiring quincunx upscaling.
    pub const SIDE_BY_SIDE_QUINCUNX: Self =
        Self(ffi::AVStereo3DType_AV_STEREO3D_SIDEBYSIDE_QUINCUNX);
    /// The views alternate by row.
    pub const LINES: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_LINES);
    /// The views alternate by column.
    pub const COLUMNS: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_COLUMNS);
    /// The video is stereoscopic but its packing is unspecified.
    pub const UNSPECIFIED: Self = Self(ffi::AVStereo3DType_AV_STEREO3D_UNSPEC);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVStereo3DType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVStereo3DType {
        self.0
    }
}

impl From<ffi::AVStereo3DType> for AVStereo3DType {
    fn from(raw: ffi::AVStereo3DType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVStereo3DType> for ffi::AVStereo3DType {
    fn from(value: AVStereo3DType) -> Self {
        value.as_raw()
    }
}

/// Wraps: AVStereo3DView
///
/// Describes which view or views a frame contains. Unknown future C values
/// remain representable and can be passed back across the ABI unchanged.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVStereo3DView(ffi::AVStereo3DView);

impl AVStereo3DView {
    /// The frame contains two packed views.
    pub const PACKED: Self = Self(ffi::AVStereo3DView_AV_STEREO3D_VIEW_PACKED);
    /// The frame contains only the left view.
    pub const LEFT: Self = Self(ffi::AVStereo3DView_AV_STEREO3D_VIEW_LEFT);
    /// The frame contains only the right view.
    pub const RIGHT: Self = Self(ffi::AVStereo3DView_AV_STEREO3D_VIEW_RIGHT);
    /// The view carried by the frame is unspecified.
    pub const UNSPECIFIED: Self = Self(ffi::AVStereo3DView_AV_STEREO3D_VIEW_UNSPEC);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVStereo3DView) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVStereo3DView {
        self.0
    }
}

impl From<ffi::AVStereo3DView> for AVStereo3DView {
    fn from(raw: ffi::AVStereo3DView) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVStereo3DView> for ffi::AVStereo3DView {
    fn from(value: AVStereo3DView) -> Self {
        value.as_raw()
    }
}


/// Wraps: AVStereo3DPrimaryEye
///
/// Identifies the primary eye for stereoscopic content. The transparent
/// integer representation keeps values introduced by newer libavutil versions
/// representable instead of manufacturing an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVStereo3DPrimaryEye(ffi::AVStereo3DPrimaryEye);

impl AVStereo3DPrimaryEye {
    /// No eye is designated as primary.
    pub const NONE: Self = Self(ffi::AVStereo3DPrimaryEye_AV_PRIMARY_EYE_NONE);
    /// The left eye is primary.
    pub const LEFT: Self = Self(ffi::AVStereo3DPrimaryEye_AV_PRIMARY_EYE_LEFT);
    /// The right eye is primary.
    pub const RIGHT: Self = Self(ffi::AVStereo3DPrimaryEye_AV_PRIMARY_EYE_RIGHT);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVStereo3DPrimaryEye) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVStereo3DPrimaryEye {
        self.0
    }
}

impl From<ffi::AVStereo3DPrimaryEye> for AVStereo3DPrimaryEye {
    fn from(raw: ffi::AVStereo3DPrimaryEye) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVStereo3DPrimaryEye> for ffi::AVStereo3DPrimaryEye {
    fn from(value: AVStereo3DPrimaryEye) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn stereo_type_is_layout_compatible_and_open() {
        assert_eq!(
            size_of::<AVStereo3DType>(),
            size_of::<ffi::AVStereo3DType>()
        );
        assert_eq!(
            align_of::<AVStereo3DType>(),
            align_of::<ffi::AVStereo3DType>()
        );

        let future = ffi::AVStereo3DType::MAX;
        assert_eq!(AVStereo3DType::from_raw(future).as_raw(), future);
        assert_eq!(
            ffi::AVStereo3DType::from(AVStereo3DType::from(future)),
            future
        );
    }

    #[test]
    fn named_stereo_types_match_the_bindings() {
        for (value, raw) in [
            (
                AVStereo3DType::TWO_DIMENSIONAL,
                ffi::AVStereo3DType_AV_STEREO3D_2D,
            ),
            (
                AVStereo3DType::SIDE_BY_SIDE,
                ffi::AVStereo3DType_AV_STEREO3D_SIDEBYSIDE,
            ),
            (
                AVStereo3DType::TOP_BOTTOM,
                ffi::AVStereo3DType_AV_STEREO3D_TOPBOTTOM,
            ),
            (
                AVStereo3DType::FRAME_SEQUENCE,
                ffi::AVStereo3DType_AV_STEREO3D_FRAMESEQUENCE,
            ),
            (
                AVStereo3DType::CHECKERBOARD,
                ffi::AVStereo3DType_AV_STEREO3D_CHECKERBOARD,
            ),
            (
                AVStereo3DType::SIDE_BY_SIDE_QUINCUNX,
                ffi::AVStereo3DType_AV_STEREO3D_SIDEBYSIDE_QUINCUNX,
            ),
            (AVStereo3DType::LINES, ffi::AVStereo3DType_AV_STEREO3D_LINES),
            (
                AVStereo3DType::COLUMNS,
                ffi::AVStereo3DType_AV_STEREO3D_COLUMNS,
            ),
            (
                AVStereo3DType::UNSPECIFIED,
                ffi::AVStereo3DType_AV_STEREO3D_UNSPEC,
            ),
        ] {
            assert_eq!(value.as_raw(), raw);
        }
    }

    #[test]
    fn stereo_view_is_layout_compatible_open_and_named() {
        assert_eq!(
            size_of::<AVStereo3DView>(),
            size_of::<ffi::AVStereo3DView>()
        );
        assert_eq!(
            align_of::<AVStereo3DView>(),
            align_of::<ffi::AVStereo3DView>()
        );

        let future = ffi::AVStereo3DView::MAX;
        assert_eq!(AVStereo3DView::from_raw(future).as_raw(), future);
        for (value, raw) in [
            (
                AVStereo3DView::PACKED,
                ffi::AVStereo3DView_AV_STEREO3D_VIEW_PACKED,
            ),
            (
                AVStereo3DView::LEFT,
                ffi::AVStereo3DView_AV_STEREO3D_VIEW_LEFT,
            ),
            (
                AVStereo3DView::RIGHT,
                ffi::AVStereo3DView_AV_STEREO3D_VIEW_RIGHT,
            ),
            (
                AVStereo3DView::UNSPECIFIED,
                ffi::AVStereo3DView_AV_STEREO3D_VIEW_UNSPEC,
            ),
        ] {
            assert_eq!(value.as_raw(), raw);
        }
    }

    #[test]
    fn primary_eye_values_match_c_and_preserve_unknowns() {
        assert_eq!(AVStereo3DPrimaryEye::NONE.as_raw(), 0);
        assert_eq!(AVStereo3DPrimaryEye::LEFT.as_raw(), 1);
        assert_eq!(AVStereo3DPrimaryEye::RIGHT.as_raw(), 2);
        assert_eq!(AVStereo3DPrimaryEye::from_raw(99).as_raw(), 99);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVStereo3D
    ///
    /// ABI-compatible stereoscopic-video metadata. Independently allocated
    /// values returned by `av_stereo3d_alloc` are owned as
    /// [`ffibox::CBox<AVStereo3D>`] and released with `av_free`, the allocator
    /// underlying the public API's documented `av_freep` release. Values kept
    /// in an `AVFrameSideData` buffer are instead reached only through borrowed
    /// handles tied to the owning frame.
    AVStereo3D,
    AVStereo3DRef,
    AVStereo3DMut,
    ffi::AVStereo3D
);

// SAFETY: `av_stereo3d_alloc` returns the base of one `av_mallocz`
// allocation. The structure contains no separately allocated fields, and
// `av_free` is the allocator-matched one-shot release documented through
// `av_freep`. Constructing a `CBox<AVStereo3D>` remains unsafe and therefore
// must not be done for metadata embedded in an `AVFrameSideData` buffer.
unsafe impl CDropped for AVStereo3D {
    unsafe fn c_drop(object: NonNull<Self>) {
        // SAFETY: the trait contract transfers unique ownership of an
        // independently allocated `AVStereo3D` at its allocation base.
        unsafe { ffi::av_free(object.as_ptr().cast::<c_void>()) }
    }
}

impl<'a> AVStereo3DRef<'a> {
    /// Field: AVStereo3D.type
    ///
    /// Returns how the views are packed.
    #[must_use]
    pub fn kind(&self) -> AVStereo3DType {
        // SAFETY: the handle keeps an initialized structure live; raw-place
        // projection copies the integer-backed enum without forming a
        // reference to C storage.
        AVStereo3DType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Field: AVStereo3D.flags
    ///
    /// Returns the frame-packing flags, including unknown future bits.
    #[must_use]
    pub fn flags(&self) -> i32 {
        // SAFETY: the handle keeps an initialized structure live; raw-place
        // projection copies the scalar without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).flags).read() }
    }

    /// Field: AVStereo3D.horizontal_field_of_view
    ///
    /// Borrows the horizontal field of view in degrees.
    #[must_use]
    pub fn horizontal_field_of_view(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the projected by-value rational is initialized and remains
        // live for `'a` with its enclosing stereo metadata. This shared handle
        // provides no write operation.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).horizontal_field_of_view).cast_mut(),
            )
            .expect("an embedded field address is non-null")
        }
    }

    /// Field: AVStereo3D.horizontal_disparity_adjustment
    ///
    /// Borrows the relative left/right-image shift.
    #[must_use]
    pub fn horizontal_disparity_adjustment(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the projected by-value rational is initialized and remains
        // live for `'a` with its enclosing stereo metadata. This shared handle
        // provides no write operation.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).horizontal_disparity_adjustment).cast_mut(),
            )
            .expect("an embedded field address is non-null")
        }
    }

    /// Field: AVStereo3D.baseline
    ///
    /// Returns the camera-lens baseline in micrometers, or zero when unset.
    #[must_use]
    pub fn baseline(&self) -> u32 {
        // SAFETY: the handle keeps an initialized structure live; raw-place
        // projection copies the scalar without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).baseline).read() }
    }

    /// Field: AVStereo3D.primary_eye
    ///
    /// Returns the eye preferred for two-dimensional rendering.
    #[must_use]
    pub fn primary_eye(&self) -> AVStereo3DPrimaryEye {
        // SAFETY: raw-place projection copies the integer-backed enum from the
        // initialized live structure without forming a reference.
        AVStereo3DPrimaryEye::from_raw(unsafe { addr_of!((*self.as_ptr()).primary_eye).read() })
    }

    /// Field: AVStereo3D.view
    ///
    /// Returns which views the frame contains.
    #[must_use]
    pub fn view(&self) -> AVStereo3DView {
        // SAFETY: raw-place projection copies the integer-backed enum from the
        // initialized live structure without forming a reference.
        AVStereo3DView::from_raw(unsafe { addr_of!((*self.as_ptr()).view).read() })
    }
}

impl AVStereo3DMut<'_> {
    /// Sets how the views are packed.
    pub fn set_kind(&mut self, kind: AVStereo3DType) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // scalar field, and the transparent wrapper yields its ABI value.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).type_).write(kind.as_raw()) }
    }

    /// Replaces all frame-packing flag bits.
    pub fn set_flags(&mut self, flags: i32) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // scalar field and raw-place projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).flags).write(flags) }
    }

    /// Exclusively borrows the horizontal field of view.
    #[must_use]
    pub fn horizontal_field_of_view_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the exclusive parent handle is the only path to the live
        // embedded rational for the returned reborrow's lifetime.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(addr_of_mut!(
                (*self.as_mut_ptr()).horizontal_field_of_view
            ))
            .expect("an embedded field address is non-null")
        }
    }

    /// Exclusively borrows the relative left/right-image shift.
    #[must_use]
    pub fn horizontal_disparity_adjustment_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the exclusive parent handle is the only path to the live
        // embedded rational for the returned reborrow's lifetime.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(addr_of_mut!(
                (*self.as_mut_ptr()).horizontal_disparity_adjustment
            ))
            .expect("an embedded field address is non-null")
        }
    }

    /// Sets the camera-lens baseline in micrometers.
    pub fn set_baseline(&mut self, baseline: u32) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // scalar field and raw-place projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).baseline).write(baseline) }
    }

    /// Sets the eye preferred for two-dimensional rendering.
    pub fn set_primary_eye(&mut self, eye: AVStereo3DPrimaryEye) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // scalar field, and the transparent wrapper yields its ABI value.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).primary_eye).write(eye.as_raw()) }
    }

    /// Sets which views the frame contains.
    pub fn set_view(&mut self, view: AVStereo3DView) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // scalar field, and the transparent wrapper yields its ABI value.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).view).write(view.as_raw()) }
    }
}

#[cfg(test)]
mod stereo_metadata_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CDropped;

    use super::*;

    #[test]
    fn stereo_metadata_layout_matches_ffi() {
        assert_eq!(size_of::<AVStereo3D>(), size_of::<ffi::AVStereo3D>());
        assert_eq!(align_of::<AVStereo3D>(), align_of::<ffi::AVStereo3D>());

        fn assert_has_allocator_matched_drop<T: CDropped>() {}
        assert_has_allocator_matched_drop::<AVStereo3D>();
    }

    #[test]
    fn borrowed_accessors_read_and_mutate_every_field() {
        let mut raw = ffi::AVStereo3D {
            type_: ffi::AVStereo3DType_AV_STEREO3D_2D,
            flags: 0,
            view: ffi::AVStereo3DView_AV_STEREO3D_VIEW_PACKED,
            primary_eye: ffi::AVStereo3DPrimaryEye_AV_PRIMARY_EYE_NONE,
            baseline: 0,
            horizontal_disparity_adjustment: ffi::AVRational { num: 0, den: 1 },
            horizontal_field_of_view: ffi::AVRational { num: 0, den: 1 },
        };

        // SAFETY: `raw` is an initialized FFI value, remains live for the
        // handle's scope, and the exclusive handle is its only access path.
        let mut stereo = unsafe { AVStereo3DMut::from_ptr(&raw mut raw) }.unwrap();
        stereo.set_kind(AVStereo3DType::SIDE_BY_SIDE);
        stereo.set_flags(0x41);
        stereo.set_view(AVStereo3DView::RIGHT);
        stereo.set_primary_eye(AVStereo3DPrimaryEye::LEFT);
        stereo.set_baseline(63_500);
        {
            let mut disparity = stereo.horizontal_disparity_adjustment_mut();
            disparity.set_num(-1);
            disparity.set_den(4);
        }
        {
            let mut field_of_view = stereo.horizontal_field_of_view_mut();
            field_of_view.set_num(95);
            field_of_view.set_den(2);
        }

        let stereo = stereo.as_ref();
        assert_eq!(stereo.kind(), AVStereo3DType::SIDE_BY_SIDE);
        assert_eq!(stereo.flags(), 0x41);
        assert_eq!(stereo.view(), AVStereo3DView::RIGHT);
        assert_eq!(stereo.primary_eye(), AVStereo3DPrimaryEye::LEFT);
        assert_eq!(stereo.baseline(), 63_500);
        assert_eq!(stereo.horizontal_disparity_adjustment().num(), -1);
        assert_eq!(stereo.horizontal_disparity_adjustment().den(), 4);
        assert_eq!(stereo.horizontal_field_of_view().num(), 95);
        assert_eq!(stereo.horizontal_field_of_view().den(), 2);
    }
}
