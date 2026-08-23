//! Ownership strategies for memory allocated by libavutil.

use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;

use ffibox::{CCloned, CDropped, CLenCloned, CLenDropped};

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
