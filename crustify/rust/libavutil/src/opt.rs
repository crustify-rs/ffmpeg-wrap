//! Wrappers for libavutil options.

use core::ffi::{CStr, c_char, c_uint, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use crate::ffi;

/// Exclusive borrowed handle to an AVClass-bearing object whose concrete type
/// has not yet been translated. It keeps the unavoidable erased pointer at one
/// explicit construction seam instead of repeating it in every option setter.
pub struct OptionObjectMut<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a mut c_void>,
}

impl<'a> OptionObjectMut<'a> {
    /// Construct a temporary exclusive option-object handle.
    ///
    /// # Safety
    ///
    /// `pointer` must remain live and exclusively borrowed for `'a`; it must
    /// identify either an object whose first field is `AVClass *`, or the fake
    /// object shape required by the search flags passed to a setter.
    pub unsafe fn from_raw(pointer: NonNull<c_void>) -> Self {
        Self {
            pointer,
            _borrow: PhantomData,
        }
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.pointer.as_ptr()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptSetError {
    LengthOverflow,
    Library(i32),
}

fn result(status: i32) -> Result<(), OptSetError> {
    if status < 0 {
        Err(OptSetError::Library(status))
    } else {
        Ok(())
    }
}

/// Wraps: av_opt_set
pub fn av_opt_set(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: &CStr,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the handle carries the live exclusive object borrow, and both
    // `CStr`s remain live for the read-only duration of the call.
    result(unsafe {
        ffi::av_opt_set(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.as_ptr(),
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_bin
pub fn av_opt_set_bin(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: &[u8],
    search_flags: i32,
) -> Result<(), OptSetError> {
    let length = i32::try_from(value.len()).map_err(|_| OptSetError::LengthOverflow)?;
    // SAFETY: the handle carries the live exclusive object borrow; `name` is
    // terminated and `value` supplies exactly `length` readable bytes.
    result(unsafe {
        ffi::av_opt_set_bin(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.as_ptr(),
            length,
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_double
pub fn av_opt_set_double(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: f64,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the handle carries the live exclusive object borrow and `name`
    // remains live and NUL-terminated for the call.
    result(unsafe {
        ffi::av_opt_set_double(object.as_mut_ptr(), name.as_ptr(), value, search_flags)
    })
}

/// Wraps: av_opt_set_image_size
pub fn av_opt_set_image_size(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    width: i32,
    height: i32,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the handle carries the live exclusive object borrow and `name`
    // remains live and NUL-terminated for the call.
    result(unsafe {
        ffi::av_opt_set_image_size(
            object.as_mut_ptr(),
            name.as_ptr(),
            width,
            height,
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_int
pub fn av_opt_set_int(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: i64,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the handle carries the live exclusive object borrow and `name`
    // remains live and NUL-terminated for the call.
    result(unsafe { ffi::av_opt_set_int(object.as_mut_ptr(), name.as_ptr(), value, search_flags) })
}

ffibox::define_ctype!(
    /// Wraps: AVOptionArrayDef
    ///
    /// Describes the limits, separator and serialized default for an array
    /// option. The layout stays compatible with C so it can remain embedded in
    /// libavutil's static option metadata. It has no destructor: neither the
    /// structure nor its borrowed default string is owned by libavutil.
    AVOptionArrayDef,
    AVOptionArrayDefRef,
    AVOptionArrayDefMut,
    ffi::AVOptionArrayDef
);

impl<'a> AVOptionArrayDefRef<'a> {
    /// Wraps: AVOptionArrayDef.def
    ///
    /// Returns the serialized default, or `None` when no default is declared.
    /// AVOptions are static metadata, so C keeps a non-null string live and
    /// NUL-terminated for at least as long as this definition can be borrowed.
    pub fn def(&self) -> Option<&'a CStr> {
        // SAFETY: the handle points to a live initialized definition. Reading
        // the pointer field forms no reference to the C object.
        let ptr = unsafe { addr_of!((*self.as_ptr()).def).read() };
        if ptr.is_null() {
            None
        } else {
            // SAFETY: the AVOption metadata contract requires a non-null `def`
            // to be a NUL-terminated static string. The returned borrow is
            // conservatively limited to the definition handle's lifetime.
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Wraps: AVOptionArrayDef.size_min
    ///
    /// Returns the minimum number of array elements. Zero means no minimum.
    pub fn size_min(&self) -> c_uint {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).size_min).read() }
    }

    /// Wraps: AVOptionArrayDef.size_max
    ///
    /// Returns the maximum number of array elements. Zero means unlimited.
    pub fn size_max(&self) -> c_uint {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).size_max).read() }
    }

    /// Wraps: AVOptionArrayDef.sep
    ///
    /// Returns the serialized array separator. Zero selects libavutil's
    /// default separator, a comma.
    pub fn sep(&self) -> c_char {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).sep).read() }
    }
}

impl AVOptionArrayDefMut<'_> {
    /// Sets the serialized default to static string metadata, or clears it.
    pub fn set_def(&mut self, value: Option<&'static CStr>) {
        let ptr = value.map_or(core::ptr::null(), CStr::as_ptr);
        // SAFETY: the exclusive handle provides write access to a live
        // definition, and the stored string is static and therefore outlives
        // every later observation of this metadata.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).def).write(ptr) }
    }

    /// Sets the minimum number of array elements. Zero disables the minimum.
    pub fn set_size_min(&mut self, value: c_uint) {
        // SAFETY: the exclusive handle provides write access to this field of
        // a live definition; the raw projection forms no Rust reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).size_min).write(value) }
    }

    /// Sets the maximum number of array elements. Zero means unlimited.
    pub fn set_size_max(&mut self, value: c_uint) {
        // SAFETY: the exclusive handle provides write access to this field of
        // a live definition; the raw projection forms no Rust reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).size_max).write(value) }
    }

    /// Sets the separator. Zero selects a comma; non-zero values should follow
    /// libavutil's documented printable, non-alphanumeric separator grammar.
    pub fn set_sep(&mut self, value: c_char) {
        // SAFETY: the exclusive handle provides write access to this field of
        // a live definition; the raw projection forms no Rust reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).sep).write(value) }
    }
}

/// Wraps: AVOptionType
///
/// Identifies the native representation and foreign access semantics of an
/// option. This is an integer newtype rather than a Rust enum because C may
/// combine a regular value with [`FLAG_ARRAY`](Self::FLAG_ARRAY), and because
/// values introduced by newer libavutil versions must remain representable.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVOptionType(ffi::AVOptionType);

impl AVOptionType {
    pub const FLAGS: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_FLAGS);
    pub const INT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_INT);
    pub const INT64: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_INT64);
    pub const DOUBLE: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_DOUBLE);
    pub const FLOAT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_FLOAT);
    pub const STRING: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_STRING);
    pub const RATIONAL: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_RATIONAL);
    pub const BINARY: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_BINARY);
    pub const DICT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_DICT);
    pub const UINT64: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_UINT64);
    pub const CONST: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_CONST);
    pub const IMAGE_SIZE: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_IMAGE_SIZE);
    pub const PIXEL_FMT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_PIXEL_FMT);
    pub const SAMPLE_FMT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_SAMPLE_FMT);
    pub const VIDEO_RATE: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_VIDEO_RATE);
    pub const DURATION: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_DURATION);
    pub const COLOR: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_COLOR);
    pub const BOOL: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_BOOL);
    pub const CHLAYOUT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_CHLAYOUT);
    pub const UINT: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_UINT);
    pub const FLAG_ARRAY: Self = Self(ffi::AVOptionType_AV_OPT_TYPE_FLAG_ARRAY);

    /// Wraps a raw C value, including one unknown to this crate version.
    pub const fn from_raw(raw: ffi::AVOptionType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    pub const fn as_raw(self) -> ffi::AVOptionType {
        self.0
    }

    /// Marks a regular option type as an array option.
    pub const fn with_array(self) -> Self {
        Self(self.0 | Self::FLAG_ARRAY.0)
    }

    /// Reports whether the array flag is present.
    pub const fn is_array(self) -> bool {
        self.0 & Self::FLAG_ARRAY.0 != 0
    }

    /// Removes the array flag while preserving every other raw bit.
    pub const fn without_array(self) -> Self {
        Self(self.0 & !Self::FLAG_ARRAY.0)
    }
}

