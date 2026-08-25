//! Wrappers for `libavutil/dovi_meta.c`.

use core::ffi::c_void;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CDropped, CSlice, CSliceMut, CVal, CValued, define_ctype};

use crate::csp::{AVColorPrimariesDescMut, AVColorPrimariesDescRef};
use crate::ffi;
use crate::rational::AVRational;

/// Wraps: AVDOVICompression
///
/// Dolby Vision metadata compression method. The transparent integer
/// representation also preserves values introduced by newer libavutil
/// versions.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVDOVICompression(ffi::AVDOVICompression);

impl AVDOVICompression {
    pub const NONE: Self = Self(ffi::AVDOVICompression_AV_DOVI_COMPRESSION_NONE);
    pub const LIMITED: Self = Self(ffi::AVDOVICompression_AV_DOVI_COMPRESSION_LIMITED);
    pub const RESERVED: Self = Self(ffi::AVDOVICompression_AV_DOVI_COMPRESSION_RESERVED);
    pub const EXTENDED: Self = Self(ffi::AVDOVICompression_AV_DOVI_COMPRESSION_EXTENDED);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVDOVICompression) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVDOVICompression {
        self.0
    }
}

impl From<ffi::AVDOVICompression> for AVDOVICompression {
    fn from(raw: ffi::AVDOVICompression) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVDOVICompression> for ffi::AVDOVICompression {
    fn from(compression: AVDOVICompression) -> Self {
        compression.as_raw()
    }
}

define_ctype!(
    /// Wraps: AVDOVIDecoderConfigurationRecord
    ///
    /// ABI-compatible view of a Dolby Vision decoder configuration record.
    AVDOVIDecoderConfigurationRecord,
    AVDOVIDecoderConfigurationRecordRef,
    AVDOVIDecoderConfigurationRecordMut,
    ffi::AVDOVIDecoderConfigurationRecord
);

macro_rules! scalar_accessors {
    (
        $ref_ty:ident, $mut_ty:ident, $value_ty:ty;
        $(
            $(#[$getter_meta:meta])*
            $getter:ident, $setter:ident, $field:ident;
        )+
    ) => {
        impl $ref_ty<'_> {
            $(
                $(#[$getter_meta])*
                #[must_use]
                pub fn $getter(&self) -> $value_ty {
                    // SAFETY: the shared handle addresses a live initialized
                    // C object; raw-place projection copies one scalar field
                    // without forming a reference to the object or field.
                    unsafe { addr_of!((*self.as_ptr()).$field).read() }
                }
            )+
        }

        impl $mut_ty<'_> {
            $(
                /// Replaces the corresponding scalar field.
                pub fn $setter(&mut self, value: $value_ty) {
                    // SAFETY: the exclusive handle supplies write provenance
                    // to the live C object; raw-place projection writes one
                    // scalar field and forms no reference to C storage.
                    unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
                }
            )+
        }
    };
}

scalar_accessors! {
    AVDOVIDecoderConfigurationRecordRef,
    AVDOVIDecoderConfigurationRecordMut,
    u8;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_md_compression
    dv_md_compression, set_dv_md_compression, dv_md_compression;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_bl_signal_compatibility_id
    dv_bl_signal_compatibility_id,
    set_dv_bl_signal_compatibility_id,
    dv_bl_signal_compatibility_id;
    /// Field: AVDOVIDecoderConfigurationRecord.bl_present_flag
    bl_present_flag, set_bl_present_flag, bl_present_flag;
    /// Field: AVDOVIDecoderConfigurationRecord.el_present_flag
    el_present_flag, set_el_present_flag, el_present_flag;
    /// Field: AVDOVIDecoderConfigurationRecord.rpu_present_flag
    rpu_present_flag, set_rpu_present_flag, rpu_present_flag;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_level
    dv_level, set_dv_level, dv_level;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_profile
    dv_profile, set_dv_profile, dv_profile;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_version_minor
    dv_version_minor, set_dv_version_minor, dv_version_minor;
    /// Field: AVDOVIDecoderConfigurationRecord.dv_version_major
    dv_version_major, set_dv_version_major, dv_version_major;
}

define_ctype!(
    /// Wraps: AVDOVIDmLevel1
    ///
    /// ABI-compatible view of per-frame Dolby Vision brightness metadata.
    AVDOVIDmLevel1,
    AVDOVIDmLevel1Ref,
    AVDOVIDmLevel1Mut,
    ffi::AVDOVIDmLevel1
);

scalar_accessors! {
    AVDOVIDmLevel1Ref, AVDOVIDmLevel1Mut, u16;
    /// Field: AVDOVIDmLevel1.avg_pq
    avg_pq, set_avg_pq, avg_pq;
    /// Field: AVDOVIDmLevel1.max_pq
    max_pq, set_max_pq, max_pq;
    /// Field: AVDOVIDmLevel1.min_pq
    min_pq, set_min_pq, min_pq;
}

define_ctype!(
    /// Wraps: AVDOVIDmLevel3
    ///
    /// ABI-compatible by-value Dolby Vision level 3 dynamic metadata.
    AVDOVIDmLevel3,
    AVDOVIDmLevel3Ref,
    AVDOVIDmLevel3Mut,
    ffi::AVDOVIDmLevel3
);

define_ctype!(
    /// Wraps: AVDOVIDmLevel4
    ///
    /// ABI-compatible by-value Dolby Vision level 4 dynamic metadata.
    AVDOVIDmLevel4,
    AVDOVIDmLevel4Ref,
    AVDOVIDmLevel4Mut,
    ffi::AVDOVIDmLevel4
);

define_ctype!(
    /// Wraps: AVDOVIDmLevel5
    ///
    /// ABI-compatible by-value Dolby Vision level 5 active-area metadata.
    AVDOVIDmLevel5,
    AVDOVIDmLevel5Ref,
    AVDOVIDmLevel5Mut,
    ffi::AVDOVIDmLevel5
);

define_ctype!(
    /// Wraps: AVDOVIDmLevel6
    ///
    /// ABI-compatible by-value Dolby Vision level 6 static HDR10 metadata.
    AVDOVIDmLevel6,
    AVDOVIDmLevel6Ref,
    AVDOVIDmLevel6Mut,
    ffi::AVDOVIDmLevel6
);

// SAFETY: level 3 metadata contains only `u16` values and owns no resources,
// so disposing a live inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel3 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

// SAFETY: level 4 metadata contains only `u16` values and owns no resources,
// so disposing a live inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel4 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

// SAFETY: level 5 metadata contains only `u16` values and owns no resources,
// so disposing a live inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel5 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

// SAFETY: level 6 metadata contains only `u16` values and owns no resources,
// so disposing a live inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel6 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

scalar_accessors! {
    AVDOVIDmLevel3Ref, AVDOVIDmLevel3Mut, u16;
    /// Field: AVDOVIDmLevel3.avg_pq_offset
    avg_pq_offset, set_avg_pq_offset, avg_pq_offset;
    /// Field: AVDOVIDmLevel3.max_pq_offset
    max_pq_offset, set_max_pq_offset, max_pq_offset;
    /// Field: AVDOVIDmLevel3.min_pq_offset
    min_pq_offset, set_min_pq_offset, min_pq_offset;
}

scalar_accessors! {
    AVDOVIDmLevel4Ref, AVDOVIDmLevel4Mut, u16;
    /// Field: AVDOVIDmLevel4.anchor_power
    anchor_power, set_anchor_power, anchor_power;
    /// Field: AVDOVIDmLevel4.anchor_pq
    anchor_pq, set_anchor_pq, anchor_pq;
}

scalar_accessors! {
    AVDOVIDmLevel5Ref, AVDOVIDmLevel5Mut, u16;
    /// Field: AVDOVIDmLevel5.bottom_offset
    bottom_offset, set_bottom_offset, bottom_offset;
    /// Field: AVDOVIDmLevel5.top_offset
    top_offset, set_top_offset, top_offset;
    /// Field: AVDOVIDmLevel5.right_offset
    right_offset, set_right_offset, right_offset;
    /// Field: AVDOVIDmLevel5.left_offset
    left_offset, set_left_offset, left_offset;
}

scalar_accessors! {
    AVDOVIDmLevel6Ref, AVDOVIDmLevel6Mut, u16;
    /// Field: AVDOVIDmLevel6.max_luminance
    max_luminance, set_max_luminance, max_luminance;
    /// Field: AVDOVIDmLevel6.min_luminance
    min_luminance, set_min_luminance, min_luminance;
    /// Field: AVDOVIDmLevel6.max_fall
    max_fall, set_max_fall, max_fall;
    /// Field: AVDOVIDmLevel6.max_cll
    max_cll, set_max_cll, max_cll;
}

define_ctype!(
    /// Wraps: AVDOVINLQParams
    ///
    /// ABI-compatible non-linear inverse-quantization parameters. The C type
    /// contains only integer values, owns no resources, and is normally
    /// embedded by value in an `AVDOVIDataMapping`.
    AVDOVINLQParams,
    AVDOVINLQParamsRef,
    AVDOVINLQParamsMut,
    ffi::AVDOVINLQParams
);

// SAFETY: `AVDOVINLQParams` contains only integer values and has no C
// lifecycle operation or owned resource, so disposing an inline value is a
// no-op and leaves no outstanding resource.
unsafe impl CValued for AVDOVINLQParams {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

scalar_accessors! {
    AVDOVINLQParamsRef, AVDOVINLQParamsMut, u16;
    /// Field: AVDOVINLQParams.nlq_offset
    nlq_offset, set_nlq_offset, nlq_offset;
}

scalar_accessors! {
    AVDOVINLQParamsRef, AVDOVINLQParamsMut, u64;
    /// Field: AVDOVINLQParams.vdr_in_max
    vdr_in_max, set_vdr_in_max, vdr_in_max;
    /// Field: AVDOVINLQParams.linear_deadzone_slope
    linear_deadzone_slope, set_linear_deadzone_slope, linear_deadzone_slope;
    /// Field: AVDOVINLQParams.linear_deadzone_threshold
    linear_deadzone_threshold,
    set_linear_deadzone_threshold,
    linear_deadzone_threshold;
}

define_ctype!(
    /// Wraps: AVDOVIRpuDataHeader
    ///
    /// ABI-compatible Dolby Vision RPU data header. The C type contains only
    /// integer values, owns no resources, and is embedded in an
    /// `AVDOVIMetadata` allocation, where `av_dovi_get_header` locates it.
    AVDOVIRpuDataHeader,
    AVDOVIRpuDataHeaderRef,
    AVDOVIRpuDataHeaderMut,
    ffi::AVDOVIRpuDataHeader
);

// SAFETY: `AVDOVIRpuDataHeader` contains only integer values and has no C
// lifecycle operation or owned resource, so disposing an inline value is a
// no-op and leaves no outstanding resource.
unsafe impl CValued for AVDOVIRpuDataHeader {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

scalar_accessors! {
    AVDOVIRpuDataHeaderRef, AVDOVIRpuDataHeaderMut, u16;
    /// Field: AVDOVIRpuDataHeader.rpu_format
    rpu_format, set_rpu_format, rpu_format;
}

scalar_accessors! {
    AVDOVIRpuDataHeaderRef, AVDOVIRpuDataHeaderMut, u8;
    /// Field: AVDOVIRpuDataHeader.rpu_type
    rpu_type, set_rpu_type, rpu_type;
    /// Field: AVDOVIRpuDataHeader.vdr_rpu_profile
    vdr_rpu_profile, set_vdr_rpu_profile, vdr_rpu_profile;
    /// Field: AVDOVIRpuDataHeader.vdr_rpu_level
    vdr_rpu_level, set_vdr_rpu_level, vdr_rpu_level;
    /// Field: AVDOVIRpuDataHeader.chroma_resampling_explicit_filter_flag
    chroma_resampling_explicit_filter_flag,
    set_chroma_resampling_explicit_filter_flag,
    chroma_resampling_explicit_filter_flag;
    /// Field: AVDOVIRpuDataHeader.coef_data_type
    coef_data_type, set_coef_data_type, coef_data_type;
    /// Field: AVDOVIRpuDataHeader.coef_log2_denom
    coef_log2_denom, set_coef_log2_denom, coef_log2_denom;
    /// Field: AVDOVIRpuDataHeader.vdr_rpu_normalized_idc
    vdr_rpu_normalized_idc, set_vdr_rpu_normalized_idc, vdr_rpu_normalized_idc;
    /// Field: AVDOVIRpuDataHeader.bl_video_full_range_flag
    bl_video_full_range_flag,
    set_bl_video_full_range_flag,
    bl_video_full_range_flag;
    /// Field: AVDOVIRpuDataHeader.bl_bit_depth
    bl_bit_depth, set_bl_bit_depth, bl_bit_depth;
    /// Field: AVDOVIRpuDataHeader.el_bit_depth
    el_bit_depth, set_el_bit_depth, el_bit_depth;
    /// Field: AVDOVIRpuDataHeader.vdr_bit_depth
    vdr_bit_depth, set_vdr_bit_depth, vdr_bit_depth;
    /// Field: AVDOVIRpuDataHeader.spatial_resampling_filter_flag
    spatial_resampling_filter_flag,
    set_spatial_resampling_filter_flag,
    spatial_resampling_filter_flag;
    /// Field: AVDOVIRpuDataHeader.el_spatial_resampling_filter_flag
    el_spatial_resampling_filter_flag,
    set_el_spatial_resampling_filter_flag,
    el_spatial_resampling_filter_flag;
    /// Field: AVDOVIRpuDataHeader.disable_residual_flag
    disable_residual_flag, set_disable_residual_flag, disable_residual_flag;
    /// Field: AVDOVIRpuDataHeader.ext_mapping_idc_0_4
    ext_mapping_idc_0_4, set_ext_mapping_idc_0_4, ext_mapping_idc_0_4;
    /// Field: AVDOVIRpuDataHeader.ext_mapping_idc_5_7
    ext_mapping_idc_5_7, set_ext_mapping_idc_5_7, ext_mapping_idc_5_7;
}

ffibox::define_ctype!(
    /// Wraps: AVDOVIDmLevel11
    ///
    /// ABI-compatible storage for Dolby Vision level 11 metadata. Access is
    /// through borrowed handles so no Rust reference covers C-visible bytes.
    AVDOVIDmLevel11,
    AVDOVIDmLevel11Ref,
    AVDOVIDmLevel11Mut,
    ffi::AVDOVIDmLevel11
);

// SAFETY: level 11 metadata contains only by-value integers and owns no
// resources, so disposing an initialized inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel11 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIDmLevel11Ref<'_> {
    /// Field: AVDOVIDmLevel11.color
    #[must_use]
    pub fn color(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).color).read() }
    }

    /// Field: AVDOVIDmLevel11.brightness
    #[must_use]
    pub fn brightness(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).brightness).read() }
    }

    /// Field: AVDOVIDmLevel11.frame_rate_conversion
    #[must_use]
    pub fn frame_rate_conversion(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).frame_rate_conversion).read() }
    }

    /// Field: AVDOVIDmLevel11.mpeg_noise_reduction
    #[must_use]
    pub fn mpeg_noise_reduction(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).mpeg_noise_reduction).read() }
    }

    /// Field: AVDOVIDmLevel11.noise_reduction
    #[must_use]
    pub fn noise_reduction(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).noise_reduction).read() }
    }

    /// Field: AVDOVIDmLevel11.sharpness
    #[must_use]
    pub fn sharpness(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).sharpness).read() }
    }

    /// Field: AVDOVIDmLevel11.reference_mode_flag
    #[must_use]
    pub fn reference_mode_flag(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).reference_mode_flag).read() }
    }

    /// Field: AVDOVIDmLevel11.whitepoint
    #[must_use]
    pub fn whitepoint(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).whitepoint).read() }
    }

    /// Field: AVDOVIDmLevel11.content_type
    #[must_use]
    pub fn content_type(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 11 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).content_type).read() }
    }
}

