//! Wrappers for libavutil frames.

use core::mem::MaybeUninit;
use core::ptr::{NonNull, addr_of};

use ffibox::{CSlice, CSliceMut, define_ctype};

use crate::buffer::AVBufferReferenceRef;
use crate::dict::{AVDictionaryMut, AVDictionaryRef};
use crate::ffi;

/// Wraps: AVFrameSideDataType
///
/// A layout-compatible value for libavutil's frame side-data type. The
/// transparent integer representation preserves values introduced by newer
/// libavutil versions instead of turning an unfamiliar C value into an invalid
/// Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVFrameSideDataType(ffi::AVFrameSideDataType);

impl AVFrameSideDataType {
    pub const PANSCAN: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_PANSCAN);
    pub const A53_CC: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_A53_CC);
    pub const STEREO3D: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_STEREO3D);
    pub const MATRIXENCODING: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MATRIXENCODING);
    pub const DOWNMIX_INFO: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_INFO);
    pub const REPLAYGAIN: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_REPLAYGAIN);
    pub const DISPLAYMATRIX: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DISPLAYMATRIX);
    pub const AFD: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AFD);
    pub const MOTION_VECTORS: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MOTION_VECTORS);
    pub const SKIP_SAMPLES: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SKIP_SAMPLES);
    pub const AUDIO_SERVICE_TYPE: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AUDIO_SERVICE_TYPE);
    pub const MASTERING_DISPLAY_METADATA: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_MASTERING_DISPLAY_METADATA);
    pub const GOP_TIMECODE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_GOP_TIMECODE);
    pub const SPHERICAL: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SPHERICAL);
    pub const CONTENT_LIGHT_LEVEL: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_CONTENT_LIGHT_LEVEL);
    pub const ICC_PROFILE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_ICC_PROFILE);
    pub const S12M_TIMECODE: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_S12M_TIMECODE);
    pub const DYNAMIC_HDR_PLUS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_PLUS);
    pub const REGIONS_OF_INTEREST: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_REGIONS_OF_INTEREST);
    pub const VIDEO_ENC_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIDEO_ENC_PARAMS);
    pub const SEI_UNREGISTERED: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_SEI_UNREGISTERED);
    pub const FILM_GRAIN_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_FILM_GRAIN_PARAMS);
    pub const DETECTION_BBOXES: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DETECTION_BBOXES);
    pub const DOVI_RPU_BUFFER: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOVI_RPU_BUFFER);
    pub const DOVI_METADATA: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOVI_METADATA);
    pub const DYNAMIC_HDR_VIVID: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_VIVID);
    pub const AMBIENT_VIEWING_ENVIRONMENT: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT);
    pub const VIDEO_HINT: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIDEO_HINT);
    pub const LCEVC: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_LCEVC);
    pub const VIEW_ID: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_VIEW_ID);
    pub const REFERENCE_DISPLAYS_3D: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_3D_REFERENCE_DISPLAYS);
    pub const EXIF: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_EXIF);
    pub const DYNAMIC_HDR_SMPTE_2094_APP5: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5);
    pub const IAMF_MIX_GAIN_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM);
    pub const IAMF_DEMIXING_INFO_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM);
    pub const IAMF_RECON_GAIN_INFO_PARAM: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM);
    pub const RAW_COLOR_PARAMS: Self =
        Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_RAW_COLOR_PARAMS);
    pub const DOWNMIX_MATRIX: Self = Self(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_MATRIX);

    /// Returns the raw side-data type accepted by libavutil.
    #[inline]
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVFrameSideDataType {
        self.0
    }

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVFrameSideDataType) -> Self {
        Self(raw)
    }
}

impl From<ffi::AVFrameSideDataType> for AVFrameSideDataType {
    fn from(value: ffi::AVFrameSideDataType) -> Self {
        Self::from_raw(value)
    }
}

impl From<AVFrameSideDataType> for ffi::AVFrameSideDataType {
    fn from(value: AVFrameSideDataType) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn side_data_types_are_layout_compatible_and_open() {
        let first = AVFrameSideDataType::from_raw(ffi::AVFrameSideDataType_AV_FRAME_DATA_PANSCAN);
        let last =
            AVFrameSideDataType::from_raw(ffi::AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_MATRIX);

        assert_eq!(first, AVFrameSideDataType::PANSCAN);
        assert_eq!(last, AVFrameSideDataType::DOWNMIX_MATRIX);
        assert_eq!(first.as_raw(), 0);
        assert_eq!(last.as_raw(), 37);
        let future = ffi::AVFrameSideDataType::MAX;
        assert_eq!(AVFrameSideDataType::from_raw(future).as_raw(), future);
        assert_eq!(
            size_of::<AVFrameSideDataType>(),
            size_of::<ffi::AVFrameSideDataType>()
        );
        assert_eq!(
            align_of::<AVFrameSideDataType>(),
            align_of::<ffi::AVFrameSideDataType>()
        );
    }
}

define_ctype!(
    /// Wraps: AVFrameSideData
    ///
    /// A side-data header borrowed from an owning frame or side-data array.
    /// The header is not independently owned: `av_mallocz` allocates it inside
    /// `add_side_data_from_buf_ext`, and `free_side_data_entry` — reached only
    /// through the array entry points `av_frame_side_data_free`,
    /// `av_frame_side_data_remove` and `av_frame_side_data_remove_by_props` —
    /// releases its buffer reference, its dictionary and the header together.
    /// libavutil publishes no way to release one entry on its own, so this type
    /// has no `CDropped` and no owning wrapper: it is reached through handles
    /// borrowed from the collection.
    ///
    /// # Invariant
    ///
    /// `AVFrameSideDataRef::from_ptr` and `AVFrameSideDataMut::from_ptr`
    /// promise only that the header itself is live and initialized. Every
    /// wrapped header additionally satisfies, and every unsafe constructor of
    /// one owes:
    ///
    /// - `buf` is null, or one live [`AVBufferReference`] owned by this entry
    ///   and satisfying that type's own invariant, kept alive for at least the
    ///   borrow. `ff_frame_side_data_add_from_buf` refuses a null `buf`, so a
    ///   header libavutil built has one; the field is nullable here only
    ///   because a hand-built C entry may leave it unset;
    /// - `data` is null, or addresses `size` **allocated** bytes kept alive by
    ///   that `buf`. Allocated is all that is claimed — see
    ///   [`data`](AVFrameSideDataRef::data) for why the contents are not;
    /// - `metadata` is null, or one live dictionary owned by this entry.
    ///
    /// [`kind`](AVFrameSideDataRef::kind) and [`size`](AVFrameSideDataRef::size)
    /// read the header itself and need nothing beyond `from_ptr`. The safe
    /// getters that rest on the list above are
    /// [`buffer`](AVFrameSideDataRef::buffer), [`data`](AVFrameSideDataRef::data),
    /// [`metadata`](AVFrameSideDataRef::metadata),
    /// [`data_mut`](AVFrameSideDataMut::data_mut),
    /// [`write_all`](AVFrameSideDataMut::write_all) and
    /// [`metadata_mut`](AVFrameSideDataMut::metadata_mut).
    ///
    /// Every producer discharges it: `add_side_data_from_buf_ext`
    /// (`libavutil/side_data.c`) is the only routine that fills a new entry,
    /// and it writes `buf`, `data` and `size` from one reference header it has
    /// just taken over, while `replace_side_data_from_buf` rewrites the same
    /// three together. So the wrappers over `av_frame_new_side_data` and
    /// `av_frame_get_side_data`, and [`AVFrameRef::side_data`] /
    /// [`AVFrameMut::side_data_mut`] reading the owning frame's table, all
    /// hold. [`write_all`](AVFrameSideDataMut::write_all) is the only safe
    /// writer in this crate and writes inside the window without touching any
    /// of the three fields.
    ///
    /// [`AVBufferReference`]: crate::buffer::AVBufferReference
    AVFrameSideData,
    AVFrameSideDataRef,
    AVFrameSideDataMut,
    ffi::AVFrameSideData
);

