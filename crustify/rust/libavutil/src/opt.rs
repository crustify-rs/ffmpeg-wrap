//! Wrappers for libavutil options.

use core::ffi::{CStr, c_char, c_uint, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use crate::ffi;
use crate::rational::AVRationalRef;

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

ffibox::define_ctype!(
    /// Wraps: AVOption
    ///
    /// ABI-compatible view of one entry in an AVClass option table. Option
    /// entries and the metadata they point at are immutable and have no
    /// lifecycle operation; the final entry in a table has a null name.
    AVOption,
    AVOptionRef,
    AVOptionMut,
    ffi::AVOption
);

/// The active member of an [`AVOption`]'s default-value union.
pub enum AVOptionDefault<'a> {
    /// Wraps: AVOption.default_val.arr
    ///
    /// Default metadata for an array option. A null definition selects the
    /// type-specific empty default.
    Array(Option<AVOptionArrayDefRef<'a>>),
    /// Wraps: AVOption.default_val.i64
    ///
    /// Default for integral, enum-like and named-constant options.
    Integer(i64),
    /// Wraps: AVOption.default_val.str
    ///
    /// Serialized default for string-parsed option types.
    String(Option<&'a CStr>),
    /// Wraps: AVOption.default_val.dbl
    ///
    /// Default for floating-point options. Libavutil also stores rational
    /// defaults as a double and converts them with `av_d2q`.
    Double(f64),
    /// A value introduced by a newer libavutil, or otherwise not described by
    /// the current public `AVOptionType` contract. No union member is read.
    Unknown(AVOptionType),
}

impl<'a> AVOptionRef<'a> {
    /// Wraps: AVOption.type
    #[must_use]
    pub fn option_type(&self) -> AVOptionType {
        // SAFETY: the handle guarantees a live initialized option. The raw
        // projection copies the integer-backed type without forming a Rust
        // reference to the C object or field.
        AVOptionType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Wraps: AVOption.offset
    ///
    /// Returns the byte offset of the represented value in its AVClass
    /// context. Named constants conventionally return zero.
    #[must_use]
    pub fn offset(&self) -> i32 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the integer field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).offset).read() }
    }

    /// Wraps: AVOption.flags
    #[must_use]
    pub fn flags(&self) -> i32 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the integer field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).flags).read() }
    }

    /// Wraps: AVOption.name
    ///
    /// Returns `None` for the sentinel that terminates an option table.
    #[must_use]
    pub fn name(&self) -> Option<&'a CStr> {
        // SAFETY: the handle guarantees initialized AVOption metadata. Reading
        // the pointer field does not form a reference to the wrapped object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).name).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: a non-null option name is immutable, NUL-terminated
            // metadata that lives for the option-table lifetime.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Wraps: AVOption.max
    #[must_use]
    pub fn max(&self) -> f64 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the floating-point field without a reference.
        unsafe { addr_of!((*self.as_ptr()).max).read() }
    }

    /// Wraps: AVOption.min
    #[must_use]
    pub fn min(&self) -> f64 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the floating-point field without a reference.
        unsafe { addr_of!((*self.as_ptr()).min).read() }
    }

    /// Wraps: AVOption.unit
    #[must_use]
    pub fn unit(&self) -> Option<&'a CStr> {
        // SAFETY: the handle guarantees initialized AVOption metadata. Reading
        // the pointer field does not form a reference to the wrapped object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).unit).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: a non-null unit is immutable, NUL-terminated metadata
            // that lives for the option-table lifetime.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Wraps: AVOption.help
    #[must_use]
    pub fn help(&self) -> Option<&'a CStr> {
        // SAFETY: the handle guarantees initialized AVOption metadata. Reading
        // the pointer field does not form a reference to the wrapped object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).help).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: non-null help is immutable, NUL-terminated metadata that
            // lives for the option-table lifetime.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Wraps: AVOption.default_val
    ///
    /// Reads only the union member selected by [`option_type`](Self::option_type).
    /// Unknown values are preserved without interpreting the union bytes.
    #[must_use]
    pub fn default_value(&self) -> AVOptionDefault<'a> {
        let option_type = self.option_type();
        if option_type.is_array() {
            // SAFETY: the array flag makes `arr` the active union member, and
            // the handle guarantees initialized metadata.
            let pointer = unsafe { addr_of!((*self.as_ptr()).default_val.arr).read() };
            // SAFETY: a non-null array definition is immutable metadata that
            // remains live with the containing option table.
            let definition = unsafe { AVOptionArrayDefRef::from_ptr(pointer.cast_mut()) };
            return AVOptionDefault::Array(definition);
        }

        let base = option_type.without_array();
        if matches!(
            base,
            AVOptionType::FLAGS
                | AVOptionType::INT
                | AVOptionType::INT64
                | AVOptionType::UINT64
                | AVOptionType::CONST
                | AVOptionType::PIXEL_FMT
                | AVOptionType::SAMPLE_FMT
                | AVOptionType::DURATION
                | AVOptionType::BOOL
                | AVOptionType::UINT
        ) {
            // SAFETY: these option types select the initialized `i64_` union
            // member according to libavutil's AVOption contract.
            return AVOptionDefault::Integer(unsafe {
                addr_of!((*self.as_ptr()).default_val.i64_).read()
            });
        }
        if matches!(
            base,
            AVOptionType::DOUBLE | AVOptionType::FLOAT | AVOptionType::RATIONAL
        ) {
            // SAFETY: these option types select the initialized `dbl` union
            // member; rational defaults are represented as doubles in C.
            return AVOptionDefault::Double(unsafe {
                addr_of!((*self.as_ptr()).default_val.dbl).read()
            });
        }
        if matches!(
            base,
            AVOptionType::STRING
                | AVOptionType::BINARY
                | AVOptionType::DICT
                | AVOptionType::IMAGE_SIZE
                | AVOptionType::VIDEO_RATE
                | AVOptionType::COLOR
                | AVOptionType::CHLAYOUT
        ) {
            // SAFETY: these types select the initialized `str_` union member.
            let pointer = unsafe { addr_of!((*self.as_ptr()).default_val.str_).read() };
            let value = if pointer.is_null() {
                None
            } else {
                // SAFETY: a non-null serialized default is immutable,
                // NUL-terminated metadata live for the option-table lifetime.
                Some(unsafe { CStr::from_ptr(pointer) })
            };
            return AVOptionDefault::String(value);
        }
        AVOptionDefault::Unknown(option_type)
    }

    /// Wraps: AVOption.default_val.q
    ///
    /// Returns the legacy rational union view. Libavutil does not currently
    /// select this member for any option type (rational defaults use `dbl`),
    /// but the public C layout retains it. Every bit pattern is valid for the
    /// two integer fields, so viewing the initialized union storage is safe.
    #[must_use]
    pub fn legacy_rational_default(&self) -> AVRationalRef<'a> {
        // SAFETY: `q` starts at the initialized union storage and consists of
        // two integers, for which every bit pattern is valid. The returned
        // shared handle remains bounded by the AVOption handle lifetime.
        unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).default_val.q).cast_mut()) }
            .expect("an AVOption union field is never null")
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

    #[test]
    fn option_metadata_and_tagged_defaults_are_accessible() {
        assert_eq!(size_of::<AVOption>(), size_of::<ffi::AVOption>());
        assert_eq!(align_of::<AVOption>(), align_of::<ffi::AVOption>());

        let mut raw = ffi::AVOption {
            name: c"threads".as_ptr(),
            help: c"worker count".as_ptr(),
            offset: 24,
            type_: ffi::AVOptionType_AV_OPT_TYPE_INT,
            default_val: ffi::AVOption__bindgen_ty_1 { i64_: 4 },
            min: 1.0,
            max: 64.0,
            flags: 3,
            unit: core::ptr::null(),
        };
        // SAFETY: `raw` is initialized and remains live and exclusively held
        // while this borrowed handle exists.
        let option = unsafe { AVOptionMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        let shared = option.as_ref();

        assert_eq!(shared.name(), Some(c"threads"));
        assert_eq!(shared.help(), Some(c"worker count"));
        assert_eq!(shared.unit(), None);
        assert_eq!(shared.offset(), 24);
        assert_eq!(shared.option_type(), AVOptionType::INT);
        assert_eq!(shared.min(), 1.0);
        assert_eq!(shared.max(), 64.0);
        assert_eq!(shared.flags(), 3);
        assert!(matches!(
            shared.default_value(),
            AVOptionDefault::Integer(4)
        ));
    }

    #[test]
    fn default_union_selects_string_array_and_legacy_rational_views() {
        let definition = ffi::AVOptionArrayDef {
            def: c"a,b".as_ptr(),
            size_min: 0,
            size_max: 2,
            sep: b',' as c_char,
        };
        let mut array = ffi::AVOption {
            name: c"values".as_ptr(),
            help: core::ptr::null(),
            offset: 0,
            type_: ffi::AVOptionType_AV_OPT_TYPE_STRING | ffi::AVOptionType_AV_OPT_TYPE_FLAG_ARRAY,
            default_val: ffi::AVOption__bindgen_ty_1 {
                arr: addr_of!(definition),
            },
            min: 0.0,
            max: 0.0,
            flags: 0,
            unit: core::ptr::null(),
        };
        // SAFETY: both raw metadata values remain initialized and live for the
        // borrow, and the local test retains exclusive access to `array`.
        let option = unsafe { AVOptionMut::from_ptr(addr_of_mut!(array)) }.unwrap();
        let AVOptionDefault::Array(Some(array_default)) = option.as_ref().default_value() else {
            panic!("expected an array default")
        };
        assert_eq!(array_default.def(), Some(c"a,b"));
        assert_eq!(array_default.size_max(), 2);

        let mut string = ffi::AVOption {
            name: c"format".as_ptr(),
            help: core::ptr::null(),
            offset: 0,
            type_: ffi::AVOptionType_AV_OPT_TYPE_STRING,
            default_val: ffi::AVOption__bindgen_ty_1 {
                str_: c"raw".as_ptr(),
            },
            min: 0.0,
            max: 0.0,
            flags: 0,
            unit: core::ptr::null(),
        };
        // SAFETY: `string` is initialized and exclusively borrowed for the
        // returned handle's lifetime.
        let string_option = unsafe { AVOptionMut::from_ptr(addr_of_mut!(string)) }.unwrap();
        assert!(matches!(
            string_option.as_ref().default_value(),
            AVOptionDefault::String(Some(value)) if value == c"raw"
        ));

        let mut rational = ffi::AVOption {
            name: c"ratio".as_ptr(),
            help: core::ptr::null(),
            offset: 0,
            type_: ffi::AVOptionType_AV_OPT_TYPE_RATIONAL,
            default_val: ffi::AVOption__bindgen_ty_1 {
                q: ffi::AVRational { num: 3, den: 5 },
            },
            min: 0.0,
            max: 1.0,
            flags: 0,
            unit: core::ptr::null(),
        };
        // SAFETY: `rational` is initialized and exclusively borrowed for the
        // returned handle's lifetime.
        let rational_option = unsafe { AVOptionMut::from_ptr(addr_of_mut!(rational)) }.unwrap();
        let legacy = rational_option.as_ref().legacy_rational_default();
        assert_eq!(legacy.num(), 3);
        assert_eq!(legacy.den(), 5);
    }
}