impl AVDOVIDmLevel11Mut<'_> {
    /// Sets the deprecated color processing parameter.
    pub fn set_color(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).color).write(value) }
    }

    /// Sets the deprecated brightness processing parameter.
    pub fn set_brightness(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).brightness).write(value) }
    }

    /// Sets the deprecated frame-rate-conversion parameter.
    pub fn set_frame_rate_conversion(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).frame_rate_conversion).write(value) }
    }

    /// Sets the deprecated MPEG noise-reduction parameter.
    pub fn set_mpeg_noise_reduction(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).mpeg_noise_reduction).write(value) }
    }

    /// Sets the deprecated noise-reduction parameter.
    pub fn set_noise_reduction(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).noise_reduction).write(value) }
    }

    /// Sets the deprecated sharpness parameter.
    pub fn set_sharpness(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).sharpness).write(value) }
    }

    /// Sets whether reference mode is enabled.
    pub fn set_reference_mode_flag(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).reference_mode_flag).write(value) }
    }

    /// Sets the whitepoint selector.
    pub fn set_whitepoint(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).whitepoint).write(value) }
    }

    /// Sets the content type.
    pub fn set_content_type(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).content_type).write(value) }
    }
}

ffibox::define_ctype!(
    /// Wraps: AVDOVIDmLevel2
    ///
    /// ABI-compatible storage for Dolby Vision level 2 trim metadata.
    AVDOVIDmLevel2,
    AVDOVIDmLevel2Ref,
    AVDOVIDmLevel2Mut,
    ffi::AVDOVIDmLevel2
);

// SAFETY: level 2 metadata contains only by-value integers and owns no
// resources, so disposing an initialized inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel2 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIDmLevel2Ref<'_> {
    /// Field: AVDOVIDmLevel2.target_max_pq
    #[must_use]
    pub fn target_max_pq(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).target_max_pq).read() }
    }

    /// Field: AVDOVIDmLevel2.ms_weight
    #[must_use]
    pub fn ms_weight(&self) -> i16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).ms_weight).read() }
    }

    /// Field: AVDOVIDmLevel2.trim_saturation_gain
    #[must_use]
    pub fn trim_saturation_gain(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).trim_saturation_gain).read() }
    }

    /// Field: AVDOVIDmLevel2.trim_chroma_weight
    #[must_use]
    pub fn trim_chroma_weight(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).trim_chroma_weight).read() }
    }

    /// Field: AVDOVIDmLevel2.trim_power
    #[must_use]
    pub fn trim_power(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).trim_power).read() }
    }

    /// Field: AVDOVIDmLevel2.trim_offset
    #[must_use]
    pub fn trim_offset(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).trim_offset).read() }
    }

    /// Field: AVDOVIDmLevel2.trim_slope
    #[must_use]
    pub fn trim_slope(&self) -> u16 {
        // SAFETY: the handle keeps an initialized level 2 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).trim_slope).read() }
    }
}

impl AVDOVIDmLevel2Mut<'_> {
    /// Sets the target display maximum PQ code.
    pub fn set_target_max_pq(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).target_max_pq).write(value) }
    }

    /// Sets the mid-tone saturation weight.
    pub fn set_ms_weight(&mut self, value: i16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).ms_weight).write(value) }
    }

    /// Sets the trim saturation gain.
    pub fn set_trim_saturation_gain(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).trim_saturation_gain).write(value) }
    }

    /// Sets the trim chroma weight.
    pub fn set_trim_chroma_weight(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).trim_chroma_weight).write(value) }
    }

    /// Sets the trim power.
    pub fn set_trim_power(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).trim_power).write(value) }
    }

    /// Sets the trim offset.
    pub fn set_trim_offset(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).trim_offset).write(value) }
    }

    /// Sets the trim slope.
    pub fn set_trim_slope(&mut self, value: u16) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).trim_slope).write(value) }
    }
}

ffibox::define_ctype!(
    /// Wraps: AVDOVIDmLevel254
    ///
    /// ABI-compatible storage for the always-present Dolby Vision DMv2 block.
    AVDOVIDmLevel254,
    AVDOVIDmLevel254Ref,
    AVDOVIDmLevel254Mut,
    ffi::AVDOVIDmLevel254
);

// SAFETY: level 254 metadata contains only by-value integers and owns no
// resources, so disposing an initialized inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel254 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIDmLevel254Ref<'_> {
    /// Field: AVDOVIDmLevel254.dm_version_index
    #[must_use]
    pub fn dm_version_index(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 254 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).dm_version_index).read() }
    }

    /// Field: AVDOVIDmLevel254.dm_mode
    #[must_use]
    pub fn dm_mode(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 254 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).dm_mode).read() }
    }
}

impl AVDOVIDmLevel254Mut<'_> {
    /// Sets the DM version index.
    pub fn set_dm_version_index(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).dm_version_index).write(value) }
    }

    /// Sets the DM mode.
    pub fn set_dm_mode(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).dm_mode).write(value) }
    }
}

ffibox::define_ctype!(
    /// Wraps: AVDOVIDmLevel255
    ///
    /// ABI-compatible storage for Dolby Vision debug metadata.
    AVDOVIDmLevel255,
    AVDOVIDmLevel255Ref,
    AVDOVIDmLevel255Mut,
    ffi::AVDOVIDmLevel255
);

// SAFETY: level 255 metadata contains only by-value integers and a by-value
// byte array, so disposing an initialized inline value is always a no-op.
unsafe impl CValued for AVDOVIDmLevel255 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIDmLevel255Ref<'_> {
    /// Field: AVDOVIDmLevel255.dm_debug
    ///
    /// Returns a copy so the result does not assert a Rust shared borrow over
    /// array storage that C may mutate.
    #[must_use]
    pub fn dm_debug(&self) -> [u8; 4] {
        // SAFETY: the handle keeps an initialized level 255 value live; the
        // raw-place projection copies the fixed array without making a slice.
        unsafe { addr_of!((*self.as_ptr()).dm_debug).read() }
    }

    /// Field: AVDOVIDmLevel255.dm_run_version
    #[must_use]
    pub fn dm_run_version(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 255 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).dm_run_version).read() }
    }

    /// Field: AVDOVIDmLevel255.dm_run_mode
    #[must_use]
    pub fn dm_run_mode(&self) -> u8 {
        // SAFETY: the handle keeps an initialized level 255 value live; the
        // raw-place projection copies this integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).dm_run_mode).read() }
    }
}