impl<'a> AVFrameSideDataRef<'a> {
    /// Wraps: AVFrameSideData.type
    #[must_use]
    pub fn kind(&self) -> AVFrameSideDataType {
        // SAFETY: the integer-backed enum is copied through a raw projection;
        // AVFrameSideDataType preserves unknown ABI values.
        AVFrameSideDataType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Wraps: AVFrameSideData.buf
    ///
    /// Borrows the reference header that owns the backing byte allocation.
    #[must_use]
    pub fn buffer(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the pointer value is copied through a raw projection from the
        // live header; no reference to C storage is formed.
        let buffer = unsafe { addr_of!((*self.as_ptr()).buf).read() };
        // SAFETY: by this type's invariant a non-null `buf` is one live
        // reference header owned by the entry, satisfying `AVBufferReference`'s
        // own invariant and kept alive for `'a` by the collection that owns the
        // entry.
        unsafe { AVBufferReferenceRef::from_ptr(buffer) }
    }

    /// Wraps: AVFrameSideData.buf
    ///
    /// Clones an owned reference to the same underlying buffer, leaving the
    /// entry's own field valid. This is how a caller keeps the bytes alive past
    /// the entry, and it is also what makes the entry's window read-only again:
    /// [`data_mut`](AVFrameSideDataMut::data_mut) and
    /// [`write_all`](AVFrameSideDataMut::write_all) both refuse a buffer with
    /// more than one reference.
    #[must_use]
    pub fn owned_buffer(&self) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.buffer()?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new, independently releasable reference header holding a
        // count of its own on the same underlying buffer.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrameSideData.size
    #[must_use]
    pub fn size(&self) -> usize {
        // SAFETY: the scalar is copied through a raw projection from the live
        // header and no reference to C storage is formed.
        unsafe { addr_of!((*self.as_ptr()).size).read() }
    }

    /// Wraps: AVFrameSideData.data
    ///
    /// Views the `size`-byte window kept alive by `buf`, without materializing
    /// a Rust slice over memory that C may mutate, and without claiming its
    /// bytes have been written. `av_frame_new_side_data` (`frame.c`) and
    /// `av_frame_side_data_new` (`side_data.c`) both size the entry with
    /// `av_buffer_alloc`, which hands back raw `av_malloc` storage, so an
    /// initialized `CSlice<u8>` here would break
    /// [`CSlice::from_raw_parts`](ffibox::CSlice::from_raw_parts)'s
    /// precondition and let entirely safe code read an uninitialised `u8`.
    /// `None` represents a null `data` slot.
    ///
    /// Fill a window from safe code with
    /// [`write_all`](AVFrameSideDataMut::write_all), then read it through
    /// [`data_assume_init`](Self::data_assume_init).
    #[must_use]
    pub fn data(&self) -> Option<CSlice<'a, MaybeUninit<u8>>> {
        // SAFETY: both fields are copied through raw projections from the live
        // header; no reference to C storage is formed.
        let (data, size) = unsafe {
            (
                addr_of!((*self.as_ptr()).data).read(),
                addr_of!((*self.as_ptr()).size).read(),
            )
        };
        NonNull::new(data.cast::<MaybeUninit<u8>>()).map(|data| {
            // SAFETY: the type invariant gives `size` allocated bytes at a
            // non-null `data`, kept alive for `'a` by the entry's `buf`. Every
            // byte pattern — and the absence of one — is a valid
            // `MaybeUninit<u8>`, which is exactly what "allocated" licenses.
            unsafe { CSlice::from_raw_parts(data, size) }
        })
    }

    /// Wraps: AVFrameSideData.data
    ///
    /// The same window, viewed as initialized bytes.
    ///
    /// # Safety
    ///
    /// All `size` bytes of the window must already have been written: by
    /// [`write_all`](AVFrameSideDataMut::write_all), by the producer that
    /// filled the entry (`frame_copy_props` memcpys the whole window,
    /// `av_frame_side_data_clone` hands over a buffer it copied), or by C code
    /// outside this crate. A window `av_frame_new_side_data` or
    /// `av_frame_side_data_new` has just sized does not satisfy this.
    #[must_use]
    pub unsafe fn data_assume_init(&self) -> Option<CSlice<'a, u8>> {
        let window = self.data()?;
        let data = NonNull::new(window.as_elem_ptr().cast::<u8>())?;
        // SAFETY: the caller asserts every byte of the window is initialized,
        // and `data` has already established `window.len()` allocated bytes at
        // this address for `'a`. `MaybeUninit<u8>` and `u8` share size and
        // alignment, so both views describe the same run.
        Some(unsafe { CSlice::from_raw_parts(data, window.len()) })
    }

    /// Wraps: AVFrameSideData.buf
    ///
    /// Whether this entry's window may be written through: `buf` must be
    /// present and be the only reference to a buffer that is not read-only.
    ///
    /// Two entries never share one `AVBufferRef` header — `av_buffer_ref`
    /// allocates a fresh header per reference — so a second holder of the
    /// underlying buffer always shows up in the count this predicate reads.
    /// It answers about the buffer, not about the handle: holding a shared
    /// side-data handle and a `true` here still licenses no write.
    #[must_use]
    pub fn is_writable(&self) -> bool {
        // SAFETY: the pointer value is copied through a raw projection from the
        // live header; no reference to C storage is formed.
        let buffer = unsafe { addr_of!((*self.as_ptr()).buf).read() };
        if buffer.is_null() {
            return false;
        }
        // SAFETY: by this type's invariant a non-null `buf` is one live
        // reference header owned by the entry. The C predicate reads the
        // underlying count and read-only flag and retains nothing.
        unsafe { ffi::av_buffer_is_writable(buffer) != 0 }
    }

    /// Wraps: AVFrameSideData.metadata
    ///
    /// Borrows the optional dictionary owned by this side-data entry.
    #[must_use]
    pub fn metadata(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: the pointer value is copied through a raw projection from the
        // live header; no reference to C storage is formed.
        let metadata = unsafe { addr_of!((*self.as_ptr()).metadata).read() };
        // SAFETY: by this type's invariant a non-null `metadata` is one live
        // dictionary owned by the entry, kept alive for `'a` by the collection
        // that owns the entry.
        unsafe { AVDictionaryRef::from_ptr(metadata) }
    }
}

impl AVFrameSideDataMut<'_> {
    /// Wraps: AVFrameSideData.data
    ///
    /// Exclusively views the byte window when its backing buffer is writable.
    /// As with [`AVFrameSideDataRef::data`], the elements are `MaybeUninit<u8>`
    /// because the type invariant claims the window is allocated, not written.
    #[must_use]
    pub fn data_mut(&mut self) -> Option<CSliceMut<'_, MaybeUninit<u8>>> {
        if !self.as_ref().is_writable() {
            return None;
        }
        // SAFETY: the fields are copied through raw projections from the live
        // exclusive header; no reference to C storage is formed.
        let (data, size) = unsafe {
            let header = self.as_mut_ptr();
            (
                addr_of!((*header).data).read(),
                addr_of!((*header).size).read(),
            )
        };
        NonNull::new(data.cast::<MaybeUninit<u8>>()).map(|data| {
            // SAFETY: the type invariant gives `size` allocated bytes at a
            // non-null `data`, every byte of which is a valid
            // `MaybeUninit<u8>`. The writability check proves nothing else
            // shares the underlying buffer, and the view is bound to
            // `&mut self`, so this is the only path to those bytes.
            unsafe { CSliceMut::from_raw_parts(data, size) }
        })
    }

    /// Wraps: AVFrameSideData.data
    ///
    /// The same exclusive window, viewed as initialized bytes, for
    /// read-modify-write access.
    ///
    /// # Safety
    ///
    /// As [`AVFrameSideDataRef::data_assume_init`]: every byte of the window
    /// must already have been written.
    #[must_use]
    pub unsafe fn data_assume_init_mut(&mut self) -> Option<CSliceMut<'_, u8>> {
        let mut window = self.data_mut()?;
        let len = window.len();
        let data = NonNull::new(window.as_mut_elem_ptr().cast::<u8>())?;
        // SAFETY: the caller asserts the window is initialized, and `data_mut`
        // has already proved exclusive access to `len` allocated bytes at
        // `data`. `MaybeUninit<u8>` and `u8` share size and alignment, so both
        // views describe the same run.
        Some(unsafe { CSliceMut::from_raw_parts(data, len) })
    }

    /// Wraps: AVFrameSideData.data
    ///
    /// Writes `src` over the whole window, which is the safe way to make
    /// [`AVFrameSideDataRef::data_assume_init`] dischargeable for an entry
    /// libavutil only sized. Returns `false` without writing anything when
    /// `src` does not cover the window exactly, or when the entry's buffer is
    /// shared or read-only.
    pub fn write_all(&mut self, src: &[u8]) -> bool {
        if src.len() != self.as_ref().size() {
            return false;
        }
        let Some(mut window) = self.data_mut() else {
            // A null `data` slot has no bytes to write, so an empty `src`
            // leaves the window vacuously initialized; any other `None` means
            // the buffer is shared, read-only or absent.
            return src.is_empty() && self.as_ref().is_writable();
        };
        // SAFETY: `window` exclusively covers exactly `src.len()` allocated
        // bytes of C storage, and `src` is a live Rust slice, which therefore
        // cannot overlap storage a C object owns. The copy initializes every
        // byte of the window.
        unsafe {
            core::ptr::copy_nonoverlapping(
                src.as_ptr(),
                window.as_mut_elem_ptr().cast::<u8>(),
                src.len(),
            );
        }
        true
    }

    /// Wraps: AVFrameSideData.metadata
    ///
    /// Exclusively borrows the optional dictionary owned by this entry.
    ///
    /// `AVDictionaryMut` deliberately carries no operations of its own: every
    /// mutating libavutil entry point takes an `AVDictionary **` owner slot,
    /// because it may reallocate or release the header. Replacing this entry's
    /// dictionary would therefore have to go through the field, which no safe
    /// wrapper offers; this handle exists so an exclusive borrow of the entry
    /// still reaches the dictionary's shared surface through
    /// [`as_ref`](crate::dict::AVDictionaryMut::as_ref).
    #[must_use]
    pub fn metadata_mut(&mut self) -> Option<AVDictionaryMut<'_>> {
        // SAFETY: the pointer value is copied through a raw projection from the
        // live exclusive header; no reference to C storage is formed.
        let metadata = unsafe { addr_of!((*self.as_mut_ptr()).metadata).read() };
        // SAFETY: by this type's invariant a non-null `metadata` is one live
        // dictionary owned by the entry; the exclusive side-data handle is the
        // only path to it for the returned lifetime.
        unsafe { AVDictionaryMut::from_ptr(metadata) }
    }
}

#[cfg(test)]
mod side_data_type_tests {
    use core::mem::{align_of, size_of};
    use core::ptr::addr_of_mut;

    use ffibox::CBox;

    use super::*;
    use crate::buffer::AVBufferReference;

