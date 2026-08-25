//! Wrappers for `libavutil/dovi_meta.c`.

use core::ptr::{addr_of, addr_of_mut};

use ffibox::define_ctype;

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

#[cfg(test)]
mod tests {
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
}