impl AVDOVIDmLevel255Mut<'_> {
    /// Replaces the four-byte debug payload.
    pub fn set_dm_debug(&mut self, value: [u8; 4]) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection replaces only the fixed byte array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).dm_debug).write(value) }
    }

    /// Sets the DM run version.
    pub fn set_dm_run_version(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).dm_run_version).write(value) }
    }

    /// Sets the DM run mode.
    pub fn set_dm_run_mode(&mut self, value: u8) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // object; raw-place projection writes only the selected integer field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).dm_run_mode).write(value) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn compression_values_include_unknown_future_values() {
        assert_eq!(AVDOVICompression::EXTENDED.as_raw(), 3);
        let future = AVDOVICompression::from_raw(99);
        assert_eq!(future.as_raw(), 99);
    }

    #[test]
    fn decoder_configuration_fields_round_trip() {
        let mut raw = ffi::AVDOVIDecoderConfigurationRecord {
            dv_version_major: 1,
            dv_version_minor: 2,
            dv_profile: 3,
            dv_level: 4,
            rpu_present_flag: 1,
            el_present_flag: 0,
            bl_present_flag: 1,
            dv_bl_signal_compatibility_id: 5,
            dv_md_compression: AVDOVICompression::LIMITED.as_raw() as u8,
        };
        // SAFETY: `raw` is a live initialized C layout value and this test
        // retains exclusive access to it while the mutable handle exists.
        let mut record = unsafe {
            AVDOVIDecoderConfigurationRecordMut::from_ptr(addr_of_mut!(raw))
                .expect("stack record is non-null")
        };

        let shared = record.as_ref();
        assert_eq!(shared.dv_version_major(), 1);
        assert_eq!(shared.dv_version_minor(), 2);
        assert_eq!(shared.dv_profile(), 3);
        assert_eq!(shared.dv_level(), 4);
        assert_eq!(shared.rpu_present_flag(), 1);
        assert_eq!(shared.el_present_flag(), 0);
        assert_eq!(shared.bl_present_flag(), 1);
        assert_eq!(shared.dv_bl_signal_compatibility_id(), 5);
        assert_eq!(shared.dv_md_compression(), 1);

        record.set_dv_version_major(2);
        record.set_dv_version_minor(3);
        record.set_dv_profile(8);
        record.set_dv_level(9);
        record.set_rpu_present_flag(0);
        record.set_el_present_flag(1);
        record.set_bl_present_flag(0);
        record.set_dv_bl_signal_compatibility_id(6);
        record.set_dv_md_compression(3);

        let shared = record.as_ref();
        assert_eq!(
            (
                shared.dv_version_major(),
                shared.dv_version_minor(),
                shared.dv_profile(),
                shared.dv_level(),
                shared.rpu_present_flag(),
                shared.el_present_flag(),
                shared.bl_present_flag(),
                shared.dv_bl_signal_compatibility_id(),
                shared.dv_md_compression(),
            ),
            (2, 3, 8, 9, 0, 1, 0, 6, 3)
        );
    }

    #[test]
    fn level_one_fields_round_trip() {
        let mut raw = ffi::AVDOVIDmLevel1 {
            min_pq: 10,
            max_pq: 20,
            avg_pq: 15,
        };
        // SAFETY: `raw` is live and initialized and no other handle to it is
        // used while this exclusive handle exists.
        let mut level = unsafe {
            AVDOVIDmLevel1Mut::from_ptr(addr_of_mut!(raw)).expect("stack level is non-null")
        };
        assert_eq!(
            (
                level.as_ref().min_pq(),
                level.as_ref().max_pq(),
                level.as_ref().avg_pq(),
            ),
            (10, 20, 15)
        );

        level.set_min_pq(100);
        level.set_max_pq(200);
        level.set_avg_pq(150);
        assert_eq!(
            (
                level.as_ref().min_pq(),
                level.as_ref().max_pq(),
                level.as_ref().avg_pq(),
            ),
            (100, 200, 150)
        );
    }

    #[test]
    fn nlq_and_rpu_layouts_match_bindgen() {
        assert_eq!(
            size_of::<AVDOVINLQParams>(),
            size_of::<ffi::AVDOVINLQParams>()
        );
        assert_eq!(
            align_of::<AVDOVINLQParams>(),
            align_of::<ffi::AVDOVINLQParams>()
        );
        assert_eq!(
            size_of::<AVDOVIRpuDataHeader>(),
            size_of::<ffi::AVDOVIRpuDataHeader>()
        );
        assert_eq!(
            align_of::<AVDOVIRpuDataHeader>(),
            align_of::<ffi::AVDOVIRpuDataHeader>()
        );
    }

    #[test]
    fn nlq_owned_value_round_trips_all_fields() {
        let mut params = CVal::new(AVDOVINLQParams::zeroed());
        let mut view = params.as_mut();
        view.set_nlq_offset(11);
        view.set_vdr_in_max(22);
        view.set_linear_deadzone_slope(33);
        view.set_linear_deadzone_threshold(44);

        let shared = view.as_ref();
        assert_eq!(shared.nlq_offset(), 11);
        assert_eq!(shared.vdr_in_max(), 22);
        assert_eq!(shared.linear_deadzone_slope(), 33);
        assert_eq!(shared.linear_deadzone_threshold(), 44);
    }

    #[test]
    fn rpu_header_owned_value_round_trips_all_fields() {
        let mut header = CVal::new(AVDOVIRpuDataHeader::zeroed());
        let mut view = header.as_mut();
        view.set_rpu_type(1);
        view.set_rpu_format(2);
        view.set_vdr_rpu_profile(3);
        view.set_vdr_rpu_level(4);
        view.set_chroma_resampling_explicit_filter_flag(5);
        view.set_coef_data_type(6);
        view.set_coef_log2_denom(7);
        view.set_vdr_rpu_normalized_idc(8);
        view.set_bl_video_full_range_flag(9);
        view.set_bl_bit_depth(10);
        view.set_el_bit_depth(11);
        view.set_vdr_bit_depth(12);
        view.set_spatial_resampling_filter_flag(13);
        view.set_el_spatial_resampling_filter_flag(14);
        view.set_disable_residual_flag(15);
        view.set_ext_mapping_idc_0_4(16);
        view.set_ext_mapping_idc_5_7(17);

        let shared = view.as_ref();
        assert_eq!(shared.rpu_type(), 1);
        assert_eq!(shared.rpu_format(), 2);
        assert_eq!(shared.vdr_rpu_profile(), 3);
        assert_eq!(shared.vdr_rpu_level(), 4);
        assert_eq!(shared.chroma_resampling_explicit_filter_flag(), 5);
        assert_eq!(shared.coef_data_type(), 6);
        assert_eq!(shared.coef_log2_denom(), 7);
        assert_eq!(shared.vdr_rpu_normalized_idc(), 8);
        assert_eq!(shared.bl_video_full_range_flag(), 9);
        assert_eq!(shared.bl_bit_depth(), 10);
        assert_eq!(shared.el_bit_depth(), 11);
        assert_eq!(shared.vdr_bit_depth(), 12);
        assert_eq!(shared.spatial_resampling_filter_flag(), 13);
        assert_eq!(shared.el_spatial_resampling_filter_flag(), 14);
        assert_eq!(shared.disable_residual_flag(), 15);
        assert_eq!(shared.ext_mapping_idc_0_4(), 16);
        assert_eq!(shared.ext_mapping_idc_5_7(), 17);
    }

    #[test]
    fn levels_three_through_six_match_bindgen() {
        assert_eq!(
            size_of::<AVDOVIDmLevel3>(),
            size_of::<ffi::AVDOVIDmLevel3>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel3>(),
            align_of::<ffi::AVDOVIDmLevel3>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel4>(),
            size_of::<ffi::AVDOVIDmLevel4>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel4>(),
            align_of::<ffi::AVDOVIDmLevel4>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel5>(),
            size_of::<ffi::AVDOVIDmLevel5>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel5>(),
            align_of::<ffi::AVDOVIDmLevel5>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel6>(),
            size_of::<ffi::AVDOVIDmLevel6>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel6>(),
            align_of::<ffi::AVDOVIDmLevel6>()
        );
    }

    #[test]
    fn levels_three_through_six_round_trip_all_fields() {
        let mut level3 = CVal::new(AVDOVIDmLevel3::zeroed());
        level3.as_mut().set_min_pq_offset(11);
        level3.as_mut().set_max_pq_offset(12);
        level3.as_mut().set_avg_pq_offset(13);
        assert_eq!(level3.as_ref().min_pq_offset(), 11);
        assert_eq!(level3.as_ref().max_pq_offset(), 12);
        assert_eq!(level3.as_ref().avg_pq_offset(), 13);

        let mut level4 = CVal::new(AVDOVIDmLevel4::zeroed());
        level4.as_mut().set_anchor_pq(21);
        level4.as_mut().set_anchor_power(22);
        assert_eq!(level4.as_ref().anchor_pq(), 21);
        assert_eq!(level4.as_ref().anchor_power(), 22);

        let mut level5 = CVal::new(AVDOVIDmLevel5::zeroed());
        level5.as_mut().set_left_offset(31);
        level5.as_mut().set_right_offset(32);
        level5.as_mut().set_top_offset(33);
        level5.as_mut().set_bottom_offset(34);
        assert_eq!(level5.as_ref().left_offset(), 31);
        assert_eq!(level5.as_ref().right_offset(), 32);
        assert_eq!(level5.as_ref().top_offset(), 33);
        assert_eq!(level5.as_ref().bottom_offset(), 34);

        let mut level6 = CVal::new(AVDOVIDmLevel6::zeroed());
        level6.as_mut().set_max_luminance(41);
        level6.as_mut().set_min_luminance(42);
        level6.as_mut().set_max_cll(43);
        level6.as_mut().set_max_fall(44);
        assert_eq!(level6.as_ref().max_luminance(), 41);
        assert_eq!(level6.as_ref().min_luminance(), 42);
        assert_eq!(level6.as_ref().max_cll(), 43);
        assert_eq!(level6.as_ref().max_fall(), 44);
    }

    #[test]
    fn layouts_match_bindgen() {
        assert_eq!(
            size_of::<AVDOVIDmLevel11>(),
            size_of::<ffi::AVDOVIDmLevel11>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel11>(),
            align_of::<ffi::AVDOVIDmLevel11>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel2>(),
            size_of::<ffi::AVDOVIDmLevel2>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel2>(),
            align_of::<ffi::AVDOVIDmLevel2>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel254>(),
            size_of::<ffi::AVDOVIDmLevel254>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel254>(),
            align_of::<ffi::AVDOVIDmLevel254>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel255>(),
            size_of::<ffi::AVDOVIDmLevel255>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel255>(),
            align_of::<ffi::AVDOVIDmLevel255>()
        );
    }

    #[test]
    fn level_11_owned_storage_supports_shared_and_exclusive_access() {
        let mut value = CVal::new(AVDOVIDmLevel11::zeroed());
        {
            let mut view = value.as_mut();
            view.set_content_type(1);
            view.set_whitepoint(2);
            view.set_reference_mode_flag(3);
            view.set_sharpness(4);
            view.set_noise_reduction(5);
            view.set_mpeg_noise_reduction(6);
            view.set_frame_rate_conversion(7);
            view.set_brightness(8);
            view.set_color(9);
        }
        let view = value.as_ref();
        assert_eq!(view.content_type(), 1);
        assert_eq!(view.whitepoint(), 2);
        assert_eq!(view.reference_mode_flag(), 3);
        assert_eq!(view.sharpness(), 4);
        assert_eq!(view.noise_reduction(), 5);
        assert_eq!(view.mpeg_noise_reduction(), 6);
        assert_eq!(view.frame_rate_conversion(), 7);
        assert_eq!(view.brightness(), 8);
        assert_eq!(view.color(), 9);
    }

    #[test]
    fn level_2_round_trips_signed_and_unsigned_fields() {
        let mut value = CVal::new(AVDOVIDmLevel2::zeroed());
        {
            let mut view = value.as_mut();
            view.set_target_max_pq(10);
            view.set_trim_slope(11);
            view.set_trim_offset(12);
            view.set_trim_power(13);
            view.set_trim_chroma_weight(14);
            view.set_trim_saturation_gain(15);
            view.set_ms_weight(-16);
        }
        let view = value.as_ref();
        assert_eq!(view.target_max_pq(), 10);
        assert_eq!(view.trim_slope(), 11);
        assert_eq!(view.trim_offset(), 12);
        assert_eq!(view.trim_power(), 13);
        assert_eq!(view.trim_chroma_weight(), 14);
        assert_eq!(view.trim_saturation_gain(), 15);
        assert_eq!(view.ms_weight(), -16);
    }

    #[test]
    fn high_numbered_levels_round_trip_scalar_and_array_fields() {
        let mut level_254 = CVal::new(AVDOVIDmLevel254::zeroed());
        level_254.as_mut().set_dm_mode(20);
        level_254.as_mut().set_dm_version_index(21);
        assert_eq!(level_254.as_ref().dm_mode(), 20);
        assert_eq!(level_254.as_ref().dm_version_index(), 21);

        let mut level_255 = CVal::new(AVDOVIDmLevel255::zeroed());
        level_255.as_mut().set_dm_run_mode(30);
        level_255.as_mut().set_dm_run_version(31);
        level_255.as_mut().set_dm_debug([32, 33, 34, 35]);
        assert_eq!(level_255.as_ref().dm_run_mode(), 30);
        assert_eq!(level_255.as_ref().dm_run_version(), 31);
        assert_eq!(level_255.as_ref().dm_debug(), [32, 33, 34, 35]);
    }
}

define_ctype!(
    /// Wraps: AVDOVIDmLevel8
    ///
    /// ABI-compatible level 8 display-management metadata. The type is
    /// normally embedded in an `AVDOVIDmData` extension block; borrowed
    /// access is carried by [`AVDOVIDmLevel8Ref`] and
    /// [`AVDOVIDmLevel8Mut`] without forming a Rust reference over C storage.
    AVDOVIDmLevel8,
    AVDOVIDmLevel8Ref,
    AVDOVIDmLevel8Mut,
    ffi::AVDOVIDmLevel8
);

// SAFETY: level 8 metadata contains only integer scalars and fixed byte
// arrays. It owns no resources, so disposing an inline value is a no-op.
unsafe impl CValued for AVDOVIDmLevel8 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