    #[test]
    fn side_data_borrows_owners_and_gates_mutable_bytes() {
        // SAFETY: the returned fully constructed buffer reference is adopted
        // immediately by its matching reference-header lifecycle.
        let backing = unsafe {
            CBox::<AVBufferReference>::from_raw(ffi::av_buffer_allocz(4))
                .expect("four-byte side-data buffer")
        };
        // `AVBufferReferenceRef::data` views the window as `MaybeUninit<u8>`
        // because C leaves an `av_buffer_alloc` window unwritten; `allocz`
        // above did write this one, and the side-data header wants the raw
        // byte address either way.
        let data = backing.as_ref().data().unwrap().as_elem_ptr().cast::<u8>();
        let mut raw = ffi::AVFrameSideData {
            type_: ffi::AVFrameSideDataType_AV_FRAME_DATA_A53_CC,
            data,
            size: 4,
            metadata: core::ptr::null_mut(),
            buf: backing.as_ptr(),
        };

        // SAFETY: `raw` and `backing` remain live and this exclusive handle is
        // the only access path to the side-data header for the scope.
        let mut side_data = unsafe { AVFrameSideDataMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert_eq!(side_data.as_ref().kind(), AVFrameSideDataType::A53_CC);
        assert_eq!(side_data.as_ref().size(), 4);
        assert_eq!(
            side_data.as_ref().buffer().unwrap().as_ptr(),
            backing.as_ptr().cast_const()
        );
        assert!(side_data.as_ref().metadata().is_none());
        assert_eq!(side_data.as_ref().data().unwrap().len(), 4);
        // `av_buffer_allocz` wrote this window, unlike the `av_buffer_alloc`
        // one every new entry gets, so the initialized view is dischargeable.
        // SAFETY: `allocz` memset all four bytes and nothing has replaced them.
        let window = unsafe { side_data.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.elems().sum::<u8>(), 0);

        assert!(side_data.write_all(&[1, 2, 3, 4]));
        // SAFETY: `write_all` covered the whole window.
        let window = unsafe { side_data.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.elem(2), Some(3));
        // A read-modify-write goes through the same discharged obligation.
        {
            // SAFETY: every byte of the window was written above.
            let mut window = unsafe { side_data.data_assume_init_mut() }.unwrap();
            assert!(window.set_elem(0, 9));
        }
        // SAFETY: still fully written; one byte changed value.
        let window = unsafe { side_data.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.elem(0), Some(9));
        assert_eq!(
            size_of::<AVFrameSideData>(),
            size_of::<ffi::AVFrameSideData>()
        );
        assert_eq!(
            align_of::<AVFrameSideData>(),
            align_of::<ffi::AVFrameSideData>()
        );
    }

    #[test]
    fn a_new_entry_owns_an_unwritten_window() {
        let mut frame = AVFrame::new().expect("frame header");
        let mut frame_mut = frame.as_mut();
        let mut entry =
            av_frame_new_side_data(&mut frame_mut, AVFrameSideDataType::SEI_UNREGISTERED, 4)
                .expect("four-byte side-data entry");

        // `av_frame_new_side_data` sizes the window with `av_buffer_alloc`,
        // which is raw `av_malloc` storage: `data` may only hand out
        // `MaybeUninit` bytes here, and calling `data_assume_init` before the
        // write below would be a read of uninitialised memory from safe code.
        assert_eq!(entry.as_ref().size(), 4);
        assert_eq!(entry.as_ref().data().unwrap().len(), 4);
        assert!(entry.as_ref().is_writable());
        // A partial write is refused rather than leaving the window ragged.
        assert!(!entry.write_all(&[7, 7]));
        assert!(entry.write_all(&[7, 7, 7, 7]));
        // SAFETY: `write_all` covered every byte of the window.
        let window = unsafe { entry.as_ref().data_assume_init() }.unwrap();
        assert_eq!(window.elems().map(u32::from).sum::<u32>(), 28);

        // A second reference to the same underlying buffer closes the exclusive
        // window: the entry no longer uniquely owns the bytes it points at.
        let held = entry.as_ref().owned_buffer().expect("entry buffer");
        assert!(!entry.as_ref().is_writable());
        assert!(entry.data_mut().is_none());
        assert!(!entry.write_all(&[1, 2, 3, 4]));
        drop(held);
        assert!(entry.as_ref().is_writable());
    }
}

define_ctype!(
    /// Wraps: AVFrame
    ///
    /// ABI-compatible frame storage. Owned frames use `CBox<AVFrame>`;
    /// borrowed access goes through pointer-carrying handles and never forms a
    /// Rust reference to memory libavutil may mutate.
    ///
    /// # Invariant
    ///
    /// `AVFrameRef::from_ptr` and `AVFrameMut::from_ptr` promise only that the
    /// header itself is live and initialized. Every wrapped frame additionally
    /// satisfies, and every unsafe constructor of one owes:
    ///
    /// - **Owners.** `buf[i]` is null, or one live [`AVBufferReference`] owned
    ///   by this frame and satisfying that type's own invariant, with the
    ///   non-null slots contiguous. `extended_buf` is null with
    ///   `nb_extended_buf == 0`, or a frame-owned table of `nb_extended_buf`
    ///   such references. `side_data` is null with `nb_side_data == 0`, or a
    ///   frame-owned table of `nb_side_data` non-null entries, each satisfying
    ///   [`AVFrameSideData`]'s invariant. `metadata` is null or one frame-owned
    ///   dictionary, `opaque_ref` null or one frame-owned reference header, and
    ///   `ch_layout` a layout owned by this frame satisfying
    ///   [`AVChannelLayout`]'s invariant.
    /// - **Planes and geometry.** `data[i]` is null, or a plane address inside
    ///   one of those buffers — inside caller-managed storage when `buf[0]` is
    ///   null — and `extended_data` is null, `data` itself, or a frame-owned
    ///   table of `ch_layout.nb_channels` such addresses. Every non-null plane
    ///   is valid for the extent the geometry describes: `format`, `width`,
    ///   `height` and `linesize` for video, `format`, `nb_samples`, `ch_layout`
    ///   and `linesize[0]` for audio. This half is a *relation*, not a
    ///   per-field property. `av_frame_copy` (`frame.c:711`) checks only that
    ///   the destination is no smaller than the source, then copies
    ///   `src->width` bytes per row for `src->height` rows through `data`
    ///   (`frame_copy_video`, `frame.c:684`), so a geometry that outruns its
    ///   allocation is a heap overflow with no `unsafe` in the caller. That is
    ///   why [`set_width`](AVFrameMut::set_width) and its siblings refuse a
    ///   frame that already holds planes, and why
    ///   [`set_line_size`](AVFrameMut::set_line_size) is `unsafe`.
    /// - **Hardware frames.** `hw_frames_ctx` is null, or one frame-owned
    ///   reference whose buffer holds the initialized hardware frames context
    ///   these planes were allocated from. `av_frame_copy` routes either frame
    ///   carrying one to `av_hwframe_transfer_data`, which loads
    ///   `hw_frames_ctx->data->hw_type->transfer_data_from`
    ///   (`hwcontext.c:490`) and hands it the planes, so both halves — that the
    ///   buffer is a frames context at all, and that it describes these
    ///   planes — are load-bearing. Neither is expressible in a signature, so
    ///   [`replace_hardware_frames_context`](AVFrameMut::replace_hardware_frames_context)
    ///   is `unsafe` and takes the typed context.
    /// - **Internal and application slots.** `private_ref` is null or an
    ///   internal RefStruct reference belonging to one libav\* library, which
    ///   this crate only tests for presence; `opaque` carries identity only and
    ///   libavutil never dereferences it.
    ///
    /// The producers discharge it. `av_frame_alloc` zeroes the header and
    /// writes the documented defaults (`get_frame_defaults`, `frame.c:31`),
    /// leaving every owner null and every geometry field describing nothing;
    /// `av_frame_get_buffer` installs planes and buffers computed from the
    /// geometry it reads; `av_frame_ref` and `av_frame_clone` re-reference or
    /// copy a frame that already satisfies it; `av_frame_unref` restores the
    /// unallocated state; and `av_hwframe_get_buffer` (`hwcontext.c:506`) is
    /// what installs `hw_frames_ctx`, together with the surfaces it describes.
    ///
    /// The safe writers in this crate preserve it: the geometry setters refuse
    /// a frame that holds planes, each `replace_*` exchanges one owner for
    /// another of the same kind, `side_data_mut` and `metadata_mut` reach the
    /// dependent types' own preserving surfaces, and the wrapped `av_frame_*`
    /// entry points leave a frame libavutil considers valid on both their
    /// success and failure paths.
    ///
    /// [`AVBufferReference`]: crate::buffer::AVBufferReference
    /// [`AVChannelLayout`]: crate::channel_layout::AVChannelLayout
    AVFrame,
    AVFrameRef,
    AVFrameMut,
    ffi::AVFrame
);

// SAFETY: av_frame_free (frame.c:63) unrefs every owner the type invariant
// names and then av_freeps the header, so the obligation this impl adds to the
// trait's "fully constructed" is twofold: the frame satisfies that invariant,
// and its header is one independent av_malloc-family allocation — what
// av_frame_alloc returns. A frame embedded in other storage, or an
// `AVFrame::zeroed()` on the stack, is fully constructed and still not a valid
// payload for this destructor; only the unsafe `CBox::from_raw` seam can offer
// one, and `av_frame_alloc` is this crate's only caller of it.
unsafe impl ffibox::CDropped for AVFrame {
    unsafe fn c_drop(obj: NonNull<Self>) {
        let mut frame = obj.as_ptr().cast::<ffi::AVFrame>();
        // SAFETY: the trait contract transfers one live owner to its matching
        // public destructor; the local pointer slot is writable.
        unsafe { ffi::av_frame_free(&raw mut frame) }
    }
}

// SAFETY: av_frame_clone is av_frame_alloc + av_frame_ref (frame.c:483). It
// leaves the source frame's own header and slots in place: for a refcounted
// source it takes an independent reference to each buffer, and for a source
// with no buf[0] it allocates and copies, reading the planes through exactly
// the geometry relation the type invariant pins. Either way the result owes one
// independent av_frame_free.
//
// The failure return is NULL, and `Clone` turns that into an abort. Failure is
// routine rather than only an allocation error — cloning a frame that holds no
// data reaches av_frame_get_buffer with format -1 and comes back EINVAL — so
// callers want `try_clone`.
unsafe impl ffibox::CCloned for AVFrame {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: the trait contract supplies a live initialized source.
        NonNull::new(unsafe { ffi::av_frame_clone(obj.as_ptr().cast()) }.cast())
    }
}

