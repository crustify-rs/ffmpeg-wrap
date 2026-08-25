//! Ownership strategies for memory allocated by libavutil.

use core::ffi::{CStr, c_char, c_void};
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use ffibox::{CCloned, CDropped, CLenCloned, CLenDropped, CVec, CVoidBox, CrustifyStr};

use crate::ffi;

/// Wraps: av_free
///
/// Releases an allocation from the `av_malloc` family — `av_malloc`,
/// `av_mallocz`, `av_calloc`, `av_realloc`, `av_memdup`, `av_strdup`,
/// `av_strndup` and the dynarray helpers built on them. The recorded contract
/// covers all three byte-level shapes the pointer may hold, and each reaches
/// `av_free` through the owner that matches it:
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
/// [`CrustifyStr<AvFree>`](ffibox::CrustifyStr): the source stays live and the
/// returned string owes its own [`AvFree`] drop.
///
/// `av_strdup` sizes the copy with `strlen(s) + 1` and takes it from
/// `av_realloc`, so it returns NULL both on OOM and for a string at or above
/// the `av_max_alloc` limit `av_realloc` enforces — `INT_MAX` by default. As
/// on the counted tier, [`try_clone`](ffibox::CrustifyStr::try_clone) reports
/// that as `None` while [`Clone::clone`] aborts.
///
/// # The obligation this impl adds
///
/// [`CCloned::c_clone`] asks its caller only for "a live, valid instance of
/// `Self`", and `Self` here is a zero-sized strategy naming a destructor
/// class, so that says nothing about the bytes behind the pointer. This impl
/// narrows it: `obj` must additionally address a NUL-terminated string,
/// because `av_strdup` recovers the extent with `strlen` and an unterminated
/// source would read past the allocation.
///
/// No safe path can violate that, and the ffibox bounds are what rule it out
/// rather than a convention. [`CrustifyStr`](ffibox::CrustifyStr) is the only
/// owner whose `Clone` reaches [`CCloned`] with `AvFree`, and a terminator is
/// precisely its type invariant: [`CBox`](ffibox::CBox) additionally requires
/// [`CCell`](ffibox::CCell), which a strategy is not;
/// [`CVoidBox`](ffibox::CVoidBox) — the erased tier, whose bytes need hold no
/// terminator at all — has no `Clone` whatsoever; and [`CVec`](ffibox::CVec)
/// clones through the length-carrying [`CLenCloned`] below.
// SAFETY: given that obligation, `av_strdup` reads the source without
// modifying, freeing or retaining it, and returns either NULL or a fresh,
// independently owned NUL-terminated allocation that one `AvFree::c_drop`
// releases.
unsafe impl CCloned for AvFree {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        // SAFETY: the caller owes a live NUL-terminated `av_malloc`-family
        // string at `obj`, so `av_strdup` may run `strlen` over it and copy
        // through the terminator. It leaves the source live and hands back
        // NULL or a fresh allocation this strategy now owns.
        NonNull::new(unsafe { ffi::av_strdup(obj.as_ptr().cast::<c_char>()) }.cast::<Self>())
    }
}

