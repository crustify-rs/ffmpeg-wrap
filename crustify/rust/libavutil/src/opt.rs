//! Wrappers for libavutil options.

use core::ffi::{CStr, c_char, c_uint, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CBox, CDropped, CrustifyStr};

use crate::channel_layout::AVChannelLayoutRef;
use crate::dict::AVDictionary;
use crate::ffi;
use crate::mem::AvFree;
use crate::pixfmt::AVPixelFormat;
use crate::rational::AVRationalRef;
use crate::samplefmt::AVSampleFormat;

/// Shared borrowed handle to a well-formed AVClass-bearing object.
#[derive(Clone, Copy)]
pub struct OptionObjectRef<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

impl<'a> OptionObjectRef<'a> {
    /// Constructs a shared option-object handle.
    ///
    /// # Safety
    ///
    /// `pointer` must remain live for `'a`, start with a valid `AVClass *`, and
    /// every option field and child returned by that class must satisfy the
    /// representation and lifetime contract declared by its `AVOption`.
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
    /// identify an object whose first field is a valid `AVClass *`. Every field
    /// described by that class's options must hold a valid value of the declared
    /// C representation, including any owned allocation it names.
    pub unsafe fn from_raw(pointer: NonNull<c_void>) -> Self {
        Self {
            pointer,
            _borrow: PhantomData,
        }
    }