impl AVFrame {
    /// Allocates a frame initialized to libavutil's documented defaults.
    pub fn new() -> Option<ffibox::CBox<Self>> {
        av_frame_alloc()
    }
}

/// Identity-only address of a frame data plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVFramePlane<'a> {
    pointer: NonNull<u8>,
    _lifetime: core::marker::PhantomData<&'a ()>,
}

impl AVFramePlane<'_> {
    /// Returns the address. Dereferencing remains unsafe because the plane's
    /// extent depends on the media type, format, dimensions and signed stride.
    #[must_use]
    pub const fn as_non_null(self) -> NonNull<u8> {
        self.pointer
    }
}

/// Lifetime-bound identity for an application-managed frame cookie.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AVFrameOpaque<'a> {
    pointer: NonNull<core::ffi::c_void>,
    _lifetime: core::marker::PhantomData<&'a ()>,
}

impl AVFrameOpaque<'_> {
    #[must_use]
    pub const fn as_non_null(self) -> NonNull<core::ffi::c_void> {
        self.pointer
    }
}

macro_rules! frame_scalar {
    ($(#[$meta:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the scalar is copied from a live frame through a raw
                // projection, without forming a reference to C storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
        impl AVFrameMut<'_> {
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle permits replacing this scalar.
                unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
            }
        }
    };
}

macro_rules! frame_geometry_scalar {
    ($(#[$meta:meta])* $get:ident, $set:ident, $field:ident, $ty:ty) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the scalar is copied from a live frame through a raw
                // projection, without forming a reference to C storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
        impl AVFrameMut<'_> {
            $(#[$meta])*
            ///
            /// Refuses, returning `false` and writing nothing, unless the frame
            /// [holds no planes](AVFrameRef::is_unallocated). This field is one
            /// half of the geometry relation in [`AVFrame`]'s invariant: while
            /// the frame describes no data it constrains nothing, and
            /// `av_frame_get_buffer` re-establishes the relation from whatever
            /// it then reads. Changing it under installed planes would leave
            /// libavutil copying an extent the allocation does not cover.
            /// `av_frame_unref` is how a frame becomes writable again.
            pub fn $set(&mut self, value: $ty) -> bool {
                if !self.as_ref().is_unallocated() {
                    return false;
                }
                // SAFETY: the exclusive handle permits replacing this scalar,
                // and the check above proves no plane or buffer currently
                // depends on its value.
                unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).$field).write(value) }
                true
            }
        }
    };
}

macro_rules! frame_scalar_readonly {
    ($(#[$meta:meta])* $get:ident, $field:ident, $ty:ty) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the scalar is copied from a live frame through a raw
                // projection, without forming a reference to C storage.
                unsafe { addr_of!((*self.as_ptr()).$field).read() }
            }
        }
    };
}

macro_rules! frame_enum {
    ($(#[$meta:meta])* $get:ident, $set:ident, $field:ident, $ty:path) => {
        impl AVFrameRef<'_> {
            $(#[$meta])*
            #[must_use]
            pub fn $get(&self) -> $ty {
                // SAFETY: the integer-backed ABI value is copied through a raw projection.
                <$ty>::from_raw(unsafe { addr_of!((*self.as_ptr()).$field).read() })
            }
        }
        impl AVFrameMut<'_> {
            pub fn $set(&mut self, value: $ty) {
                // SAFETY: the exclusive handle permits storing the ABI value.
                unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).$field).write(value.as_raw()) }
            }
        }
    };
}

impl<'a> AVFrameRef<'a> {
    /// Wraps: AVFrame.buf
    #[must_use]
    pub fn buffer(&self, index: usize) -> Option<AVBufferReferenceRef<'a>> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        let buffers = unsafe { addr_of!((*self.as_ptr()).buf).read() };
        // SAFETY: by this type's invariant a non-null slot is one live
        // reference header owned by the frame, satisfying AVBufferReference's
        // own invariant and kept alive for `'a` by the frame.
        unsafe { AVBufferReferenceRef::from_ptr(buffers[index]) }
    }

    /// Clones an owned reference header while leaving the frame slot valid.
    #[must_use]
    pub fn owned_buffer(
        &self,
        index: usize,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.buffer(index)?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrame.data
    ///
    /// Whether the frame currently describes no picture or sample data: every
    /// `data` and `buf` slot is null, it owns no extended table, and
    /// `extended_data` is either unset or the inline `data` array itself.
    ///
    /// This is the state `av_frame_alloc` produces and `av_frame_unref`
    /// restores, and it is the one in which the geometry fields constrain
    /// nothing, so the setters that write them are gated on it. It is also what
    /// C's own `av_frame_get_buffer` warning asks for — "if frame already has
    /// been allocated, calling this function will leak memory" — since the call
    /// overwrites `data`, `linesize` and `buf` without releasing what was there.
    #[must_use]
    pub fn is_unallocated(&self) -> bool {
        let frame = self.as_ptr();
        // SAFETY: the inline arrays and scalars are copied through raw
        // projections from the live frame, and `inline` is only the address of
        // the `data` array, so no reference to C storage is formed.
        let (planes, buffers, extended_data, extended_buf, nb_extended_buf, inline) = unsafe {
            (
                addr_of!((*frame).data).read(),
                addr_of!((*frame).buf).read(),
                addr_of!((*frame).extended_data).read(),
                addr_of!((*frame).extended_buf).read(),
                addr_of!((*frame).nb_extended_buf).read(),
                addr_of!((*frame).data).cast::<*mut u8>(),
            )
        };
        planes.iter().all(|plane| plane.is_null())
            && buffers.iter().all(|buffer| buffer.is_null())
            && (extended_data.is_null() || extended_data.cast_const() == inline)
            && extended_buf.is_null()
            && nb_extended_buf == 0
    }

    /// Wraps: AVFrame.data
    ///
    /// Returns only plane identity because no format-independent byte extent exists.
    #[must_use]
    pub fn data_plane(&self, index: usize) -> Option<AVFramePlane<'a>> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        let data = unsafe { addr_of!((*self.as_ptr()).data).read() };
        NonNull::new(data[index]).map(|pointer| AVFramePlane {
            pointer,
            _lifetime: core::marker::PhantomData,
        })
    }

    /// Wraps: AVFrame.side_data
    #[must_use]
    pub fn side_data(&self, index: usize) -> Option<AVFrameSideDataRef<'a>> {
        let len = usize::try_from(self.nb_side_data()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: by this type's invariant a non-null `side_data` addresses
        // `nb_side_data` entries, and `index` is checked against that count.
        let entry = unsafe {
            addr_of!((*self.as_ptr()).side_data)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: the same invariant makes every entry in that table a live
        // header satisfying AVFrameSideData's own invariant, owned by the
        // frame and so alive for `'a`.
        unsafe { AVFrameSideDataRef::from_ptr(entry) }
    }

    /// Wraps: AVFrame.opaque
    #[must_use]
    pub fn opaque(&self) -> Option<AVFrameOpaque<'a>> {
        // SAFETY: only the identity is copied; libavutil never dereferences it.
        NonNull::new(unsafe { addr_of!((*self.as_ptr()).opaque).read() }).map(|pointer| {
            AVFrameOpaque {
                pointer,
                _lifetime: core::marker::PhantomData,
            }
        })
    }

    /// Wraps: AVFrame.metadata
    #[must_use]
    pub fn metadata(&self) -> Option<AVDictionaryRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).metadata).read() };
        // SAFETY: a non-null dictionary remains live for the frame borrow.
        unsafe { AVDictionaryRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.ch_layout
    #[must_use]
    pub fn channel_layout(&self) -> crate::channel_layout::AVChannelLayoutRef<'a> {
        // SAFETY: the initialized layout is embedded in the live frame.
        unsafe {
            crate::channel_layout::AVChannelLayoutRef::from_ptr(
                addr_of!((*self.as_ptr()).ch_layout).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.linesize
    #[must_use]
    pub fn line_size(&self, index: usize) -> Option<i32> {
        if index >= 8 {
            return None;
        }
        // SAFETY: the initialized inline array is copied through a raw projection.
        Some(unsafe { addr_of!((*self.as_ptr()).linesize).read() }[index])
    }

    /// Wraps: AVFrame.private_ref
    ///
    /// Public callers may only test presence of this internal RefStruct reference.
    #[must_use]
    pub fn has_private_reference(&self) -> bool {
        // SAFETY: only the address is copied; it is neither exposed nor dereferenced.
        !unsafe { addr_of!((*self.as_ptr()).private_ref).read() }.is_null()
    }

    /// Wraps: AVFrame.opaque_ref
    #[must_use]
    pub fn opaque_reference(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).opaque_ref).read() };
        // SAFETY: a non-null header remains live for the frame borrow.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.opaque_ref
    ///
    /// Clones an owned reference to the frame owner's reference-counted
    /// private data, leaving the frame's own slot valid. Libavutil propagates
    /// this field but never inspects the bytes behind it.
    #[must_use]
    pub fn owned_opaque_reference(&self) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.opaque_reference()?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrame.hw_frames_ctx
    ///
    /// The reference header itself. The frames context behind it comes out
    /// typed through
    /// [`owned_hardware_frames_context`](Self::owned_hardware_frames_context).
    #[must_use]
    pub fn hardware_frames_context(&self) -> Option<AVBufferReferenceRef<'a>> {
        // SAFETY: the pointer is copied from the live owning frame.
        let pointer = unsafe { addr_of!((*self.as_ptr()).hw_frames_ctx).read() };
        // SAFETY: a non-null header remains live for the frame borrow.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Wraps: AVFrame.hw_frames_ctx
    ///
    /// Clones an owned, typed reference to the hardware frames context these
    /// planes were allocated from, leaving the frame's own slot valid. This is
    /// the getter that pairs with
    /// [`replace_hardware_frames_context`](AVFrameMut::replace_hardware_frames_context):
    /// the context can be moved into another frame that holds surfaces from it,
    /// which is the only use the setter's contract admits.
    #[must_use]
    pub fn owned_hardware_frames_context(&self) -> Option<crate::hwcontext::HWFramesContext> {
        let source = self.hardware_frames_context()?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        let cloned =
            unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }?;
        // SAFETY: by this type's invariant a non-null `hw_frames_ctx` addresses
        // an initialized hardware frames context, which is what the typed
        // wrapper claims; the clone above holds a count of its own on it.
        Some(unsafe { crate::hwcontext::HWFramesContext::from_reference(cloned) })
    }

    /// Wraps: AVFrame.extended_buf
    #[must_use]
    pub fn extended_buffer(&self, index: usize) -> Option<AVBufferReferenceRef<'a>> {
        let len = usize::try_from(self.nb_extended_buffers()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: by this type's invariant a non-null `extended_buf` addresses
        // `nb_extended_buf` entries, and `index` is checked against that count.
        let pointer = unsafe {
            addr_of!((*self.as_ptr()).extended_buf)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: the same invariant makes it one live reference header owned
        // by the frame and alive for `'a`.
        unsafe { AVBufferReferenceRef::from_ptr(pointer) }
    }

    /// Clones an owned extra reference header while leaving the frame valid.
    #[must_use]
    pub fn owned_extended_buffer(
        &self,
        index: usize,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let source = self.extended_buffer(index)?;
        // SAFETY: the borrowed source header is live for this call; a non-null
        // result is a new independently releasable reference header.
        unsafe { ffibox::CBox::from_raw(ffi::av_buffer_ref(source.as_ptr().cast_mut())) }
    }

    /// Wraps: AVFrame.time_base
    #[must_use]
    pub fn time_base(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the initialized rational is embedded in the live frame.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).time_base).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.sample_aspect_ratio
    #[must_use]
    pub fn sample_aspect_ratio(&self) -> crate::rational::AVRationalRef<'a> {
        // SAFETY: the initialized rational is embedded in the live frame.
        unsafe {
            crate::rational::AVRationalRef::from_ptr(
                addr_of!((*self.as_ptr()).sample_aspect_ratio).cast_mut(),
            )
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.extended_data
    #[must_use]
    pub fn extended_data_plane(&self, index: usize) -> Option<AVFramePlane<'a>> {
        let frame = self.as_ptr();
        // SAFETY: the table address is copied and the inline address projected
        // from the same live frame without forming references.
        let (table, inline) = unsafe {
            (
                addr_of!((*frame).extended_data).read(),
                addr_of!((*frame).data).cast::<*mut u8>(),
            )
        };
        // By this type's invariant `extended_data` is either the inline array
        // — eight slots, the video and packed-audio case — or a frame-owned
        // table of `ch_layout.nb_channels` plane pointers, which is what
        // get_audio_buffer (`frame.c:161`) allocates and av_frame_ref
        // (`frame.c:358`) memdups for planar audio with more than eight
        // channels.
        let len = if table.cast_const() == inline {
            8
        } else {
            usize::try_from(self.channel_layout().nb_channels()).ok()?
        };
        if table.is_null() || index >= len {
            return None;
        }
        // SAFETY: the invariant gives the table `len` entries and `index` is
        // checked against that count.
        NonNull::new(unsafe { table.add(index).read() }).map(|pointer| AVFramePlane {
            pointer,
            _lifetime: core::marker::PhantomData,
        })
    }
}

