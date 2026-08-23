//! Wrappers for libavutil options.

use core::ffi::{CStr, c_char, c_uint, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::CBox;

use crate::channel_layout::AVChannelLayoutRef;
use crate::dict::AVDictionary;
use crate::ffi;
use crate::pixfmt::AVPixelFormat;
use crate::rational::AVRationalRef;
use crate::samplefmt::AVSampleFormat;

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

/// Shared handle to the alternate fake-object shape accepted by option lookup:
/// a live pointer slot containing an AVClass pointer.
#[derive(Clone, Copy)]
pub struct FakeOptionObjectRef<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

impl<'a> FakeOptionObjectRef<'a> {
    /// Constructs a borrowed fake-object handle.
    ///
    /// # Safety
    ///
    /// `pointer` must address a live `const AVClass *` slot whose class and
    /// immutable option metadata remain live for `'a`.
    pub unsafe fn from_raw(pointer: NonNull<c_void>) -> Self {
        Self {
            pointer,
            _borrow: PhantomData,
        }
    }

    fn as_ptr(self) -> *mut c_void {
        self.pointer.as_ptr()
    }
}

/// A successful option lookup together with the object that owns the option.
/// The exclusive target handle is tied to the lookup's borrow of the root
/// object, so callers may safely pass it to the option setters.
pub struct AVOptionMatch<'a> {
    option: AVOptionRef<'a>,
    target: OptionObjectMut<'a>,
}

impl<'a> AVOptionMatch<'a> {
    #[must_use]
    pub fn option(&self) -> AVOptionRef<'a> {
        self.option
    }

    #[must_use]
    pub fn target_mut(&mut self) -> &mut OptionObjectMut<'a> {
        &mut self.target
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptFindError {
    /// Fake-object searches have a different pointer shape and cannot safely
    /// produce an exclusive target-object handle.
    FakeObjectSearch,
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
    /// Describes the element-count limits, separator and serialized default of
    /// an array option. C never stores one inline: an [`AVOption`] reaches it
    /// by pointer through `default_val.arr`, and every definition in the tree
    /// is a `static const` initializer, so the layout stays C-compatible. It
    /// has no lifecycle operation — libavutil owns neither the structure nor
    /// the string `def` points at, and never frees or mutates either.
    ///
    /// # Invariant
    ///
    /// A handle over a definition asserts that its `def` field is null or
    /// addresses a NUL-terminated string that outlives the handle's borrow.
    /// That is what makes the safe getter [`AVOptionArrayDefRef::def`] sound:
    /// the unsafe `from_ptr` constructors are where a caller establishes it,
    /// and [`AVOptionArrayDefMut::set_def`] preserves it by accepting only a
    /// `&'static CStr`. C upholds it by initializing `def` from a string
    /// literal or leaving it null.
    ///
    /// Libavutil documents `size_min`, `size_max` and `sep` as readable by
    /// foreign code and `def` as native access only — that is an API-stability
    /// contract, not a memory-safety one, so [`def`](AVOptionArrayDefRef::def)
    /// is exposed but its meaning may change between libavutil versions.
    AVOptionArrayDef,
    AVOptionArrayDefRef,
    AVOptionArrayDefMut,
    ffi::AVOptionArrayDef
);

impl<'a> AVOptionArrayDefRef<'a> {
    /// Wraps: AVOptionArrayDef.def
    ///
    /// Returns the serialized default — the element list as `av_opt_get` would
    /// render it, joined by [`sep`](Self::sep) — or `None` when the definition
    /// declares no default. Native access only in libavutil's contract; see
    /// the [type documentation](AVOptionArrayDef).
    #[must_use]
    pub fn def(&self) -> Option<&'a CStr> {
        // SAFETY: the handle points to a live initialized definition. Reading
        // the pointer field forms no reference to the C object.
        let ptr = unsafe { addr_of!((*self.as_ptr()).def).read() };
        if ptr.is_null() {
            None
        } else {
            // SAFETY: `AVOptionArrayDef`'s handle invariant makes a non-null
            // `def` a NUL-terminated string outliving this borrow. Every
            // producer of the handle carries that obligation: `from_ptr`'s
            // caller asserts it, and `set_def` cannot break it because it
            // accepts only `&'static CStr`. The result is further narrowed to
            // `'a`, which the definition itself already outlives.
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Wraps: AVOptionArrayDef.size_min
    ///
    /// Returns the minimum number of array elements. Zero means no minimum;
    /// a non-zero minimum additionally requires [`def`](Self::def) to be
    /// present and to list at least that many elements.
    #[must_use]
    pub fn size_min(&self) -> c_uint {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).size_min).read() }
    }

    /// Wraps: AVOptionArrayDef.size_max
    ///
    /// Returns the maximum number of array elements. Zero means unlimited.
    #[must_use]
    pub fn size_max(&self) -> c_uint {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).size_max).read() }
    }

    /// Wraps: AVOptionArrayDef.sep
    ///
    /// Returns the serialized array separator. Zero selects libavutil's
    /// default separator, a comma. The field is a C `char`, so a value above
    /// 127 reads back negative where `c_char` is signed; the documented
    /// separator grammar is printable ASCII, which never does.
    #[must_use]
    pub fn sep(&self) -> c_char {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).sep).read() }
    }
}