    #[must_use]
    pub fn as_ref(&self) -> OptionObjectRef<'_> {
        OptionObjectRef {
            pointer: self.pointer,
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
    /// The search flags asked for a fake-object lookup. No setter can carry
    /// one out — a fake object is a bare class pointer with no storage behind
    /// it — and reaching C with the flag would dereference the NULL target it
    /// produces, so every setter refuses it before the call.
    FakeObjectSearch,
    Library(i32),
}

fn result(status: i32) -> Result<(), OptSetError> {
    if status < 0 {
        Err(OptSetError::Library(status))
    } else {
        Ok(())
    }
}

/// Refuses a fake-object search before it reaches C.
///
/// Every setter resolves its option through `opt_set_init`, which honours
/// `AV_OPT_SEARCH_FAKE_OBJ` far enough for `av_opt_find2` to report *no target
/// object* — the flag's whole meaning is that the caller passed a bare class
/// pointer with no storage behind it — and then loads the class out of that
/// NULL target to look for state flags. So the flag turns a found option into
/// a NULL dereference in `opt.c`, which this campaign's sanitiser build
/// reports as a SEGV at `opt_set_init`, and it can never turn into a write,
/// because a fake object has no field to write to. Rejecting it here costs a
/// caller nothing and keeps the setters safe for every value of a flag word
/// that safe code is free to compose.
///
/// [`av_opt_find2`] rejects the same flag for the same reason: its normal
/// search cannot hand back an exclusive target handle it was never given.
fn reject_fake_object(search_flags: i32) -> Result<(), OptSetError> {
    if search_flags & ffi::AV_OPT_SEARCH_FAKE_OBJ as i32 == 0 {
        Ok(())
    } else {
        Err(OptSetError::FakeObjectSearch)
    }
}

/// Wraps: av_opt_set
///
/// Sets an option from its string form. `None` is the C API's null value,
/// which is not an absent argument but a request C tests for and acts on:
/// `opt_set_elem` accepts it for exactly the string, pixel-format,
/// sample-format, image-size, duration, color and boolean types and rejects
/// every other type with `EINVAL`. What it means there is per type — a string
/// option releases its allocation and becomes null, an image size becomes
/// `0x0`, a format becomes `NONE`, a duration becomes zero, and a color or
/// boolean is left as it stands.
pub fn av_opt_set(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    value: Option<&CStr>,
    search_flags: i32,
) -> Result<(), OptSetError> {
    reject_fake_object(search_flags)?;
    // SAFETY: the handle carries the live exclusive object borrow, and both
    // `CStr`s remain live for the read-only duration of the call. A null value
    // is a value C accepts and tests for, not a missing pointer.
    result(unsafe {
        ffi::av_opt_set(
            object.as_mut_ptr(),
            name.as_ptr(),
            value.map_or(core::ptr::null(), CStr::as_ptr),
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
    reject_fake_object(search_flags)?;
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
    reject_fake_object(search_flags)?;
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
    reject_fake_object(search_flags)?;
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
    reject_fake_object(search_flags)?;
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
    /// Field: AVOptionArrayDef.def
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

    /// Field: AVOptionArrayDef.size_min
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

    /// Field: AVOptionArrayDef.size_max
    ///
    /// Returns the maximum number of array elements. Zero means unlimited.
    #[must_use]
    pub fn size_max(&self) -> c_uint {
        // SAFETY: the handle points to a live initialized definition. The raw
        // field projection and copy do not form a reference to the C object.
        unsafe { addr_of!((*self.as_ptr()).size_max).read() }
    }

    /// Field: AVOptionArrayDef.sep
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
    /// ABI-compatible view of one entry in an AVClass option table: the name of
    /// a settable field, the C type it holds, its byte offset in the object,
    /// and its default value. Entries and the metadata they point at are
    /// immutable and have no lifecycle operation — nothing in the tree
    /// allocates, frees, clones or writes one, and every option table is a
    /// `static const` array built from string literals. The final entry has a
    /// null name, which is where `av_opt_next` stops iterating.
    ///
    /// # Invariant
    ///
    /// The unsafe `from_ptr` constructors promise only that the `AVOption`
    /// itself is live, initialized and outlives `'a`. They say nothing about
    /// the three string pointers or the untagged union inside it, so a handle
    /// asserts the rest, and every safe getter below relies on it:
    ///
    /// - `name`, `help` and `unit` are each null or address a NUL-terminated
    ///   string that outlives the borrow;
    /// - `default_val` is initialized across its whole width, and `type` names
    ///   its active member — the array flag selects `arr`, and otherwise the
    ///   base type selects `i64`, `dbl` or `str` exactly as
    ///   `av_opt_set_defaults2` and `read_number` read it in `opt.c`;
    /// - a `str` default is null or a NUL-terminated string outliving the
    ///   borrow, and an `arr` default is null or a live [`AVOptionArrayDef`]
    ///   that also satisfies *that* type's invariant, both outliving it.
    ///
    /// The obligation is closed because only two kinds of producer exist.
    /// `from_ptr` puts it on its caller. [`av_opt_find2`] and
    /// [`av_opt_find2_fake`] discharge it from libavutil's side: they return an
    /// entry of a table reached through the searched object's `AVClass`, which
    /// is static metadata. No safe operation can break it either —
    /// [`AVOptionMut`] deliberately exposes no setter, and
    /// [`AVOption::zeroed`] leaves every pointer null and `type` zero, which
    /// [`default_value`](AVOptionRef::default_value) reports as
    /// [`Unknown`](AVOptionDefault::Unknown) without reading the union at all.
    AVOption,
    AVOptionRef,
    AVOptionMut,
    ffi::AVOption
);

/// The active member of an [`AVOption`]'s default-value union.
pub enum AVOptionDefault<'a> {
    /// Field: AVOption.default_val.arr
    ///
    /// Default metadata for an array option. A null definition selects the
    /// type-specific empty default.
    Array(Option<AVOptionArrayDefRef<'a>>),
    /// Field: AVOption.default_val.i64
    ///
    /// Default for integral, enum-like and named-constant options.
    Integer(i64),
    /// Field: AVOption.default_val.str
    ///
    /// Serialized default for string-parsed option types.
    String(Option<&'a CStr>),
    /// Field: AVOption.default_val.dbl
    ///
    /// Default for floating-point options. Libavutil also stores rational
    /// defaults as a double and converts them with `av_d2q`.
    Double(f64),
    /// A value introduced by a newer libavutil, or otherwise not described by
    /// the current public `AVOptionType` contract. No union member is read.
    Unknown(AVOptionType),
}

impl<'a> AVOptionRef<'a> {
    /// Field: AVOption.type
    #[must_use]
    pub fn option_type(&self) -> AVOptionType {
        // SAFETY: the handle guarantees a live initialized option. The raw
        // projection copies the integer-backed type without forming a Rust
        // reference to the C object or field.
        AVOptionType::from_raw(unsafe { addr_of!((*self.as_ptr()).type_).read() })
    }

    /// Field: AVOption.offset
    ///
    /// Returns the byte offset of the represented value in its AVClass
    /// context. Named constants conventionally return zero.
    #[must_use]
    pub fn offset(&self) -> i32 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the integer field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).offset).read() }
    }

    /// Field: AVOption.flags
    ///
    /// Returns the raw `AV_OPT_FLAG_*` word. It stays an integer because the
    /// C constants are macros rather than an enum and libavutil keeps adding
    /// to them; `av_opt_find2`'s `option_flags` argument takes the same word
    /// and matches an entry only when every requested bit is present.
    #[must_use]
    pub fn flags(&self) -> i32 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the integer field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).flags).read() }
    }

    /// Field: AVOption.name
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
            // SAFETY: `AVOption`'s handle invariant makes a non-null `name`
            // a NUL-terminated string that outlives this borrow; the result
            // is narrowed to `'a`, which the entry itself already outlives.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVOption.max
    #[must_use]
    pub fn max(&self) -> f64 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the floating-point field without a reference.
        unsafe { addr_of!((*self.as_ptr()).max).read() }
    }

    /// Field: AVOption.min
    #[must_use]
    pub fn min(&self) -> f64 {
        // SAFETY: the handle guarantees a live initialized option; raw-place
        // projection copies the floating-point field without a reference.
        unsafe { addr_of!((*self.as_ptr()).min).read() }
    }

    /// Field: AVOption.unit
    #[must_use]
    pub fn unit(&self) -> Option<&'a CStr> {
        // SAFETY: the handle guarantees initialized AVOption metadata. Reading
        // the pointer field does not form a reference to the wrapped object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).unit).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: `AVOption`'s handle invariant makes a non-null `unit`
            // a NUL-terminated string that outlives this borrow; the result
            // is narrowed to `'a`, which the entry itself already outlives.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVOption.help
    #[must_use]
    pub fn help(&self) -> Option<&'a CStr> {
        // SAFETY: the handle guarantees initialized AVOption metadata. Reading
        // the pointer field does not form a reference to the wrapped object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).help).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: `AVOption`'s handle invariant makes a non-null `help`
            // a NUL-terminated string that outlives this borrow; the result
            // is narrowed to `'a`, which the entry itself already outlives.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVOption.default_val
    ///
    /// Reads only the union member selected by [`option_type`](Self::option_type).
    /// Unknown values are preserved without interpreting the union bytes.
    #[must_use]
    pub fn default_value(&self) -> AVOptionDefault<'a> {
        let option_type = self.option_type();
        if option_type.is_array() {
            // SAFETY: `AVOption`'s handle invariant makes `arr` the active
            // union member whenever the array flag is set, and the whole union
            // initialized, so this reads an initialized pointer.
            let pointer = unsafe { addr_of!((*self.as_ptr()).default_val.arr).read() };
            // SAFETY: the same invariant makes a non-null `arr` a live
            // `AVOptionArrayDef` outliving this borrow that itself satisfies
            // `AVOptionArrayDef`'s handle invariant, which is exactly what
            // `from_ptr` requires and what `AVOptionArrayDefRef::def` needs.
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
            // SAFETY: `AVOption`'s handle invariant makes `type` name the
            // active union member, and this is the base-type set for which
            // `opt.c` reads `default_val.i64`. Every bit pattern is a valid
            // `i64`, so no further obligation attaches to the value.
            return AVOptionDefault::Integer(unsafe {
                addr_of!((*self.as_ptr()).default_val.i64_).read()
            });
        }
        if matches!(
            base,
            AVOptionType::DOUBLE | AVOptionType::FLOAT | AVOptionType::RATIONAL
        ) {
            // SAFETY: `AVOption`'s handle invariant makes `type` name the
            // active union member, and this is the base-type set for which
            // `opt.c` reads `default_val.dbl` — including RATIONAL, whose
            // default C converts with `av_d2q`. Every bit pattern is a valid
            // `f64`.
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
            // SAFETY: `AVOption`'s handle invariant makes `type` name the
            // active union member, and this is the base-type set for which
            // `opt.c` reads `default_val.str`, so this reads an initialized
            // pointer rather than reinterpreting another member's bytes.
            let pointer = unsafe { addr_of!((*self.as_ptr()).default_val.str_).read() };
            let value = if pointer.is_null() {
                None
            } else {
                // SAFETY: the same invariant makes a non-null `str` default
                // a NUL-terminated string that outlives this borrow, narrowed
                // here to `'a`.
                Some(unsafe { CStr::from_ptr(pointer) })
            };
            return AVOptionDefault::String(value);
        }
        AVOptionDefault::Unknown(option_type)
    }

    /// Field: AVOption.default_val.q
    ///
    /// Returns the legacy rational view of the default-value union. Nothing in
    /// the tree writes or reads this member — opt.h marks it unused and a
    /// rational default is stored in `dbl` — but the public C layout retains
    /// it, so a caller reading a foreign option table can still look. Unlike
    /// [`default_value`](Self::default_value) this ignores `type`: it is a
    /// reinterpretation of whichever member is active, not a decode.
    ///
    /// It is nonetheless safe, on two counts. Every bit pattern is a valid
    /// `AVRational`, since both fields are plain `int`s, so no value read back
    /// can be invalid — only meaningless. And `AVOption`'s invariant makes the
    /// union initialized across its whole width, which matters because reading
    /// padding would be undefined rather than merely useless; on this ABI no
    /// member is narrower than the union, which
    /// `every_default_union_member_covers_the_whole_union` pins.
    #[must_use]
    pub fn legacy_rational_default(&self) -> AVRationalRef<'a> {
        // SAFETY: `q` addresses the union storage, which `AVOption`'s handle
        // invariant makes initialized across its whole width, and every bit
        // pattern of two `int`s is a valid `AVRational`. The returned shared
        // handle is bounded by `'a`, which the entry outlives.
        unsafe { AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).default_val.q).cast_mut()) }
            .expect("an AVOption union field is never null")
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, offset_of, size_of};

    use super::*;

    #[test]
    fn array_definition_is_layout_compatible_and_accessible() {
        // `AVOptionArrayDef` is `#[repr(transparent)]` over the bindgen
        // struct, so comparing the two sizes cannot fail. Assert the C ABI
        // opt.h actually describes: a pointer, two `unsigned` and a `char`,
        // padded to pointer alignment.
        assert_eq!(size_of::<ffi::AVOptionArrayDef>(), 24);
        assert_eq!(align_of::<ffi::AVOptionArrayDef>(), 8);
        assert_eq!(offset_of!(ffi::AVOptionArrayDef, def), 0);
        assert_eq!(offset_of!(ffi::AVOptionArrayDef, size_min), 8);
        assert_eq!(offset_of!(ffi::AVOptionArrayDef, size_max), 12);
        assert_eq!(offset_of!(ffi::AVOptionArrayDef, sep), 16);
        assert_eq!(size_of::<AVOptionArrayDef>(), size_of::<ffi::AVOptionArrayDef>());

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
        // Transparent over `ffi::AVOptionType`, so compare against the type C
        // gives the enum instead: every enumerator is non-negative, so its
        // underlying type is `unsigned int`.
        assert_eq!(size_of::<AVOptionType>(), size_of::<c_uint>());
        assert_eq!(align_of::<AVOptionType>(), align_of::<c_uint>());
        assert_eq!(AVOptionType::FLAG_ARRAY.as_raw(), 1 << 16);

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
        // `AVOption` is `#[repr(transparent)]` over the bindgen struct, so
        // comparing the two sizes cannot fail. Assert the C ABI opt.h
        // describes: two pointers, an `int` and the enum packed into one
        // eight-byte slot, the eight-byte default union, two doubles, and a
        // trailing `int` padded out before the last pointer.
        assert_eq!(size_of::<ffi::AVOption>(), 64);
        assert_eq!(align_of::<ffi::AVOption>(), 8);
        assert_eq!(offset_of!(ffi::AVOption, name), 0);
        assert_eq!(offset_of!(ffi::AVOption, help), 8);
        assert_eq!(offset_of!(ffi::AVOption, offset), 16);
        assert_eq!(offset_of!(ffi::AVOption, type_), 20);
        assert_eq!(offset_of!(ffi::AVOption, default_val), 24);
        assert_eq!(offset_of!(ffi::AVOption, min), 32);
        assert_eq!(offset_of!(ffi::AVOption, max), 40);
        assert_eq!(offset_of!(ffi::AVOption, flags), 48);
        assert_eq!(offset_of!(ffi::AVOption, unit), 56);
        assert_eq!(size_of::<AVOption>(), size_of::<ffi::AVOption>());

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
    fn every_default_union_member_covers_the_whole_union() {
        // What `legacy_rational_default` needs beyond validity: `AVOption`'s
        // invariant says the union is initialized, and reading it back as two
        // `int`s is only defined because no member is narrower than the union
        // itself. A narrower one would leave trailing bytes untouched by a C
        // initializer, and reading those would be undefined rather than merely
        // meaningless. Pin the ABI that makes the difference.
        assert_eq!(size_of::<ffi::AVOption__bindgen_ty_1>(), 8);
        assert_eq!(align_of::<ffi::AVOption__bindgen_ty_1>(), 8);
        assert_eq!(size_of::<i64>(), 8);
        assert_eq!(size_of::<f64>(), 8);
        assert_eq!(size_of::<*const c_char>(), 8);
        assert_eq!(size_of::<*const ffi::AVOptionArrayDef>(), 8);
        assert_eq!(size_of::<ffi::AVRational>(), 8);
    }

    #[test]
    fn a_zeroed_option_reads_no_pointer_field() {
        // The other half of the invariant's closure argument: `zeroed` is the
        // one safe constructor, so it must not be able to produce a handle
        // whose getters dereference anything. Every string is null and the
        // zero type is not a base type, so the union is never decoded.
        let mut option = AVOption::zeroed();
        // SAFETY: `AVOption` is `#[repr(transparent)]` over the C struct, the
        // local is live and initialized by `zeroed`, and this test holds no
        // other handle to it.
        let handle =
            unsafe { AVOptionMut::from_ptr(addr_of_mut!(option).cast::<ffi::AVOption>()) }.unwrap();

        let shared = handle.as_ref();
        assert_eq!(shared.name(), None);
        assert_eq!(shared.help(), None);
        assert_eq!(shared.unit(), None);
        assert_eq!(shared.offset(), 0);
        assert_eq!(shared.flags(), 0);
        assert!(!shared.option_type().is_array());
        assert!(matches!(
            shared.default_value(),
            AVOptionDefault::Unknown(kind) if kind == AVOptionType::from_raw(0)
        ));
        // The union view is still readable, and reads back the zeros.
        let legacy = shared.legacy_rational_default();
        assert_eq!((legacy.num(), legacy.den()), (0, 0));
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
///
/// Inserts or replaces a run of array elements. `search_flags` also carries
/// `AV_OPT_ARRAY_REPLACE`, which selects overwrite instead of insert;
/// `AV_OPT_SEARCH_FAKE_OBJ` is refused before the call, as it is by every
/// other setter, for the reason [`OptSetError::FakeObjectSearch`] gives.
pub fn av_opt_set_array(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    search_flags: i32,
    start_element: u32,
    values: OptArrayValues<'_>,
) -> Result<(), OptSetError> {
    reject_fake_object(search_flags)?;
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
    reject_fake_object(search_flags)?;
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
///
/// Applies every entry of `options` to the object as an ordinary string
/// option, and leaves `options` holding the entries no option matched.
///
/// The slot is an in/out parameter that C both frees and rewrites, so the
/// wrapper hands the dictionary over by value: `options` is emptied before the
/// call and re-adopts whatever C left. On success that is a fresh dictionary
/// of the unmatched entries — or `None` when every entry was consumed, since C
/// frees the original. On failure C frees the partial leftovers and leaves the
/// original in the slot untouched, so the caller gets its dictionary back
/// rather than losing it to the error path. Either way exactly one owner
/// exists at every point, which is why the re-adoption is unconditional.
///
/// C's own `search_flags` are fixed at zero by this entry point, so there is
/// no fake-object flag for it to refuse.
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
    reject_fake_object(search_flags)?;
    // SAFETY: the object borrow and C string are live for the call; the value
    // is an ABI-compatible open pixel-format integer. C range-checks it
    // against the option's own bounds and `AV_PIX_FMT_NB` before storing.
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
    reject_fake_object(search_flags)?;
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
    reject_fake_object(search_flags)?;
    // SAFETY: the object borrow and C string are live for the call; the value
    // is an ABI-compatible open sample-format integer. C range-checks it
    // against the option's own bounds and `AV_SAMPLE_FMT_NB` before storing.
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
    reject_fake_object(search_flags)?;
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
    use core::mem::offset_of;

    use super::*;

    /// An object whose first field is a class pointer — the only shape
    /// `av_opt_find2` accepts — with one `int` per non-constant option.
    #[repr(C)]
    struct SearchObject {
        class: *const ffi::AVClass,
        first: i32,
        second: i32,
        flavour: i32,
    }

    /// opt.h numbers the enumerators densely from 1, so `AV_OPT_TYPE_INT` is 2
    /// and `AV_OPT_TYPE_CONST` is 11. The table below writes the literals
    /// rather than the bindings on purpose: `av_opt_find2` matches a named
    /// constant only when C agrees the entry's type *is* CONST, so the search
    /// result is evidence about the numbering, which comparing an
    /// `AVOptionType` constant against the binding that defines it is not.
    const TYPE_INT: ffi::AVOptionType = 2;
    const TYPE_CONST: ffi::AVOptionType = 11;

    fn options() -> [ffi::AVOption; 5] {
        let plain = |name: &'static CStr, offset: usize, flags: i32| ffi::AVOption {
            name: name.as_ptr(),
            help: core::ptr::null(),
            offset: i32::try_from(offset).expect("field offsets are small"),
            type_: TYPE_INT,
            default_val: ffi::AVOption__bindgen_ty_1 { i64_: 0 },
            min: 0.0,
            max: 255.0,
            flags,
            unit: core::ptr::null(),
        };
        [
            plain(c"first", offset_of!(SearchObject, first), 0),
            plain(c"second", offset_of!(SearchObject, second), 1 | 4),
            ffi::AVOption {
                unit: c"flavour".as_ptr(),
                ..plain(c"flavour", offset_of!(SearchObject, flavour), 0)
            },
            // A named constant: offset 0, its value in the `i64` union member,
            // and the unit that ties it to the option above.
            ffi::AVOption {
                name: c"spicy".as_ptr(),
                help: c"the hot one".as_ptr(),
                offset: 0,
                type_: TYPE_CONST,
                default_val: ffi::AVOption__bindgen_ty_1 { i64_: 7 },
                min: -1.0,
                max: 64.0,
                flags: 2,
                unit: c"flavour".as_ptr(),
            },
            // `av_opt_next` stops at the first entry with a NULL name.
            ffi::AVOption {
                name: core::ptr::null(),
                ..plain(c"", 0, 0)
            },
        ]
    }

    fn class(options: &[ffi::AVOption; 5]) -> ffi::AVClass {
        ffi::AVClass {
            class_name: c"crustify-find-test".as_ptr(),
            item_name: None,
            option: options.as_ptr(),
            version: 0,
            log_level_offset_offset: 0,
            parent_log_context_offset: 0,
            category: 0,
            get_category: None,
            query_ranges: None,
            child_next: None,
            child_class_iterate: None,
            state_flags_offset: 0,
        }
    }

    #[test]
    fn a_found_entry_reads_back_what_c_matched() {
        // The round trip the wrapper's field getters need and a Rust-only test
        // cannot give: C walks the table with its own `sizeof(AVOption)` and
        // its own `name`/`type`/`unit`/`flags` offsets, and every field read
        // back here comes out of the entry C stopped at. A stride or offset
        // Rust disagreed with would surface as the wrong entry or none at all.
        let options = options();
        let class = class(&options);
        let mut object = SearchObject {
            class: core::ptr::from_ref(&class),
            first: 0,
            second: 0,
            flavour: 0,
        };
        let address = NonNull::from(&mut object).cast::<c_void>();
        // SAFETY: `object` is live, initialized, and exclusively borrowed for
        // the rest of this test through this handle alone.
        let mut handle = unsafe { OptionObjectMut::from_raw(address) };

        let mut found = av_opt_find2(&mut handle, c"spicy", Some(c"flavour"), 0, 0)
            .expect("a normal search")
            .expect("the named constant is in the table");
        let option = found.option();
        assert_eq!(option.name(), Some(c"spicy"));
        assert_eq!(option.help(), Some(c"the hot one"));
        assert_eq!(option.unit(), Some(c"flavour"));
        assert_eq!(option.offset(), 0);
        assert_eq!(option.option_type(), AVOptionType::CONST);
        assert!(!option.option_type().is_array());
        assert_eq!(option.min(), -1.0);
        assert_eq!(option.max(), 64.0);
        assert_eq!(option.flags(), 2);
        assert!(matches!(
            option.default_value(),
            AVOptionDefault::Integer(7)
        ));
        // The match also hands back the object the option applies to, which
        // for a normal search is the object that was searched.
        assert_eq!(found.target_mut().as_mut_ptr(), address.as_ptr());
    }

    #[test]
    fn the_unit_argument_selects_constants_by_type() {
        // Both directions of the C rule, and what makes `AVOptionType::CONST`
        // pinned by behaviour: without a unit `av_opt_find2` skips CONST
        // entries, and with one it accepts nothing else.
        let options = options();
        let class = class(&options);
        let mut object = SearchObject {
            class: core::ptr::from_ref(&class),
            first: 0,
            second: 0,
            flavour: 0,
        };
        // SAFETY: `object` is live and exclusively borrowed through this
        // handle for the remainder of the test.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        assert!(
            av_opt_find2(&mut handle, c"spicy", None, 0, 0)
                .expect("a normal search")
                .is_none(),
            "a named constant is not reachable without its unit"
        );
        assert!(
            av_opt_find2(&mut handle, c"flavour", Some(c"flavour"), 0, 0)
                .expect("a normal search")
                .is_none(),
            "a unit search matches only CONST entries"
        );
        assert!(
            av_opt_find2(&mut handle, c"flavour", None, 0, 0)
                .expect("a normal search")
                .is_some()
        );

        // `option_flags` is matched against the entry's own flag word, so it
        // reads the field `AVOptionRef::flags` reports.
        {
            let matched = av_opt_find2(&mut handle, c"second", None, 4, 0)
                .expect("a normal search")
                .expect("the entry carries the requested flag");
            assert_eq!(matched.option().flags(), 1 | 4);
        }
        assert!(
            av_opt_find2(&mut handle, c"second", None, 8, 0)
                .expect("a normal search")
                .is_none(),
            "a flag the entry lacks excludes it"
        );
    }

    #[test]
    fn found_metadata_outlives_the_match_handle() {
        // `AVOptionRef<'a>`'s getters hand back `&'a CStr`, tied to the
        // searched object rather than to the local handle. This would not
        // compile if they were elided to `&self`.
        let options = options();
        let class = class(&options);
        let mut object = SearchObject {
            class: core::ptr::from_ref(&class),
            first: 0,
            second: 0,
            flavour: 0,
        };
        // SAFETY: `object` is live and exclusively borrowed through the handle
        // for the rest of the test.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        let help = {
            let found = av_opt_find2(&mut handle, c"spicy", Some(c"flavour"), 0, 0)
                .expect("a normal search")
                .expect("the named constant is in the table");
            found.option().help()
        };
        assert_eq!(help, Some(c"the hot one"));
    }

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
///
/// The option field is left owning an independent deep copy: C runs
/// `av_channel_layout_copy`, which disposes whatever the field held before
/// installing the new value, so setting twice frees the first copy rather than
/// leaking it.
pub fn av_opt_set_chlayout(
    object: &mut OptionObjectMut<'_>,
    name: &CStr,
    layout: AVChannelLayoutRef<'_>,
    search_flags: i32,
) -> Result<(), OptSetError> {
    reject_fake_object(search_flags)?;
    // SAFETY: the object is exclusively borrowed, the layout is shared and —
    // by `AVChannelLayout`'s invariant — carries a readable map for every
    // channel it claims, which is what `av_channel_layout_copy` memcpy's from.
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

#[cfg(test)]
mod scheduled_set_tests {
    use core::mem::offset_of;

    use ffibox::{CVec, CrustifyStr};

    use super::*;
    use crate::dict::{Dictionary, av_dict_count, av_dict_get, av_dict_set};
    use crate::mem::AvFree;
    use crate::rational::AVRational;

    /// The smallest thing the option setters can act on: a struct whose first
    /// field is a class pointer, with one field per scheduled setter behind
    /// an option that names its offset. libavutil publishes no such object of
    /// its own — every in-tree option table belongs to a library this campaign
    /// does not build — so the target has to be assembled here, which is also
    /// what makes the layout requirements explicit.
    #[repr(C)]
    struct TestObject {
        class: *const ffi::AVClass,
        integer: i64,
        number: f64,
        /// `AV_OPT_TYPE_IMAGE_SIZE` writes `dst[0]` and `dst[1]`, so the two
        /// halves must be adjacent `int`s in this order.
        width: i32,
        height: i32,
        text: *mut c_char,
        /// `AV_OPT_TYPE_BINARY` writes the pointer at `dst` and the length at
        /// `dst + 1`, so the count must directly follow the pointer.
        binary: *mut u8,
        binary_len: i32,
        rational: ffi::AVRational,
        video_rate: ffi::AVRational,
        pixel_fmt: ffi::AVPixelFormat,
        sample_fmt: ffi::AVSampleFormat,
        /// An `AV_OPT_TYPE_FLAG_ARRAY` option addresses a
        /// `{ void *elements; unsigned count; }` pair: `opt_array_pcount`
        /// reads the count one `void *` past the option's offset, so the two
        /// must be adjacent and in this order.
        int_array: *mut i32,
        int_array_count: c_uint,
    }

    fn option(
        name: &'static CStr,
        offset: usize,
        option_type: ffi::AVOptionType,
        min: f64,
        max: f64,
    ) -> ffi::AVOption {
        ffi::AVOption {
            name: name.as_ptr(),
            help: core::ptr::null(),
            offset: i32::try_from(offset).expect("field offsets are small"),
            type_: option_type,
            default_val: ffi::AVOption__bindgen_ty_1 { i64_: 0 },
            min,
            max,
            flags: 0,
            unit: core::ptr::null(),
        }
    }

    fn options() -> [ffi::AVOption; 11] {
        [
            option(
                c"integer",
                offset_of!(TestObject, integer),
                ffi::AVOptionType_AV_OPT_TYPE_INT64,
                0.0,
                1000.0,
            ),
            option(
                c"number",
                offset_of!(TestObject, number),
                ffi::AVOptionType_AV_OPT_TYPE_DOUBLE,
                -1000.0,
                1000.0,
            ),
            option(
                c"size",
                offset_of!(TestObject, width),
                ffi::AVOptionType_AV_OPT_TYPE_IMAGE_SIZE,
                0.0,
                0.0,
            ),
            option(
                c"text",
                offset_of!(TestObject, text),
                ffi::AVOptionType_AV_OPT_TYPE_STRING,
                0.0,
                0.0,
            ),
            option(
                c"binary",
                offset_of!(TestObject, binary),
                ffi::AVOptionType_AV_OPT_TYPE_BINARY,
                0.0,
                0.0,
            ),
            option(
                c"rational",
                offset_of!(TestObject, rational),
                ffi::AVOptionType_AV_OPT_TYPE_RATIONAL,
                -1000.0,
                1000.0,
            ),
            option(
                c"rate",
                offset_of!(TestObject, video_rate),
                ffi::AVOptionType_AV_OPT_TYPE_VIDEO_RATE,
                0.0,
                1000.0,
            ),
            // `set_format` clamps the accepted range to
            // `[FFMAX(min, -1), FFMIN(max, NB - 1)]`, so these bounds hand it
            // the library's own table extent rather than a second opinion.
            option(
                c"pixel_fmt",
                offset_of!(TestObject, pixel_fmt),
                ffi::AVOptionType_AV_OPT_TYPE_PIXEL_FMT,
                -1.0,
                f64::from(i32::MAX),
            ),
            option(
                c"sample_fmt",
                offset_of!(TestObject, sample_fmt),
                ffi::AVOptionType_AV_OPT_TYPE_SAMPLE_FMT,
                -1.0,
                f64::from(i32::MAX),
            ),
            option(
                c"numbers",
                offset_of!(TestObject, int_array),
                ffi::AVOptionType_AV_OPT_TYPE_INT | ffi::AVOptionType_AV_OPT_TYPE_FLAG_ARRAY,
                0.0,
                1000.0,
            ),
            // `av_opt_next` stops at the first entry whose name is NULL, so
            // the terminator cannot go through `option` — an empty C string
            // is a name, and iteration would run off the end of the array.
            ffi::AVOption {
                name: core::ptr::null(),
                ..option(c"", 0, ffi::AVOptionType_AV_OPT_TYPE_INT64, 0.0, 0.0)
            },
        ]
    }

    fn class(options: &[ffi::AVOption; 11]) -> ffi::AVClass {
        ffi::AVClass {
            class_name: c"crustify-test".as_ptr(),
            // NULL is the documented default: libavutil substitutes
            // `av_default_item_name` when it needs a name for a log line.
            item_name: None,
            option: options.as_ptr(),
            version: 0,
            log_level_offset_offset: 0,
            parent_log_context_offset: 0,
            category: 0,
            get_category: None,
            query_ranges: None,
            child_next: None,
            child_class_iterate: None,
            state_flags_offset: 0,
        }
    }

    impl TestObject {
        fn new(class: &ffi::AVClass) -> Self {
            Self {
                class: core::ptr::from_ref(class),
                integer: 0,
                number: 0.0,
                width: 0,
                height: 0,
                text: core::ptr::null_mut(),
                binary: core::ptr::null_mut(),
                binary_len: 0,
                rational: ffi::AVRational { num: 0, den: 0 },
                video_rate: ffi::AVRational { num: 0, den: 0 },
                pixel_fmt: ffi::AVPixelFormat_AV_PIX_FMT_NONE,
                sample_fmt: ffi::AVSampleFormat_AV_SAMPLE_FMT_NONE,
                int_array: core::ptr::null_mut(),
                int_array_count: 0,
            }
        }

        /// Releases the two option values C allocated into this object, which
        /// `av_opt_free` would do for a real one. Leaving them behind would
        /// show up as a leak in the sanitiser run.
        fn release_owned_options(&mut self) {
            if !self.text.is_null() {
                // SAFETY: `set_string` filled this field with an `av_strdup`
                // result — a uniquely owned NUL-terminated av_malloc-family
                // string, which is exactly what `CrustifyStr<AvFree>` adopts
                // and frees. The field is emptied so no second owner forms.
                drop(unsafe { CrustifyStr::<AvFree>::from_raw(self.text) });
                self.text = core::ptr::null_mut();
            }
            if !self.binary.is_null() {
                let count = usize::try_from(self.binary_len).expect("a non-negative length");
                // SAFETY: `av_opt_set_bin` filled this field with an
                // `av_malloc` allocation holding exactly `binary_len`
                // initialized bytes copied from the caller's slice, uniquely
                // owned and released by `av_free`.
                drop(unsafe { CVec::<u8, AvFree>::from_raw_parts(self.binary, count) });
                self.binary = core::ptr::null_mut();
                self.binary_len = 0;
            }
            if !self.int_array.is_null() {
                let count = usize::try_from(self.int_array_count).expect("a slot count");
                // SAFETY: `av_opt_set_array` filled this field with an
                // `av_calloc` block of exactly `int_array_count` initialized
                // `int`s — it writes every element it counts — uniquely owned
                // and released by `av_free`.
                drop(unsafe { CVec::<i32, AvFree>::from_raw_parts(self.int_array, count) });
                self.int_array = core::ptr::null_mut();
                self.int_array_count = 0;
            }
        }
    }

    #[test]
    fn setters_write_through_to_the_object() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: `object` is a live, initialized struct whose first field
            // is a class pointer, and this handle is the only access to it for
            // the block below.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

            av_opt_set_int(&mut handle, c"integer", 7, 0).expect("set integer");
            av_opt_set_double(&mut handle, c"number", 0.5, 0).expect("set number");
            av_opt_set_image_size(&mut handle, c"size", 640, 480, 0).expect("set size");
            av_opt_set(&mut handle, c"text", Some(c"crustify"), 0).expect("set text");
            av_opt_set_bin(&mut handle, c"binary", &[1, 2, 3], 0).expect("set binary");

            // A string setter reaches the numeric option too: C parses it with
            // the same code path `av_opt_set_int` writes through.
            av_opt_set(&mut handle, c"integer", Some(c"42"), 0).expect("parse integer");
        }

        assert_eq!(object.integer, 42);
        assert!((object.number - 0.5).abs() < f64::EPSILON);
        assert_eq!((object.width, object.height), (640, 480));
        assert_eq!(object.binary_len, 3);
        // SAFETY: `av_opt_set_bin` wrote three initialized bytes at this
        // pointer, and nothing has freed or replaced them.
        assert_eq!(unsafe { core::slice::from_raw_parts(object.binary, 3) }, [
            1, 2, 3
        ]);
        // SAFETY: the string option holds an `av_strdup` result that is still
        // live and NUL-terminated.
        assert_eq!(unsafe { CStr::from_ptr(object.text) }, c"crustify");

        object.release_owned_options();
    }

    #[test]
    fn a_null_value_clears_a_string_option() {
        // The contract a `&CStr` argument would hide: NULL is a value C tests
        // for, not an absent argument, and for a string option it means
        // "release what is there and store NULL".
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: as above — a live object exclusively borrowed for the
            // duration of this block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set(&mut handle, c"text", Some(c"crustify"), 0).expect("set text");
            assert!(!object_text_is_null(&handle));
            av_opt_set(&mut handle, c"text", None, 0).expect("clear text");
        }

        assert!(object.text.is_null(), "the old allocation was released");
        object.release_owned_options();
    }

    #[test]
    fn a_null_value_is_accepted_or_refused_by_option_type() {
        // The other half of the null contract, and the reason the wrapper does
        // not simply forward `None` everywhere: C admits it for one fixed set
        // of types and rejects the rest. An image size takes it and becomes
        // 0x0; a binary or numeric option refuses it with EINVAL.
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

            av_opt_set_image_size(&mut handle, c"size", 640, 480, 0).expect("set size");
            av_opt_set(&mut handle, c"size", None, 0).expect("clear size");

            assert!(matches!(
                av_opt_set(&mut handle, c"binary", None, 0),
                Err(OptSetError::Library(_))
            ));
            assert!(matches!(
                av_opt_set(&mut handle, c"integer", None, 0),
                Err(OptSetError::Library(_))
            ));
        }

        assert_eq!((object.width, object.height), (0, 0));
        assert!(object.binary.is_null());
        assert_eq!(object.integer, 0);
    }

    /// Reads the string field back through the borrowed handle rather than
    /// through `object`, so the exclusive borrow is not interrupted.
    fn object_text_is_null(handle: &OptionObjectMut<'_>) -> bool {
        // SAFETY: the handle addresses a live `TestObject`, and this reads one
        // initialized pointer field out of it without forming a reference.
        unsafe {
            addr_of!((*handle.pointer.as_ptr().cast::<TestObject>()).text)
                .read()
                .is_null()
        }
    }

    #[test]
    fn every_setter_rejects_a_fake_object_search() {
        // With `AV_OPT_SEARCH_FAKE_OBJ` set, C resolves the option, finds no
        // target object — that is what the flag means — and then loads the
        // class out of that NULL target, which the sanitiser build reports as
        // a SEGV inside `opt_set_init`. The flag can never do useful work in a
        // setter, since a fake object has no storage to write to, so each one
        // refuses it before the call.
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);
        // SAFETY: `object` is live and exclusively borrowed for the block.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        let fake = ffi::AV_OPT_SEARCH_FAKE_OBJ as i32;
        assert_eq!(
            av_opt_set(&mut handle, c"integer", Some(c"1"), fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_int(&mut handle, c"integer", 1, fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_double(&mut handle, c"number", 1.0, fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_image_size(&mut handle, c"size", 1, 1, fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_bin(&mut handle, c"binary", &[0], fake),
            Err(OptSetError::FakeObjectSearch)
        );

        assert_eq!(object.integer, 0, "no setter reached the object");
    }

    #[test]
    fn setters_report_library_errors_without_writing() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);
        // SAFETY: `object` is live and exclusively borrowed for the block.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        // An unknown name, a value outside the option's range, a negative
        // image size and a type mismatch are all ordinary C failures, and each
        // arrives as the negative code C returned.
        assert!(matches!(
            av_opt_set_int(&mut handle, c"missing", 1, 0),
            Err(OptSetError::Library(_))
        ));
        assert!(matches!(
            av_opt_set_int(&mut handle, c"integer", 100_000, 0),
            Err(OptSetError::Library(_))
        ));
        assert!(matches!(
            av_opt_set_image_size(&mut handle, c"size", -1, 1, 0),
            Err(OptSetError::Library(_))
        ));
        assert!(matches!(
            av_opt_set_bin(&mut handle, c"integer", &[0], 0),
            Err(OptSetError::Library(_))
        ));

        assert_eq!(object.integer, 0);
        assert_eq!((object.width, object.height), (0, 0));
    }

    #[test]
    fn an_empty_binary_value_is_stored_as_no_allocation() {
        // C reads the value pointer only when the length is nonzero, and
        // stores NULL for an empty one. An empty Rust slice is a dangling
        // pointer, so this pins that C never dereferences it.
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set_bin(&mut handle, c"binary", &[], 0).expect("set empty binary");
        }

        assert!(object.binary.is_null());
        assert_eq!(object.binary_len, 0);
    }

    #[test]
    fn typed_setters_write_through_to_the_object() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

            let quarter = AVRational::new(1, 4);
            av_opt_set_q(&mut handle, c"rational", quarter.as_ref(), 0).expect("set rational");
            let rate = AVRational::new(30, 1);
            av_opt_set_video_rate(&mut handle, c"rate", rate.as_ref(), 0).expect("set rate");
            av_opt_set_pixel_fmt(&mut handle, c"pixel_fmt", AVPixelFormat::RGB24, 0)
                .expect("set pixel format");
            av_opt_set_sample_fmt(&mut handle, c"sample_fmt", AVSampleFormat::S16P, 0)
                .expect("set sample format");
        }

        assert_eq!((object.rational.num, object.rational.den), (1, 4));
        assert_eq!((object.video_rate.num, object.video_rate.den), (30, 1));
        assert_eq!(object.pixel_fmt, AVPixelFormat::RGB24.as_raw());
        assert_eq!(object.sample_fmt, AVSampleFormat::S16P.as_raw());

        object.release_owned_options();
    }

    #[test]
    fn typed_setters_report_the_ranges_c_enforces() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);
        // SAFETY: `object` is live and exclusively borrowed for the block.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        // `set_format` clamps the option's own maximum to `NB - 1`, so a
        // format integer past the table this build knows is refused rather
        // than stored and later used to index it. `AVPixelFormat` and
        // `AVSampleFormat` are open integers precisely because a newer
        // libavutil may return one, so this bound is C's to enforce.
        assert!(matches!(
            av_opt_set_pixel_fmt(
                &mut handle,
                c"pixel_fmt",
                AVPixelFormat::from_raw(i32::MAX),
                0
            ),
            Err(OptSetError::Library(_))
        ));
        assert!(matches!(
            av_opt_set_sample_fmt(
                &mut handle,
                c"sample_fmt",
                AVSampleFormat::from_raw(i32::MAX),
                0
            ),
            Err(OptSetError::Library(_))
        ));

        // A rational outside `[min, max]` is an ordinary ERANGE, and a
        // mismatched option type an ordinary EINVAL.
        let large = AVRational::new(5000, 1);
        assert!(matches!(
            av_opt_set_q(&mut handle, c"rational", large.as_ref(), 0),
            Err(OptSetError::Library(_))
        ));
        let rate = AVRational::new(30, 1);
        assert!(matches!(
            av_opt_set_video_rate(&mut handle, c"rational", rate.as_ref(), 0),
            Err(OptSetError::Library(_))
        ));

        assert_eq!(object.pixel_fmt, ffi::AVPixelFormat_AV_PIX_FMT_NONE);
        assert_eq!(object.sample_fmt, ffi::AVSampleFormat_AV_SAMPLE_FMT_NONE);
        assert_eq!((object.rational.num, object.rational.den), (0, 0));
    }

    #[test]
    fn an_array_option_grows_and_shrinks_through_its_own_count() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

            av_opt_set_array(
                &mut handle,
                c"numbers",
                0,
                0,
                OptArrayValues::Int(&[1, 2, 3]),
            )
            .expect("insert three elements");
            // A second insert at the end appends rather than replacing.
            av_opt_set_array(&mut handle, c"numbers", 0, 3, OptArrayValues::Int(&[4]))
                .expect("append one element");

            // The element type need not match the option's: C converts an
            // `int64_t` source into the `int` the option stores.
            av_opt_set_array(&mut handle, c"numbers", 0, 4, OptArrayValues::Int64(&[5]))
                .expect("append a widened element");

            // A start element past the current count is refused by C, which is
            // what keeps the write inside the array it just sized.
            assert!(matches!(
                av_opt_set_array(&mut handle, c"numbers", 0, 99, OptArrayValues::Int(&[0])),
                Err(OptSetError::Library(_))
            ));
            // And so is a value outside the option's declared range.
            assert!(matches!(
                av_opt_set_array(&mut handle, c"numbers", 0, 5, OptArrayValues::Int(&[9999])),
                Err(OptSetError::Library(_))
            ));
        }

        assert_eq!(object.int_array_count, 5);
        // SAFETY: `av_opt_set_array` wrote five initialized `int`s at this
        // pointer and the count above is the one it recorded for them.
        let stored = unsafe { core::slice::from_raw_parts(object.int_array, 5) };
        assert_eq!(stored, [1, 2, 3, 4, 5]);

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_remove_array(&mut handle, c"numbers", 0, 1, 3).expect("remove three elements");
            // Removing more than the array holds is refused before any free.
            assert!(matches!(
                av_opt_remove_array(&mut handle, c"numbers", 0, 0, 99),
                Err(OptSetError::Library(_))
            ));
        }

        assert_eq!(object.int_array_count, 2);
        // SAFETY: as above, for the two elements that survived the removal.
        let stored = unsafe { core::slice::from_raw_parts(object.int_array, 2) };
        assert_eq!(stored, [1, 5]);

        object.release_owned_options();
    }

    #[test]
    fn every_typed_setter_rejects_a_fake_object_search() {
        // The other half of `every_setter_rejects_a_fake_object_search`: these
        // six reach `opt_set_init` by the same route and load the class out of
        // the same NULL target, so they refuse the flag on the same terms.
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);
        // SAFETY: `object` is live and exclusively borrowed for the block.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        let fake = ffi::AV_OPT_SEARCH_FAKE_OBJ as i32;
        let value = AVRational::new(1, 4);
        assert_eq!(
            av_opt_set_q(&mut handle, c"rational", value.as_ref(), fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_video_rate(&mut handle, c"rate", value.as_ref(), fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_pixel_fmt(&mut handle, c"pixel_fmt", AVPixelFormat::RGB24, fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_sample_fmt(&mut handle, c"sample_fmt", AVSampleFormat::S16P, fake),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_set_array(&mut handle, c"numbers", fake, 0, OptArrayValues::Int(&[1])),
            Err(OptSetError::FakeObjectSearch)
        );
        assert_eq!(
            av_opt_remove_array(&mut handle, c"numbers", fake, 0, 1),
            Err(OptSetError::FakeObjectSearch)
        );

        assert_eq!((object.rational.num, object.rational.den), (0, 0));
        assert!(object.int_array.is_null(), "no setter reached the object");
    }

    #[test]
    fn a_dictionary_is_consumed_and_its_unmatched_entries_come_back() {
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        let mut input = Dictionary::default();
        av_dict_set(&mut input, c"integer", Some(c"11"), 0).expect("set integer entry");
        av_dict_set(&mut input, c"text", Some(c"from-dict"), 0).expect("set text entry");
        av_dict_set(&mut input, c"unknown", Some(c"kept"), 0).expect("set unmatched entry");
        let mut owner = input.into_owner();
        assert!(owner.is_some());

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set_dict(&mut handle, &mut owner).expect("apply the dictionary");
        }

        assert_eq!(object.integer, 11);
        // SAFETY: the string option holds a live `av_strdup` result.
        assert_eq!(unsafe { CStr::from_ptr(object.text) }, c"from-dict");

        // C freed the dictionary it was handed and left a fresh one holding
        // only the entry no option matched. Re-adopting it is what keeps that
        // second allocation from leaking, which the sanitiser run would show.
        let leftovers = Dictionary::from_owner(owner);
        assert_eq!(av_dict_count(leftovers.as_ref()), 1);
        assert_eq!(
            av_dict_get(leftovers.as_ref(), c"unknown", None, 0)
                .expect("a well-formed lookup")
                .expect("the unmatched entry")
                .value(),
            c"kept"
        );

        object.release_owned_options();
    }

    #[test]
    fn a_failing_dictionary_entry_leaves_the_dictionary_with_its_caller() {
        // C's error path frees only the partial leftovers and returns without
        // touching the caller's slot, so the wrapper's unconditional
        // re-adoption hands the original dictionary back rather than leaking
        // it or leaving the caller with nothing.
        let options = options();
        let class = class(&options);
        let mut object = TestObject::new(&class);

        let mut input = Dictionary::default();
        av_dict_set(&mut input, c"integer", Some(c"99999"), 0).expect("set out-of-range entry");
        av_dict_set(&mut input, c"text", Some(c"unreached"), 0).expect("set text entry");
        let mut owner = input.into_owner();

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            assert!(matches!(
                av_opt_set_dict(&mut handle, &mut owner),
                Err(OptSetError::Library(_))
            ));
        }

        let returned = Dictionary::from_owner(owner);
        assert_eq!(av_dict_count(returned.as_ref()), 2);
        assert_eq!(object.integer, 0);
        assert!(object.text.is_null());

        object.release_owned_options();
    }
}