impl AVFrameMut<'_> {
    /// Replaces one owned buffer header and returns the prior owner.
    ///
    /// # Safety
    /// The resulting fixed table must remain contiguous, and every stored data
    /// plane must remain inside one of the frame's remaining buffer owners.
    pub unsafe fn replace_buffer(
        &mut self,
        index: usize,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Result<
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    > {
        if index >= 8 {
            return Err(replacement);
        }
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: index is checked; the caller guarantees the relational frame
        // invariants, and this handle exclusively owns the pointer slot.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).buf)
                .cast::<*mut ffi::AVBufferRef>()
                .add(index)
                .replace(replacement)
        };
        // SAFETY: a non-null previous slot was one owned reference header, now
        // removed from the frame and transferred to the returned owner.
        Ok(unsafe { ffibox::CBox::from_raw(previous) })
    }

    pub fn side_data_mut(&mut self, index: usize) -> Option<AVFrameSideDataMut<'_>> {
        let len = usize::try_from(self.as_ref().nb_side_data()).ok()?;
        if index >= len {
            return None;
        }
        // SAFETY: by this type's invariant a non-null `side_data` addresses
        // `nb_side_data` entries, and `index` is checked against that count.
        let entry = unsafe {
            addr_of!((*self.as_mut_ptr()).side_data)
                .read()
                .add(index)
                .read()
        };
        // SAFETY: the mutable frame reborrow prevents competing access.
        unsafe { AVFrameSideDataMut::from_ptr(entry) }
    }

    pub fn metadata_mut(&mut self) -> Option<AVDictionaryMut<'_>> {
        // SAFETY: the pointer is copied through the exclusive frame handle.
        let pointer = unsafe { addr_of!((*self.as_mut_ptr()).metadata).read() };
        // SAFETY: the mutable frame reborrow prevents competing access.
        unsafe { AVDictionaryMut::from_ptr(pointer) }
    }

    /// Replaces the optional owned dictionary and returns the prior owner.
    pub fn replace_metadata(
        &mut self,
        replacement: Option<ffibox::CBox<crate::dict::AVDictionary>>,
    ) -> Option<ffibox::CBox<crate::dict::AVDictionary>> {
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot; replacing it
        // transfers the prior nullable dictionary out without freeing it.
        let previous =
            unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).metadata).replace(replacement) };
        // SAFETY: a non-null prior value was one frame-owned dictionary and is
        // no longer reachable from the frame.
        unsafe { ffibox::CBox::from_raw(previous) }
    }

    /// Replaces the optional reference-counted opaque owner.
    pub fn replace_opaque_reference(
        &mut self,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Option<ffibox::CBox<crate::buffer::AVBufferReference>> {
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: the exclusive handle owns this pointer slot.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque_ref).replace(replacement)
        };
        // SAFETY: a non-null prior value was one frame-owned reference header.
        unsafe { ffibox::CBox::from_raw(previous) }
    }

    /// Wraps: AVFrame.hw_frames_ctx
    ///
    /// Replaces the optional hardware frames context and returns the prior
    /// owner. Normally libavutil installs this field itself, in
    /// `av_hwframe_get_buffer` (`hwcontext.c:506`), together with the surfaces
    /// it describes; this operation exists to move that pairing between frames.
    ///
    /// # Safety
    ///
    /// After the call the frame's planes must be surfaces belonging to
    /// `replacement`, or `replacement` must be `None` and the frame must hold
    /// no hardware surfaces. Neither direction is checkable here, and both are
    /// load-bearing: `av_frame_copy` sends any frame carrying a context to
    /// `av_hwframe_transfer_data`, which loads
    /// `hw_frames_ctx->data->hw_type->transfer_data_from` (`hwcontext.c:490`)
    /// and hands it these planes, while clearing the field on a frame that does
    /// hold surfaces makes the same copy read opaque surface handles as
    /// addressable rows. The typed argument carries only the first half of the
    /// [invariant](AVFrame#invariant) — that the buffer is an initialized
    /// frames context at all.
    pub unsafe fn replace_hardware_frames_context(
        &mut self,
        replacement: Option<crate::hwcontext::HWFramesContext>,
    ) -> Option<crate::hwcontext::HWFramesContext> {
        let replacement = replacement.map_or(core::ptr::null_mut(), |context| {
            context.into_reference().into_raw()
        });
        // SAFETY: the exclusive handle owns this pointer slot, and the caller
        // guarantees the replacement describes the frame's planes.
        let previous = unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).hw_frames_ctx).replace(replacement)
        };
        // SAFETY: a non-null prior value was one frame-owned reference header
        // over an initialized frames context, now removed from the frame.
        let previous = unsafe { ffibox::CBox::from_raw(previous) }?;
        // SAFETY: as above, the removed header is the frame's own initialized
        // hardware frames context, which is what the typed wrapper claims.
        Some(unsafe { crate::hwcontext::HWFramesContext::from_reference(previous) })
    }

    /// Replaces one extra owned buffer header and returns the prior owner.
    ///
    /// # Safety
    /// The replacement and all data-plane addresses must preserve the frame's
    /// buffer-containment invariants for the full frame lifetime.
    pub unsafe fn replace_extended_buffer(
        &mut self,
        index: usize,
        replacement: Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    ) -> Result<
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
        Option<ffibox::CBox<crate::buffer::AVBufferReference>>,
    > {
        let len = usize::try_from(self.as_ref().nb_extended_buffers()).unwrap_or(0);
        if index >= len {
            return Err(replacement);
        }
        let replacement = replacement.map_or(core::ptr::null_mut(), ffibox::CBox::into_raw);
        // SAFETY: index is checked against the live table; the caller preserves
        // the relational invariants and this frame handle is exclusive.
        let previous = unsafe {
            core::ptr::addr_of!((*self.as_mut_ptr()).extended_buf)
                .read()
                .add(index)
                .replace(replacement)
        };
        // SAFETY: a non-null previous entry was owned by the frame and has now
        // been transferred out of its table slot.
        Ok(unsafe { ffibox::CBox::from_raw(previous) })
    }

    pub fn channel_layout_mut(&mut self) -> crate::channel_layout::AVChannelLayoutMut<'_> {
        // SAFETY: the embedded layout is live and exclusively reborrowed.
        unsafe {
            crate::channel_layout::AVChannelLayoutMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).ch_layout
            ))
            .expect("embedded field is non-null")
        }
    }

    /// Wraps: AVFrame.linesize
    ///
    /// Overrides one plane's stride before the frame is allocated, which is the
    /// only phase in which libavutil reads a caller-supplied one: with
    /// `linesize[0]` already non-zero, `get_video_buffer` (`frame.c:91`) skips
    /// its own computation and sizes the allocation from what it finds.
    /// Returns `false` without writing for an out-of-range `index`, or for a
    /// frame that [already holds planes](AVFrameRef::is_unallocated).
    ///
    /// # Safety
    ///
    /// The stride must be at least the format's byte width for the geometry the
    /// frame carries when it is allocated — `av_image_get_linesize(format,
    /// width, index)` for video, `nb_samples * block_align` for audio. C never
    /// checks that relation: `av_image_fill_plane_sizes` derives the allocation
    /// from the stride, while `av_image_copy2` derives each row's byte width
    /// from `width`, so a stride below it leaves `av_frame_copy` writing past
    /// the buffer C sized for it. The obligation cannot be discharged in this
    /// setter because the geometry fields it depends on may be written after
    /// it, in any order, while the frame stays unallocated.
    pub unsafe fn set_line_size(&mut self, index: usize, value: i32) -> bool {
        if index >= 8 || !self.as_ref().is_unallocated() {
            return false;
        }
        // SAFETY: index is in bounds, the exclusive handle permits the write,
        // no installed plane currently depends on the stride, and the caller
        // guarantees the value covers the geometry the frame is allocated with.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).linesize)
                .cast::<i32>()
                .add(index)
                .write(value)
        }
        true
    }

    pub fn clear_opaque(&mut self) {
        // SAFETY: the pointee is application-owned and the exclusive handle permits clearing it.
        unsafe { core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque).write(core::ptr::null_mut()) }
    }

    /// Stores an application-managed cookie address.
    ///
    /// # Safety
    /// The pointee must outlive this frame and every clone retaining the address.
    pub unsafe fn set_opaque(&mut self, value: Option<NonNull<core::ffi::c_void>>) {
        // SAFETY: the caller provides the external lifetime; this handle is exclusive.
        unsafe {
            core::ptr::addr_of_mut!((*self.as_mut_ptr()).opaque)
                .write(value.map_or(core::ptr::null_mut(), NonNull::as_ptr))
        }
    }

    pub fn time_base_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the embedded rational is live and exclusively reborrowed.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).time_base
            ))
            .expect("embedded field is non-null")
        }
    }

    pub fn sample_aspect_ratio_mut(&mut self) -> crate::rational::AVRationalMut<'_> {
        // SAFETY: the embedded rational is live and exclusively reborrowed.
        unsafe {
            crate::rational::AVRationalMut::from_ptr(core::ptr::addr_of_mut!(
                (*self.as_mut_ptr()).sample_aspect_ratio
            ))
            .expect("embedded field is non-null")
        }
    }
}