macro_rules! level8_scalar {
    ($(#[$attr:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVDOVIDmLevel8Ref<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: this handle addresses a live initialized level 8
                // block. Raw-place projection copies the scalar field without
                // forming a reference to the wrapped object or field.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }

        impl AVDOVIDmLevel8Mut<'_> {
            #[doc = concat!("Sets `", stringify!($field), "`.")]
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle supplies write provenance to a
                // live level 8 block. Raw-place projection writes only the
                // named scalar field and forms no Rust reference.
                unsafe { addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

level8_scalar!(
    /// Field: AVDOVIDmLevel8.target_display_index
    target_display_index,
    set_target_display_index,
    target_display_index,
    u8
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.trim_slope
    trim_slope,
    set_trim_slope,
    trim_slope,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.trim_offset
    trim_offset,
    set_trim_offset,
    trim_offset,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.trim_power
    trim_power,
    set_trim_power,
    trim_power,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.trim_chroma_weight
    trim_chroma_weight,
    set_trim_chroma_weight,
    trim_chroma_weight,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.trim_saturation_gain
    trim_saturation_gain,
    set_trim_saturation_gain,
    trim_saturation_gain,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.ms_weight
    ms_weight,
    set_ms_weight,
    ms_weight,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.target_mid_contrast
    target_mid_contrast,
    set_target_mid_contrast,
    target_mid_contrast,
    u16
);
level8_scalar!(
    /// Field: AVDOVIDmLevel8.clip_trim
    clip_trim,
    set_clip_trim,
    clip_trim,
    u16
);

impl AVDOVIDmLevel8Ref<'_> {
    /// Field: AVDOVIDmLevel8.saturation_vector_field
    ///
    /// Returns a copy of the six saturation adjustments. A copy, rather than
    /// a slice, avoids placing a Rust reference over storage C may retain.
    #[must_use]
    pub fn saturation_vector_field(&self) -> [u8; 6] {
        // SAFETY: the handle addresses a live initialized level 8 block. The
        // fixed-size byte array is `Copy`, and raw-place projection creates no
        // reference to the wrapped object or field.
        unsafe { addr_of!((*self.as_ptr()).saturation_vector_field).read() }
    }

    /// Field: AVDOVIDmLevel8.hue_vector_field
    ///
    /// Returns a copy of the six hue adjustments.
    #[must_use]
    pub fn hue_vector_field(&self) -> [u8; 6] {
        // SAFETY: the handle addresses a live initialized level 8 block. The
        // fixed-size byte array is `Copy`, and raw-place projection creates no
        // reference to the wrapped object or field.
        unsafe { addr_of!((*self.as_ptr()).hue_vector_field).read() }
    }
}

impl AVDOVIDmLevel8Mut<'_> {
    /// Replaces all six saturation adjustments.
    pub fn set_saturation_vector_field(&mut self, value: [u8; 6]) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // block, and the raw-place projection writes exactly the array field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).saturation_vector_field).write(value) }
    }

    /// Replaces all six hue adjustments.
    pub fn set_hue_vector_field(&mut self, value: [u8; 6]) {
        // SAFETY: the exclusive handle supplies write provenance to the live
        // block, and the raw-place projection writes exactly the array field.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).hue_vector_field).write(value) }
    }
}

/// Wraps: AVDOVIMappingMethod
///
/// Identifies the piece-wise reshaping function. The transparent integer
/// representation keeps unknown values representable when metadata comes from
/// a newer libavutil instead of creating an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVDOVIMappingMethod(ffi::AVDOVIMappingMethod);

impl AVDOVIMappingMethod {
    /// Polynomial reshaping coefficients.
    pub const POLYNOMIAL: Self = Self(ffi::AVDOVIMappingMethod_AV_DOVI_MAPPING_POLYNOMIAL);

    /// Multi-variate multiple regression coefficients.
    pub const MMR: Self = Self(ffi::AVDOVIMappingMethod_AV_DOVI_MAPPING_MMR);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVDOVIMappingMethod) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVDOVIMappingMethod {
        self.0
    }
}

impl From<ffi::AVDOVIMappingMethod> for AVDOVIMappingMethod {
    fn from(raw: ffi::AVDOVIMappingMethod) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVDOVIMappingMethod> for ffi::AVDOVIMappingMethod {
    fn from(value: AVDOVIMappingMethod) -> Self {
        value.as_raw()
    }
}

define_ctype!(
    /// Wraps: AVDOVIMetadata
    ///
    /// ABI-compatible header for libavutil's contiguous Dolby Vision metadata
    /// allocation. An owning [`ffibox::CBox<AVDOVIMetadata>`] must come from
    /// `av_dovi_metadata_alloc`; dropping it releases the complete allocation
    /// with `av_free`. Borrowed field access never assumes ownership.
    AVDOVIMetadata,
    AVDOVIMetadataRef,
    AVDOVIMetadataMut,
    ffi::AVDOVIMetadata
);

// SAFETY: `av_dovi_metadata_alloc` obtains one contiguous allocation from
// `av_mallocz` and returns its address because `metadata` is the first member
// of `AVDOVIMetadataInternal`. `av_free` is the matching one-shot releaser and
// the allocation contains no separately allocated fields.
unsafe impl CDropped for AVDOVIMetadata {
    unsafe fn c_drop(object: NonNull<Self>) {
        // SAFETY: the trait contract supplies unique ownership of a fully
        // constructed allocation returned by `av_dovi_metadata_alloc`; its
        // pointer is the allocation base and belongs to the `av_malloc` family.
        unsafe { ffi::av_free(object.as_ptr().cast::<c_void>()) }
    }
}

macro_rules! metadata_scalar {
    ($(#[$attr:meta])* $get:ident, $field:ident, $ty:ty) => {
        impl AVDOVIMetadataRef<'_> {
            $(#[$attr])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: this handle addresses a live initialized metadata
                // header. Raw-place projection copies the scalar field and
                // forms no reference to the wrapped object or field.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
    };
}

metadata_scalar!(
    /// Field: AVDOVIMetadata.header_offset
    ///
    /// Byte offset from the header to the `AVDOVIRpuDataHeader` payload.
    header_offset,
    header_offset,
    usize
);
metadata_scalar!(
    /// Field: AVDOVIMetadata.mapping_offset
    ///
    /// Byte offset from the header to the `AVDOVIDataMapping` payload.
    mapping_offset,
    mapping_offset,
    usize
);
metadata_scalar!(
    /// Field: AVDOVIMetadata.color_offset
    ///
    /// Byte offset from the header to the `AVDOVIColorMetadata` payload.
    color_offset,
    color_offset,
    usize
);
metadata_scalar!(
    /// Field: AVDOVIMetadata.ext_block_offset
    ///
    /// Byte offset from the header to the extension-block array.
    ext_block_offset,
    ext_block_offset,
    usize
);
metadata_scalar!(
    /// Field: AVDOVIMetadata.ext_block_size
    ///
    /// Stride in bytes between extension blocks.
    ext_block_size,
    ext_block_size,
    usize
);
metadata_scalar!(
    /// Field: AVDOVIMetadata.num_ext_blocks
    ///
    /// Number of initialized extension blocks in the allocation.
    num_ext_blocks,
    num_ext_blocks,
    i32
);

/// Wraps: AVDOVINLQMethod
///
/// Identifies the non-linear inverse-quantization method. This is an open
/// transparent wrapper because C may supply reserved or future values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVDOVINLQMethod(ffi::AVDOVINLQMethod);

impl AVDOVINLQMethod {
    /// Non-linear inverse quantization is disabled.
    pub const NONE: Self = Self(ffi::AVDOVINLQMethod_AV_DOVI_NLQ_NONE);

    /// Linear dead-zone inverse quantization.
    pub const LINEAR_DZ: Self = Self(ffi::AVDOVINLQMethod_AV_DOVI_NLQ_LINEAR_DZ);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVDOVINLQMethod) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVDOVINLQMethod {
        self.0
    }
}

impl From<ffi::AVDOVINLQMethod> for AVDOVINLQMethod {
    fn from(raw: ffi::AVDOVINLQMethod) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVDOVINLQMethod> for ffi::AVDOVINLQMethod {
    fn from(value: AVDOVINLQMethod) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod level8_and_metadata_tests {
    use super::*;

    #[test]
    fn level8_owned_and_borrowed_access_covers_every_field_shape() {
        let mut owned = ffibox::CVal::new(AVDOVIDmLevel8::zeroed());
        let mut level = owned.as_mut();
        level.set_target_display_index(21);
        level.set_trim_slope(22);
        level.set_trim_offset(23);
        level.set_trim_power(24);
        level.set_trim_chroma_weight(25);
        level.set_trim_saturation_gain(26);
        level.set_ms_weight(27);
        level.set_target_mid_contrast(28);
        level.set_clip_trim(29);
        level.set_saturation_vector_field([1, 2, 3, 4, 5, 6]);
        level.set_hue_vector_field([6, 5, 4, 3, 2, 1]);

        assert_eq!(level.as_ref().target_display_index(), 21);
        assert_eq!(level.as_ref().trim_slope(), 22);
        assert_eq!(level.as_ref().trim_offset(), 23);
        assert_eq!(level.as_ref().trim_power(), 24);
        assert_eq!(level.as_ref().trim_chroma_weight(), 25);
        assert_eq!(level.as_ref().trim_saturation_gain(), 26);
        assert_eq!(level.as_ref().ms_weight(), 27);
        assert_eq!(level.as_ref().target_mid_contrast(), 28);
        assert_eq!(level.as_ref().clip_trim(), 29);
        assert_eq!(level.as_ref().saturation_vector_field(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(level.as_ref().hue_vector_field(), [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn metadata_shared_handle_copies_offsets_and_count() {
        let mut raw = ffi::AVDOVIMetadata {
            header_offset: 8,
            mapping_offset: 16,
            color_offset: 24,
            ext_block_offset: 32,
            ext_block_size: 40,
            num_ext_blocks: 3,
        };

        // SAFETY: `raw` is initialized and remains live without mutation for
        // the duration of the shared handle.
        let metadata = unsafe { AVDOVIMetadataRef::from_ptr(&mut raw) }.unwrap();
        assert_eq!(metadata.header_offset(), 8);
        assert_eq!(metadata.mapping_offset(), 16);
        assert_eq!(metadata.color_offset(), 24);
        assert_eq!(metadata.ext_block_offset(), 32);
        assert_eq!(metadata.ext_block_size(), 40);
        assert_eq!(metadata.num_ext_blocks(), 3);
    }

    #[test]
    fn dovi_enum_wrappers_round_trip_known_and_future_values() {
        assert_eq!(AVDOVIMappingMethod::POLYNOMIAL.as_raw(), 0);
        assert_eq!(AVDOVIMappingMethod::MMR.as_raw(), 1);
        assert_eq!(AVDOVIMappingMethod::from_raw(17).as_raw(), 17);

        assert_eq!(AVDOVINLQMethod::NONE.as_raw(), -1);
        assert_eq!(AVDOVINLQMethod::LINEAR_DZ.as_raw(), 0);
        assert_eq!(AVDOVINLQMethod::from_raw(17).as_raw(), 17);
    }

    #[test]
    fn wrapped_layouts_match_bindgen_layouts() {
        fn assert_has_drop_strategy<T: CDropped>() {}
        assert_has_drop_strategy::<AVDOVIMetadata>();

        assert_eq!(
            core::mem::size_of::<AVDOVIDmLevel8>(),
            core::mem::size_of::<ffi::AVDOVIDmLevel8>()
        );
        assert_eq!(
            core::mem::align_of::<AVDOVIDmLevel8>(),
            core::mem::align_of::<ffi::AVDOVIDmLevel8>()
        );
        assert_eq!(
            core::mem::size_of::<AVDOVIMetadata>(),
            core::mem::size_of::<ffi::AVDOVIMetadata>()
        );
        assert_eq!(
            core::mem::align_of::<AVDOVIMetadata>(),
            core::mem::align_of::<ffi::AVDOVIMetadata>()
        );
    }
}

define_ctype!(
    /// Wraps: AVDOVIReshapingCurve
    ///
    /// Layout-compatible coefficients for one piece-wise Dolby Vision
    /// reshaping function. The structure owns no resources and is normally
    /// embedded by value in an `AVDOVIDataMapping`.
    AVDOVIReshapingCurve,
    AVDOVIReshapingCurveRef,
    AVDOVIReshapingCurveMut,
    ffi::AVDOVIReshapingCurve
);

// SAFETY: the C structure contains only integer scalars and fixed-size arrays
// and owns no resources, so disposing a live inline value is a no-op.
unsafe impl CValued for AVDOVIReshapingCurve {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIReshapingCurve {
    pub const MAX_PIECES: usize = 8;
    pub const MAX_PIVOTS: usize = Self::MAX_PIECES + 1;
}

impl AVDOVIReshapingCurveRef<'_> {
    /// Field: AVDOVIReshapingCurve.num_pivots
    #[must_use]
    pub fn num_pivots(&self) -> u8 {
        // SAFETY: the handle addresses an initialized curve and this copies a
        // scalar through a raw-place projection without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).num_pivots).read() }
    }

    /// Field: AVDOVIReshapingCurve.pivots
    #[must_use]
    pub fn pivots(&self) -> [u16; AVDOVIReshapingCurve::MAX_PIVOTS] {
        // SAFETY: the handle addresses an initialized curve and the fixed
        // integer array is copied without forming a reference to C storage.
        unsafe { addr_of!((*self.as_ptr()).pivots).read() }
    }

    /// Field: AVDOVIReshapingCurve.mapping_idc
    #[must_use]
    pub fn mapping_idc(&self) -> [AVDOVIMappingMethod; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: bindgen represents the open C enum as its integer ABI type;
        // copying the initialized array through a raw projection is valid.
        unsafe { addr_of!((*self.as_ptr()).mapping_idc).read() }.map(AVDOVIMappingMethod::from_raw)
    }

    /// Field: AVDOVIReshapingCurve.poly_order
    #[must_use]
    pub fn poly_order(&self) -> [u8; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: the initialized fixed integer array is copied through a raw
        // projection, so no reference covers storage C may mutate.
        unsafe { addr_of!((*self.as_ptr()).poly_order).read() }
    }

    /// Field: AVDOVIReshapingCurve.poly_coef
    #[must_use]
    pub fn poly_coef(&self) -> [[i64; 3]; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: the initialized fixed integer array is copied through a raw
        // projection, so no reference covers storage C may mutate.
        unsafe { addr_of!((*self.as_ptr()).poly_coef).read() }
    }

    /// Field: AVDOVIReshapingCurve.mmr_order
    #[must_use]
    pub fn mmr_order(&self) -> [u8; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: the initialized fixed integer array is copied through a raw
        // projection, so no reference covers storage C may mutate.
        unsafe { addr_of!((*self.as_ptr()).mmr_order).read() }
    }

    /// Field: AVDOVIReshapingCurve.mmr_constant
    #[must_use]
    pub fn mmr_constant(&self) -> [i64; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: the initialized fixed integer array is copied through a raw
        // projection, so no reference covers storage C may mutate.
        unsafe { addr_of!((*self.as_ptr()).mmr_constant).read() }
    }

    /// Field: AVDOVIReshapingCurve.mmr_coef
    #[must_use]
    pub fn mmr_coef(&self) -> [[[i64; 7]; 3]; AVDOVIReshapingCurve::MAX_PIECES] {
        // SAFETY: the initialized fixed integer array is copied through a raw
        // projection, so no reference covers storage C may mutate.
        unsafe { addr_of!((*self.as_ptr()).mmr_coef).read() }
    }
}

impl AVDOVIReshapingCurveMut<'_> {
    /// Sets the active pivot count.
    ///
    /// # Panics
    ///
    /// Panics unless `num_pivots` is in the documented range `2..=9`.
    pub fn set_num_pivots(&mut self, num_pivots: u8) {
        assert!((2..=AVDOVIReshapingCurve::MAX_PIVOTS as u8).contains(&num_pivots));
        // SAFETY: the exclusive handle permits a raw write to this scalar.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).num_pivots).write(num_pivots) }
    }

    /// Replaces the sorted pivot table.
    ///
    /// # Panics
    ///
    /// Panics unless the pivots are sorted in ascending order.
    pub fn set_pivots(&mut self, pivots: [u16; AVDOVIReshapingCurve::MAX_PIVOTS]) {
        assert!(pivots.windows(2).all(|pair| pair[0] <= pair[1]));
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).pivots).write(pivots) }
    }

    /// Replaces the mapping method for every piece.
    pub fn set_mapping_idc(
        &mut self,
        mapping_idc: [AVDOVIMappingMethod; AVDOVIReshapingCurve::MAX_PIECES],
    ) {
        let raw = mapping_idc.map(AVDOVIMappingMethod::as_raw);
        // SAFETY: the exclusive handle permits a raw write of the ABI integer
        // array; open enum values remain representable.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).mapping_idc).write(raw) }
    }

    /// Replaces the polynomial order table.
    ///
    /// # Panics
    ///
    /// Panics unless every order is in the documented range `1..=2`.
    pub fn set_poly_order(&mut self, poly_order: [u8; AVDOVIReshapingCurve::MAX_PIECES]) {
        assert!(poly_order.iter().all(|order| (1..=2).contains(order)));
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).poly_order).write(poly_order) }
    }

    /// Replaces all polynomial coefficients.
    pub fn set_poly_coef(&mut self, poly_coef: [[i64; 3]; AVDOVIReshapingCurve::MAX_PIECES]) {
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).poly_coef).write(poly_coef) }
    }

    /// Replaces the MMR order table.
    ///
    /// # Panics
    ///
    /// Panics unless every order is in the documented range `1..=3`.
    pub fn set_mmr_order(&mut self, mmr_order: [u8; AVDOVIReshapingCurve::MAX_PIECES]) {
        assert!(mmr_order.iter().all(|order| (1..=3).contains(order)));
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).mmr_order).write(mmr_order) }
    }

    /// Replaces all MMR constants.
    pub fn set_mmr_constant(&mut self, mmr_constant: [i64; AVDOVIReshapingCurve::MAX_PIECES]) {
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).mmr_constant).write(mmr_constant) }
    }

    /// Replaces all MMR coefficients.
    pub fn set_mmr_coef(&mut self, mmr_coef: [[[i64; 7]; 3]; AVDOVIReshapingCurve::MAX_PIECES]) {
        // SAFETY: the exclusive handle permits a raw write of the fixed array.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).mmr_coef).write(mmr_coef) }
    }
}

