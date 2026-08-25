//! Wrappers for `libavutil/stereo3d.c`.

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
}
