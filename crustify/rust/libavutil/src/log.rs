//! Wrappers for libavutil logging.

use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
use core::ptr::{NonNull, addr_of};

use ffibox::CBox;

use crate::ffi;
use crate::opt::{AVOptionRanges, AVOptionRef, OptionObjectMut};

/// The raw C logging callback shape. Its raw arguments are confined to the
/// callback boundary because a `va_list` cannot be represented as a safe Rust
/// iterator and the context's concrete AVClass-bearing type is erased.
pub type LogCallback = unsafe extern "C" fn(
    context: *mut c_void,
    level: c_int,
    format: *const c_char,
    arguments: *mut ffi::__va_list_tag,
);

/// Shared borrowed handle to an AVClass-bearing logging object whose concrete
/// wrapper has not yet been translated.
#[derive(Clone, Copy)]
pub struct LogContextRef<'a> {
    pointer: NonNull<c_void>,
    _borrow: PhantomData<&'a c_void>,
}

impl<'a> LogContextRef<'a> {
    /// Construct a temporary erased logging-context handle.
    ///
    /// # Safety
    ///
    /// `pointer` must remain live and unmodified for `'a` and identify an
    /// object whose first field is a valid `AVClass *`.
    pub unsafe fn from_raw(pointer: NonNull<c_void>) -> Self {
        Self {
            pointer,
            _borrow: PhantomData,
        }
    }

    pub(crate) fn as_ptr(self) -> *mut c_void {
        self.pointer.as_ptr()
    }
}

/// Wraps: av_log_get_flags
#[must_use]
pub fn av_log_get_flags() -> i32 {
    // SAFETY: the C implementation performs an atomic load and has no caller
    // obligations.
    unsafe { ffi::av_log_get_flags() }
}

/// Wraps: av_log_get_level
#[must_use]
pub fn av_log_get_level() -> i32 {
    // SAFETY: the C implementation performs an atomic load and has no caller
    // obligations.
    unsafe { ffi::av_log_get_level() }
}

/// Wraps: av_log_set_callback
///
/// # Safety
///
/// An installed callback is retained process-globally, may be called from any
/// thread, and must remain valid and thread-safe until replaced. Every callback
/// invocation must also honor the raw context, format and `va_list` contracts.
pub unsafe fn av_log_set_callback(callback: Option<LogCallback>) {
    // SAFETY: the caller accepts the global lifetime and concurrency contract;
    // the function atomically stores the supplied code pointer.
    unsafe { ffi::av_log_set_callback(callback) }
}

/// Wraps: av_log_set_flags
pub fn av_log_set_flags(flags: i32) {
    // SAFETY: the C implementation performs an atomic store and takes no
    // pointers.
    unsafe { ffi::av_log_set_flags(flags) }
}

/// Wraps: av_log_set_level
pub fn av_log_set_level(level: i32) {
    // SAFETY: the C implementation performs an atomic store and takes no
    // pointers.
    unsafe { ffi::av_log_set_level(level) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_level_and_flags() {
        let old_level = av_log_get_level();
        let old_flags = av_log_get_flags();
        av_log_set_level(17);
        av_log_set_flags(23);
        assert_eq!(av_log_get_level(), 17);
        assert_eq!(av_log_get_flags(), 23);
        av_log_set_level(old_level);
        av_log_set_flags(old_flags);
    }
}

/// Wraps: AVClassCategory
///
/// Classifies the component represented by an [`AVClass`].
/// The transparent integer representation preserves values introduced by a
/// newer libavutil instead of creating an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AVClassCategory(ffi::AVClassCategory);

impl AVClassCategory {
    pub const NA: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_NA);
    pub const INPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_INPUT);
    pub const OUTPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_OUTPUT);
    pub const MUXER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_MUXER);
    pub const DEMUXER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEMUXER);
    pub const ENCODER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_ENCODER);
    pub const DECODER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DECODER);
    pub const FILTER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_FILTER);
    pub const BITSTREAM_FILTER: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_BITSTREAM_FILTER);
    pub const SWSCALER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_SWSCALER);
    pub const SWRESAMPLER: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_SWRESAMPLER);
    pub const HWDEVICE: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_HWDEVICE);
    pub const DEVICE_VIDEO_OUTPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT);
    pub const DEVICE_VIDEO_INPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT);
    pub const DEVICE_AUDIO_OUTPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT);
    pub const DEVICE_AUDIO_INPUT: Self =
        Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT);
    pub const DEVICE_OUTPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_OUTPUT);
    pub const DEVICE_INPUT: Self = Self(ffi::AVClassCategory_AV_CLASS_CATEGORY_DEVICE_INPUT);

    /// Wraps a raw C enum value, including one unknown to this crate version.
    #[must_use]
    pub const fn from_raw(raw: ffi::AVClassCategory) -> Self {
        Self(raw)
    }

    /// Returns the ABI value accepted by libavutil.
    #[must_use]
    pub const fn as_raw(self) -> ffi::AVClassCategory {
        self.0
    }
}