impl From<ffi::AVOptionType> for AVOptionType {
    fn from(raw: ffi::AVOptionType) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVOptionType> for ffi::AVOptionType {
    fn from(value: AVOptionType) -> Self {
        value.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn array_definition_is_layout_compatible_and_accessible() {
        assert_eq!(
            size_of::<AVOptionArrayDef>(),
            size_of::<ffi::AVOptionArrayDef>()
        );
        assert_eq!(
            align_of::<AVOptionArrayDef>(),
            align_of::<ffi::AVOptionArrayDef>()
        );

        let mut raw = ffi::AVOptionArrayDef {
            def: core::ptr::null(),
            size_min: 0,
            size_max: 0,
            sep: 0,
        };
        // SAFETY: `raw` is a live initialized definition, and this test keeps
        // exclusive access to it for the entire lifetime of the handle.
        let mut definition = unsafe { AVOptionArrayDefMut::from_ptr(addr_of_mut!(raw)) }.unwrap();

        definition.set_def(Some(c"one|two"));
        definition.set_size_min(1);
        definition.set_size_max(4);
        definition.set_sep(b'|' as c_char);

        let shared = definition.as_ref();
        assert_eq!(shared.def(), Some(c"one|two"));
        assert_eq!(shared.size_min(), 1);
        assert_eq!(shared.size_max(), 4);
        assert_eq!(shared.sep(), b'|' as c_char);
    }

    #[test]
    fn option_type_preserves_raw_and_array_values() {
        assert_eq!(size_of::<AVOptionType>(), size_of::<ffi::AVOptionType>());
        assert_eq!(align_of::<AVOptionType>(), align_of::<ffi::AVOptionType>());

        let array = AVOptionType::STRING.with_array();
        assert!(array.is_array());
        assert_eq!(array.without_array(), AVOptionType::STRING);
        assert_eq!(
            array.as_raw(),
            ffi::AVOptionType_AV_OPT_TYPE_STRING | ffi::AVOptionType_AV_OPT_TYPE_FLAG_ARRAY
        );

        let unknown = ffi::AVOptionType::MAX;
        assert_eq!(AVOptionType::from_raw(unknown).as_raw(), unknown);
    }
}