#[cfg(test)]
mod reshaping_curve_tests {
    use core::mem::{align_of, size_of};

    use ffibox::CVal;

    use super::*;

    #[test]
    fn layout_and_fixed_tables_round_trip() {
        assert_eq!(
            size_of::<AVDOVIReshapingCurve>(),
            size_of::<ffi::AVDOVIReshapingCurve>()
        );
        assert_eq!(
            align_of::<AVDOVIReshapingCurve>(),
            align_of::<ffi::AVDOVIReshapingCurve>()
        );

        let mut curve = CVal::new(AVDOVIReshapingCurve::zeroed());
        curve.as_mut().set_num_pivots(9);
        curve.as_mut().set_pivots([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        curve.as_mut().set_mapping_idc([
            AVDOVIMappingMethod::POLYNOMIAL,
            AVDOVIMappingMethod::MMR,
            AVDOVIMappingMethod::from_raw(77),
            AVDOVIMappingMethod::POLYNOMIAL,
            AVDOVIMappingMethod::MMR,
            AVDOVIMappingMethod::POLYNOMIAL,
            AVDOVIMappingMethod::MMR,
            AVDOVIMappingMethod::POLYNOMIAL,
        ]);
        curve.as_mut().set_poly_order([1, 2, 1, 2, 1, 2, 1, 2]);
        curve.as_mut().set_poly_coef([[3; 3]; 8]);
        curve.as_mut().set_mmr_order([1, 2, 3, 1, 2, 3, 1, 2]);
        curve.as_mut().set_mmr_constant([4; 8]);
        curve.as_mut().set_mmr_coef([[[5; 7]; 3]; 8]);

        let view = curve.as_ref();
        assert_eq!(view.num_pivots(), 9);
        assert_eq!(view.pivots()[8], 8);
        assert_eq!(view.mapping_idc()[2].as_raw(), 77);
        assert_eq!(view.poly_order()[7], 2);
        assert_eq!(view.poly_coef()[7][2], 3);
        assert_eq!(view.mmr_order()[2], 3);
        assert_eq!(view.mmr_constant()[7], 4);
        assert_eq!(view.mmr_coef()[7][2][6], 5);
    }
}

define_ctype!(
    /// Wraps: AVDOVIColorMetadata
    ///
    /// ABI-compatible Dolby Vision RPU colorspace metadata. The type is plain
    /// by-value storage and contains no pointers or owned resources.
    AVDOVIColorMetadata,
    AVDOVIColorMetadataRef,
    AVDOVIColorMetadataMut,
    ffi::AVDOVIColorMetadata
);

// SAFETY: the structure contains only integer scalars and fixed arrays of
// by-value AVRational pairs, so disposing an inline value is a no-op.
unsafe impl CValued for AVDOVIColorMetadata {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIColorMetadata {
    /// Creates zero-initialized colorspace metadata in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

scalar_accessors! {
    AVDOVIColorMetadataRef, AVDOVIColorMetadataMut, u16;
    /// Field: AVDOVIColorMetadata.source_diagonal
    source_diagonal, set_source_diagonal, source_diagonal;
    /// Field: AVDOVIColorMetadata.source_max_pq
    source_max_pq, set_source_max_pq, source_max_pq;
    /// Field: AVDOVIColorMetadata.source_min_pq
    source_min_pq, set_source_min_pq, source_min_pq;
    /// Field: AVDOVIColorMetadata.signal_eotf_param1
    signal_eotf_param1, set_signal_eotf_param1, signal_eotf_param1;
    /// Field: AVDOVIColorMetadata.signal_eotf_param0
    signal_eotf_param0, set_signal_eotf_param0, signal_eotf_param0;
    /// Field: AVDOVIColorMetadata.signal_eotf
    signal_eotf, set_signal_eotf, signal_eotf;
}

scalar_accessors! {
    AVDOVIColorMetadataRef, AVDOVIColorMetadataMut, u32;
    /// Field: AVDOVIColorMetadata.signal_eotf_param2
    signal_eotf_param2, set_signal_eotf_param2, signal_eotf_param2;
}

scalar_accessors! {
    AVDOVIColorMetadataRef, AVDOVIColorMetadataMut, u8;
    /// Field: AVDOVIColorMetadata.signal_chroma_format
    signal_chroma_format, set_signal_chroma_format, signal_chroma_format;
    /// Field: AVDOVIColorMetadata.signal_color_space
    signal_color_space, set_signal_color_space, signal_color_space;
    /// Field: AVDOVIColorMetadata.signal_bit_depth
    signal_bit_depth, set_signal_bit_depth, signal_bit_depth;
    /// Field: AVDOVIColorMetadata.scene_refresh_flag
    scene_refresh_flag, set_scene_refresh_flag, scene_refresh_flag;
    /// Field: AVDOVIColorMetadata.dm_metadata_id
    dm_metadata_id, set_dm_metadata_id, dm_metadata_id;
}

impl AVDOVIColorMetadataRef<'_> {
    /// Field: AVDOVIColorMetadata.signal_full_range_flag
    #[must_use]
    pub fn signal_full_range_flag(&self) -> u8 {
        // SAFETY: the shared handle addresses initialized metadata; raw-place
        // projection copies one byte without forming a Rust reference.
        unsafe { addr_of!((*self.as_ptr()).signal_full_range_flag).read() }
    }
}

impl AVDOVIColorMetadataMut<'_> {
    /// Sets the full-range signal flag.
    ///
    /// # Panics
    ///
    /// Panics when `value` is outside the C header's documented range `0..=3`.
    pub fn set_signal_full_range_flag(&mut self, value: u8) {
        assert!(value <= 3, "signal_full_range_flag must be in 0..=3");
        // SAFETY: the exclusive handle supplies write provenance; raw-place
        // projection writes the validated byte without forming a reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).signal_full_range_flag).write(value) }
    }
}

macro_rules! color_rational_array {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident, $len:expr) => {
        impl<'a> AVDOVIColorMetadataRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> CSlice<'a, AVRational> {
                // SAFETY: raw-place projection locates the fixed array without
                // forming a reference. It contains exactly `$len` initialized
                // AVRational values and lives for the metadata handle's
                // lifetime.
                unsafe {
                    let pointer = addr_of!((*self.as_ptr()).$field)
                        .cast::<AVRational>()
                        .cast_mut();
                    CSlice::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }

        impl AVDOVIColorMetadataMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVDOVIColorMetadataRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> CSliceMut<'_, AVRational> {
                // SAFETY: the exclusive handle supplies write provenance to
                // the fixed initialized array, and the returned view is bound
                // to this mutable borrow.
                unsafe {
                    let pointer = addr_of_mut!((*self.as_mut_ptr()).$field)
                        .cast::<AVRational>();
                    CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), $len)
                }
            }
        }
    };
}

color_rational_array!(
    /// Field: AVDOVIColorMetadata.rgb_to_lms_matrix
    rgb_to_lms_matrix,
    rgb_to_lms_matrix_mut,
    9
);
color_rational_array!(
    /// Field: AVDOVIColorMetadata.ycc_to_rgb_offset
    ycc_to_rgb_offset,
    ycc_to_rgb_offset_mut,
    3
);
color_rational_array!(
    /// Field: AVDOVIColorMetadata.ycc_to_rgb_matrix
    ycc_to_rgb_matrix,
    ycc_to_rgb_matrix_mut,
    9
);

#[cfg(test)]
mod color_metadata_tests {
    use super::*;