#[cfg(test)]
mod scheduled_chlayout_tests {
    use core::mem::offset_of;

    use ffibox::CVal;

    use super::*;
    use crate::channel_layout::{
        AVChannelLayout, AVChannelOrder, av_channel_layout_compare, av_channel_layout_from_string,
    };

    /// An option object carrying a single `AV_OPT_TYPE_CHLAYOUT` field.
    ///
    /// It is its own object rather than another field on `scheduled_set_tests`'
    /// `TestObject` because this option type owns storage: C deep-copies the
    /// caller's layout into the field, and something has to release it the way
    /// `av_opt_free` would. `CVal<AVChannelLayout>` is `#[repr(transparent)]`
    /// down to `ffi::AVChannelLayout`, so the field is exactly the layout C
    /// writes at `offset_of!(.., ch_layout)` and dropping the object disposes
    /// it — a leak here would show up in the campaign's LSan run.
    #[repr(C)]
    struct ChLayoutObject {
        class: *const ffi::AVClass,
        ch_layout: CVal<AVChannelLayout>,
    }

    fn options() -> [ffi::AVOption; 2] {
        [
            ffi::AVOption {
                name: c"chlayout".as_ptr(),
                help: core::ptr::null(),
                offset: i32::try_from(offset_of!(ChLayoutObject, ch_layout))
                    .expect("field offsets are small"),
                type_: ffi::AVOptionType_AV_OPT_TYPE_CHLAYOUT,
                default_val: ffi::AVOption__bindgen_ty_1 { i64_: 0 },
                min: 0.0,
                max: 0.0,
                flags: 0,
                unit: core::ptr::null(),
            },
            // `av_opt_next` stops at the first NULL name, so the terminator
            // cannot carry one.
            ffi::AVOption {
                name: core::ptr::null(),
                help: core::ptr::null(),
                offset: 0,
                type_: ffi::AVOptionType_AV_OPT_TYPE_CHLAYOUT,
                default_val: ffi::AVOption__bindgen_ty_1 { i64_: 0 },
                min: 0.0,
                max: 0.0,
                flags: 0,
                unit: core::ptr::null(),
            },
        ]
    }