impl From<ffi::AVClassCategory> for AVClassCategory {
    fn from(raw: ffi::AVClassCategory) -> Self {
        Self::from_raw(raw)
    }
}

impl From<AVClassCategory> for ffi::AVClassCategory {
    fn from(category: AVClassCategory) -> Self {
        category.as_raw()
    }
}

#[cfg(test)]
mod category_tests {
    use super::*;

    #[test]
    fn known_and_future_categories_round_trip() {
        assert_eq!(AVClassCategory::DEVICE_INPUT.as_raw(), 45);
        let future = AVClassCategory::from_raw(10_000);
        assert_eq!(future.as_raw(), 10_000);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVClass
    ///
    /// ABI-compatible borrowed view of immutable AVClass metadata. C normally
    /// stores these records in `static const` definitions. String and option
    /// pointers borrow static storage, and callback pointers are optional.
    /// The type has no allocator or lifecycle operation of its own.
    AVClass,
    AVClassRef,
    AVClassMut,
    ffi::AVClass
);

type ItemNameFn = unsafe extern "C" fn(*mut c_void) -> *const c_char;
type GetCategoryFn = unsafe extern "C" fn(*mut c_void) -> ffi::AVClassCategory;
type QueryRangesFn =
    unsafe extern "C" fn(*mut *mut ffi::AVOptionRanges, *mut c_void, *const c_char, c_int) -> c_int;
type ChildNextFn = unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void;
type ChildClassIterateFn = unsafe extern "C" fn(*mut *mut c_void) -> *const ffi::AVClass;

/// Failure to invoke an AVClass callback through its checked safe surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AVClassCallbackError {
    /// The object is governed by a different AVClass than the callback.
    ContextClassMismatch,
    /// The callback returned a negative libavutil error code.
    Library(i32),
    /// A successful range callback failed to return its required allocation.
    NullResult,
}

fn context_has_class(pointer: *mut c_void, class: NonNull<ffi::AVClass>) -> bool {
    // SAFETY: the typed erased-context handles promise that their first field
    // is a live AVClass pointer. This raw copy forms no reference to the
    // context or its first field.
    unsafe { pointer.cast::<*const ffi::AVClass>().read() == class.as_ptr() }
}

/// Callable view of [`AVClassRef::item_name`].
#[derive(Clone, Copy)]
pub struct AVClassItemNameCallback<'a> {
    function: ItemNameFn,
    class: NonNull<ffi::AVClass>,
    _borrow: PhantomData<&'a ()>,
}

impl AVClassItemNameCallback<'_> {
    /// Calls the callback after verifying that it belongs to `context`.
    pub fn call<'a>(
        &'a self,
        context: LogContextRef<'a>,
    ) -> Result<Option<&'a CStr>, AVClassCallbackError> {
        if !context_has_class(context.as_ptr(), self.class) {
            return Err(AVClassCallbackError::ContextClassMismatch);
        }
        // SAFETY: the context handle proves liveness and the class check proves
        // that this is the callback governing its concrete layout. AVClass's
        // contract makes a non-null result a NUL string valid for the context
        // borrow.
        let pointer = unsafe { (self.function)(context.as_ptr()) };
        Ok(if pointer.is_null() {
            None
        } else {
            // SAFETY: established by the AVClass callback contract above.
            Some(unsafe { CStr::from_ptr(pointer) })
        })
    }
}

/// Callable view of [`AVClassRef::get_category`].
#[derive(Clone, Copy)]
pub struct AVClassGetCategoryCallback<'a> {
    function: GetCategoryFn,
    class: NonNull<ffi::AVClass>,
    _borrow: PhantomData<&'a ()>,
}