    #[test]
    fn layout_and_field_access_cover_scalars_and_rational_arrays() {
        assert_eq!(
            core::mem::size_of::<AVDOVIColorMetadata>(),
            core::mem::size_of::<ffi::AVDOVIColorMetadata>()
        );
        assert_eq!(
            core::mem::align_of::<AVDOVIColorMetadata>(),
            core::mem::align_of::<ffi::AVDOVIColorMetadata>()
        );

        let mut metadata = AVDOVIColorMetadata::new();
        let mut view = metadata.as_mut();
        view.set_dm_metadata_id(7);
        view.set_scene_refresh_flag(1);
        view.set_signal_eotf(2);
        view.set_signal_eotf_param0(3);
        view.set_signal_eotf_param1(4);
        view.set_signal_eotf_param2(5);
        view.set_signal_bit_depth(12);
        view.set_signal_color_space(6);
        view.set_signal_chroma_format(7);
        view.set_signal_full_range_flag(3);
        view.set_source_min_pq(8);
        view.set_source_max_pq(9);
        view.set_source_diagonal(10);

        {
            let mut matrix = view.ycc_to_rgb_matrix_mut();
            let mut first = matrix.get_mut(0).unwrap();
            first.set_num(11);
            first.set_den(12);
        }
        {
            let mut offset = view.ycc_to_rgb_offset_mut();
            offset.get_mut(2).unwrap().set_num(13);
        }
        {
            let mut matrix = view.rgb_to_lms_matrix_mut();
            matrix.get_mut(8).unwrap().set_den(14);
        }

        let shared = view.as_ref();
        assert_eq!(shared.dm_metadata_id(), 7);
        assert_eq!(shared.signal_full_range_flag(), 3);
        assert_eq!(shared.source_diagonal(), 10);
        assert_eq!(shared.ycc_to_rgb_matrix().get(0).unwrap().num(), 11);
        assert_eq!(shared.ycc_to_rgb_offset().get(2).unwrap().num(), 13);
        assert_eq!(shared.rgb_to_lms_matrix().get(8).unwrap().den(), 14);
    }

    #[test]
    #[should_panic(expected = "signal_full_range_flag must be in 0..=3")]
    fn full_range_flag_rejects_out_of_range_values() {
        AVDOVIColorMetadata::new()
            .as_mut()
            .set_signal_full_range_flag(4);
    }
}
/// Wraps: av_dovi_get_header
///
/// Borrows the header stored inside the same allocation as `metadata`.
#[must_use]
pub fn av_dovi_get_header<'a>(metadata: AVDOVIMetadataRef<'a>) -> AVDOVIRpuDataHeaderRef<'a> {
    // SAFETY: the metadata handle identifies a live allocation whose offsets
    // were initialized by libavutil. The inline helper returns the embedded
    // header, and the resulting handle is tied to the allocation borrow.
    unsafe {
        AVDOVIRpuDataHeaderRef::from_ptr(ffi::crustify_av_dovi_get_header(metadata.as_ptr()))
            .expect("a live metadata allocation has a non-null embedded header")
    }
}

#[cfg(test)]
mod header_tests {
    use super::*;

    #[repr(C)]
    struct MetadataWithHeader {
        metadata: ffi::AVDOVIMetadata,
        header: ffi::AVDOVIRpuDataHeader,
    }

    #[repr(C)]
    struct MetadataWithColor {
        metadata: ffi::AVDOVIMetadata,
        color: ffi::AVDOVIColorMetadata,
    }

    #[test]
    fn header_borrow_uses_the_metadata_offset() {
        // SAFETY: both C records consist only of integer fields, so all-zero
        // bytes are a valid initialized value before the offset is installed.
        let mut storage: MetadataWithHeader = unsafe { core::mem::zeroed() };
        storage.metadata.header_offset = core::mem::offset_of!(MetadataWithHeader, header);
        // SAFETY: storage is live, initialized and exclusively retained here;
        // the configured offset addresses its embedded header field.
        let metadata = unsafe { AVDOVIMetadataRef::from_ptr(&mut storage.metadata) }.unwrap();
        assert_eq!(
            av_dovi_get_header(metadata).as_ptr(),
            &raw const storage.header
        );
    }

    #[test]
    fn color_borrow_uses_the_metadata_offset() {
        // SAFETY: these C layouts contain integer/rational fields for which
        // all-zero is a valid initialized representation.
        let mut storage: MetadataWithColor = unsafe { core::mem::zeroed() };
        storage.metadata.color_offset = core::mem::offset_of!(MetadataWithColor, color);
        // SAFETY: the live allocation contains color at the configured offset.
        let metadata = unsafe { AVDOVIMetadataRef::from_ptr(&mut storage.metadata) }.unwrap();
        assert_eq!(
            av_dovi_get_color(metadata).as_ptr(),
            &raw const storage.color
        );
    }
}

/// Wraps: av_dovi_get_color
///
/// Borrows the color metadata embedded in the same allocation as `metadata`.
#[must_use]
pub fn av_dovi_get_color<'a>(metadata: AVDOVIMetadataRef<'a>) -> AVDOVIColorMetadataRef<'a> {
    // SAFETY: the metadata handle keeps the complete offset-based allocation
    // live. The inline helper returns its non-null embedded color record.
    unsafe {
        AVDOVIColorMetadataRef::from_ptr(ffi::crustify_av_dovi_get_color(metadata.as_ptr()))
            .expect("a live metadata allocation has embedded color metadata")
    }
}

define_ctype!(
    /// Wraps: AVDOVIDataMapping
    ///
    /// ABI-compatible by-value Dolby Vision RPU mapping metadata. Every member
    /// is an integer or fixed inline array and the record owns no resources.
    AVDOVIDataMapping,
    AVDOVIDataMappingRef,
    AVDOVIDataMappingMut,
    ffi::AVDOVIDataMapping
);

// SAFETY: the record contains only integer-backed values and fixed inline
// arrays of resource-free wrapped values, so by-value disposal is a no-op.
unsafe impl CValued for AVDOVIDataMapping {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

scalar_accessors! {
    AVDOVIDataMappingRef, AVDOVIDataMappingMut, u8;
    /// Field: AVDOVIDataMapping.mapping_chroma_format_idc
    mapping_chroma_format_idc, set_mapping_chroma_format_idc, mapping_chroma_format_idc;
    /// Field: AVDOVIDataMapping.mapping_color_space
    mapping_color_space, set_mapping_color_space, mapping_color_space;
    /// Field: AVDOVIDataMapping.vdr_rpu_id
    vdr_rpu_id, set_vdr_rpu_id, vdr_rpu_id;
}

scalar_accessors! {
    AVDOVIDataMappingRef, AVDOVIDataMappingMut, u32;
    /// Field: AVDOVIDataMapping.num_y_partitions
    num_y_partitions, set_num_y_partitions, num_y_partitions;
    /// Field: AVDOVIDataMapping.num_x_partitions
    num_x_partitions, set_num_x_partitions, num_x_partitions;
}

impl<'a> AVDOVIDataMappingRef<'a> {
    /// Field: AVDOVIDataMapping.curves
    #[must_use]
    pub fn curves(&self) -> CSlice<'a, AVDOVIReshapingCurve> {
        // SAFETY: raw-place projection locates the three initialized inline
        // curves without forming a reference. Their layout wrapper is
        // transparent and their lifetime is bounded by the mapping handle.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).curves)
                .cast::<AVDOVIReshapingCurve>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 3)
        }
    }

    /// Field: AVDOVIDataMapping.nlq_method_idc
    #[must_use]
    pub fn nlq_method_idc(&self) -> AVDOVINLQMethod {
        // SAFETY: raw-place projection copies the initialized integer-backed
        // open enum without forming a reference to C storage.
        AVDOVINLQMethod::from_raw(unsafe { addr_of!((*self.as_ptr()).nlq_method_idc).read() })
    }

    /// Field: AVDOVIDataMapping.nlq
    #[must_use]
    pub fn nlq(&self) -> CSlice<'a, AVDOVINLQParams> {
        // SAFETY: raw-place projection locates the three initialized inline
        // parameter records. The result is bound to the containing mapping.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).nlq)
                .cast::<AVDOVINLQParams>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 3)
        }
    }

    /// Field: AVDOVIDataMapping.nlq_pivots
    #[must_use]
    pub fn nlq_pivots(&self) -> CSlice<'a, u16> {
        // SAFETY: raw-place projection locates exactly two initialized u16
        // elements; the borrowed view is tied to the mapping handle.
        unsafe {
            let pointer = addr_of!((*self.as_ptr()).nlq_pivots)
                .cast::<u16>()
                .cast_mut();
            CSlice::from_raw_parts(NonNull::new_unchecked(pointer), 2)
        }
    }
}

impl AVDOVIDataMappingMut<'_> {
    /// Exclusively borrows [`curves`](AVDOVIDataMappingRef::curves).
    #[must_use]
    pub fn curves_mut(&mut self) -> CSliceMut<'_, AVDOVIReshapingCurve> {
        // SAFETY: the exclusive mapping handle supplies write provenance to
        // the three inline curves and the result is tied to this reborrow.
        unsafe {
            let pointer = addr_of_mut!((*self.as_mut_ptr()).curves).cast::<AVDOVIReshapingCurve>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 3)
        }
    }

    /// Replaces the non-linear inverse-quantization method.
    pub fn set_nlq_method_idc(&mut self, value: AVDOVINLQMethod) {
        // SAFETY: the exclusive handle supplies write provenance and the open
        // wrapper carries exactly the C integer representation.
        unsafe {
            addr_of_mut!((*self.as_mut_ptr()).nlq_method_idc).write(value.as_raw());
        }
    }

    /// Exclusively borrows [`nlq`](AVDOVIDataMappingRef::nlq).
    #[must_use]
    pub fn nlq_mut(&mut self) -> CSliceMut<'_, AVDOVINLQParams> {
        // SAFETY: the exclusive mapping handle supplies write provenance to
        // all three inline records and the result is tied to this reborrow.
        unsafe {
            let pointer = addr_of_mut!((*self.as_mut_ptr()).nlq).cast::<AVDOVINLQParams>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 3)
        }
    }

    /// Exclusively borrows [`nlq_pivots`](AVDOVIDataMappingRef::nlq_pivots).
    #[must_use]
    pub fn nlq_pivots_mut(&mut self) -> CSliceMut<'_, u16> {
        // SAFETY: the exclusive mapping handle supplies write provenance to
        // both inline u16 elements and the result is tied to this reborrow.
        unsafe {
            let pointer = addr_of_mut!((*self.as_mut_ptr()).nlq_pivots).cast::<u16>();
            CSliceMut::from_raw_parts(NonNull::new_unchecked(pointer), 2)
        }
    }
}

#[cfg(test)]
mod data_mapping_tests {
    use super::*;

    #[test]
    fn layout_and_scalar_array_accessors_round_trip() {
        assert_eq!(
            core::mem::size_of::<AVDOVIDataMapping>(),
            core::mem::size_of::<ffi::AVDOVIDataMapping>()
        );
        assert_eq!(
            core::mem::align_of::<AVDOVIDataMapping>(),
            core::mem::align_of::<ffi::AVDOVIDataMapping>()
        );

        let mut mapping = CVal::new(AVDOVIDataMapping::zeroed());
        let mut view = mapping.as_mut();
        view.set_vdr_rpu_id(1);
        view.set_mapping_color_space(2);
        view.set_mapping_chroma_format_idc(3);
        view.set_num_x_partitions(4);
        view.set_num_y_partitions(5);
        view.set_nlq_method_idc(AVDOVINLQMethod::LINEAR_DZ);
        assert!(view.nlq_pivots_mut().set_elem(1, 17));
        view.nlq_mut().get_mut(2).unwrap().set_nlq_offset(18);
        view.curves_mut().get_mut(1).unwrap().set_num_pivots(2);

        let shared = view.as_ref();
        assert_eq!(shared.vdr_rpu_id(), 1);
        assert_eq!(shared.mapping_color_space(), 2);
        assert_eq!(shared.mapping_chroma_format_idc(), 3);
        assert_eq!(shared.num_x_partitions(), 4);
        assert_eq!(shared.num_y_partitions(), 5);
        assert_eq!(shared.nlq_method_idc(), AVDOVINLQMethod::LINEAR_DZ);
        assert_eq!(shared.nlq_pivots().elem(1), Some(17));
        assert_eq!(shared.nlq().get(2).unwrap().nlq_offset(), 18);
        assert_eq!(shared.curves().get(1).unwrap().num_pivots(), 2);
    }
}