    fn class(options: &[ffi::AVOption; 2]) -> ffi::AVClass {
        ffi::AVClass {
            class_name: c"crustify-chlayout-test".as_ptr(),
            item_name: None,
            option: options.as_ptr(),
            version: 0,
            log_level_offset_offset: 0,
            parent_log_context_offset: 0,
            category: 0,
            get_category: None,
            query_ranges: None,
            child_next: None,
            child_class_iterate: None,
            state_flags_offset: 0,
        }
    }

    fn object(class: &ffi::AVClass) -> ChLayoutObject {
        ChLayoutObject {
            class: core::ptr::from_ref(class),
            ch_layout: CVal::new(AVChannelLayout::zeroed()),
        }
    }

    #[test]
    fn a_layout_is_deep_copied_into_the_option_field() {
        let options = options();
        let class = class(&options);
        let mut object = object(&class);

        let stereo = av_channel_layout_from_string(c"stereo").expect("a named layout");
        {
            // SAFETY: `object` is live, initialized and exclusively borrowed
            // through this handle for the block; its first field is the class
            // pointer C reads.
            let mut handle =
                unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set_chlayout(&mut handle, c"chlayout", stereo.as_ref(), 0)
                .expect("set the channel layout");
        }

        // The field holds an independently disposable copy, not the caller's.
        assert_eq!(object.ch_layout.as_ref().order(), AVChannelOrder::NATIVE);
        assert_eq!(object.ch_layout.as_ref().nb_channels(), 2);
        assert_eq!(
            av_channel_layout_compare(stereo.as_ref(), object.ch_layout.as_ref()),
            Ok(true)
        );

        // Setting a second time makes C dispose the first copy before storing
        // the next one, so exactly one owner exists at every point.
        let mono = av_channel_layout_from_string(c"mono").expect("a named layout");
        {
            // SAFETY: as above.
            let mut handle =
                unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set_chlayout(&mut handle, c"chlayout", mono.as_ref(), 0)
                .expect("replace the channel layout");
        }
        assert_eq!(object.ch_layout.as_ref().nb_channels(), 1);
    }