impl AVClassGetCategoryCallback<'_> {
    /// Calls the callback after verifying that it belongs to `context`.
    pub fn call(
        &self,
        context: LogContextRef<'_>,
    ) -> Result<AVClassCategory, AVClassCallbackError> {
        if !context_has_class(context.as_ptr(), self.class) {
            return Err(AVClassCallbackError::ContextClassMismatch);
        }
        // SAFETY: the live context is governed by this exact AVClass, so its
        // category callback accepts the erased pointer. The integer result is
        // preserved by the open transparent Rust wrapper.
        Ok(AVClassCategory::from_raw(unsafe {
            (self.function)(context.as_ptr())
        }))
    }
}

/// Callable view of [`AVClassRef::query_ranges`].
#[derive(Clone, Copy)]
pub struct AVClassQueryRangesCallback<'a> {
    function: QueryRangesFn,
    class: NonNull<ffi::AVClass>,
    _borrow: PhantomData<&'a ()>,
}

impl AVClassQueryRangesCallback<'_> {
    /// Invokes the class-specific range query and adopts its result.
    pub fn call(
        &self,
        object: &mut OptionObjectMut<'_>,
        key: &CStr,
        flags: i32,
    ) -> Result<CBox<AVOptionRanges>, AVClassCallbackError> {
        if !context_has_class(object.as_mut_ptr(), self.class) {
            return Err(AVClassCallbackError::ContextClassMismatch);
        }
        let mut ranges: *mut ffi::AVOptionRanges = core::ptr::null_mut();
        // SAFETY: the exclusive option-object handle is governed by this exact
        // class, `key` is live and NUL-terminated, and the out slot is live.
        let status = unsafe {
            (self.function)(
                core::ptr::addr_of_mut!(ranges),
                object.as_mut_ptr(),
                key.as_ptr(),
                flags,
            )
        };
        if status < 0 {
            return Err(AVClassCallbackError::Library(status));
        }
        if ranges.is_null() {
            return Err(AVClassCallbackError::NullResult);
        }
        let components = if flags & ffi::AV_OPT_MULTI_COMPONENT_RANGE as i32 == 0 {
            1
        } else {
            status
        };
        // SAFETY: a successful callback returned a writable uniquely owned
        // aggregate. This mirrors `av_opt_query_ranges`, which installs the
        // callback's component count before publishing the result.
        unsafe { core::ptr::addr_of_mut!((*ranges).nb_components).write(components) };
        // SAFETY: the successful callback returned one fully initialized,
        // uniquely owned aggregate allocated for `av_opt_freep_ranges`.
        Ok(unsafe { CBox::<AVOptionRanges>::from_raw(ranges) }
            .expect("the null result was rejected above"))
    }
}

/// Callable view of [`AVClassRef::child_next`].
#[derive(Clone, Copy)]
pub struct AVClassChildNextCallback<'a> {
    function: ChildNextFn,
    class: NonNull<ffi::AVClass>,
    _borrow: PhantomData<&'a ()>,
}

impl AVClassChildNextCallback<'_> {
    /// Creates a lending iterator over the actual children of `parent`.
    pub fn children<'a>(
        self,
        parent: &'a mut OptionObjectMut<'_>,
    ) -> Result<AVClassChildren<'a>, AVClassCallbackError> {
        if !context_has_class(parent.as_mut_ptr(), self.class) {
            return Err(AVClassCallbackError::ContextClassMismatch);
        }
        Ok(AVClassChildren {
            function: self.function,
            parent: NonNull::new(parent.as_mut_ptr()).expect("option object handles are non-null"),
            previous: core::ptr::null_mut(),
            _borrow: PhantomData,
        })
    }
}

/// Lending iterator returned by [`AVClassChildNextCallback::children`].
pub struct AVClassChildren<'a> {
    function: ChildNextFn,
    parent: NonNull<c_void>,
    previous: *mut c_void,
    _borrow: PhantomData<&'a mut OptionObjectMut<'a>>,
}

impl AVClassChildren<'_> {
    /// Borrows the next child. The result cannot outlive this mutable borrow,
    /// preventing two exclusively borrowed child handles from being retained.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<OptionObjectMut<'_>> {
        // SAFETY: construction verified the callback/class pair and keeps the
        // parent exclusively borrowed. `previous` is null or the exact child
        // returned by the preceding call, as required by the callback.
        let pointer = unsafe { (self.function)(self.parent.as_ptr(), self.previous) };
        self.previous = pointer;
        let pointer = NonNull::new(pointer)?;
        // SAFETY: the AVClass child callback contract makes every non-null
        // result a live AVClass-bearing option object borrowed from the parent.
        unsafe { Some(OptionObjectMut::from_raw(pointer)) }
    }
}