impl AVOptionArrayDefMut<'_> {
    /// Sets the serialized default to static string metadata, or clears it.
    ///
    /// The `'static` bound is what keeps this setter safe: it is the half of
    /// [`AVOptionArrayDef`]'s handle invariant that safe Rust could otherwise
    /// break, and it matches how C declares defaults, as string literals in a
    /// `static const` initializer.
    pub fn set_def(&mut self, value: Option<&'static CStr>) {
        let ptr = value.map_or(core::ptr::null(), CStr::as_ptr);
        // SAFETY: the exclusive handle provides write access to a live
        // definition, and the stored string is static and therefore outlives
        // every later observation of this metadata, re-establishing the type
        // invariant that `AVOptionArrayDefRef::def` relies on.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).def).write(ptr) }
    }

    /// Sets the minimum number of array elements. Zero disables the minimum.
    ///
    /// Libavutil requires a non-zero minimum to be paired with a
    /// [`set_def`](Self::set_def) default listing at least that many elements.
    /// Nothing here enforces that; a definition that violates it makes
    /// `av_opt_set_array` reject writes that would leave the array short.
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

    /// Sets the separator. Zero selects a comma; a non-zero value must be a
    /// printable ASCII character that is neither alphanumeric nor a backslash.
    /// Libavutil checks that grammar with `av_assert0` while applying option
    /// defaults, so a definition outside it aborts the process inside C rather
    /// than returning an error.
    pub fn set_sep(&mut self, value: c_char) {
        // SAFETY: the exclusive handle provides write access to this field of
        // a live definition; the raw projection forms no Rust reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).sep).write(value) }
    }
}

