//! Wrappers for `libavutil/dovi_meta.c`.

use core::ffi::c_void;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CDropped, CValued, define_ctype};

use crate::ffi;

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