    #[test]
    fn a_custom_layout_copy_is_owned_by_the_option_field() {
        // The only layout shape whose copy allocates: the option field ends up
        // owning a second map, which the object's `CVal` has to free.
        let options = options();
        let class = class(&options);
        let mut object = object(&class);

        let custom = av_channel_layout_from_string(c"FL@head+FR@tail").expect("a custom layout");
        assert_eq!(custom.as_ref().order(), AVChannelOrder::CUSTOM);
        {
            // SAFETY: as above.
            let mut handle =
                unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            av_opt_set_chlayout(&mut handle, c"chlayout", custom.as_ref(), 0)
                .expect("set the custom layout");
        }

        let stored = object
            .ch_layout
            .as_ref()
            .custom_map()
            .expect("a custom map");
        let source = custom.as_ref().custom_map().expect("a custom map");
        assert_eq!(stored.len(), 2);
        assert_ne!(
            stored.get(0).unwrap().as_ptr(),
            source.get(0).unwrap().as_ptr()
        );
        assert_eq!(stored.get(1).unwrap().id(), source.get(1).unwrap().id());
    }

    #[test]
    fn the_chlayout_setter_rejects_a_fake_object_search() {
        // The twelfth setter, on the same terms as its eleven siblings:
        // `opt_set_init` resolves the option, gets no target object back —
        // that is what the flag means — and then loads the class out of that
        // NULL, which the sanitiser build reports as a SEGV in `opt.c`.
        let options = options();
        let class = class(&options);
        let mut object = object(&class);
        let stereo = av_channel_layout_from_string(c"stereo").expect("a named layout");

        {
            // SAFETY: `object` is live and exclusively borrowed for the block.
            let mut handle =
                unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };
            assert_eq!(
                av_opt_set_chlayout(
                    &mut handle,
                    c"chlayout",
                    stereo.as_ref(),
                    ffi::AV_OPT_SEARCH_FAKE_OBJ as i32,
                ),
                Err(OptSetError::FakeObjectSearch)
            );
        }