frame_geometry_scalar!(/// Wraps: AVFrame.height
    height, set_height, height, i32);
frame_geometry_scalar!(/// Wraps: AVFrame.format
    format, set_format, format, i32);
frame_geometry_scalar!(/// Wraps: AVFrame.nb_samples
    sample_count, set_sample_count, nb_samples, i32);
frame_scalar!(/// Wraps: AVFrame.flags
    flags, set_flags, flags, i32);
frame_geometry_scalar!(/// Wraps: AVFrame.width
    width, set_width, width, i32);
frame_enum!(/// Wraps: AVFrame.pict_type
    picture_type, set_picture_type, pict_type, crate::avutil::AVPictureType);
frame_scalar!(/// Wraps: AVFrame.duration
    duration, set_duration, duration, i64);
frame_scalar_readonly!(/// Wraps: AVFrame.nb_extended_buf
    nb_extended_buffers, nb_extended_buf, i32);
frame_enum!(/// Wraps: AVFrame.alpha_mode
    alpha_mode, set_alpha_mode, alpha_mode, crate::pixfmt::AVAlphaMode);
frame_scalar!(/// Wraps: AVFrame.crop_right
    crop_right, set_crop_right, crop_right, usize);
frame_scalar!(/// Wraps: AVFrame.crop_left
    crop_left, set_crop_left, crop_left, usize);
frame_scalar!(/// Wraps: AVFrame.crop_bottom
    crop_bottom, set_crop_bottom, crop_bottom, usize);
frame_scalar!(/// Wraps: AVFrame.crop_top
    crop_top, set_crop_top, crop_top, usize);
frame_scalar!(/// Wraps: AVFrame.decode_error_flags
    decode_error_flags, set_decode_error_flags, decode_error_flags, i32);
frame_scalar!(/// Wraps: AVFrame.best_effort_timestamp
    best_effort_timestamp, set_best_effort_timestamp, best_effort_timestamp, i64);
frame_enum!(/// Wraps: AVFrame.chroma_location
    chroma_location, set_chroma_location, chroma_location, crate::pixfmt::AVChromaLocation);
frame_enum!(/// Wraps: AVFrame.colorspace
    color_space, set_color_space, colorspace, crate::pixfmt::AVColorSpace);
frame_enum!(/// Wraps: AVFrame.color_trc
    color_transfer, set_color_transfer, color_trc, crate::pixfmt::AVColorTransferCharacteristic);
frame_enum!(/// Wraps: AVFrame.color_primaries
    color_primaries, set_color_primaries, color_primaries, crate::pixfmt::AVColorPrimaries);
frame_enum!(/// Wraps: AVFrame.color_range
    color_range, set_color_range, color_range, crate::pixfmt::AVColorRange);
frame_scalar_readonly!(/// Wraps: AVFrame.nb_side_data
    nb_side_data, nb_side_data, i32);
frame_scalar!(/// Wraps: AVFrame.sample_rate
    sample_rate, set_sample_rate, sample_rate, i32);
frame_scalar!(/// Wraps: AVFrame.repeat_pict
    repeat_picture, set_repeat_picture, repeat_pict, i32);
frame_scalar!(/// Wraps: AVFrame.quality
    quality, set_quality, quality, i32);
frame_scalar!(/// Wraps: AVFrame.pkt_dts
    packet_dts, set_packet_dts, pkt_dts, i64);
frame_scalar!(/// Wraps: AVFrame.pts
    pts, set_pts, pts, i64);

#[cfg(test)]
mod frame_type_tests {
    use core::mem::size_of;

    use super::*;
    use crate::pixfmt::{
        AVAlphaMode, AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic, AVPixelFormat,
    };

    /// A video frame whose planes libavutil allocated from its own geometry.
    fn allocated_video_frame() -> ffibox::CBox<AVFrame> {
        let mut frame = AVFrame::new().expect("frame allocation");
        assert!(frame.as_mut().set_width(64));
        assert!(frame.as_mut().set_height(32));
        assert!(frame.as_mut().set_format(AVPixelFormat::GRAY8.as_raw()));
        av_frame_get_buffer(&mut frame.as_mut(), 32).expect("frame buffer allocation");
        frame
    }

    #[test]
    fn owned_defaults_mutation_and_clone_use_typed_handles() {
        let mut frame = AVFrame::new().expect("frame allocation");
        assert_eq!(frame.as_ref().format(), -1);
        assert_eq!(frame.as_ref().time_base().den(), 1);
        assert!(frame.as_ref().data_plane(0).is_none());
        assert!(frame.as_ref().buffer(0).is_none());
        assert!(frame.as_ref().is_unallocated());
        assert!(frame.as_mut().set_width(16));
        assert!(frame.as_mut().set_height(16));
        assert!(frame.as_mut().set_format(AVPixelFormat::RGBA.as_raw()));
        // The frame is newly allocated and has valid video dimensions and
        // format, so it can adopt the buffers installed by the call.
        av_frame_get_buffer(&mut frame.as_mut(), 32).expect("frame buffer allocation");
        let opaque_owner = frame
            .as_ref()
            .owned_buffer(0)
            .expect("independent buffer reference");
        assert!(
            frame
                .as_mut()
                .replace_opaque_reference(Some(opaque_owner))
                .is_none()
        );
        assert!(frame.as_ref().opaque_reference().is_some());
        // The owning getter reaches the same underlying buffer the frame still
        // holds — a different reference *header*, since `av_buffer_ref`
        // allocates one per reference, over the one `AVBuffer` both count.
        let held = frame
            .as_ref()
            .owned_opaque_reference()
            .expect("independent opaque reference");
        let installed = frame.as_ref().opaque_reference().unwrap();
        assert_ne!(held.as_ptr().cast_const(), installed.as_ptr());
        assert_eq!(held.as_ref().buffer().as_ptr(), installed.buffer().as_ptr());
        drop(held);
        drop(frame.as_mut().replace_opaque_reference(None));
        frame.as_mut().set_pts(42);
        // SAFETY: refused for the out-of-range index before the stride
        // obligation could matter.
        assert!(!unsafe { frame.as_mut().set_line_size(8, 1) });
        frame.as_mut().time_base_mut().set_num(1);
        frame.as_mut().time_base_mut().set_den(25);
        let cloned = frame.try_clone().expect("frame clone");
        assert_eq!(
            (cloned.as_ref().width(), cloned.as_ref().height()),
            (16, 16)
        );
        assert!(
            cloned
                .as_ref()
                .line_size(0)
                .is_some_and(|stride| stride > 0)
        );
        assert_eq!(
            (
                cloned.as_ref().time_base().num(),
                cloned.as_ref().time_base().den()
            ),
            (1, 25)
        );
    }