/// Callable iterator factory stored in `AVClass.child_class_iterate`.
#[derive(Clone, Copy)]
pub struct AVClassChildClassIterateCallback<'a> {
    function: ChildClassIterateFn,
    _borrow: PhantomData<&'a ()>,
}

impl<'a> AVClassChildClassIterateCallback<'a> {
    /// Starts iteration over the potential child classes.
    #[must_use]
    pub fn classes(self) -> AVClassChildClasses<'a> {
        AVClassChildClasses {
            function: self.function,
            state: core::ptr::null_mut(),
            _borrow: PhantomData,
        }
    }
}

/// Iterator over the potential child classes of an AVClass.
pub struct AVClassChildClasses<'a> {
    function: ChildClassIterateFn,
    state: *mut c_void,
    _borrow: PhantomData<&'a ()>,
}

impl<'a> Iterator for AVClassChildClasses<'a> {
    type Item = AVClassRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: the callback writes the opaque iteration state back through
        // this slot, so the exclusive borrow supplies its write provenance.
        // `state` starts null and only values written by this callback are fed
        // back to it. A non-null result is a live immutable child class whose
        // lifetime is bounded by the parent AVClass borrow.
        let pointer = unsafe { (self.function)(&raw mut self.state) };
        // SAFETY: the callback contract establishes the returned class layout,
        // initialization and lifetime; null marks the end of iteration.
        unsafe { AVClassRef::from_ptr(pointer.cast_mut()) }
    }
}

impl<'a> AVClassRef<'a> {
    /// Field: AVClass.class_name
    #[must_use]
    pub fn class_name(&self) -> Option<&'a CStr> {
        // SAFETY: the handle addresses initialized class metadata. The raw
        // projection copies the pointer without forming a reference to C data.
        let pointer = unsafe { addr_of!((*self.as_ptr()).class_name).read() };
        if pointer.is_null() {
            None
        } else {
            // SAFETY: AVClass metadata points at a NUL-terminated static class
            // name; the returned borrow is conservatively tied to the handle.
            Some(unsafe { CStr::from_ptr(pointer) })
        }
    }

    /// Field: AVClass.item_name
    #[must_use]
    pub fn item_name(&self) -> Option<AVClassItemNameCallback<'a>> {
        // SAFETY: copies an optional code pointer from initialized immutable
        // class metadata without forming a reference to the object.
        unsafe { addr_of!((*self.as_ptr()).item_name).read() }.map(|function| {
            AVClassItemNameCallback {
                function,
                class: NonNull::new(self.as_ptr().cast_mut())
                    .expect("AVClass handles are non-null"),
                _borrow: PhantomData,
            }
        })
    }

    /// Field: AVClass.option
    #[must_use]
    pub fn option(&self) -> Option<AVOptionRef<'a>> {
        // SAFETY: copies the optional first-entry pointer without forming a
        // reference to either class or option storage.
        let pointer = unsafe { addr_of!((*self.as_ptr()).option).read() };
        // SAFETY: a non-null AVClass option pointer names the first initialized
        // entry of its static, null-name-terminated option table.
        unsafe { AVOptionRef::from_ptr(pointer.cast_mut()) }
    }

    /// Field: AVClass.version
    #[must_use]
    pub fn version(&self) -> i32 {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).version).read() }
    }

    /// Field: AVClass.log_level_offset_offset
    #[must_use]
    pub fn log_level_offset_offset(&self) -> i32 {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).log_level_offset_offset).read() }
    }

    /// Field: AVClass.parent_log_context_offset
    #[must_use]
    pub fn parent_log_context_offset(&self) -> i32 {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).parent_log_context_offset).read() }
    }

    /// Field: AVClass.category
    #[must_use]
    pub fn category(&self) -> AVClassCategory {
        // SAFETY: raw-place projection copies the integer-backed open enum.
        AVClassCategory::from_raw(unsafe { addr_of!((*self.as_ptr()).category).read() })
    }

    /// Field: AVClass.get_category
    #[must_use]
    pub fn get_category(&self) -> Option<AVClassGetCategoryCallback<'a>> {
        // SAFETY: copies an optional code pointer from initialized metadata.
        unsafe { addr_of!((*self.as_ptr()).get_category).read() }.map(|function| {
            AVClassGetCategoryCallback {
                function,
                class: NonNull::new(self.as_ptr().cast_mut())
                    .expect("AVClass handles are non-null"),
                _borrow: PhantomData,
            }
        })
    }

    /// Field: AVClass.query_ranges
    #[must_use]
    pub fn query_ranges(&self) -> Option<AVClassQueryRangesCallback<'a>> {
        // SAFETY: copies an optional code pointer from initialized metadata.
        unsafe { addr_of!((*self.as_ptr()).query_ranges).read() }.map(|function| {
            AVClassQueryRangesCallback {
                function,
                class: NonNull::new(self.as_ptr().cast_mut())
                    .expect("AVClass handles are non-null"),
                _borrow: PhantomData,
            }
        })
    }

    /// Field: AVClass.child_next
    #[must_use]
    pub fn child_next(&self) -> Option<AVClassChildNextCallback<'a>> {
        // SAFETY: copies an optional code pointer from initialized metadata.
        unsafe { addr_of!((*self.as_ptr()).child_next).read() }.map(|function| {
            AVClassChildNextCallback {
                function,
                class: NonNull::new(self.as_ptr().cast_mut())
                    .expect("AVClass handles are non-null"),
                _borrow: PhantomData,
            }
        })
    }

    /// Field: AVClass.child_class_iterate
    #[must_use]
    pub fn child_class_iterate(&self) -> Option<AVClassChildClassIterateCallback<'a>> {
        // SAFETY: copies an optional code pointer from initialized metadata.
        unsafe { addr_of!((*self.as_ptr()).child_class_iterate).read() }.map(|function| {
            AVClassChildClassIterateCallback {
                function,
                _borrow: PhantomData,
            }
        })
    }

    /// Field: AVClass.state_flags_offset
    #[must_use]
    pub fn state_flags_offset(&self) -> i32 {
        // SAFETY: raw-place projection copies one initialized scalar.
        unsafe { addr_of!((*self.as_ptr()).state_flags_offset).read() }
    }
}