        assert_eq!(
            object.ch_layout.as_ref().order(),
            AVChannelOrder::UNSPECIFIED,
            "the setter did not reach the object"
        );
    }

    #[test]
    fn an_unknown_name_is_an_ordinary_library_error() {
        let options = options();
        let class = class(&options);
        let mut object = object(&class);
        let stereo = av_channel_layout_from_string(c"stereo").expect("a named layout");
        // SAFETY: `object` is live and exclusively borrowed for the block.
        let mut handle = unsafe { OptionObjectMut::from_raw(NonNull::from(&mut object).cast()) };

        assert!(matches!(
            av_opt_set_chlayout(&mut handle, c"missing", stereo.as_ref(), 0),
            Err(OptSetError::Library(_))
        ));
    }
}

ffibox::define_ctype!(
    /// Wraps: AVOptionRange
    ///
    /// ABI-compatible view of one range returned by an AVClass range query.
    /// The numeric fields live inline. `str` is either null or a uniquely
    /// owned, NUL-terminated allocation from libavutil's allocator family;
    /// `av_opt_freep_ranges` releases it with `av_freep` before freeing the
    /// containing range.
    ///
    /// # Handle invariant
    ///
    /// In addition to being live and initialized, a value used through these
    /// handles must have a null `str` or an `av_malloc`-family string that is
    /// NUL-terminated and owned by this range. A caller of the unsafe
    /// `from_ptr` constructors must establish that invariant. [`zeroed`](Self::zeroed)
    /// and every safe setter below preserve it.
    AVOptionRange,
    AVOptionRangeRef,
    AVOptionRangeMut,
    ffi::AVOptionRange
);

impl<'a> AVOptionRangeRef<'a> {
    /// Field: AVOptionRange.str
    ///
    /// Borrows the optional string for as long as the range is borrowed.
    #[must_use]
    pub fn string(&self) -> Option<&'a CStr> {
        // SAFETY: the handle addresses a live initialized range. Raw-place
        // projection copies the pointer without forming a Rust reference to
        // the C object.
        let pointer = unsafe { addr_of!((*self.as_ptr()).str_).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: the handle invariant makes a non-null pointer a live
            // NUL-terminated string that the range keeps alive for `'a`.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVOptionRange.is_range
    ///
    /// Reports whether the entry describes an interval rather than one value.
    #[must_use]
    pub fn is_range(&self) -> bool {
        // SAFETY: the handle invariant guarantees an initialized range; the
        // raw projection copies the integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).is_range).read() != 0 }
    }

    /// Field: AVOptionRange.component_max
    #[must_use]
    pub fn component_max(&self) -> f64 {
        // SAFETY: the handle guarantees an initialized range and this raw
        // projection copies the field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).component_max).read() }
    }

    /// Field: AVOptionRange.component_min
    #[must_use]
    pub fn component_min(&self) -> f64 {
        // SAFETY: the handle guarantees an initialized range and this raw
        // projection copies the field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).component_min).read() }
    }

    /// Field: AVOptionRange.value_max
    #[must_use]
    pub fn value_max(&self) -> f64 {
        // SAFETY: the handle guarantees an initialized range and this raw
        // projection copies the field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).value_max).read() }
    }

    /// Field: AVOptionRange.value_min
    #[must_use]
    pub fn value_min(&self) -> f64 {
        // SAFETY: the handle guarantees an initialized range and this raw
        // projection copies the field without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).value_min).read() }
    }
}

impl AVOptionRangeMut<'_> {
    /// Replaces the range's optional owned string, dropping the old value.
    pub fn set_string(&mut self, value: Option<CrustifyStr<AvFree>>) {
        let new_pointer = value.map_or(core::ptr::null_mut(), CrustifyStr::into_raw);
        // SAFETY: the exclusive handle permits replacing this pointer field.
        // `new_pointer` is null or transfers one allocator-matched owned,
        // terminated string into the range, preserving the handle invariant.
        let old_pointer =
            unsafe { addr_of_mut!((*self.as_mut_ptr()).str_).replace(new_pointer.cast_const()) };
        // SAFETY: by the incoming handle invariant, the old pointer is null or
        // one uniquely owned av_malloc-family NUL-terminated string. Adoption
        // transfers that ownership out of the range exactly once.
        drop(unsafe { CrustifyStr::<AvFree>::from_raw(old_pointer.cast_mut()) });
    }

