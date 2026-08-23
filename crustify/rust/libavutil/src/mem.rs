//! Ownership strategies for memory allocated by libavutil.

use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;

use ffibox::{CCloned, CDropped, CLenCloned, CLenDropped};

use crate::ffi;

/// Wraps: av_free
///
/// Releases an allocation from the `av_malloc` family — `av_malloc`,
/// `av_mallocz`, `av_calloc`, `av_realloc`, `av_memdup`, `av_strdup` and the
/// dynarray helpers built on them. The recorded contract covers all three
/// byte-level shapes the pointer may hold, and each reaches `av_free` through
/// the owner that matches it:
///
/// | shape | owner | callsite it models |
/// |---|---|---|
/// | single value | [`CVoidBox<AvFree>`](ffibox::CVoidBox) | `av_free(info)` in `av_encryption_info_free` |
/// | counted buffer | [`CVec<T, AvFree>`](ffibox::CVec) | `av_free(e->threads)` in `executor_free` |
/// | NUL-terminated string | [`CrustifyStr<AvFree>`](ffibox::CrustifyStr) | `av_free(copy_key)` in `av_dict_set` |
///
/// `av_free` never needs the extent, so one strategy serves all three: the
/// [`CDropped`] impl carries the pointer-only owners and the [`CLenDropped`]
/// impl the counted one. For a counted buffer it also enables bytewise cloning
/// through [`av_memdup`](ffi::av_memdup).
///
/// Building the counted owner over a *fresh* `av_malloc` takes two steps.
/// `av_malloc` hands back uninitialised bytes — its only write is the optional
/// `CONFIG_MEMORY_POISONING` memset, compiled out here — while
/// [`CVec::from_raw_parts`](ffibox::CVec::from_raw_parts) requires the pointer
/// to already hold its `count` elements, which
/// [`as_slice`](ffibox::CVec::as_slice) then materialises as one `&[T]`
/// covering all of them. [`CElem`](ffibox::CElem) does not excuse that: its
/// obligation ranges over the bit patterns the buffer may hold, and
/// uninitialised memory is not a bit pattern — read as a `u8` or a `*mut u8`
/// it is an invalid value either way. Adopt the allocation as
/// `CVec<MaybeUninit<T>, AvFree>`, ffibox's escape hatch for a buffer C has
/// not filled, fill it through that owner, then promote it. `MaybeUninit<T>`
/// has `T`'s size, so `AvFree` releases the same extent from either tier. An
/// [`av_memdup`](ffi::av_memdup) result needs none of this: it arrives
/// initialised over every byte its element count can reach.
///
/// Unlike `munmap`, the length-aware release must not short-circuit on a zero
/// byte length: `av_malloc(0)` retries with one byte, so a zero-length owner
/// still holds a live allocation that has to be freed.
///
/// `av_free` accepts NULL, but no owner reaching this strategy can hold one —
/// every ffibox owner is non-null by construction, so the null case is absorbed
/// at `from_raw` and never becomes a drop.
///
/// This is deliberately distinct from [`LibcFree`](libc::stdlib::LibcFree).
/// The two allocation families are not interchangeable: `av_free` resolves to
/// `_aligned_free` wherever `HAVE_ALIGNED_MALLOC` holds and to a prefixed
/// allocator under `MALLOC_PREFIX`, matching an `av_malloc` that allocates
/// through `posix_memalign`, `_aligned_malloc` or `memalign`. An `av_malloc`
/// allocation must never reach `LibcFree`, nor a `malloc` allocation this one.
pub struct AvFree;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `av_free`.
unsafe impl CDropped for AvFree {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract requires `obj` to denote a uniquely owned
        // allocation from the `av_malloc` family, which `av_free` accepts.
        unsafe { ffi::av_free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: `av_free` does not need the buffer length and accepts every
// allocation produced by the `av_malloc` family.
unsafe impl CLenDropped for AvFree {
    unsafe fn c_drop_len(ptr: *mut u8, _byte_len: usize) {
        // SAFETY: the trait contract requires an `av_malloc`-family allocation
        // and transfers its ownership to this call.
        unsafe { ffi::av_free(ptr.cast::<c_void>()) }
    }
}

/// Wraps: av_strdup
///
/// Deep-clones an owned NUL-terminated string into an independent allocation
/// from the `av_malloc` family. This is the ordinary clone strategy for
/// [`CrustifyStr<AvFree>`](ffibox::CrustifyStr): the source stays live, the
/// returned string owes its own [`AvFree`] drop, and allocation failure is
/// reported by [`try_clone`](ffibox::CrustifyStr::try_clone).
// SAFETY: `av_strdup` preserves the source and returns either NULL or a fresh,
// independently owned NUL-terminated allocation released by `AvFree`.
unsafe impl CCloned for AvFree {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: this impl is reached by `CrustifyStr`, whose invariant makes
        // `obj` a live NUL-terminated string. `av_strdup` reads that string,
        // preserves it, and returns a fresh allocation or NULL.
        NonNull::new(unsafe { ffi::av_strdup(obj.as_ptr().cast::<c_char>()) }.cast::<Self>())
    }
}

/// Wraps: av_strndup
///
/// Alternative `av_malloc` string strategy whose clone uses `av_strndup`.
///
/// `av_strndup` accepts an external maximum rather than recovering it itself.
/// [`CrustifyStr`](ffibox::CrustifyStr) already guarantees a terminator, so
/// this strategy recovers the exact byte length and supplies it as the bound.
/// The operation therefore copies the complete string rather than truncating
/// it, while still exercising the length-bounded primitive. Drop delegates to
/// the same allocator-matched [`av_free`](ffi::av_free) as [`AvFree`].
pub struct AvFreeWithStrndup;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `av_free`.
unsafe impl CDropped for AvFreeWithStrndup {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the strategy only adopts `av_malloc`-family strings, which
        // `av_free` accepts; ownership transfers to this call exactly once.
        unsafe { ffi::av_free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: the source is a `CrustifyStr` string, and using its recovered length
// as `av_strndup`'s bound produces an exact independent clone released by this
// strategy's `CDropped` implementation.
unsafe impl CCloned for AvFreeWithStrndup {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        let ptr = obj.as_ptr().cast::<c_char>();
        // SAFETY: this impl is reached by `CrustifyStr`, whose invariant makes
        // `ptr` live and NUL-terminated for the duration of the shared clone.
        let byte_len = unsafe { CStr::from_ptr(ptr) }.to_bytes().len();
        // SAFETY: `byte_len` is the number of readable non-NUL bytes before the
        // terminator, so `av_strndup` may inspect that range and returns a fresh
        // terminated allocation or NULL without modifying the source.
        NonNull::new(unsafe { ffi::av_strndup(ptr, byte_len) }.cast::<Self>())
    }
}

/// Wraps: av_memdup
///
/// Deep-copies a counted buffer: `av_memdup` takes `byte_len` bytes from
/// `av_malloc` and byte-copies the source into them, so the result is an
/// independent allocation of the same family that this strategy releases.
/// Consumers reach it as [`CVec::try_clone`](ffibox::CVec::try_clone) and the
/// [`Clone`] built on it, never as a free-standing function.
///
/// Only the counted owner gets a clone, and that is deliberate. A single value
/// ([`CVoidBox<AvFree>`](ffibox::CVoidBox)) has no extent to pass, and ffibox
/// gives it no clone at all. A NUL-terminated string
/// ([`CrustifyStr<AvFree>`](ffibox::CrustifyStr)) clones through
/// [`CCloned`](ffibox::CCloned), whose pointer-only signature is shaped for
/// `av_strdup` — libavutil's own string cloner, which allocates through
/// `av_realloc` and so is released by `AvFree` too. Binding `CCloned` here to a
/// `strlen`-derived `av_memdup` would take the one available impl away from the
/// primitive that matches it, so the string tier keeps it.
///
/// The copy is bytewise, which is why ffibox demands `T: Copy`: for a buffer of
/// pointers it duplicates the addresses and aliases the pointees rather than
/// copying them. That is exactly what `av_frame_ref` asks of it when it
/// duplicates `extended_data`, a buffer of channel pointers whose planes stay
/// owned by the frame's buffer references.
///
/// Cloning can fail — `av_malloc` returns NULL on OOM and for any request above
/// the `av_max_alloc` limit, `INT_MAX` by default — so ported code that checked
/// C's return uses `try_clone`; [`Clone::clone`] aborts on that path. A
/// zero-length clone is not a failure: as in [`c_drop_len`](CLenDropped),
/// `av_malloc(0)` retries at one byte, so an empty [`CVec`](ffibox::CVec)
/// clones to a live allocation that still has to be released.
// SAFETY: `av_memdup` returns an independent `av_malloc` allocation containing
// exactly the requested byte copy, and `AvFree` releases that allocation.
unsafe impl CLenCloned for AvFree {
    unsafe fn c_clone_len(ptr: *mut u8, byte_len: usize) -> Option<NonNull<u8>> {
        // SAFETY: the trait contract guarantees `byte_len` readable bytes at
        // `ptr`; `av_memdup` only reads that range and preserves the source.
        NonNull::new(unsafe { ffi::av_memdup(ptr.cast_const().cast(), byte_len) }.cast())
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use ffibox::{CVec, CVoidBox, CrustifyStr};

    use super::*;

    /// Allocate `values.len()` elements with `av_malloc` and return the counted
    /// owner over them, filled.
    ///
    /// The two tiers are the point, not ceremony, and this is the only place in
    /// the module allowed to build a typed [`CVec`] over `av_malloc` storage.
    /// `av_malloc` returns uninitialised bytes, so the fresh allocation does not
    /// yet hold `count` values of `T`, which is what
    /// [`CVec::from_raw_parts`] requires and what [`CVec::as_slice`] asserts for
    /// the whole buffer at once. [`CElem`](ffibox::CElem) is a claim about bit
    /// patterns and says nothing about uninitialised memory, so it does not
    /// license the shortcut for `u8` or for `*mut u8`. Owning the allocation as
    /// `MaybeUninit<T>` first makes the fill safe code, and the promotion an
    /// isolated step that can state why every element is now valid.
    fn av_alloc_filled<T: Copy>(values: &[T]) -> CVec<T, AvFree> {
        // SAFETY: `av_malloc` returns null or a uniquely owned allocation of at
        // least this many bytes, aligned to its ALIGN (16 at the weakest, past
        // every `T` used here), which this owner adopts exactly once. Adopting
        // it as `MaybeUninit<T>` claims nothing about the contents — that is
        // what the escape hatch is for — so the precondition holds before a
        // single byte is written, and a panic in the fill below still releases
        // the allocation through `AvFree`.
        let mut storage = unsafe {
            CVec::<MaybeUninit<T>, AvFree>::from_raw_parts(
                ffi::av_malloc(size_of_val(values)).cast(),
                values.len(),
            )
        }
        .expect("av_malloc failed");

        for (slot, value) in storage.as_mut_slice().iter_mut().zip(values) {
            slot.write(*value);
        }

        let (ptr, count) = storage.into_raw_parts();

        // SAFETY: the loop wrote every one of the `count` slots, so the
        // allocation now holds `count` contiguous initialised `T` — exactly the
        // precondition `from_raw_parts` states and `as_slice` relies on.
        // `into_raw_parts` surrendered ownership without freeing, and
        // `MaybeUninit<T>` shares `T`'s size and alignment, so the promoted
        // owner hands `AvFree` the same pointer and the same byte length the
        // uninitialised tier would have. `ptr` came out of a `NonNull`.
        unsafe { CVec::<T, AvFree>::from_raw_parts(ptr.cast::<T>(), count) }
            .expect("av_malloc failed")
    }

    #[test]
    fn drops_scalar_allocation() {
        // SAFETY: `av_malloc(1)` returns null or a uniquely owned allocation
        // compatible with `AvFree`; `CVoidBox` adopts it exactly once.
        let allocation =
            unsafe { CVoidBox::<AvFree>::from_raw(ffi::av_malloc(1)) }.expect("av_malloc failed");
        drop(allocation);
    }

    #[test]
    fn clones_and_drops_counted_buffer() {
        let original = av_alloc_filled(&[1u8, 2, 3, 4]);

        let mut cloned = original.try_clone().expect("av_memdup failed");
        assert_eq!(cloned.as_slice(), original.as_slice());
        assert_ne!(cloned.as_ptr(), original.as_ptr());

        // The copy is independent storage, not a view: writing through one
        // owner leaves the other untouched, and both are freed separately.
        cloned.as_mut_slice()[0] = 9;
        assert_eq!(original.as_slice(), &[1, 2, 3, 4]);
        assert_eq!(cloned.as_slice(), &[9, 2, 3, 4]);
    }

    #[test]
    fn clones_empty_counted_buffer() {
        // `av_memdup(p, 0)` reaches `av_malloc(0)`, which retries at one byte,
        // so an empty clone comes back live rather than as the NULL that
        // signals failure — `try_clone` must succeed and its result must still
        // be released, which the sanitiser run checks.
        let empty = av_alloc_filled::<u8>(&[]);

        let cloned = empty.try_clone().expect("av_memdup failed");
        assert!(cloned.is_empty());
        assert_ne!(cloned.as_ptr(), empty.as_ptr());
    }

    #[test]
    fn clones_pointer_buffer_by_aliasing_its_elements() {
        // The `av_frame_ref` shape: `extended_data` is a buffer of channel
        // pointers, and duplicating it with `av_memdup` copies the addresses
        // while both buffers keep pointing at the one set of planes.
        //
        // The element type does not change the construction rule. `*mut u8` is
        // a `CElem`, but that marker covers the bit patterns a pointer may
        // hold, not the absence of one: the freshly allocated slots are
        // uninitialised until written, so they too are filled through the
        // `MaybeUninit` tier before any `&[*mut u8]` covers them.
        let mut planes = [7u8, 8];
        let (first, second) = planes.split_at_mut(1);

        let table = av_alloc_filled(&[first.as_mut_ptr(), second.as_mut_ptr()]);

        let cloned = table.try_clone().expect("av_memdup failed");
        assert_ne!(cloned.as_ptr(), table.as_ptr());
        assert_eq!(cloned.as_slice(), table.as_slice());
    }

    #[test]
    fn drops_empty_counted_buffer() {
        // `av_malloc(0)` allocates one byte rather than returning null, so a
        // zero-length owner still owns storage. `c_drop_len` must therefore
        // reach `av_free` for it — a `munmap`-style zero-length short-circuit
        // would leak here, which the sanitiser run turns into a failure.
        let empty = av_alloc_filled::<u8>(&[]);
        assert!(empty.is_empty());
        drop(empty);
    }

    #[test]
    fn promotes_a_filled_buffer_without_changing_its_extent() {
        const VALUES: [u8; 4] = [1, 2, 3, 4];

        let promoted = av_alloc_filled(&VALUES);

        // What the promotion in `av_alloc_filled` has to preserve, pinned so a
        // regression shows up here rather than as a silent invalid slice: every
        // slot reads back what was written, so the typed view covers
        // initialised elements only, and the count and byte extent are the ones
        // the uninitialised tier held, so `AvFree::c_drop_len` still releases
        // exactly the allocation `av_malloc` handed out.
        assert_eq!(promoted.as_slice(), &VALUES);
        assert_eq!(promoted.count(), VALUES.len());
        assert_eq!(promoted.byte_len(), VALUES.len() * size_of::<u8>());
    }

    #[test]
    fn drops_an_unfilled_buffer_through_the_uninitialised_tier() {
        const LEN: usize = 4;

        // The other half of the escape hatch, and `av_alloc_filled`'s bail
        // path: a buffer that is never filled must still be owned and released,
        // which is only expressible while the element type is `MaybeUninit<T>`.
        // The byte extent matches the typed tier's, so the two are
        // interchangeable to `AvFree` — a mismatch would reach `av_free` as an
        // invalid free, which the sanitiser run catches.
        //
        // SAFETY: `av_malloc` returns null or a uniquely owned allocation of at
        // least `LEN` `u8`-sized slots, which this owner adopts exactly once;
        // every bit pattern, and the absence of one, is a valid
        // `MaybeUninit<u8>`, so no element is ever read as anything else.
        let unfilled = unsafe {
            CVec::<MaybeUninit<u8>, AvFree>::from_raw_parts(ffi::av_malloc(LEN).cast(), LEN)
        }
        .expect("av_malloc failed");

        assert_eq!(unfilled.byte_len(), LEN * size_of::<u8>());
        drop(unfilled);
    }

    #[test]
    fn drops_terminated_string() {
        const TEXT: &[u8] = b"crustify\0";

        // SAFETY: `TEXT` is a distinct, fully initialised source of
        // `TEXT.len()` bytes ending in a NUL, so `av_memdup` copies the
        // terminator along with the text; the result is null or a uniquely
        // owned NUL-terminated `av_malloc` string, which is what `CrustifyStr`
        // adopts with `AvFree` as its matching destructor.
        let string = unsafe {
            CrustifyStr::<AvFree>::from_raw(ffi::av_memdup(TEXT.as_ptr().cast(), TEXT.len()).cast())
        }
        .expect("av_memdup failed");

        assert_eq!(string.as_bytes(), b"crustify");
        assert_eq!(string.len(), 8);
        drop(string);
    }

    #[test]
    fn clones_terminated_string_with_av_strdup() {
        // SAFETY: the C literal is NUL-terminated and remains live throughout
        // the call. `av_strdup` returns NULL or a fresh av_malloc-family string
        // that `CrustifyStr<AvFree>` adopts exactly once.
        let original =
            unsafe { CrustifyStr::<AvFree>::from_raw(ffi::av_strdup(c"crustify".as_ptr())) }
                .expect("av_strdup failed");

        let cloned = original.try_clone().expect("av_strdup failed");
        assert_eq!(cloned.as_bytes(), original.as_bytes());
        assert_ne!(cloned.as_ptr(), original.as_ptr());
    }

    #[test]
    fn clones_terminated_string_with_av_strndup() {
        // SAFETY: the C literal is NUL-terminated and remains live throughout
        // the call. The initial `av_strndup` copies all eight non-NUL bytes and
        // returns NULL or a fresh string adopted exactly once by the matching
        // av_malloc-family strategy.
        let original = unsafe {
            CrustifyStr::<AvFreeWithStrndup>::from_raw(ffi::av_strndup(c"crustify".as_ptr(), 8))
        }
        .expect("av_strndup failed");

        let cloned = original.try_clone().expect("av_strndup failed");
        assert_eq!(cloned.as_bytes(), b"crustify");
        assert_eq!(cloned.as_bytes(), original.as_bytes());
        assert_ne!(cloned.as_ptr(), original.as_ptr());
    }
}