#[cfg(test)]
mod avclass_tests {
    use super::*;

    unsafe extern "C" fn item_name(context: *mut c_void) -> *const c_char {
        // SAFETY: the test passes a live `TestContext` with this class.
        unsafe { (*context.cast::<TestContext>()).name }
    }

    unsafe extern "C" fn get_category(_context: *mut c_void) -> ffi::AVClassCategory {
        AVClassCategory::FILTER.as_raw()
    }

    unsafe extern "C" fn query_ranges(
        ranges: *mut *mut ffi::AVOptionRanges,
        _object: *mut c_void,
        _key: *const c_char,
        _flags: c_int,
    ) -> c_int {
        // SAFETY: the callback contract provides a live writable out slot.
        unsafe { ranges.write(core::ptr::null_mut()) };
        -22
    }

    unsafe extern "C" fn child_next(_object: *mut c_void, _previous: *mut c_void) -> *mut c_void {
        core::ptr::null_mut()
    }

    unsafe extern "C" fn child_class_iterate(_state: *mut *mut c_void) -> *const ffi::AVClass {
        core::ptr::null()
    }

    /// Mirrors C's `static const AVClass`: a process-lifetime immutable
    /// record whose pointer fields address string literals and code.
    #[repr(transparent)]
    struct StaticClass(ffi::AVClass);

    // SAFETY: the value is never mutated after initialization and every
    // pointer it holds addresses immutable static storage, so concurrent
    // shared access observes only constants.
    unsafe impl Sync for StaticClass {}

    static CHILD_CLASS: StaticClass = StaticClass(ffi::AVClass {
        class_name: c"ChildClass".as_ptr(),
        item_name: None,
        option: core::ptr::null(),
        version: 1,
        log_level_offset_offset: 0,
        parent_log_context_offset: 0,
        category: 0,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    });

    /// The real iteration protocol: the callback both reads and WRITES the
    /// caller's opaque state slot, as `av_opt_child_class_iterate` does.
    unsafe extern "C" fn one_child_class_iterate(state: *mut *mut c_void) -> *const ffi::AVClass {
        // SAFETY: the caller supplies a live writable slot holding either null
        // or a value this callback previously wrote.
        let seen = unsafe { state.read() };
        if seen.is_null() {
            let class: *const ffi::AVClass = &raw const CHILD_CLASS.0;
            // SAFETY: the same live writable slot; the callback contract makes
            // the opaque state the callee's to define.
            unsafe { state.write(class.cast_mut().cast::<c_void>()) };
            class
        } else {
            core::ptr::null()
        }
    }