    /// Removes and returns the optional owned string, leaving the field null.
    #[must_use]
    pub fn take_string(&mut self) -> Option<CrustifyStr<AvFree>> {
        // SAFETY: the exclusive handle permits replacing the live field. Null
        // preserves the range invariant and makes the ownership transfer
        // explicit to both Rust and any later C disposer.
        let pointer = unsafe { addr_of_mut!((*self.as_mut_ptr()).str_).replace(core::ptr::null()) };
        // SAFETY: the handle invariant makes a non-null value a uniquely owned
        // av_malloc-family NUL-terminated string, now removed from the range.
        unsafe { CrustifyStr::<AvFree>::from_raw(pointer.cast_mut()) }
    }

    /// Selects interval (`true`) or single-value (`false`) encoding.
    pub fn set_is_range(&mut self, value: bool) {
        // SAFETY: the exclusive handle provides field write access and the
        // bool-to-int conversion writes one of the two documented values.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).is_range).write(i32::from(value)) }
    }

    /// Sets the maximum allowed component value.
    pub fn set_component_max(&mut self, value: f64) {
        // SAFETY: the exclusive handle provides field write access; every f64
        // bit pattern is valid and raw projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).component_max).write(value) }
    }

    /// Sets the minimum allowed component value.
    pub fn set_component_min(&mut self, value: f64) {
        // SAFETY: the exclusive handle provides field write access; every f64
        // bit pattern is valid and raw projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).component_min).write(value) }
    }

    /// Sets the maximum allowed value.
    pub fn set_value_max(&mut self, value: f64) {
        // SAFETY: the exclusive handle provides field write access; every f64
        // bit pattern is valid and raw projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).value_max).write(value) }
    }

    /// Sets the minimum allowed value.
    pub fn set_value_min(&mut self, value: f64) {
        // SAFETY: the exclusive handle provides field write access; every f64
        // bit pattern is valid and raw projection forms no reference.
        unsafe { addr_of_mut!((*self.as_mut_ptr()).value_min).write(value) }
    }
}

#[cfg(test)]
mod option_range_tests {
    use core::mem::{align_of, offset_of, size_of};

    use super::*;

    #[test]
    fn range_layout_and_scalar_access_match_c() {
        assert_eq!(size_of::<AVOptionRange>(), size_of::<ffi::AVOptionRange>());
        assert_eq!(
            align_of::<AVOptionRange>(),
            align_of::<ffi::AVOptionRange>()
        );
        assert_eq!(size_of::<ffi::AVOptionRange>(), 48);
        assert_eq!(offset_of!(ffi::AVOptionRange, str_), 0);
        assert_eq!(offset_of!(ffi::AVOptionRange, value_min), 8);
        assert_eq!(offset_of!(ffi::AVOptionRange, value_max), 16);
        assert_eq!(offset_of!(ffi::AVOptionRange, component_min), 24);
        assert_eq!(offset_of!(ffi::AVOptionRange, component_max), 32);
        assert_eq!(offset_of!(ffi::AVOptionRange, is_range), 40);

        let mut raw = ffi::AVOptionRange {
            str_: core::ptr::null(),
            value_min: 0.0,
            value_max: 0.0,
            component_min: 0.0,
            component_max: 0.0,
            is_range: 0,
        };
        // SAFETY: `raw` remains live and exclusively borrowed through the
        // handle, and its null string satisfies the documented invariant.
        let mut range =
            unsafe { AVOptionRangeMut::from_ptr(addr_of_mut!(raw)) }.expect("stack address");
        range.set_is_range(true);
        range.set_value_min(-2.0);
        range.set_value_max(9.0);
        range.set_component_min(1.0);
        range.set_component_max(7.0);

        let range = range.as_ref();
        assert!(range.is_range());
        assert_eq!(range.value_min(), -2.0);
        assert_eq!(range.value_max(), 9.0);
        assert_eq!(range.component_min(), 1.0);
        assert_eq!(range.component_max(), 7.0);
        assert!(range.string().is_none());
    }

    #[test]
    fn range_string_ownership_can_be_replaced_and_taken() {
        let mut raw = ffi::AVOptionRange {
            str_: core::ptr::null(),
            value_min: 0.0,
            value_max: 0.0,
            component_min: 0.0,
            component_max: 0.0,
            is_range: 0,
        };
        // SAFETY: `raw` remains live and exclusively borrowed, and every
        // string stored below uses the allocator required by its invariant.
        let mut range =
            unsafe { AVOptionRangeMut::from_ptr(addr_of_mut!(raw)) }.expect("stack address");
        // SAFETY: `av_strdup` returns null or one uniquely owned terminated
        // av_malloc-family string, exactly what `AvFree` releases.
        let first = unsafe { CrustifyStr::<AvFree>::from_raw(ffi::av_strdup(c"first".as_ptr())) }
            .expect("av_strdup failed");
        range.set_string(Some(first));
        assert_eq!(range.as_ref().string(), Some(c"first"));

        // SAFETY: the same allocation and termination contract as above.
        let second = unsafe { CrustifyStr::<AvFree>::from_raw(ffi::av_strdup(c"second".as_ptr())) }
            .expect("av_strdup failed");
        range.set_string(Some(second));
        assert_eq!(range.as_ref().string(), Some(c"second"));

        let second = range.take_string().expect("string was stored");
        assert_eq!(second.as_c_str(), c"second");
        assert!(range.as_ref().string().is_none());
    }
}

/// Wraps: av_opt_child_next
#[must_use]
pub fn av_opt_child_next<'a>(
    object: OptionObjectRef<'a>,
    previous: Option<OptionObjectRef<'a>>,
) -> Option<OptionObjectRef<'a>> {
    // SAFETY: the handle invariant includes the class callback contract: it may
    // inspect these borrowed objects and returns null or another well-formed
    // child kept alive by the root object for `'a`.
    let pointer = unsafe {
        ffi::av_opt_child_next(
            object.as_ptr(),
            previous.map_or(core::ptr::null_mut(), OptionObjectRef::as_ptr),
        )
    };
    NonNull::new(pointer).map(|pointer| OptionObjectRef {
        pointer,
        _borrow: PhantomData,
    })
}

/// Wraps: av_opt_copy
pub fn av_opt_copy(
    destination: &mut OptionObjectMut<'_>,
    source: OptionObjectRef<'_>,
) -> Result<(), i32> {
    // SAFETY: both handles guarantee well-formed fields for one identical class;
    // Rust makes the destination exclusive and source shared for the call.
    let status = unsafe { ffi::av_opt_copy(destination.as_mut_ptr(), source.as_ptr()) };
    if status < 0 { Err(status) } else { Ok(()) }
}

/// Wraps: av_opt_flag_is_set
#[must_use]
pub fn av_opt_flag_is_set(
    object: OptionObjectRef<'_>,
    field_name: &CStr,
    flag_name: &CStr,
) -> bool {
    // SAFETY: the handle carries a well-formed option object and both names are
    // live terminated strings read only during the call.
    unsafe {
        ffi::av_opt_flag_is_set(object.as_ptr(), field_name.as_ptr(), flag_name.as_ptr()) != 0
    }
}

/// Wraps: av_opt_free
///
/// Disposes all option-owned fields without freeing the object itself.
pub fn av_opt_free(object: &mut OptionObjectMut<'_>) {
    // SAFETY: the handle invariant guarantees every option-owned field is valid;
    // exclusive access permits C to dispose and reset those fields.
    unsafe { ffi::av_opt_free(object.as_mut_ptr()) }
}