/// Wraps: av_dovi_get_mapping
///
/// Borrows the mapping embedded in the same allocation as `metadata`.
#[must_use]
pub fn av_dovi_get_mapping<'a>(metadata: AVDOVIMetadataRef<'a>) -> AVDOVIDataMappingRef<'a> {
    // SAFETY: the metadata handle keeps the complete offset-based allocation
    // live. The inline helper returns its non-null embedded mapping record;
    // this shared variant does not expose C's const-discarding return type.
    unsafe {
        AVDOVIDataMappingRef::from_ptr(ffi::crustify_av_dovi_get_mapping(metadata.as_ptr()))
            .expect("a live metadata allocation has embedded mapping metadata")
    }
}

/// Wraps: av_dovi_get_mapping
///
/// Exclusively borrows the embedded mapping. This separate variant represents
/// the C API's writable return without deriving mutation from a shared handle.
#[must_use]
pub fn av_dovi_get_mapping_mut<'a>(
    mut metadata: AVDOVIMetadataMut<'a>,
) -> AVDOVIDataMappingMut<'a> {
    // SAFETY: consuming the exclusive metadata handle preserves its exclusive
    // borrow for `'a`. The helper returns the mapping inside that allocation,
    // so the new exclusive handle cannot outlive or alias its parent borrow.
    unsafe {
        AVDOVIDataMappingMut::from_ptr(ffi::crustify_av_dovi_get_mapping(metadata.as_mut_ptr()))
            .expect("a live metadata allocation has embedded mapping metadata")
    }
}

#[cfg(test)]
mod mapping_borrow_tests {
    use super::*;

    #[repr(C)]
    struct MetadataWithMapping {
        metadata: ffi::AVDOVIMetadata,
        mapping: ffi::AVDOVIDataMapping,
    }

    fn storage() -> MetadataWithMapping {
        // SAFETY: both records contain only integer-backed values and fixed
        // inline arrays, for which all-zero is an initialized representation.
        let mut storage: MetadataWithMapping = unsafe { core::mem::zeroed() };
        storage.metadata.mapping_offset = core::mem::offset_of!(MetadataWithMapping, mapping);
        storage
    }

    #[test]
    fn shared_borrow_uses_the_mapping_offset() {
        let mut storage = storage();
        // SAFETY: the live aggregate contains mapping at the installed offset.
        let metadata = unsafe { AVDOVIMetadataRef::from_ptr(&mut storage.metadata) }.unwrap();
        assert_eq!(
            av_dovi_get_mapping(metadata).as_ptr(),
            &raw const storage.mapping
        );
    }

    #[test]
    fn exclusive_borrow_can_update_the_mapping() {
        let mut storage = storage();
        // SAFETY: the aggregate is live and exclusively accessed by this
        // handle, and its mapping offset addresses the initialized member.
        let metadata = unsafe { AVDOVIMetadataMut::from_ptr(&mut storage.metadata) }.unwrap();
        av_dovi_get_mapping_mut(metadata).set_vdr_rpu_id(9);
        assert_eq!(storage.mapping.vdr_rpu_id, 9);
    }
}

define_ctype!(
    /// Wraps: AVDOVIDmLevel10
    ///
    /// ABI-compatible inline Dolby Vision target-display metadata. The value
    /// owns no resources and is normally embedded in an extension block.
    AVDOVIDmLevel10,
    AVDOVIDmLevel10Ref,
    AVDOVIDmLevel10Mut,
    ffi::AVDOVIDmLevel10
);

define_ctype!(
    /// Wraps: AVDOVIDmLevel9
    ///
    /// ABI-compatible inline Dolby Vision source-display metadata. The value
    /// owns no resources and is normally embedded in an extension block.
    AVDOVIDmLevel9,
    AVDOVIDmLevel9Ref,
    AVDOVIDmLevel9Mut,
    ffi::AVDOVIDmLevel9
);

// SAFETY: level 10 metadata contains only integers and one resource-free
// inline color-primaries description, so inline disposal is a no-op.
unsafe impl CValued for AVDOVIDmLevel10 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

// SAFETY: level 9 metadata contains only an integer and one resource-free
// inline color-primaries description, so inline disposal is a no-op.
unsafe impl CValued for AVDOVIDmLevel9 {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVDOVIDmLevel10 {
    /// Creates zero-initialized target-display metadata in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

impl AVDOVIDmLevel9 {
    /// Creates zero-initialized source-display metadata in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

impl<'a> AVDOVIDmLevel10Ref<'a> {
    /// Field: AVDOVIDmLevel10.target_display_primaries
    #[must_use]
    pub fn target_display_primaries(&self) -> AVColorPrimariesDescRef<'a> {
        // SAFETY: raw-place projection locates the initialized inline gamut
        // description, which remains live for the parent handle's lifetime.
        unsafe {
            AVColorPrimariesDescRef::from_ptr(
                addr_of!((*self.as_ptr()).target_display_primaries).cast_mut(),
            )
        }
        .expect("an inline field address is non-null")
    }
}

impl AVDOVIDmLevel10Mut<'_> {
    /// Exclusively borrows the inline target-display primaries.
    #[must_use]
    pub fn target_display_primaries_mut(&mut self) -> AVColorPrimariesDescMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to the
        // initialized inline field for the duration of this reborrow.
        unsafe {
            AVColorPrimariesDescMut::from_ptr(addr_of_mut!(
                (*self.as_mut_ptr()).target_display_primaries
            ))
        }
        .expect("an inline field address is non-null")
    }
}

scalar_accessors! {
    AVDOVIDmLevel10Ref, AVDOVIDmLevel10Mut, u8;
    /// Field: AVDOVIDmLevel10.target_primary_index
    target_primary_index, set_target_primary_index, target_primary_index;
    /// Field: AVDOVIDmLevel10.target_display_index
    target_display_index, set_target_display_index, target_display_index;
}

scalar_accessors! {
    AVDOVIDmLevel10Ref, AVDOVIDmLevel10Mut, u16;
    /// Field: AVDOVIDmLevel10.target_min_pq
    target_min_pq, set_target_min_pq, target_min_pq;
    /// Field: AVDOVIDmLevel10.target_max_pq
    target_max_pq, set_target_max_pq, target_max_pq;
}

impl<'a> AVDOVIDmLevel9Ref<'a> {
    /// Field: AVDOVIDmLevel9.source_display_primaries
    #[must_use]
    pub fn source_display_primaries(&self) -> AVColorPrimariesDescRef<'a> {
        // SAFETY: raw-place projection locates the initialized inline gamut
        // description, which remains live for the parent handle's lifetime.
        unsafe {
            AVColorPrimariesDescRef::from_ptr(
                addr_of!((*self.as_ptr()).source_display_primaries).cast_mut(),
            )
        }
        .expect("an inline field address is non-null")
    }
}

impl AVDOVIDmLevel9Mut<'_> {
    /// Exclusively borrows the inline source-display primaries.
    #[must_use]
    pub fn source_display_primaries_mut(&mut self) -> AVColorPrimariesDescMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to the
        // initialized inline field for the duration of this reborrow.
        unsafe {
            AVColorPrimariesDescMut::from_ptr(addr_of_mut!(
                (*self.as_mut_ptr()).source_display_primaries
            ))
        }
        .expect("an inline field address is non-null")
    }
}

scalar_accessors! {
    AVDOVIDmLevel9Ref, AVDOVIDmLevel9Mut, u8;
    /// Field: AVDOVIDmLevel9.source_primary_index
    source_primary_index, set_source_primary_index, source_primary_index;
}

#[cfg(test)]
mod dm_level_9_10_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layouts_and_nested_primaries_match_c() {
        assert_eq!(
            size_of::<AVDOVIDmLevel9>(),
            size_of::<ffi::AVDOVIDmLevel9>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel9>(),
            align_of::<ffi::AVDOVIDmLevel9>()
        );
        assert_eq!(
            size_of::<AVDOVIDmLevel10>(),
            size_of::<ffi::AVDOVIDmLevel10>()
        );
        assert_eq!(
            align_of::<AVDOVIDmLevel10>(),
            align_of::<ffi::AVDOVIDmLevel10>()
        );

        let mut level9 = AVDOVIDmLevel9::new();
        level9.as_mut().set_source_primary_index(2);
        level9
            .as_mut()
            .source_display_primaries_mut()
            .wp_mut()
            .x_mut()
            .set_num(31_270);
        assert_eq!(level9.as_ref().source_primary_index(), 2);
        assert_eq!(
            level9.as_ref().source_display_primaries().wp().x().num(),
            31_270
        );

        let mut level10 = AVDOVIDmLevel10::new();
        let mut view = level10.as_mut();
        view.set_target_display_index(3);
        view.set_target_primary_index(4);
        view.set_target_min_pq(16);
        view.set_target_max_pq(3_072);
        view.target_display_primaries_mut()
            .prim_mut()
            .r_mut()
            .x_mut()
            .set_num(64);
        let view = level10.as_ref();
        assert_eq!(view.target_display_index(), 3);
        assert_eq!(view.target_primary_index(), 4);
        assert_eq!(view.target_min_pq(), 16);
        assert_eq!(view.target_max_pq(), 3_072);
        assert_eq!(view.target_display_primaries().prim().r().x().num(), 64);
    }
}

define_ctype!(
    /// Wraps: AVDOVIDmData
    ///
    /// Layout-compatible tagged union for one Dolby Vision metadata extension
    /// block. Borrowed access validates `level` before projecting the matching
    /// union member, so safe callers cannot read an inactive member.
    AVDOVIDmData,
    AVDOVIDmDataRef,
    AVDOVIDmDataMut,
    ffi::AVDOVIDmData
);