    #[test]
    fn geometry_setters_refuse_a_frame_that_holds_planes() {
        let mut source = allocated_video_frame();
        let mut destination = allocated_video_frame();

        // Each of these once wrote through. `av_frame_copy` then checked only
        // that the destination was no smaller than the source and copied 64
        // rows out of a 32-row allocation: a heap-buffer-overflow READ of 32
        // bytes at `imgutils.c:353`, reached from `frame_copy_video`
        // (`frame.c:684`) with no `unsafe` anywhere in this test.
        assert!(!source.as_ref().is_unallocated());
        assert!(!source.as_mut().set_height(64));
        assert!(!destination.as_mut().set_height(64));
        assert!(!source.as_mut().set_width(4096));
        assert!(!source.as_mut().set_format(AVPixelFormat::RGBA.as_raw()));
        assert!(!source.as_mut().set_sample_count(1024));
        // SAFETY: refused before the write, so the stride obligation is not
        // reached; a stride of 4 under a 64-byte row is exactly what it exists
        // to forbid.
        assert!(!unsafe { source.as_mut().set_line_size(0, 4) });

        // Nothing moved, so the copy stays inside both allocations.
        assert_eq!(source.as_ref().height(), 32);
        assert_eq!(source.as_ref().line_size(0), Some(64));
        av_frame_copy(&mut destination.as_mut(), source.as_ref()).expect("copy data");
    }

    #[test]
    fn unreferencing_a_frame_makes_its_geometry_writable_again() {
        let mut frame = allocated_video_frame();
        assert!(!frame.as_mut().set_width(16));

        av_frame_unref(&mut frame.as_mut());
        assert!(frame.as_ref().is_unallocated());
        assert_eq!(frame.as_ref().format(), -1);

        // A stride below the row width is the caller's obligation, so ask for
        // one above it: 128 bytes for a 64-pixel GRAY8 row.
        assert!(frame.as_mut().set_width(64));
        assert!(frame.as_mut().set_height(32));
        assert!(frame.as_mut().set_format(AVPixelFormat::GRAY8.as_raw()));
        // SAFETY: 128 >= av_image_get_linesize(GRAY8, 64, 0) == 64, which is
        // the whole obligation, and the geometry above is already final.
        assert!(unsafe { frame.as_mut().set_line_size(0, 128) });

        // C adopts a non-zero linesize[0] verbatim (`frame.c:91`) and sizes the
        // allocation from it, so the override survives into the frame.
        av_frame_get_buffer(&mut frame.as_mut(), 32).expect("frame buffer allocation");
        assert_eq!(frame.as_ref().line_size(0), Some(128));
        assert!(!frame.as_ref().is_unallocated());
    }

    #[test]
    fn a_hardware_frames_context_only_arrives_typed() {
        let mut frame = allocated_video_frame();
        // A software frame carries none, and the typed getter says so rather
        // than handing out a bare reference header for a caller to interpret.
        assert!(frame.as_ref().hardware_frames_context().is_none());
        assert!(frame.as_ref().owned_hardware_frames_context().is_none());

        // This is the compile-time half of the guard: the argument and result
        // are `HWFramesContext`, so the plain `CBox<AVBufferReference>` that
        // used to be accepted here — an `av_buffer_allocz(64)`, say — no longer
        // type-checks. It was loaded as `hw_frames_ctx->data->hw_type` by
        // `av_hwframe_transfer_data` (`hwcontext.c:490`) on the next
        // `av_frame_copy`, an ASan heap-buffer-overflow read that a function
        // pointer call followed.
        let previous: Option<crate::hwcontext::HWFramesContext> =
            // SAFETY: the frame holds no hardware surfaces, so clearing an
            // already-null slot leaves nothing describing them.
            unsafe { frame.as_mut().replace_hardware_frames_context(None) };
        assert!(previous.is_none());
    }

    #[test]
    fn c_and_this_wrapper_index_the_same_frame_fields() {
        // A size assertion against `ffi::AVFrame` cannot fail — `AVFrame` is
        // `repr(transparent)` over it. So let C do the indexing instead, in
        // both directions: it allocates from geometry this crate wrote, and it
        // fills a clone field by field from a frame this crate filled.
        let mut frame = AVFrame::new().expect("frame allocation");
        assert!(frame.as_mut().set_width(64));
        assert!(frame.as_mut().set_height(32));
        assert!(frame.as_mut().set_format(AVPixelFormat::GRAY8.as_raw()));
        av_frame_get_buffer(&mut frame.as_mut(), 32).expect("frame buffer allocation");

        // C computed these three from the fields above: one byte per GRAY8
        // pixel, aligned to 32, is exactly 64.
        assert_eq!(frame.as_ref().line_size(0), Some(64));
        // The plane token borrows the frame, so keep only its address across
        // the mutations below; holding the token itself would — correctly —
        // refuse the `as_mut` that follows.
        let plane = frame.as_ref().data_plane(0).expect("first plane");
        assert!(frame.as_ref().buffer(0).is_some());
        // `extended_data` points at the inline `data` array for video.
        assert_eq!(frame.as_ref().extended_data_plane(0), Some(plane));
        let plane = plane.as_non_null();

        // One distinct value per scalar, so a getter reading its neighbour's
        // storage cannot coincide with the expected answer.
        {
            let mut handle = frame.as_mut();
            handle.set_pts(0x0102_0304_0506_0708);
            handle.set_packet_dts(-9_000_000_001);
            handle.set_duration(1_234_567);
            handle.set_best_effort_timestamp(77);
            handle.set_sample_rate(48_000);
            handle.set_quality(13);
            handle.set_repeat_picture(3);
            handle.set_flags(0b1010);
            handle.set_decode_error_flags(6);
            handle.set_crop_top(1);
            handle.set_crop_bottom(2);
            handle.set_crop_left(3);
            handle.set_crop_right(4);
            handle.set_picture_type(crate::avutil::AVPictureType::B);
            handle.set_color_range(AVColorRange::JPEG);
            handle.set_color_primaries(AVColorPrimaries::BT709);
            handle.set_color_transfer(AVColorTransferCharacteristic::SMPTE170M);
            handle.set_color_space(AVColorSpace::BT709);
            handle.set_chroma_location(AVChromaLocation::TOP_LEFT);
            handle.set_alpha_mode(AVAlphaMode::PREMULTIPLIED);
            handle.time_base_mut().set_num(1);
            handle.time_base_mut().set_den(90_000);
            handle.sample_aspect_ratio_mut().set_num(4);
            handle.sample_aspect_ratio_mut().set_den(3);
        }

        // `frame_copy_props` (`frame.c:220`) and `av_frame_ref` assign these
        // one by one at C's own offsets; reading them back at this crate's
        // offsets is the crossing.
        let clone = frame.try_clone().expect("frame clone");
        let view = clone.as_ref();
        assert_eq!(view.width(), 64);
        assert_eq!(view.height(), 32);
        assert_eq!(view.format(), AVPixelFormat::GRAY8.as_raw());
        assert_eq!(view.line_size(0), Some(64));
        assert_eq!(view.pts(), 0x0102_0304_0506_0708);
        assert_eq!(view.packet_dts(), -9_000_000_001);
        assert_eq!(view.duration(), 1_234_567);
        assert_eq!(view.best_effort_timestamp(), 77);
        assert_eq!(view.sample_rate(), 48_000);
        assert_eq!(view.quality(), 13);
        assert_eq!(view.repeat_picture(), 3);
        assert_eq!(view.flags(), 0b1010);
        assert_eq!(view.decode_error_flags(), 6);
        assert_eq!(
            (
                view.crop_top(),
                view.crop_bottom(),
                view.crop_left(),
                view.crop_right()
            ),
            (1, 2, 3, 4)
        );
        assert_eq!(view.picture_type(), crate::avutil::AVPictureType::B);
        assert_eq!(view.color_range(), AVColorRange::JPEG);
        assert_eq!(view.color_primaries(), AVColorPrimaries::BT709);
        assert_eq!(
            view.color_transfer(),
            AVColorTransferCharacteristic::SMPTE170M
        );
        assert_eq!(view.color_space(), AVColorSpace::BT709);
        assert_eq!(view.chroma_location(), AVChromaLocation::TOP_LEFT);
        assert_eq!(view.alpha_mode(), AVAlphaMode::PREMULTIPLIED);
        assert_eq!(
            (view.time_base().num(), view.time_base().den()),
            (1, 90_000)
        );
        assert_eq!(
            (
                view.sample_aspect_ratio().num(),
                view.sample_aspect_ratio().den()
            ),
            (4, 3)
        );
        // The clone holds its own reference to the same buffer, which is what
        // the plane pointer C memcpy'd into it points inside of.
        assert_eq!(
            view.data_plane(0).map(AVFramePlane::as_non_null),
            Some(plane)
        );
        assert!(!av_frame_is_writable(view));

        // Both handles stay one pointer wide, which is what lets them be
        // passed and stored like the C pointer they stand for.
        assert_eq!(
            size_of::<AVFrameRef<'_>>(),
            size_of::<*const ffi::AVFrame>()
        );
        assert_eq!(size_of::<AVFrameMut<'_>>(), size_of::<*mut ffi::AVFrame>());
    }
}

/// Wraps: av_frame_alloc
///
/// Allocates a frame initialized to libavutil's defaults and adopts its unique
/// header allocation.
#[must_use]
pub fn av_frame_alloc() -> Option<ffibox::CBox<AVFrame>> {
    // SAFETY: a non-null result is a fully initialized unique allocation
    // matched by AVFrame's destructor implementation.
    unsafe { ffibox::CBox::from_raw(ffi::av_frame_alloc()) }
}

/// Wraps: av_frame_clone
///
/// Creates an independently releasable frame header that shares or copies the
/// source frame's data according to libavutil's frame-reference rules.
#[must_use]
pub fn av_frame_clone(source: AVFrameRef<'_>) -> Option<ffibox::CBox<AVFrame>> {
    // SAFETY: `source` is a live shared frame borrow and is retained only by
    // creating independently owned references in the returned frame.
    unsafe { ffibox::CBox::from_raw(ffi::av_frame_clone(source.as_ptr())) }
}

fn frame_status(status: i32) -> Result<(), i32> {
    if status < 0 { Err(status) } else { Ok(()) }
}

/// Wraps: av_frame_copy
///
/// Copies frame data into buffers already allocated on `destination`.
pub fn av_frame_copy(destination: &mut AVFrameMut<'_>, source: AVFrameRef<'_>) -> Result<(), i32> {
    // SAFETY: the exclusive destination and shared source handles are live for
    // the call. C retains neither header and reports incompatible layouts.
    frame_status(unsafe { ffi::av_frame_copy(destination.as_mut_ptr(), source.as_ptr()) })
}

/// Wraps: av_frame_copy_props
///
/// Copies non-layout properties and installs independently owned metadata in
/// `destination`.
pub fn av_frame_copy_props(
    destination: &mut AVFrameMut<'_>,
    source: AVFrameRef<'_>,
) -> Result<(), i32> {
    // SAFETY: the destination is exclusively borrowed, the source is shared,
    // and every installed pointer receives its own ownership count or copy.
    frame_status(unsafe { ffi::av_frame_copy_props(destination.as_mut_ptr(), source.as_ptr()) })
}

/// Wraps: av_frame_free
///
/// Releases a nullable owned frame. Consuming the Rust owner prevents any
/// handle from surviving the release.
pub fn av_frame_free(frame: Option<ffibox::CBox<AVFrame>>) {
    drop(frame);
}

/// Wraps: av_frame_get_buffer
///
/// Allocates audio or video buffers from the properties already configured on
/// `frame`. An alignment of zero asks libavutil to choose its preferred value.
pub fn av_frame_get_buffer(frame: &mut AVFrameMut<'_>, alignment: i32) -> Result<(), i32> {
    // SAFETY: the exclusive handle supplies a live initialized frame. Any
    // allocations installed on success become owned by the frame lifecycle.
    frame_status(unsafe { ffi::av_frame_get_buffer(frame.as_mut_ptr(), alignment) })
}

/// Wraps: av_frame_get_side_data
///
/// Borrows the first side-data entry of `kind` from the owning frame.
#[must_use]
pub fn av_frame_get_side_data<'a>(
    frame: AVFrameRef<'a>,
    kind: AVFrameSideDataType,
) -> Option<AVFrameSideDataRef<'a>> {
    // SAFETY: the frame is live and shared for 'a; C returns null or an
    // interior entry owned by that frame and does not mutate it.
    let side_data = unsafe { ffi::av_frame_get_side_data(frame.as_ptr(), kind.as_raw()) };
    // SAFETY: a non-null entry remains live for the frame borrow.
    unsafe { AVFrameSideDataRef::from_ptr(side_data) }
}