/// Wraps: av_strndup
///
/// String strategy whose clone runs `av_strndup` instead of `av_strdup`. Drop
/// delegates to the same allocator-matched [`av_free`](ffi::av_free) as
/// [`AvFree`], so the allocations the two strategies hold are interchangeable
/// even though `CrustifyStr<AvFree>` and `CrustifyStr<AvFreeWithStrndup>` are
/// distinct Rust types.
///
/// `av_strndup` takes its maximum from the caller rather than recovering one,
/// and truncates at the first NUL inside that bound. Adoption is where that
/// buys something: a bounded copy out of a buffer that need not be terminated
/// at all is the one thing `av_strdup` cannot do. The bound is a read
/// obligation, not a hint — `memchr` may scan the whole range before finding
/// nothing — so the caller must supply that many readable bytes.
///
/// The clone cannot use that freedom, and saying so is the point of this type.
/// [`CrustifyStr`](ffibox::CrustifyStr) already guarantees a terminator, so
/// the only bound that preserves the string is its own `strlen`; supplying it
/// makes `av_strndup` allocate `strlen + 1` and copy every byte, which is
/// byte-for-byte what `av_strdup` produces from the same source. The two
/// clones are therefore observably identical, and this strategy exists only
/// because [`CCloned`] admits one impl per type — [`AvFree`] spends its on
/// `av_strdup`, so `av_strndup` needs a type of its own to be reachable at
/// all. Choose it to keep an adopted `av_strndup` result on the primitive the
/// ported C code called; choose [`AvFree`] otherwise, and necessarily for a
/// counted buffer, since [`CLenDropped`] and [`CLenCloned`] live only there.
///
/// Failure is reported as on every other tier: `None` from
/// [`try_clone`](ffibox::CrustifyStr::try_clone), an abort from
/// [`Clone::clone`], for OOM and for the `av_max_alloc` cap alike.
pub struct AvFreeWithStrndup;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `av_free`.
unsafe impl CDropped for AvFreeWithStrndup {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the strategy only adopts `av_malloc`-family strings, which
        // `av_free` accepts; ownership transfers to this call exactly once.
        unsafe { ffi::av_free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: the caller owes what `AvFree`'s `CCloned` impl documents — a live
// NUL-terminated `av_malloc`-family string at `obj`, which is more than
// `CCloned` itself demands and which only `CrustifyStr` can reach this impl
// with. Given it, the recovered `strlen` is a bound `av_strndup` may read in
// full, and the result is a fresh independently owned string released by this
// strategy's `CDropped`.
unsafe impl CCloned for AvFreeWithStrndup {
    unsafe fn c_clone(obj: NonNull<Self>) -> Option<NonNull<Self>> {
        let ptr = obj.as_ptr().cast::<c_char>();
        // SAFETY: the caller owes a live NUL-terminated string at `ptr`, and
        // the borrow this forms over it ends with the statement, well inside
        // the window in which the caller keeps the source alive and unwritten.
        let byte_len = unsafe { CStr::from_ptr(ptr) }.to_bytes().len();
        // SAFETY: `byte_len` counts the non-NUL bytes before the terminator, so
        // every byte of the bound is readable: `memchr` scans the whole range,
        // stays inside the allocation and finds nothing, after which
        // `av_strndup` copies all of it and terminates the copy. The source is
        // neither modified nor retained, and NULL means allocation failure.
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
/// [`CCloned`], whose pointer-only signature is shaped for
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

    #[test]
    fn clones_empty_string_with_both_strategies() {
        // The string mirror of `clones_empty_counted_buffer`. `av_strdup("")`
        // asks `av_realloc` for one byte and `av_strndup(s, 0)` reaches the
        // same call, so an empty clone comes back live rather than as the NULL
        // that signals failure — and still owns an allocation that has to be
        // released, which the sanitiser run checks. It also pins the
        // `byte_len == 0` bound on the strndup strategy's clone.

        // SAFETY: the C literal is an empty NUL-terminated string that outlives
        // the call; `av_strdup` returns NULL or a fresh av_malloc-family string
        // adopted exactly once.
        let by_strdup = unsafe { CrustifyStr::<AvFree>::from_raw(ffi::av_strdup(c"".as_ptr())) }
            .expect("av_strdup failed");
        // SAFETY: as above, with a zero bound `av_strndup` may read none of —
        // the terminator is inside the literal either way.
        let by_strndup =
            unsafe { CrustifyStr::<AvFreeWithStrndup>::from_raw(ffi::av_strndup(c"".as_ptr(), 0)) }
                .expect("av_strndup failed");

        assert!(by_strdup.is_empty());
        assert!(by_strndup.is_empty());

        let strdup_clone = by_strdup.try_clone().expect("av_strdup failed");
        let strndup_clone = by_strndup.try_clone().expect("av_strndup failed");

        assert!(strdup_clone.is_empty());
        assert!(strndup_clone.is_empty());
        assert_ne!(strdup_clone.as_ptr(), by_strdup.as_ptr());
        assert_ne!(strndup_clone.as_ptr(), by_strndup.as_ptr());
    }

    #[test]
    fn strndup_bounds_a_source_that_carries_no_terminator() {
        const TEXT: [u8; 6] = *b"abcdef";

        // What `av_strndup` can do and `av_strdup` cannot, and the reason the
        // recorded contract puts a read obligation on the caller: copy out of a
        // buffer with no terminator in it. The source is an `av_malloc`
        // allocation rather than a Rust array so that the obligation is
        // actually checked — the C side is ASan-instrumented, so the allocation
        // carries redzones and a `memchr` running past `TEXT.len()` fails the
        // run instead of passing silently.
        let source = av_alloc_filled(&TEXT);

        // SAFETY: `source` owns exactly `TEXT.len()` initialised bytes, which is
        // the bound handed over, so `memchr` and `memcpy` stay inside the
        // allocation. Finding no terminator, `av_strndup` returns NULL or a
        // fresh NUL-terminated av_malloc-family string adopted exactly once.
        let whole = unsafe {
            CrustifyStr::<AvFreeWithStrndup>::from_raw(ffi::av_strndup(
                source.as_ptr().cast(),
                TEXT.len(),
            ))
        }
        .expect("av_strndup failed");
        assert_eq!(whole.as_bytes(), b"abcdef");

        // SAFETY: as above with a shorter bound, which is also readable and
        // which `av_strndup` truncates the copy to.
        let prefix = unsafe {
            CrustifyStr::<AvFreeWithStrndup>::from_raw(ffi::av_strndup(source.as_ptr().cast(), 3))
        }
        .expect("av_strndup failed");
        assert_eq!(prefix.as_bytes(), b"abc");

        // Cloning is not truncating twice: the strategy recovers the adopted
        // string's own `strlen`, so a truncated owner clones to itself rather
        // than to something shorter still.
        let cloned = prefix.try_clone().expect("av_strndup failed");
        assert_eq!(cloned.as_bytes(), b"abc");
        assert_ne!(cloned.as_ptr(), prefix.as_ptr());
    }

    #[test]
    fn the_two_string_strategies_clone_to_the_same_bytes() {
        const TEXT: &CStr = c"crustify";

        // The load-bearing claim in `AvFreeWithStrndup`'s doc: handing
        // `av_strndup` the recovered `strlen` makes it allocate `strlen + 1`
        // and copy every byte, which is exactly what `av_strdup` does. If that
        // ever stopped holding — a bound off by one, a truncation — the choice
        // between the two strategies would become observable, so it is pinned
        // here rather than left to the prose.

        // SAFETY: `TEXT` is NUL-terminated and outlives both calls; each
        // returns NULL or a fresh av_malloc-family string adopted exactly once
        // by the strategy that matches the primitive.
        let by_strdup = unsafe { CrustifyStr::<AvFree>::from_raw(ffi::av_strdup(TEXT.as_ptr())) }
            .expect("av_strdup failed");
        // SAFETY: as above, with the literal's own length as the bound, so
        // `av_strndup` reads only bytes the literal holds.
        let by_strndup = unsafe {
            CrustifyStr::<AvFreeWithStrndup>::from_raw(ffi::av_strndup(
                TEXT.as_ptr(),
                TEXT.to_bytes().len(),
            ))
        }
        .expect("av_strndup failed");

        let strdup_clone = by_strdup.try_clone().expect("av_strdup failed");
        let strndup_clone = by_strndup.try_clone().expect("av_strndup failed");

        assert_eq!(strdup_clone.as_bytes(), TEXT.to_bytes());
        assert_eq!(strndup_clone.as_bytes(), strdup_clone.as_bytes());
        assert_eq!(strndup_clone.len(), strdup_clone.len());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynArrayAddError {
    CountOverflow,
    AllocationFailed,
}

/// A table of opaque element pointers grown by [`av_dynarray_add`].
///
/// The element count is not the allocated capacity, and that is the whole
/// reason this type exists rather than a bare
/// [`CVec<*mut T, AvFree>`](ffibox::CVec). `FF_DYNARRAY_ADD` reallocates only
/// when the count reaches a power of two, doubling it, so a table holding
/// `count` elements is backed by `count.next_power_of_two()` slots and the
/// next append writes into the spare one without asking the allocator. Hand
/// C a buffer sized to its element count instead — a
/// [`CVec::try_clone`](ffibox::CVec::try_clone) result, or any allocation the
/// caller sized itself — and an append at a count that is not a power of two
/// writes one element past the end. That is a heap overflow reached without
/// writing `unsafe`, so the invariant has to be a property of the type rather
/// than a sentence in the caller's documentation.
///
/// Every safe way to obtain one of these preserves the invariant: [`new`](Self::new)
/// starts with no allocation at all, and [`av_dynarray_add`] hands back exactly
/// the pointer and count C produced. Adopting one C grew elsewhere goes through
/// the `unsafe` [`from_raw_parts`](Self::from_raw_parts), which states the
/// capacity as a caller obligation. [`into_table`](Self::into_table) surrenders
/// the buffer, which is safe in the other direction — nothing downstream of it
/// appends.
pub struct AvDynArray<T> {
    table: Option<CVec<*mut T, AvFree>>,
}

impl<T> Default for AvDynArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AvDynArray<T> {
    /// Creates an empty table, which owns no allocation until the first add.
    #[must_use]
    pub const fn new() -> Self {
        Self { table: None }
    }

    /// Adopts a table that C already grew, in the two halves C keeps it in.
    ///
    /// # Safety
    ///
    /// - `ptr` is null, in which case `count` must be zero and the result owns
    ///   nothing, or a uniquely owned allocation from the `av_malloc` family
    ///   whose first `count` element slots hold initialized pointers.
    /// - A non-null `ptr` must have the capacity `FF_DYNARRAY_ADD` assumes:
    ///   at least `count.next_power_of_two()` element slots, which is what
    ///   `av_dynarray_add` leaves behind and what a buffer sized to `count`
    ///   does not have.
    #[must_use]
    pub unsafe fn from_raw_parts(ptr: *mut *mut T, count: usize) -> Self {
        Self {
            // SAFETY: the caller transferred a uniquely owned av_malloc-family
            // allocation holding `count` initialized element pointers, which is
            // what this owner requires; a null pointer becomes no owner at all.
            table: unsafe { CVec::from_raw_parts(ptr, count) },
        }
    }

    /// Surrenders the underlying buffer, which holds `count` elements.
    #[must_use]
    pub fn into_table(self) -> Option<CVec<*mut T, AvFree>> {
        self.table
    }

    /// Number of elements appended so far.
    #[must_use]
    pub fn count(&self) -> usize {
        self.table.as_ref().map_or(0, CVec::count)
    }

    /// Whether no element has been appended.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// The appended element pointers, in order.
    #[must_use]
    pub fn as_slice(&self) -> &[*mut T] {
        self.table.as_ref().map_or(&[], CVec::as_slice)
    }
}

/// Wraps: av_dynarray_add
///
/// Appends one opaque element pointer, growing the owned table geometrically.
/// The allocation is owned by [`AvFree`]; element pointees are never
/// dereferenced or freed by C, and are only copied as values. On allocation
/// failure C frees the old table and resets the count, so the array is left
/// empty and the elements already in it are gone.
pub fn av_dynarray_add<T>(
    array: &mut AvDynArray<T>,
    element: Option<NonNull<T>>,
) -> Result<(), DynArrayAddError> {
    let count = array.count();
    let mut count = i32::try_from(count).map_err(|_| DynArrayAddError::CountOverflow)?;
    let mut pointer = array
        .table
        .take()
        .map_or(core::ptr::null_mut(), |owner| owner.into_raw_parts().0);

    // SAFETY: `pointer` is null, or the uniquely owned av_malloc-family table
    // this type's invariant guarantees C itself grew — so it has the
    // `count.next_power_of_two()` slots `FF_DYNARRAY_ADD` assumes before it
    // writes element `count`. Both local slots are writable and distinct, and
    // `element` is only copied as an opaque value: C never dereferences it and
    // retains it no further than the table it hands back.
    unsafe {
        ffi::av_dynarray_add(
            (&raw mut pointer).cast::<c_void>(),
            &raw mut count,
            element
                .map_or(core::ptr::null_mut(), NonNull::as_ptr)
                .cast(),
        )
    };

    if pointer.is_null() {
        debug_assert_eq!(count, 0);
        return Err(DynArrayAddError::AllocationFailed);
    }
    let count = usize::try_from(count).expect("av_dynarray_add returned a negative count");
    // SAFETY: on success C returns the uniquely owned av_malloc-family table
    // containing exactly `count` initialized pointer elements.
    array.table = unsafe { CVec::from_raw_parts(pointer, count) };
    Ok(())
}

mod sealed {
    pub trait Sealed {}
}

/// Owner shapes that can be consumed by [`av_freep`]. This trait is sealed so
/// an allocator-incompatible owner cannot opt into the safe freeing surface.
pub trait AvFreepTarget: sealed::Sealed {
    #[doc(hidden)]
    fn free_with_av_freep(self);
}

fn freep_raw(mut pointer: *mut c_void) {
    // SAFETY: every caller surrendered one av_malloc-family owner into this
    // local pointer slot; `av_freep` consumes it and writes NULL to the slot.
    unsafe { ffi::av_freep((&raw mut pointer).cast::<c_void>()) }
}

impl sealed::Sealed for CVoidBox<AvFree> {}
impl AvFreepTarget for CVoidBox<AvFree> {
    fn free_with_av_freep(self) {
        freep_raw(self.into_raw());
    }
}

impl<T> sealed::Sealed for CVec<T, AvFree> {}
impl<T> AvFreepTarget for CVec<T, AvFree> {
    fn free_with_av_freep(self) {
        freep_raw(self.into_raw_parts().0.cast());
    }
}

impl sealed::Sealed for CrustifyStr<AvFree> {}
impl AvFreepTarget for CrustifyStr<AvFree> {
    fn free_with_av_freep(self) {
        freep_raw(self.into_raw().cast());
    }
}

impl sealed::Sealed for CrustifyStr<AvFreeWithStrndup> {}
impl AvFreepTarget for CrustifyStr<AvFreeWithStrndup> {
    fn free_with_av_freep(self) {
        freep_raw(self.into_raw().cast());
    }
}

/// Wraps: av_freep
///
/// Consumes an allocator-matched owner and leaves its Rust option empty, the
/// typed equivalent of freeing a C pointer slot and setting it to NULL.
pub fn av_freep<T: AvFreepTarget>(slot: &mut Option<T>) {
    if let Some(owner) = slot.take() {
        owner.free_with_av_freep();
    }
}

/// Wraps: av_malloc
///
/// Allocates an opaque, uniquely owned byte block. Its bytes remain inaccessible
/// until a higher-level wrapper establishes their initialization and type.
#[must_use]
pub fn av_malloc(size: usize) -> Option<CVoidBox<AvFree>> {
    // SAFETY: a non-null result is a fresh uniquely owned av_malloc-family
    // allocation and `AvFree` is its matching destructor.
    unsafe { CVoidBox::from_raw(ffi::av_malloc(size)) }
}

/// Wraps: av_malloc_array
///
/// Checks multiplication in C and returns an opaque owned allocation.
#[must_use]
pub fn av_malloc_array(count: usize, element_size: usize) -> Option<CVoidBox<AvFree>> {
    // SAFETY: a non-null result is a fresh uniquely owned av_malloc-family
    // allocation and `AvFree` is its matching destructor.
    unsafe { CVoidBox::from_raw(ffi::av_malloc_array(count, element_size)) }
}

/// Wraps: av_mallocz
///
/// Allocates an opaque owned block whose requested extent is zero-filled.
#[must_use]
pub fn av_mallocz(size: usize) -> Option<CVoidBox<AvFree>> {
    // SAFETY: a non-null result is a fresh uniquely owned av_malloc-family
    // allocation and `AvFree` is its matching destructor.
    unsafe { CVoidBox::from_raw(ffi::av_mallocz(size)) }
}

/// A failed [`av_realloc`]. A non-null input allocation is returned to the
/// caller because the C failure path leaves it live and unchanged.
#[derive(Debug)]
pub struct ReallocError {
    pub allocation: Option<CVoidBox<AvFree>>,
}

/// Wraps: av_realloc
///
/// Resizes opaque av_malloc-family storage while preserving ownership on both
/// success and failure. Passing `None` models C's null allocation input.
pub fn av_realloc(
    allocation: Option<CVoidBox<AvFree>>,
    size: usize,
) -> Result<CVoidBox<AvFree>, ReallocError> {
    let original = allocation.map_or(core::ptr::null_mut(), CVoidBox::into_raw);
    // SAFETY: `original` is null or a uniquely owned av_malloc-family
    // allocation surrendered for this call.
    let resized = unsafe { ffi::av_realloc(original, size) };
    if resized.is_null() {
        // SAFETY: on realloc failure a non-null input remains live, uniquely
        // owned and allocator-compatible; reconstructing restores its owner.
        let allocation = unsafe { CVoidBox::from_raw(original) };
        Err(ReallocError { allocation })
    } else {
        // SAFETY: success transfers the one resized allocation back, with
        // `AvFree` still its matching destructor.
        Ok(unsafe { CVoidBox::from_raw(resized) }.expect("non-null realloc result"))
    }
}

#[cfg(test)]
mod scheduled_symbol_tests {
    use super::*;

    #[test]
    fn allocation_wrappers_own_and_resize_storage() {
        let allocation = av_malloc(8).expect("av_malloc failed");
        let allocation = av_realloc(Some(allocation), 32).expect("av_realloc failed");
        drop(allocation);
        drop(av_malloc_array(4, 8).expect("av_malloc_array failed"));
        drop(av_mallocz(16).expect("av_mallocz failed"));
    }

    #[test]
    fn freep_empties_owner_slot() {
        let mut allocation = av_malloc(8);
        assert!(allocation.is_some());
        av_freep(&mut allocation);
        assert!(allocation.is_none());
        av_freep(&mut allocation);
    }

    #[test]
    fn dynarray_grows_an_owned_pointer_table() {
        let mut first = 1_i32;
        let mut second = 2_i32;
        let mut array = AvDynArray::new();
        assert!(array.is_empty());
        av_dynarray_add(&mut array, Some(NonNull::from(&mut first))).expect("first add");
        av_dynarray_add(&mut array, Some(NonNull::from(&mut second))).expect("second add");
        assert_eq!(array.count(), 2);
        assert_eq!(array.as_slice()[0], &raw mut first);
        assert_eq!(array.as_slice()[1], &raw mut second);

        let table = array.into_table().expect("allocated table");
        assert_eq!(table.count(), 2);
    }

    #[test]
    fn dynarray_keeps_growing_past_a_non_power_of_two_count() {
        // The reason [`AvDynArray`] exists. C reallocates only when the count
        // reaches a power of two, so after three adds the table holds three
        // elements in four slots and the fourth add writes the spare one with
        // no allocator call at all. Routing every append through the owner C
        // itself grew is what keeps that write in bounds; appending to a
        // buffer sized to its element count — a `try_clone` result, say —
        // overflows the allocation by one element, which the sanitiser run
        // reports against `mem.c`.
        let mut values = [1_i32, 2, 3, 4, 5];
        let mut array = AvDynArray::new();
        for value in &mut values {
            av_dynarray_add(&mut array, Some(NonNull::from(value))).expect("add");
        }

        assert_eq!(array.count(), values.len());
        for (slot, value) in array.as_slice().iter().zip(&mut values) {
            assert_eq!(*slot, &raw mut *value);
        }
    }

    #[test]
    fn dynarray_survives_a_round_trip_through_the_raw_seam() {
        // What a ported C caller holds: the `void **` and the `int`, which the
        // adopting seam takes back apart and the owner reassembles. The
        // capacity travels with the pointer, so appending after the round trip
        // is the same in-bounds write it would have been without it.
        let mut values = [1_i32, 2, 3];
        let mut array = AvDynArray::new();
        for value in &mut values {
            av_dynarray_add(&mut array, Some(NonNull::from(value))).expect("add");
        }

        let (pointer, count) = array
            .into_table()
            .expect("allocated table")
            .into_raw_parts();
        assert_eq!(count, 3);

        // SAFETY: `pointer` and `count` are the halves C itself produced and
        // this test has not touched, so the allocation still holds three
        // initialized element pointers in the four slots `FF_DYNARRAY_ADD`
        // gave it, and ownership passes here exactly once.
        let mut array = unsafe { AvDynArray::from_raw_parts(pointer, count) };
        assert_eq!(array.count(), 3);

        let mut extra = 4_i32;
        av_dynarray_add(&mut array, Some(NonNull::from(&mut extra))).expect("add after adoption");
        assert_eq!(array.count(), 4);
        assert_eq!(array.as_slice()[3], &raw mut extra);
    }

    #[test]
    fn dynarray_accepts_a_null_element() {
        // `elem` is copied as an opaque value, so NULL is an ordinary entry
        // rather than a failure: C stores it and increments the count.
        let mut array = AvDynArray::<u8>::new();
        av_dynarray_add(&mut array, None).expect("add");
        assert_eq!(array.count(), 1);
        assert!(array.as_slice()[0].is_null());
    }
}

/// Wraps: av_calloc
///
/// Returns the complete zero-filled extent as bytes. Keeping the element type
/// erased is intentional: all-zero is not a valid value of every Rust type.
#[must_use]
pub fn av_calloc(count: usize, element_size: usize) -> Option<CVec<u8, AvFree>> {
    let byte_len = count.checked_mul(element_size)?;
    // SAFETY: a non-null result is a fresh av_malloc-family allocation whose
    // requested `byte_len` bytes C initialized to zero.
    unsafe { CVec::from_raw_parts(ffi::av_calloc(count, element_size).cast(), byte_len) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynArray2AddError {
    EmptyElement,
    ElementSizeChanged,
    CountOverflow,
    LengthOverflow,
    AllocationFailed,
}

/// An initialized byte array grown with `av_dynarray2_add`.
pub struct AvByteDynArray {
    storage: Option<CVec<u8, AvFree>>,
    element_size: usize,
    elements: usize,
}

impl AvByteDynArray {
    #[must_use]
    pub const fn new(element_size: usize) -> Self {
        Self {
            storage: None,
            element_size,
            elements: 0,
        }
    }

    #[must_use]
    pub fn element_size(&self) -> usize {
        self.element_size
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements == 0
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.storage.as_ref().map_or(&[], CVec::as_slice)
    }
}

/// Wraps: av_dynarray2_add
///
/// Appends one fully initialized byte element. The dedicated owner preserves
/// the geometric spare-capacity invariant required by the C growth macro.
pub fn av_dynarray2_add(
    array: &mut AvByteDynArray,
    element: &[u8],
) -> Result<(), DynArray2AddError> {
    if element.is_empty() {
        return Err(DynArray2AddError::EmptyElement);
    }
    if array.element_size == 0 {
        array.element_size = element.len();
    } else if array.element_size != element.len() {
        return Err(DynArray2AddError::ElementSizeChanged);
    }
    let next_elements = array
        .elements
        .checked_add(1)
        .ok_or(DynArray2AddError::CountOverflow)?;
    next_elements
        .checked_mul(array.element_size)
        .ok_or(DynArray2AddError::LengthOverflow)?;
    let mut count = i32::try_from(array.elements).map_err(|_| DynArray2AddError::CountOverflow)?;
    let mut pointer = array.storage.take().map_or(core::ptr::null_mut(), |owner| {
        owner.into_raw_parts().0.cast::<c_void>()
    });
    // SAFETY: `pointer` is null or the uniquely owned allocation this type only
    // ever obtains from the same geometric C grower. `element` supplies exactly
    // one initialized element and both local slots are writable.
    let appended = unsafe {
        ffi::av_dynarray2_add(
            &raw mut pointer,
            &raw mut count,
            array.element_size,
            element.as_ptr(),
        )
    };
    if appended.is_null() {
        debug_assert!(pointer.is_null());
        array.elements = 0;
        return Err(DynArray2AddError::AllocationFailed);
    }
    let elements = usize::try_from(count).expect("C returned a negative dynarray count");
    let byte_len = elements
        .checked_mul(array.element_size)
        .ok_or(DynArray2AddError::LengthOverflow)?;
    // SAFETY: C returned a unique av_malloc-family allocation and initialized
    // the newly appended element; all earlier logical elements were initialized
    // by prior calls. Spare capacity remains outside this logical byte count.
    array.storage = unsafe { CVec::from_raw_parts(pointer.cast(), byte_len) };
    array.elements = elements;
    Ok(())
}

/// Wraps: av_dynarray_add_nofree
///
/// Like [`av_dynarray_add`], but allocation failure preserves the old table.
pub fn av_dynarray_add_nofree<T>(
    array: &mut AvDynArray<T>,
    element: Option<NonNull<T>>,
) -> Result<(), DynArrayAddError> {
    let old_count = array.count();
    let mut count = i32::try_from(old_count).map_err(|_| DynArrayAddError::CountOverflow)?;
    let mut pointer = array
        .table
        .take()
        .map_or(core::ptr::null_mut(), |owner| owner.into_raw_parts().0);
    let old_pointer = pointer;
    // SAFETY: the array invariant supplies the geometric capacity C assumes;
    // C only copies the opaque element pointer and does not retain it elsewhere.
    let status = unsafe {
        ffi::av_dynarray_add_nofree(
            (&raw mut pointer).cast(),
            &raw mut count,
            element
                .map_or(core::ptr::null_mut(), NonNull::as_ptr)
                .cast(),
        )
    };
    if status < 0 {
        // SAFETY: `av_realloc` failure leaves the original allocation live and
        // unchanged; ownership is restored with its original logical count.
        array.table = unsafe { CVec::from_raw_parts(old_pointer, old_count) };
        return Err(DynArrayAddError::AllocationFailed);
    }
    let count = usize::try_from(count).expect("C returned a negative dynarray count");
    // SAFETY: success returns the unique grown table with `count` initialized
    // pointer elements and the same geometric capacity invariant.
    array.table = unsafe { CVec::from_raw_parts(pointer, count) };
    Ok(())
}

/// Owned storage managed by the `av_fast_*` family.
pub struct AvFastBuffer {
    storage: Option<CVec<MaybeUninit<u8>, AvFree>>,
}

impl Default for AvFastBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AvFastBuffer {
    #[must_use]
    pub const fn new() -> Self {
        Self { storage: None }
    }
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.storage.as_ref().map_or(0, CVec::count)
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.storage.is_none()
    }
}

fn fast_malloc(buffer: &mut AvFastBuffer, min_size: usize, zeroed: bool) -> bool {
    let mut size = match u32::try_from(buffer.capacity()) {
        Ok(size) => size,
        Err(_) => return false,
    };
    let mut pointer = buffer
        .storage
        .take()
        .map_or(core::ptr::null_mut(), |owner| {
            owner.into_raw_parts().0.cast::<c_void>()
        });
    // SAFETY: the pointer slot contains null or one unique av_malloc-family
    // allocation, paired with its exact capacity. C consumes/replaces it only
    // when growth is required and writes both local slots consistently.
    unsafe {
        if zeroed {
            ffi::av_fast_mallocz((&raw mut pointer).cast(), &raw mut size, min_size);
        } else {
            ffi::av_fast_malloc((&raw mut pointer).cast(), &raw mut size, min_size);
        }
    }
    // SAFETY: C returned null or unique av_malloc-family storage of `size`
    // bytes. `MaybeUninit<u8>` makes no initialization assertion.
    buffer.storage = unsafe { CVec::from_raw_parts(pointer.cast(), size as usize) };
    !pointer.is_null()
}

/// Wraps: av_fast_malloc
pub fn av_fast_malloc(buffer: &mut AvFastBuffer, min_size: usize) -> bool {
    fast_malloc(buffer, min_size, false)
}

/// Wraps: av_fast_mallocz
pub fn av_fast_mallocz(buffer: &mut AvFastBuffer, min_size: usize) -> bool {
    fast_malloc(buffer, min_size, true)
}

/// Wraps: av_fast_realloc
///
/// On failure the old allocation is restored to `buffer` rather than leaked.
pub fn av_fast_realloc(buffer: &mut AvFastBuffer, min_size: usize) -> bool {
    let old_count = buffer.capacity();
    let mut size = match u32::try_from(old_count) {
        Ok(size) => size,
        Err(_) => return false,
    };
    let original = buffer
        .storage
        .take()
        .map_or(core::ptr::null_mut(), |owner| {
            owner.into_raw_parts().0.cast::<c_void>()
        });
    // SAFETY: `original` is null or unique av_malloc-family storage paired with
    // `size`; C returns that allocation, its resized successor, or null while
    // leaving the original live.
    let resized = unsafe { ffi::av_fast_realloc(original, &raw mut size, min_size) };
    if resized.is_null() {
        // SAFETY: on failure av_realloc leaves the old allocation live.
        buffer.storage = unsafe { CVec::from_raw_parts(original.cast(), old_count) };
        false
    } else {
        // SAFETY: success returns unique storage of the reported capacity.
        buffer.storage = unsafe { CVec::from_raw_parts(resized.cast(), size as usize) };
        true
    }
}

/// Wraps: av_max_alloc
pub fn av_max_alloc(maximum: usize) {
    // SAFETY: C atomically updates a process-global numeric limit.
    unsafe { ffi::av_max_alloc(maximum) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackPointerError {
    RangeOverflow,
    InvalidBackDistance,
    DestinationTooSmall,
}

/// Wraps: av_memcpy_backptr
pub fn av_memcpy_backptr(
    buffer: &mut [u8],
    destination: usize,
    back: usize,
    count: usize,
) -> Result<(), BackPointerError> {
    if back == 0 || back > destination {
        return Err(BackPointerError::InvalidBackDistance);
    }
    let end = destination
        .checked_add(count)
        .ok_or(BackPointerError::RangeOverflow)?;
    if end > buffer.len() {
        return Err(BackPointerError::DestinationTooSmall);
    }
    let back = i32::try_from(back).map_err(|_| BackPointerError::RangeOverflow)?;
    let count = i32::try_from(count).map_err(|_| BackPointerError::RangeOverflow)?;
    // SAFETY: the checked range provides `back` initialized bytes before the
    // destination and `count` writable bytes at it. C's overlap algorithm is
    // specifically defined to repeat from that prefix.
    unsafe { ffi::av_memcpy_backptr(buffer.as_mut_ptr().add(destination), back, count) }
    Ok(())
}

#[cfg(test)]
mod scheduled_more_tests {
    use super::*;

    #[test]
    fn calloc_and_byte_dynarray_own_initialized_bytes() {
        let zeroes = av_calloc(4, 3).expect("calloc");
        assert_eq!(zeroes.as_slice(), &[0; 12]);
        let mut array = AvByteDynArray::new(2);
        av_dynarray2_add(&mut array, &[1, 2]).unwrap();
        av_dynarray2_add(&mut array, &[3, 4]).unwrap();
        assert_eq!(array.as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn nofree_array_and_fast_buffers_preserve_ownership() {
        let mut value = 7;
        let mut array = AvDynArray::new();
        av_dynarray_add_nofree(&mut array, Some(NonNull::from(&mut value))).unwrap();
        assert_eq!(array.count(), 1);
        let mut buffer = AvFastBuffer::new();
        assert!(av_fast_malloc(&mut buffer, 10));
        assert!(buffer.capacity() >= 10);
        assert!(av_fast_realloc(&mut buffer, 100));
        assert!(buffer.capacity() >= 100);
        assert!(av_fast_mallocz(&mut buffer, 200));
    }

    #[test]
    fn copies_from_an_initialized_back_reference() {
        let mut bytes = *b"abc.........";
        av_memcpy_backptr(&mut bytes, 3, 3, 9).unwrap();
        assert_eq!(&bytes, b"abcabcabcabc");
        assert_eq!(
            av_memcpy_backptr(&mut bytes, 2, 3, 1),
            Err(BackPointerError::InvalidBackDistance)
        );
    }
}