/// Wraps: AVOptionType
///
/// Names the C type of the object field an option controls, and whether that
/// field holds one value or an array of them. This is an integer newtype
/// rather than a Rust enum because [`FLAG_ARRAY`](Self::FLAG_ARRAY) is a
/// modifier rather than a type of its own — C ors it into a regular value, so
/// the set of representable values is not the set of enumerators — and because
/// values introduced by newer libavutil versions must survive a round trip.
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
    #[must_use]
    pub const fn from_raw(raw: ffi::AVOptionType) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVOptionType {
        self.0
    }

    /// Marks a regular option type as an array option. Applying it to
    /// [`FLAG_ARRAY`](Self::FLAG_ARRAY) or to a value that already carries the
    /// flag is idempotent.
    #[must_use]
    pub const fn with_array(self) -> Self {
        Self(self.0 | Self::FLAG_ARRAY.0)
    }

    /// Reports whether the array flag is present, meaning the object field is
    /// a pointer to the elements followed by an `unsigned` element count.
    #[must_use]
    pub const fn is_array(self) -> bool {
        self.0 & Self::FLAG_ARRAY.0 != 0
    }

    /// Removes the array flag while preserving every other raw bit, giving the
    /// element type of an array option. This is libavutil's internal
    /// `TYPE_BASE`, which selects the per-type element size and parser.
    #[must_use]
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
            // remains live with the containing option table, so it outlives
            // `'a`. It also satisfies `AVOptionArrayDef`'s handle invariant:
            // C initializes `def` from a string literal or leaves it null, and
            // this shared handle cannot write the field.
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
    fn zeroed_array_definition_declares_no_default_and_no_limits() {
        let mut definition = AVOptionArrayDef::zeroed();
        // SAFETY: `AVOptionArrayDef` is `#[repr(transparent)]` over the C
        // struct, the local is live and initialized by `zeroed`, and this
        // test holds no other handle to it.
        let handle = unsafe {
            AVOptionArrayDefMut::from_ptr(
                addr_of_mut!(definition).cast::<ffi::AVOptionArrayDef>(),
            )
        }
        .unwrap();

        let shared = handle.as_ref();
        // The all-zero definition is the one every C initializer that omits a
        // member produces: no default, no bounds, and a comma separator.
        assert_eq!(shared.def(), None);
        assert_eq!(shared.size_min(), 0);
        assert_eq!(shared.size_max(), 0);
        assert_eq!(shared.sep(), 0);
    }

    #[test]
    fn clearing_the_default_is_observable_as_none() {
        let mut raw = ffi::AVOptionArrayDef {
            def: c"a,b".as_ptr(),
            size_min: 2,
            size_max: 2,
            sep: 0,
        };
        // SAFETY: `raw` is live and initialized, and this test keeps exclusive
        // access to it for the whole lifetime of the handle.
        let mut definition = unsafe { AVOptionArrayDefMut::from_ptr(addr_of_mut!(raw)) }.unwrap();

        assert_eq!(definition.as_ref().def(), Some(c"a,b"));
        definition.set_def(None);
        assert_eq!(definition.as_ref().def(), None);
        assert!(raw.def.is_null());
    }

    #[test]
    fn every_option_type_constant_round_trips_and_is_scalar() {
        const NAMED: [AVOptionType; 20] = [
            AVOptionType::FLAGS,
            AVOptionType::INT,
            AVOptionType::INT64,
            AVOptionType::DOUBLE,
            AVOptionType::FLOAT,
            AVOptionType::STRING,
            AVOptionType::RATIONAL,
            AVOptionType::BINARY,
            AVOptionType::DICT,
            AVOptionType::UINT64,
            AVOptionType::CONST,
            AVOptionType::IMAGE_SIZE,
            AVOptionType::PIXEL_FMT,
            AVOptionType::SAMPLE_FMT,
            AVOptionType::VIDEO_RATE,
            AVOptionType::DURATION,
            AVOptionType::COLOR,
            AVOptionType::BOOL,
            AVOptionType::CHLAYOUT,
            AVOptionType::UINT,
        ];

        for (index, &kind) in NAMED.iter().enumerate() {
            assert_eq!(AVOptionType::from_raw(kind.as_raw()), kind);
            // The enumerators are a dense 1..=20 run, disjoint from the array
            // flag, so `with_array` never collides with a regular type.
            assert_eq!(kind.as_raw(), index as ffi::AVOptionType + 1);
            assert!(!kind.is_array());
            assert!(kind.with_array().is_array());
            assert_eq!(kind.with_array().without_array(), kind);
            assert_eq!(kind.without_array(), kind);
        }
    }

    #[test]
    fn array_flag_is_idempotent_and_preserves_unknown_bits() {
        assert!(AVOptionType::FLAG_ARRAY.is_array());
        assert_eq!(
            AVOptionType::FLAG_ARRAY.with_array(),
            AVOptionType::FLAG_ARRAY
        );

        let unknown = AVOptionType::from_raw(1 << 20 | AVOptionType::INT.as_raw());
        assert!(!unknown.is_array());
        assert_eq!(unknown.without_array(), unknown);
        assert_eq!(unknown.with_array().without_array(), unknown);
        // An unrecognized base type must not be decoded as any union member.
        let mut raw = ffi::AVOption {
            name: c"future".as_ptr(),
            help: core::ptr::null(),
            offset: 0,
            type_: unknown.as_raw(),
            default_val: ffi::AVOption__bindgen_ty_1 { i64_: 7 },
            min: 0.0,
            max: 0.0,
            flags: 0,
            unit: core::ptr::null(),
        };
        // SAFETY: `raw` is initialized and exclusively borrowed for the
        // lifetime of the handle.
        let option = unsafe { AVOptionMut::from_ptr(addr_of_mut!(raw)) }.unwrap();
        assert!(matches!(
            option.as_ref().default_value(),
            AVOptionDefault::Unknown(kind) if kind == unknown
        ));
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

/// A homogeneous, ABI-compatible input array for [`av_opt_set_array`].
pub enum OptArrayValues<'a> {
    Int(&'a [i32]),
    Int64(&'a [i64]),
    Float(&'a [f32]),
    Double(&'a [f64]),
}

impl OptArrayValues<'_> {
    fn raw(&self) -> (AVOptionType, *const c_void, usize) {
        match self {
            Self::Int(v) => (AVOptionType::INT, v.as_ptr().cast(), v.len()),
            Self::Int64(v) => (AVOptionType::INT64, v.as_ptr().cast(), v.len()),
            Self::Float(v) => (AVOptionType::FLOAT, v.as_ptr().cast(), v.len()),
            Self::Double(v) => (AVOptionType::DOUBLE, v.as_ptr().cast(), v.len()),
        }
    }
}

/// Wraps: av_opt_set_array
pub fn av_opt_set_array(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    search_flags: i32,
    start_element: u32,
    values: OptArrayValues<'_>,
) -> Result<(), OptSetError> {
    let (kind, pointer, len) = values.raw();
    let len = u32::try_from(len).map_err(|_| OptSetError::LengthOverflow)?;
    // SAFETY: the exclusive object handle is live, `name` is terminated, and
    // the enum couples the C element tag to a contiguous slice of that ABI.
    result(unsafe {
        ffi::av_opt_set_array(
            object.as_mut_ptr(),
            name.as_ptr(),
            search_flags,
            start_element,
            len,
            kind.as_raw(),
            pointer,
        )
    })
}

/// Wraps: av_opt_set_array
pub fn av_opt_remove_array(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    search_flags: i32,
    start_element: u32,
    count: u32,
) -> Result<(), OptSetError> {
    // SAFETY: a null value selects removal and makes the ignored type harmless;
    // the object and name satisfy the ordinary setter contract.
    result(unsafe {
        ffi::av_opt_set_array(
            object.as_mut_ptr(),
            name.as_ptr(),
            search_flags,
            start_element,
            count,
            AVOptionType::INT.as_raw(),
            core::ptr::null(),
        )
    })
}

/// Wraps: av_opt_set_dict
pub fn av_opt_set_dict(
    object: &mut OptionObjectMut<'_>,
    options: &mut Option<CBox<AVDictionary>>,
) -> Result<(), OptSetError> {
    let mut raw = options.take().map_or(core::ptr::null_mut(), CBox::into_raw);
    // SAFETY: ownership of the dictionary was surrendered to the writable
    // local slot; C consumes it and leaves null or an independently owned
    // dictionary of unrecognized options in the same slot.
    let status = unsafe { ffi::av_opt_set_dict(object.as_mut_ptr(), &raw mut raw) };
    // SAFETY: after the call, a non-null pointer is a uniquely owned, fully
    // constructed dictionary which uses AVDictionary's matching destructor.
    *options = unsafe { CBox::from_raw(raw) };
    result(status)
}

/// Wraps: av_opt_set_pixel_fmt
pub fn av_opt_set_pixel_fmt(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: AVPixelFormat,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the object borrow and C string are live for the call; the value
    // is an ABI-compatible open pixel-format integer.
    result(unsafe {
        ffi::av_opt_set_pixel_fmt(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.as_raw(),
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_q
pub fn av_opt_set_q(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: AVRationalRef<'_>,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the rational is copied by value and no input is retained.
    result(unsafe {
        ffi::av_opt_set_q(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.copy_ffi(),
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_sample_fmt
pub fn av_opt_set_sample_fmt(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: AVSampleFormat,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the object borrow and C string are live for the call; the value
    // is an ABI-compatible open sample-format integer.
    result(unsafe {
        ffi::av_opt_set_sample_fmt(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.as_raw(),
            search_flags,
        )
    })
}

/// Wraps: av_opt_set_video_rate
pub fn av_opt_set_video_rate(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: AVRationalRef<'_>,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the rational is copied by value and no input is retained.
    result(unsafe {
        ffi::av_opt_set_video_rate(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.copy_ffi(),
            search_flags,
        )
    })
}

/// Wraps: av_opt_find2
///
/// Searches a normal AVClass-bearing object. On success the returned option
/// metadata and exclusive target-object handle remain tied to the exclusive
/// borrow of `object`. Fake-object searches are rejected because their input
/// is a pointer to an AVClass pointer rather than an object.
pub fn av_opt_find2<'a>(
    object: &'a mut OptionObjectMut<'_>,
    name: &CStr,
    unit: Option<&CStr>,
    option_flags: i32,
    search_flags: i32,
) -> Result<Option<AVOptionMatch<'a>>, OptFindError> {
    if search_flags & ffi::AV_OPT_SEARCH_FAKE_OBJ as i32 != 0 {
        return Err(OptFindError::FakeObjectSearch);
    }

    let mut target = core::ptr::null_mut();
    // SAFETY: the exclusive handle supplies a live normal AVClass-bearing
    // object for the call; both strings are readable and NUL-terminated, and
    // the target slot is writable. C retains none of those temporary inputs.
    let option = unsafe {
        ffi::av_opt_find2(
            object.as_mut_ptr(),
            name.as_ptr(),
            unit.map_or(core::ptr::null(), CStr::as_ptr),
            option_flags,
            search_flags,
            addr_of_mut!(target),
        )
    };
    let Some(option) = NonNull::new(option.cast_mut()) else {
        return Ok(None);
    };
    let target = NonNull::new(target).expect("normal av_opt_find2 match has a target object");
    // SAFETY: a successful search returns immutable option metadata bounded by
    // the exclusively borrowed root hierarchy.
    let option = unsafe { AVOptionRef::from_ptr(option.as_ptr()) }.expect("nonnull option pointer");
    // SAFETY: a successful normal-object search returns a target within the
    // exclusively borrowed root hierarchy, and this handle is bounded by that
    // borrow so no raw pointer escapes.
    let target = unsafe { OptionObjectMut::from_raw(target) };
    Ok(Some(AVOptionMatch { option, target }))
}

/// Wraps: av_opt_find2
///
/// Searches the function's alternate fake-object input shape. This variant
/// always supplies the required fake-object flag and omits the target object,
/// which the C contract ignores for such searches.
pub fn av_opt_find2_fake<'a>(
    object: FakeOptionObjectRef<'a>,
    name: &CStr,
    unit: Option<&CStr>,
    option_flags: i32,
    search_flags: i32,
) -> Option<AVOptionRef<'a>> {
    // SAFETY: the handle carries the live class-pointer slot and metadata
    // lifetime; strings are readable for the call, and C retains no input.
    let option = unsafe {
        ffi::av_opt_find2(
            object.as_ptr(),
            name.as_ptr(),
            unit.map_or(core::ptr::null(), CStr::as_ptr),
            option_flags,
            search_flags | ffi::AV_OPT_SEARCH_FAKE_OBJ as i32,
            core::ptr::null_mut(),
        )
    };
    // SAFETY: null means no match; a non-null result is immutable option
    // metadata whose lifetime is bounded by the fake-object handle.
    unsafe { AVOptionRef::from_ptr(option.cast_mut()) }
}

#[cfg(test)]
mod scheduled_find_tests {
    use super::*;

    #[test]
    fn fake_object_flag_is_rejected_before_ffi() {
        let mut class_pointer: *const c_void = core::ptr::null();
        let pointer = NonNull::from(&mut class_pointer).cast::<c_void>();
        // SAFETY: `class_pointer` is live and exclusively borrowed and models
        // the required first AVClass-pointer field for this rejection test.
        let mut object = unsafe { OptionObjectMut::from_raw(pointer) };
        assert!(matches!(
            av_opt_find2(
                &mut object,
                c"name",
                None,
                0,
                ffi::AV_OPT_SEARCH_FAKE_OBJ as i32,
            ),
            Err(OptFindError::FakeObjectSearch)
        ));
    }
}

/// Wraps: av_opt_set_chlayout
pub fn av_opt_set_chlayout(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    layout: AVChannelLayoutRef<'_>,
    search_flags: i32,
) -> Result<(), OptSetError> {
    // SAFETY: the object is exclusively borrowed, the layout is shared, and
    // `name` is NUL-terminated. C retains none of the pointers and deep-copies
    // the layout into the selected option storage.
    result(unsafe {
        ffi::av_opt_set_chlayout(
            object.as_mut_ptr(),
            name.as_ptr(),
            layout.as_ptr(),
            search_flags,
        )
    })
}