// SAFETY: every active variant contains only by-value integer or rational
// metadata and owns no resource, so disposing an initialized inline block is a
// no-op regardless of its level tag.
unsafe impl CValued for AVDOVIDmData {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

/// A validated shared view of the active `AVDOVIDmData` union member.
#[derive(Clone, Copy)]
pub enum AVDOVIDmDataActiveRef<'a> {
    Level1(AVDOVIDmLevel1Ref<'a>),
    Level2(AVDOVIDmLevel2Ref<'a>),
    Level3(AVDOVIDmLevel3Ref<'a>),
    Level4(AVDOVIDmLevel4Ref<'a>),
    Level5(AVDOVIDmLevel5Ref<'a>),
    Level6(AVDOVIDmLevel6Ref<'a>),
    Level8(AVDOVIDmLevel8Ref<'a>),
    Level9(AVDOVIDmLevel9Ref<'a>),
    Level10(AVDOVIDmLevel10Ref<'a>),
    Level11(AVDOVIDmLevel11Ref<'a>),
    Level254(AVDOVIDmLevel254Ref<'a>),
    Level255(AVDOVIDmLevel255Ref<'a>),
    /// A level that this crate does not know how to interpret.
    Unknown(u8),
}

/// A validated exclusive view of the active `AVDOVIDmData` union member.
pub enum AVDOVIDmDataActiveMut<'a> {
    Level1(AVDOVIDmLevel1Mut<'a>),
    Level2(AVDOVIDmLevel2Mut<'a>),
    Level3(AVDOVIDmLevel3Mut<'a>),
    Level4(AVDOVIDmLevel4Mut<'a>),
    Level5(AVDOVIDmLevel5Mut<'a>),
    Level6(AVDOVIDmLevel6Mut<'a>),
    Level8(AVDOVIDmLevel8Mut<'a>),
    Level9(AVDOVIDmLevel9Mut<'a>),
    Level10(AVDOVIDmLevel10Mut<'a>),
    Level11(AVDOVIDmLevel11Mut<'a>),
    Level254(AVDOVIDmLevel254Mut<'a>),
    Level255(AVDOVIDmLevel255Mut<'a>),
    /// A level that this crate does not know how to interpret.
    Unknown(u8),
}

impl AVDOVIDmDataRef<'_> {
    /// Field: AVDOVIDmData.level
    #[must_use]
    pub fn level(&self) -> u8 {
        // SAFETY: raw-place projection copies the initialized tag from the
        // live block without forming a reference to C-visible storage.
        unsafe { addr_of!((*self.as_ptr()).level).read() }
    }

    /// Field: AVDOVIDmData.(unknown field)
    ///
    /// Interprets the anonymous union according to its public level tag.
    #[must_use]
    pub fn active(&self) -> AVDOVIDmDataActiveRef<'_> {
        match self.level() {
            1 => AVDOVIDmDataActiveRef::Level1(self.l1().expect("level was checked")),
            2 => AVDOVIDmDataActiveRef::Level2(self.l2().expect("level was checked")),
            3 => AVDOVIDmDataActiveRef::Level3(self.l3().expect("level was checked")),
            4 => AVDOVIDmDataActiveRef::Level4(self.l4().expect("level was checked")),
            5 => AVDOVIDmDataActiveRef::Level5(self.l5().expect("level was checked")),
            6 => AVDOVIDmDataActiveRef::Level6(self.l6().expect("level was checked")),
            8 => AVDOVIDmDataActiveRef::Level8(self.l8().expect("level was checked")),
            9 => AVDOVIDmDataActiveRef::Level9(self.l9().expect("level was checked")),
            10 => AVDOVIDmDataActiveRef::Level10(self.l10().expect("level was checked")),
            11 => AVDOVIDmDataActiveRef::Level11(self.l11().expect("level was checked")),
            254 => AVDOVIDmDataActiveRef::Level254(self.l254().expect("level was checked")),
            255 => AVDOVIDmDataActiveRef::Level255(self.l255().expect("level was checked")),
            level => AVDOVIDmDataActiveRef::Unknown(level),
        }
    }
}

impl AVDOVIDmDataMut<'_> {
    /// Exclusively borrows the active anonymous-union member.
    #[must_use]
    pub fn active_mut(&mut self) -> AVDOVIDmDataActiveMut<'_> {
        match self.as_ref().level() {
            1 => AVDOVIDmDataActiveMut::Level1(self.l1_mut().expect("level was checked")),
            2 => AVDOVIDmDataActiveMut::Level2(self.l2_mut().expect("level was checked")),
            3 => AVDOVIDmDataActiveMut::Level3(self.l3_mut().expect("level was checked")),
            4 => AVDOVIDmDataActiveMut::Level4(self.l4_mut().expect("level was checked")),
            5 => AVDOVIDmDataActiveMut::Level5(self.l5_mut().expect("level was checked")),
            6 => AVDOVIDmDataActiveMut::Level6(self.l6_mut().expect("level was checked")),
            8 => AVDOVIDmDataActiveMut::Level8(self.l8_mut().expect("level was checked")),
            9 => AVDOVIDmDataActiveMut::Level9(self.l9_mut().expect("level was checked")),
            10 => AVDOVIDmDataActiveMut::Level10(self.l10_mut().expect("level was checked")),
            11 => AVDOVIDmDataActiveMut::Level11(self.l11_mut().expect("level was checked")),
            254 => AVDOVIDmDataActiveMut::Level254(self.l254_mut().expect("level was checked")),
            255 => AVDOVIDmDataActiveMut::Level255(self.l255_mut().expect("level was checked")),
            level => AVDOVIDmDataActiveMut::Unknown(level),
        }
    }
}

macro_rules! dm_data_union_field {
    (
        $(#[$meta:meta])*
        $getter:ident, $getter_mut:ident, $select:ident,
        $level:literal, $shared:ident, $exclusive:ident, $raw:ty, $field:ident
    ) => {
        impl<'a> AVDOVIDmDataRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $getter(&self) -> Option<$shared<'a>> {
                if self.level() != $level {
                    return None;
                }
                // SAFETY: the checked tag identifies this as the active union
                // member; raw projection forms no reference, and the member
                // remains live for the enclosing handle's lifetime.
                unsafe {
                    $shared::from_ptr(
                        addr_of!((*self.as_ptr()).__bindgen_anon_1.$field).cast_mut(),
                    )
                }
            }
        }

        impl AVDOVIDmDataMut<'_> {
            #[doc = "Exclusively borrows this member when its level is active."]
            #[must_use]
            pub fn $getter_mut(&mut self) -> Option<$exclusive<'_>> {
                if self.as_ref().level() != $level {
                    return None;
                }
                // SAFETY: the checked tag identifies the active member and
                // the exclusive parent handle supplies write provenance for
                // the duration of the returned reborrow.
                unsafe {
                    $exclusive::from_ptr(addr_of_mut!(
                        (*self.as_mut_ptr()).__bindgen_anon_1.$field
                    ))
                }
            }

            #[doc = "Selects this level, zero-initializes its union member, and returns it exclusively."]
            #[must_use]
            pub fn $select(&mut self) -> $exclusive<'_> {
                let ptr = self.as_mut_ptr();
                // SAFETY: all fields of this metadata type accept zero, and
                // the exclusive parent handle permits initializing the chosen
                // union member and then publishing its matching tag.
                unsafe {
                    addr_of_mut!((*ptr).__bindgen_anon_1.$field)
                        .write(core::mem::zeroed::<$raw>());
                    addr_of_mut!((*ptr).level).write($level);
                    $exclusive::from_ptr(addr_of_mut!((*ptr).__bindgen_anon_1.$field))
                        .expect("an embedded union member address is non-null")
                }
            }
        }
    };
}

dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l1
    l1, l1_mut, select_l1, 1, AVDOVIDmLevel1Ref, AVDOVIDmLevel1Mut,
    ffi::AVDOVIDmLevel1, l1
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l2
    l2, l2_mut, select_l2, 2, AVDOVIDmLevel2Ref, AVDOVIDmLevel2Mut,
    ffi::AVDOVIDmLevel2, l2
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l3
    l3, l3_mut, select_l3, 3, AVDOVIDmLevel3Ref, AVDOVIDmLevel3Mut,
    ffi::AVDOVIDmLevel3, l3
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l4
    l4, l4_mut, select_l4, 4, AVDOVIDmLevel4Ref, AVDOVIDmLevel4Mut,
    ffi::AVDOVIDmLevel4, l4
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l5
    l5, l5_mut, select_l5, 5, AVDOVIDmLevel5Ref, AVDOVIDmLevel5Mut,
    ffi::AVDOVIDmLevel5, l5
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l6
    l6, l6_mut, select_l6, 6, AVDOVIDmLevel6Ref, AVDOVIDmLevel6Mut,
    ffi::AVDOVIDmLevel6, l6
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l8
    l8, l8_mut, select_l8, 8, AVDOVIDmLevel8Ref, AVDOVIDmLevel8Mut,
    ffi::AVDOVIDmLevel8, l8
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l9
    l9, l9_mut, select_l9, 9, AVDOVIDmLevel9Ref, AVDOVIDmLevel9Mut,
    ffi::AVDOVIDmLevel9, l9
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l10
    l10, l10_mut, select_l10, 10, AVDOVIDmLevel10Ref, AVDOVIDmLevel10Mut,
    ffi::AVDOVIDmLevel10, l10
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l11
    l11, l11_mut, select_l11, 11, AVDOVIDmLevel11Ref, AVDOVIDmLevel11Mut,
    ffi::AVDOVIDmLevel11, l11
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l254
    l254, l254_mut, select_l254, 254, AVDOVIDmLevel254Ref, AVDOVIDmLevel254Mut,
    ffi::AVDOVIDmLevel254, l254
);
dm_data_union_field!(
    /// Field: AVDOVIDmData.(unknown field).l255
    l255, l255_mut, select_l255, 255, AVDOVIDmLevel255Ref, AVDOVIDmLevel255Mut,
    ffi::AVDOVIDmLevel255, l255
);

#[cfg(test)]
mod dm_data_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_matches_bindgen() {
        assert_eq!(size_of::<AVDOVIDmData>(), size_of::<ffi::AVDOVIDmData>());
        assert_eq!(align_of::<AVDOVIDmData>(), align_of::<ffi::AVDOVIDmData>());
    }

    #[test]
    fn selection_keeps_tag_and_union_member_in_sync() {
        let mut data = CVal::new(AVDOVIDmData::zeroed());
        {
            let mut data_view = data.as_mut();
            let mut level = data_view.select_l2();
            level.set_target_max_pq(123);
            level.set_ms_weight(-4);
        }

        let shared = data.as_ref();
        assert_eq!(shared.level(), 2);
        assert!(shared.l1().is_none());
        let level = shared.l2().expect("level 2 is active");
        assert_eq!(level.target_max_pq(), 123);
        assert_eq!(level.ms_weight(), -4);
        assert!(matches!(shared.active(), AVDOVIDmDataActiveRef::Level2(_)));
    }

    #[test]
    fn unknown_levels_are_not_interpreted_as_union_members() {
        let mut data = CVal::new(AVDOVIDmData::zeroed());
        data.as_mut().select_l255().set_dm_debug([1, 2, 3, 4]);
        assert!(matches!(
            data.as_ref().active(),
            AVDOVIDmDataActiveRef::Level255(_)
        ));

        // SAFETY: the test has exclusive access to initialized inline storage;
        // changing only the tag to an unknown value leaves a valid opaque union.
        unsafe { addr_of_mut!((*data.as_mut().as_mut_ptr()).level).write(42) };
        assert!(matches!(
            data.as_ref().active(),
            AVDOVIDmDataActiveRef::Unknown(42)
        ));
    }
}

/// Wraps: av_dovi_get_ext
///
/// Borrows extension block `index` from the metadata allocation. Returns
/// `None` when `index` is outside the initialized extension-block range.
#[must_use]
pub fn av_dovi_get_ext<'a>(
    metadata: AVDOVIMetadataRef<'a>,
    index: usize,
) -> Option<AVDOVIDmDataRef<'a>> {
    let count = usize::try_from(metadata.num_ext_blocks()).unwrap_or(0);
    if index >= count {
        return None;
    }
    let index = i32::try_from(index).ok()?;

    // SAFETY: `index` was checked against the allocation's initialized block
    // count. The metadata handle keeps the complete offset-based allocation
    // live, and the returned shared handle is tied to that borrow.
    unsafe { AVDOVIDmDataRef::from_ptr(ffi::crustify_av_dovi_get_ext(metadata.as_ptr(), index)) }
}

/// Wraps: av_dovi_get_ext
///
/// Exclusively borrows extension block `index`. This separate variant exposes
/// C's writable result only when the complete metadata allocation is borrowed
/// exclusively. Returns `None` when `index` is out of range.
#[must_use]
pub fn av_dovi_get_ext_mut<'a>(
    mut metadata: AVDOVIMetadataMut<'a>,
    index: usize,
) -> Option<AVDOVIDmDataMut<'a>> {
    let count = usize::try_from(metadata.as_ref().num_ext_blocks()).unwrap_or(0);
    if index >= count {
        return None;
    }
    let index = i32::try_from(index).ok()?;

    // SAFETY: consuming the exclusive metadata handle preserves its exclusive
    // borrow for `'a`; the checked index selects one initialized block inside
    // the live allocation, so the result cannot outlive or alias its parent.
    unsafe {
        AVDOVIDmDataMut::from_ptr(ffi::crustify_av_dovi_get_ext(metadata.as_mut_ptr(), index))
    }
}

#[cfg(test)]
mod get_ext_tests {
    use super::*;

    #[repr(C)]
    struct MetadataWithExtensions {
        metadata: ffi::AVDOVIMetadata,
        extensions: [ffi::AVDOVIDmData; 3],
    }

    fn storage() -> MetadataWithExtensions {
        // SAFETY: the metadata header and extension blocks contain only
        // integer-backed, resource-free fields for which zero is initialized.
        let mut storage: MetadataWithExtensions = unsafe { core::mem::zeroed() };
        storage.metadata.ext_block_offset =
            core::mem::offset_of!(MetadataWithExtensions, extensions);
        storage.metadata.ext_block_size = core::mem::size_of::<ffi::AVDOVIDmData>();
        storage.metadata.num_ext_blocks = 3;
        storage
    }

    #[test]
    fn shared_borrow_checks_bounds_and_uses_the_c_stride() {
        let mut storage = storage();
        // SAFETY: the live aggregate has the initialized extension-block
        // layout described by its metadata header.
        let metadata = unsafe { AVDOVIMetadataRef::from_ptr(&mut storage.metadata) }.unwrap();

        let second = av_dovi_get_ext(metadata, 1).expect("index 1 is initialized");
        assert_eq!(second.as_ptr(), &raw const storage.extensions[1]);
        assert!(av_dovi_get_ext(metadata, 3).is_none());
        assert!(av_dovi_get_ext(metadata, usize::MAX).is_none());
    }

    #[test]
    fn exclusive_borrow_updates_only_the_selected_block() {
        let mut storage = storage();
        // SAFETY: the live aggregate is exclusively accessed and its header
        // describes all three initialized extension blocks.
        let metadata = unsafe { AVDOVIMetadataMut::from_ptr(&mut storage.metadata) }.unwrap();
        av_dovi_get_ext_mut(metadata, 1)
            .expect("index 1 is initialized")
            .select_l5()
            .set_left_offset(17);

        assert_eq!(storage.extensions[0].level, 0);
        assert_eq!(storage.extensions[1].level, 5);
        assert_eq!(storage.extensions[2].level, 0);
    }
}