/// Wraps: av_frame_is_writable
///
/// Reports whether every data buffer currently has a unique writable owner.
#[must_use]
pub fn av_frame_is_writable(frame: AVFrameRef<'_>) -> bool {
    // SAFETY: despite its legacy non-const signature, the implementation only
    // reads the live frame and its buffer reference counts and retains nothing.
    unsafe { ffi::av_frame_is_writable(frame.as_ptr().cast_mut()) != 0 }
}

/// Wraps: av_frame_make_writable
///
/// Replaces shared or non-reference-counted data with uniquely writable
/// buffers while preserving the frame's initialized state.
pub fn av_frame_make_writable(frame: &mut AVFrameMut<'_>) -> Result<(), i32> {
    // SAFETY: the exclusive frame handle permits C to replace its owned buffer
    // fields. Both success and failure leave the frame initialized.
    frame_status(unsafe { ffi::av_frame_make_writable(frame.as_mut_ptr()) })
}

/// Wraps: av_frame_new_side_data
///
/// Adds an owned side-data buffer and exclusively borrows its header for the
/// duration of the frame reborrow. C sizes the window with `av_buffer_alloc`
/// and writes none of it, so the new entry reads back only through
/// [`AVFrameSideDataRef::data`]; use [`AVFrameSideDataMut::write_all`] first if
/// you need the initialized view.
#[must_use]
pub fn av_frame_new_side_data<'a>(
    frame: &'a mut AVFrameMut<'_>,
    kind: AVFrameSideDataType,
    size: usize,
) -> Option<AVFrameSideDataMut<'a>> {
    // SAFETY: the frame is exclusively borrowed. A non-null returned entry is
    // installed in and kept alive by the frame, which owns its new buffer.
    let side_data = unsafe { ffi::av_frame_new_side_data(frame.as_mut_ptr(), kind.as_raw(), size) };
    // SAFETY: the exclusive frame reborrow prevents competing access to the
    // newly installed non-null entry for the returned lifetime.
    unsafe { AVFrameSideDataMut::from_ptr(side_data) }
}

/// Wraps: av_frame_remove_side_data
///
/// Removes and releases every side-data entry of `kind`.
pub fn av_frame_remove_side_data(frame: &mut AVFrameMut<'_>, kind: AVFrameSideDataType) {
    // SAFETY: the exclusive frame handle permits mutation of its side-data
    // collection; the call retains no pointer.
    unsafe { ffi::av_frame_remove_side_data(frame.as_mut_ptr(), kind.as_raw()) }
}

/// Wraps: av_frame_unref
///
/// Releases every owner held by the frame and restores its documented default
/// values while retaining the reusable header allocation.
pub fn av_frame_unref(frame: &mut AVFrameMut<'_>) {
    // SAFETY: the exclusive handle identifies a live initialized frame. C
    // disposes its fields and immediately restores a valid initialized state.
    unsafe { ffi::av_frame_unref(frame.as_mut_ptr()) }
}

#[cfg(test)]
mod scheduled_frame_function_tests {
    use super::*;

    fn configured_video_frame() -> ffibox::CBox<AVFrame> {
        let mut frame = av_frame_alloc().expect("frame allocation");
        // The geometry setters answer `false` on a frame that already holds
        // planes; this one is fresh from `av_frame_alloc`.
        assert!(frame.as_mut().set_width(8));
        assert!(frame.as_mut().set_height(8));
        assert!(
            frame
                .as_mut()
                .set_format(crate::pixfmt::AVPixelFormat::RGBA.as_raw())
        );
        av_frame_get_buffer(&mut frame.as_mut(), 0).expect("frame buffer allocation");
        frame
    }

    #[test]
    fn allocation_clone_copy_writability_and_release_are_typed() {
        let source = configured_video_frame();
        let mut destination = configured_video_frame();
        av_frame_copy(&mut destination.as_mut(), source.as_ref()).expect("copy data");

        destination.as_mut().set_pts(7);
        av_frame_copy_props(&mut destination.as_mut(), source.as_ref()).expect("copy properties");
        assert_eq!(destination.as_ref().pts(), source.as_ref().pts());

        let mut clone = av_frame_clone(source.as_ref()).expect("frame clone");
        assert!(!av_frame_is_writable(source.as_ref()));
        av_frame_make_writable(&mut clone.as_mut()).expect("copy-on-write");
        assert!(av_frame_is_writable(clone.as_ref()));
        av_frame_unref(&mut clone.as_mut());
        assert_eq!(clone.as_ref().format(), -1);
        av_frame_free(Some(clone));
    }

    #[test]
    fn side_data_borrow_tracks_the_frame_collection() {
        let mut frame = av_frame_alloc().expect("frame allocation");
        {
            let mut frame_mut = frame.as_mut();
            let side_data =
                av_frame_new_side_data(&mut frame_mut, AVFrameSideDataType::SEI_UNREGISTERED, 16)
                    .expect("side-data allocation");
            assert_eq!(side_data.as_ref().size(), 16);
        }
        assert!(
            av_frame_get_side_data(frame.as_ref(), AVFrameSideDataType::SEI_UNREGISTERED).is_some()
        );
        av_frame_remove_side_data(&mut frame.as_mut(), AVFrameSideDataType::SEI_UNREGISTERED);
        assert!(
            av_frame_get_side_data(frame.as_ref(), AVFrameSideDataType::SEI_UNREGISTERED).is_none()
        );
    }
}