    #[repr(C)]
    struct TestContext {
        class: *const ffi::AVClass,
        name: *const c_char,
    }

    fn test_class() -> ffi::AVClass {
        ffi::AVClass {
            class_name: c"TestClass".as_ptr(),
            item_name: Some(item_name),
            option: core::ptr::null(),
            version: 123,
            log_level_offset_offset: 4,
            parent_log_context_offset: 8,
            category: AVClassCategory::INPUT.as_raw(),
            get_category: Some(get_category),
            query_ranges: Some(query_ranges),
            child_next: Some(child_next),
            child_class_iterate: Some(child_class_iterate),
            state_flags_offset: 12,
        }
    }

    #[test]
    fn layout_scalars_and_checked_callbacks_round_trip() {
        assert_eq!(
            core::mem::size_of::<AVClass>(),
            core::mem::size_of::<ffi::AVClass>()
        );
        let raw = test_class();
        let context = TestContext {
            class: addr_of!(raw),
            name: c"instance".as_ptr(),
        };
        // SAFETY: both test values stay live for the handles, and the context's
        // first field points to the governing initialized AVClass.
        let class = unsafe { AVClassRef::from_ptr(addr_of!(raw).cast_mut()) }.unwrap();
        // SAFETY: the initialized context remains live and its first field is
        // the governing class pointer installed immediately above.
        let context = unsafe {
            LogContextRef::from_raw(
                NonNull::new(addr_of!(context).cast_mut())
                    .unwrap()
                    .cast::<c_void>(),
            )
        };

        assert_eq!(class.class_name(), Some(c"TestClass"));
        assert_eq!(class.version(), 123);
        assert_eq!(class.log_level_offset_offset(), 4);
        assert_eq!(class.parent_log_context_offset(), 8);
        assert_eq!(class.category(), AVClassCategory::INPUT);
        assert_eq!(class.state_flags_offset(), 12);
        assert!(class.option().is_none());
        assert_eq!(
            class.item_name().unwrap().call(context).unwrap(),
            Some(c"instance")
        );
        assert_eq!(
            class.get_category().unwrap().call(context).unwrap(),
            AVClassCategory::FILTER
        );

        let mut mutable_context = TestContext {
            class: addr_of!(raw),
            name: c"instance".as_ptr(),
        };
        // SAFETY: this is a live exclusively borrowed option object with no
        // option table and the governing class in its first field.
        let mut object = unsafe {
            OptionObjectMut::from_raw(NonNull::from(&mut mutable_context).cast::<c_void>())
        };
        assert!(matches!(
            class
                .query_ranges()
                .unwrap()
                .call(&mut object, c"missing", 0),
            Err(AVClassCallbackError::Library(-22))
        ));
        assert!(
            class
                .child_next()
                .unwrap()
                .children(&mut object)
                .unwrap()
                .next()
                .is_none()
        );
        assert!(
            class
                .child_class_iterate()
                .unwrap()
                .classes()
                .next()
                .is_none()
        );
    }

    #[test]
    fn child_class_iteration_writes_and_reuses_its_opaque_state() {
        let mut raw = test_class();
        raw.child_class_iterate = Some(one_child_class_iterate);
        // SAFETY: the initialized class stays live for the borrow below.
        let class = unsafe { AVClassRef::from_ptr(addr_of!(raw).cast_mut()) }.unwrap();

        let mut classes = class.child_class_iterate().unwrap().classes();
        let child = classes.next().expect("the first child class is produced");
        assert_eq!(child.class_name(), Some(c"ChildClass"));
        assert!(classes.next().is_none());
    }

    #[test]
    fn callback_rejects_a_context_from_another_class() {
        let raw = test_class();
        let other = test_class();
        let context = TestContext {
            class: addr_of!(other),
            name: c"instance".as_ptr(),
        };
        // SAFETY: initialized stack values remain live for this test.
        let class = unsafe { AVClassRef::from_ptr(addr_of!(raw).cast_mut()) }.unwrap();
        // SAFETY: the initialized context remains live and its first field is
        // the governing `other` class pointer installed immediately above.
        let context = unsafe {
            LogContextRef::from_raw(
                NonNull::new(addr_of!(context).cast_mut())
                    .unwrap()
                    .cast::<c_void>(),
            )
        };
        assert_eq!(
            class.item_name().unwrap().call(context),
            Err(AVClassCallbackError::ContextClassMismatch)
        );
    }
}