/// Wraps: av_opt_get
pub fn av_opt_get(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<Option<CrustifyStr<AvFree>>, i32> {
    let mut output = core::ptr::null_mut();
    // SAFETY: the object and name borrows are live; `output` is a writable slot.
    // C returns null or a new av_malloc-family terminated string.
    let status = unsafe {
        ffi::av_opt_get(
            object.as_ptr(),
            name.as_ptr(),
            search_flags,
            &raw mut output,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        // SAFETY: the successful C contract described above transfers ownership.
        Ok(unsafe { CrustifyStr::from_raw(output.cast()) })
    }
}

/// Wraps: av_opt_get_array_size
pub fn av_opt_get_array_size(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<u32, i32> {
    let mut output = 0;
    // SAFETY: the object and name are live and output is one writable `unsigned`.
    let status = unsafe {
        ffi::av_opt_get_array_size(
            object.as_ptr(),
            name.as_ptr(),
            search_flags,
            &raw mut output,
        )
    };
    if status < 0 { Err(status) } else { Ok(output) }
}

/// Wraps: av_opt_get_double
pub fn av_opt_get_double(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<f64, i32> {
    let mut output = 0.0;
    // SAFETY: the object and name are live and output is one writable `double`.
    let status = unsafe {
        ffi::av_opt_get_double(
            object.as_ptr(),
            name.as_ptr(),
            search_flags,
            &raw mut output,
        )
    };
    if status < 0 { Err(status) } else { Ok(output) }
}

/// Wraps: av_opt_get_image_size
pub fn av_opt_get_image_size(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<(i32, i32), i32> {
    let (mut width, mut height) = (0, 0);
    // SAFETY: the object and name are live and both outputs are writable ints.
    let status = unsafe {
        ffi::av_opt_get_image_size(
            object.as_ptr(),
            name.as_ptr(),
            search_flags,
            &raw mut width,
            &raw mut height,
        )
    };
    if status < 0 {
        Err(status)
    } else {
        Ok((width, height))
    }
}

/// Wraps: av_opt_get_int
pub fn av_opt_get_int(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<i64, i32> {
    let mut output = 0;
    // SAFETY: the object and name are live and output is one writable int64.
    let status = unsafe {
        ffi::av_opt_get_int(
            object.as_ptr(),
            name.as_ptr(),
            search_flags,
            &raw mut output,
        )
    };
    if status < 0 { Err(status) } else { Ok(output) }
}

pub struct OptKeyValue {
    pub key: Option<CrustifyStr<AvFree>>,
    pub value: CrustifyStr<AvFree>,
}

/// Wraps: av_opt_get_key_value
pub fn av_opt_get_key_value(
    options: &mut &CStr,
    key_value_separators: &CStr,
    pair_separators: &CStr,
    flags: u32,
) -> Result<OptKeyValue, i32> {
    let mut cursor = options.as_ptr();
    let (mut key, mut value) = (core::ptr::null_mut(), core::ptr::null_mut());
    // SAFETY: cursor and both separator strings are terminated and live; the
    // three local pointer slots are writable. C advances cursor within the
    // original string and returns newly allocated terminated strings.
    let status = unsafe {
        ffi::av_opt_get_key_value(
            &raw mut cursor,
            key_value_separators.as_ptr(),
            pair_separators.as_ptr(),
            flags,
            &raw mut key,
            &raw mut value,
        )
    };
    if status < 0 {
        return Err(status);
    }
    // SAFETY: success leaves cursor within the original terminated string.
    *options = unsafe { CStr::from_ptr(cursor) };
    // SAFETY: successful outputs are null or fresh av_malloc-family strings;
    // value is guaranteed non-null by the C success path.
    let key = unsafe { CrustifyStr::from_raw(key) };
    // SAFETY: as above; `av_get_token` must have succeeded for status zero.
    let value =
        unsafe { CrustifyStr::from_raw(value) }.expect("C returned a null value on success");
    Ok(OptKeyValue { key, value })
}

/// Wraps: av_opt_is_set_to_default_by_name
pub fn av_opt_is_set_to_default_by_name(
    object: OptionObjectRef<'_>,
    name: &CStr,
    search_flags: i32,
) -> Result<bool, i32> {
    // SAFETY: the object and name are live and read-only for the call.
    let status = unsafe {
        ffi::av_opt_is_set_to_default_by_name(object.as_ptr(), name.as_ptr(), search_flags)
    };
    if status < 0 {
        Err(status)
    } else {
        Ok(status != 0)
    }
}

#[cfg(test)]
mod scheduled_get_tests {
    use super::*;

    #[test]
    fn parses_owned_key_value_strings_and_advances() {
        let mut options = c" key = 'some value',next=2";
        let parsed = av_opt_get_key_value(&mut options, c"=", c",", 0).unwrap();
        assert_eq!(parsed.key.unwrap().as_c_str(), c"key");
        assert_eq!(parsed.value.as_c_str(), c"some value");
        assert_eq!(options, c",next=2");
    }
}

ffibox::define_ctype!(
    /// Wraps: AVOptionRanges
    ///
    /// ABI-compatible aggregate returned by an option range query. The object
    /// owns its pointer table, every non-null range in that table, and each
    /// range's optional string.
    ///
    /// # Handle invariant
    ///
    /// Both counts are non-negative and their product fits a C `int`. That
    /// product is the pointer-table length; a non-zero length requires a live
    /// table of that many initialized slots.
    /// Each non-null slot satisfies [`AVOptionRange`]'s handle invariant. The
    /// table, elements and object use libavutil's allocator family and are
    /// uniquely owned by this aggregate.
    AVOptionRanges,
    AVOptionRangesRef,
    AVOptionRangesMut,
    ffi::AVOptionRanges
);

// SAFETY: a uniquely owned, well-formed `AVOptionRanges` has exactly the
// ownership graph expected by `av_opt_freep_ranges`. That function frees every
// range string and range object, then the table and aggregate allocation.
unsafe impl CDropped for AVOptionRanges {
    unsafe fn c_drop(object: NonNull<Self>) {
        let mut raw = object.as_ptr().cast::<ffi::AVOptionRanges>();
        // SAFETY: the caller transfers one uniquely owned aggregate satisfying
        // the documented handle invariant. The local slot is live and mutable;
        // the C destructor consumes its pointee and writes null back.
        unsafe { ffi::av_opt_freep_ranges(addr_of_mut!(raw)) }
    }
}

impl AVOptionRangesRef<'_> {
    /// Field: AVOptionRanges.nb_components
    ///
    /// Returns the number of components represented by the table.
    #[must_use]
    pub fn nb_components(&self) -> i32 {
        // SAFETY: the handle keeps an initialized aggregate live and this
        // raw-place projection copies its integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).nb_components).read() }
    }

    /// Field: AVOptionRanges.nb_ranges
    ///
    /// Returns the number of ranges per component.
    #[must_use]
    pub fn nb_ranges(&self) -> i32 {
        // SAFETY: the handle keeps an initialized aggregate live and this
        // raw-place projection copies its integer without forming a reference.
        unsafe { addr_of!((*self.as_ptr()).nb_ranges).read() }
    }
}

impl<'a> AVOptionRangesRef<'a> {
    /// Field: AVOptionRanges.range
    ///
    /// Borrows one range using the public component-major table indexing.
    /// Returns `None` for an out-of-bounds coordinate or a null slot.
    #[must_use]
    pub fn range(&self, component: usize, range: usize) -> Option<AVOptionRangeRef<'a>> {
        let components = usize::try_from(self.nb_components()).ok()?;
        let ranges = usize::try_from(self.nb_ranges()).ok()?;
        if component >= components || range >= ranges {
            return None;
        }
        let index = ranges.checked_mul(component)?.checked_add(range)?;
        // SAFETY: the handle invariant makes `range` null only for an empty
        // table; this path has in-bounds coordinates and therefore a non-empty
        // table. It has `components * ranges` initialized pointer slots, so the
        // checked index is readable without forming a reference.
        let table = unsafe { addr_of!((*self.as_ptr()).range).read() };
        let table = NonNull::new(table)?;
        // SAFETY: `index` is in the initialized table established above. A
        // non-null element addresses a range owned by the aggregate for `'a`.
        let pointer = unsafe { table.as_ptr().add(index).read() };
        // SAFETY: the aggregate handle invariant guarantees liveness, layout
        // and the nested range invariant for every non-null element.
        unsafe { AVOptionRangeRef::from_ptr(pointer) }
    }
}

impl AVOptionRangesMut<'_> {
    /// Exclusively borrows one range using component-major table indexing.
    /// Returns `None` for an out-of-bounds coordinate or a null slot.
    #[must_use]
    pub fn range_mut(&mut self, component: usize, range: usize) -> Option<AVOptionRangeMut<'_>> {
        let shared = self.as_ref();
        let components = usize::try_from(shared.nb_components()).ok()?;
        let ranges = usize::try_from(shared.nb_ranges()).ok()?;
        if component >= components || range >= ranges {
            return None;
        }
        let index = ranges.checked_mul(component)?.checked_add(range)?;
        // SAFETY: as the shared accessor, while the exclusive parent handle
        // supplies write provenance for the selected owned element.
        let table = unsafe { addr_of_mut!((*self.as_mut_ptr()).range).read() };
        let table = NonNull::new(table)?;
        // SAFETY: the checked coordinate is within the initialized table.
        let pointer = unsafe { table.as_ptr().add(index).read() };
        // SAFETY: the exclusive borrow of the parent prevents another handle
        // from being obtained through this API for the returned reborrow.
        unsafe { AVOptionRangeMut::from_ptr(pointer) }
    }
}

#[cfg(test)]
mod option_ranges_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    unsafe fn malloc_zeroed<T>() -> *mut T {
        // SAFETY: allocation size is exactly one `T`; the caller takes
        // responsibility for initializing its semantic invariants and freeing
        // it with the matching libavutil allocator.
        unsafe { ffi::av_mallocz(size_of::<T>()).cast::<T>() }
    }

    #[test]
    fn layout_and_owned_pointer_table_match_c() {
        assert_eq!(
            size_of::<AVOptionRanges>(),
            size_of::<ffi::AVOptionRanges>()
        );
        assert_eq!(
            align_of::<AVOptionRanges>(),
            align_of::<ffi::AVOptionRanges>()
        );

        // SAFETY: all allocations below use libavutil's allocator and are
        // initialized into one valid ownership graph before CBox adopts it.
        let mut owned = unsafe {
            let aggregate = malloc_zeroed::<ffi::AVOptionRanges>();
            let table = ffi::av_mallocz(4 * size_of::<*mut ffi::AVOptionRange>())
                .cast::<*mut ffi::AVOptionRange>();
            assert!(!aggregate.is_null());
            assert!(!table.is_null());
            for index in 0..4 {
                let item = malloc_zeroed::<ffi::AVOptionRange>();
                assert!(!item.is_null());
                addr_of_mut!((*item).value_min).write(index as f64);
                addr_of_mut!((*item).value_max).write(index as f64 + 0.5);
                table.add(index).write(item);
            }
            addr_of_mut!((*aggregate).range).write(table);
            addr_of_mut!((*aggregate).nb_ranges).write(2);
            addr_of_mut!((*aggregate).nb_components).write(2);
            CBox::<AVOptionRanges>::from_raw(aggregate).expect("non-null allocation")
        };

        let view = owned.as_ref();
        assert_eq!(view.nb_ranges(), 2);
        assert_eq!(view.nb_components(), 2);
        assert_eq!(
            view.range(1, 0).expect("component 1 range 0").value_min(),
            2.0
        );
        assert!(view.range(2, 0).is_none());

        owned
            .as_mut()
            .range_mut(0, 1)
            .expect("component 0 range 1")
            .set_value_max(9.0);
        assert_eq!(owned.as_ref().range(0, 1).unwrap().value_max(), 9.0);

        // `Drop` exercises `av_opt_freep_ranges` for all four elements, their
        // pointer table, and the aggregate. ASan catches an ownership mismatch.
        drop(owned);
    }
}
