//! Ownership strategies for the C standard allocator.

use core::ffi::c_void;
use core::ptr::NonNull;

use ffibox::{CDropped, CLenDropped};

use crate::ffi;

/// Wraps: free
///
/// Releases an allocation from the C standard allocation family. The recorded
/// contract covers all three byte-level shapes the pointer may hold, and each
/// reaches `free` through the owner that matches it:
///
/// | shape | owner |
/// |---|---|
/// | single value | [`CVoidBox<LibcFree>`](ffibox::CVoidBox) |
/// | counted buffer | [`CVec<T, LibcFree>`](ffibox::CVec) |
/// | NUL-terminated string | [`CrustifyStr<LibcFree>`](ffibox::CrustifyStr) |
///
/// `free` never needs the extent, so one strategy serves all three: the
/// [`CDropped`] impl carries the pointer-only owners and the [`CLenDropped`]
/// impl the counted one.
///
/// # Building the counted owner over fresh storage
///
/// `free` itself is indifferent to what the block holds — it releases storage
/// and never reads a value out of it — so the shape table above says what an
/// allocation *may* hold, not what it *does* hold when released. The owner is
/// not indifferent. [`CVec::from_raw_parts`](ffibox::CVec::from_raw_parts)
/// requires the pointer to already hold its `count` elements, and
/// [`as_slice`](ffibox::CVec::as_slice) then materialises all of them as one
/// `&[T]`; `malloc` writes nothing whatsoever into the block it returns, so a
/// typed [`CVec`](ffibox::CVec) over a *fresh* `malloc` breaks that
/// precondition at construction, before any view is ever taken.
/// [`CElem`](ffibox::CElem) does not excuse it: that marker ranges over the bit
/// patterns the buffer may hold, and uninitialised memory is not a bit pattern
/// — read as a `u8` or a `*mut u8` it is an invalid value either way. Adopt the
/// allocation as `CVec<MaybeUninit<T>, LibcFree>`, ffibox's escape hatch for a
/// buffer C has not filled, fill it through that owner, then promote it.
/// `MaybeUninit<T>` has `T`'s size, so `LibcFree` releases the same extent from
/// either tier, and a bail before the promotion still frees the allocation.
///
/// The two allocators differ here as well as at the seam: libavutil's
/// `av_malloc` at least has a `CONFIG_MEMORY_POISONING` memset (compiled out in
/// this campaign's configuration), while `malloc` has no such path at all. The
/// singleton and string owners are unaffected —
/// [`CVoidBox`](ffibox::CVoidBox) keeps the bytes erased, and a
/// [`CrustifyStr`](ffibox::CrustifyStr) is only adopted once the terminator has
/// been written.
///
/// # Zero length
///
/// Unlike [`Munmap`](crate::mman::Munmap), the length-aware release must not
/// short-circuit on a zero byte length. `malloc(0)` may return either NULL or a
/// unique pointer that `free` still has to release; on this campaign's glibc it
/// is the latter, so a zero-length owner holds a live allocation and skipping
/// the call would leak it. The NULL outcome needs no guard either — it is
/// rejected by the owners' `from_raw*`, which return `None`.
///
/// This is deliberately distinct from libavutil's `AvFree`. The two allocation
/// families are not interchangeable — `av_free` resolves to `_aligned_free`
/// wherever `HAVE_ALIGNED_MALLOC` holds, and to a prefixed allocator under
/// `MALLOC_PREFIX` — so an `av_malloc` allocation must never reach this
/// strategy, nor a `malloc` allocation reach that one.
pub struct LibcFree;

// SAFETY: `c_drop` delegates exactly once to the allocator-matched `free`.
unsafe impl CDropped for LibcFree {
    unsafe fn c_drop(obj: NonNull<Self>) {
        // SAFETY: the trait contract requires `obj` to denote a uniquely owned
        // C-standard-library allocation, which `free` accepts.
        unsafe { ffi::free(obj.as_ptr().cast::<c_void>()) }
    }
}

// SAFETY: `free` does not need the allocation length and releases any buffer
// produced by the C standard allocation family.
unsafe impl CLenDropped for LibcFree {
    unsafe fn c_drop_len(ptr: *mut u8, _byte_len: usize) {
        // SAFETY: the trait contract transfers one C-standard-library
        // allocation to this call, so it may be released exactly once. A zero
        // `byte_len` is deliberately not short-circuited: a `malloc(0)` block
        // is live storage, and skipping the call would leak it.
        unsafe { ffi::free(ptr.cast::<c_void>()) }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use ffibox::{CVec, CVoidBox, CrustifyStr};

    use super::*;

    /// Allocate `values.len()` elements with `malloc` and return the counted
    /// owner over them, filled.
    ///
    /// The two tiers are the point, not ceremony, and this is the only place in
    /// the module allowed to build a typed [`CVec`] over `malloc` storage.
    /// `malloc` writes nothing into the block it hands back, so the fresh
    /// allocation does not yet hold `count` values of `T`, which is what
    /// [`CVec::from_raw_parts`] requires and what [`CVec::as_slice`] asserts
    /// for the whole buffer at once. [`CElem`](ffibox::CElem) is a claim about
    /// bit patterns and says nothing about uninitialised memory, so it does not
    /// license the shortcut for `u8` or for `*mut u8`. Owning the allocation as
    /// `MaybeUninit<T>` first makes the fill safe code, and the promotion an
    /// isolated step that can state why every element is now valid.
    fn malloc_filled<T: Copy>(values: &[T]) -> CVec<T, LibcFree> {
        // SAFETY: `malloc` returns null or a uniquely owned allocation of at
        // least this many bytes, aligned for any fundamental type and so past
        // every `T` used here, which this owner adopts exactly once. Adopting
        // it as `MaybeUninit<T>` claims nothing about the contents — that is
        // what the escape hatch is for — so the precondition holds before a
        // single byte is written, and a panic in the fill below still releases
        // the allocation through `LibcFree`.
        let mut storage = unsafe {
            CVec::<MaybeUninit<T>, LibcFree>::from_raw_parts(
                ffi::malloc(size_of_val(values) as _).cast(),
                values.len(),
            )
        }
        .expect("malloc failed");

        for (slot, value) in storage.as_mut_slice().iter_mut().zip(values) {
            slot.write(*value);
        }

        let (ptr, count) = storage.into_raw_parts();

        // SAFETY: the loop wrote every one of the `count` slots, so the
        // allocation now holds `count` contiguous initialised `T` — exactly the
        // precondition `from_raw_parts` states and `as_slice` relies on.
        // `into_raw_parts` surrendered ownership without freeing, and
        // `MaybeUninit<T>` shares `T`'s size and alignment, so the promoted
        // owner hands `LibcFree` the same pointer and the same byte length the
        // uninitialised tier would have. `ptr` came out of a `NonNull`.
        unsafe { CVec::<T, LibcFree>::from_raw_parts(ptr.cast::<T>(), count) }
            .expect("malloc failed")
    }

    #[test]
    fn drops_scalar_allocation() {
        // The singleton arm needs no fill: `CVoidBox` is type-erased storage
        // and its `from_raw` asks only for a valid, uniquely owned pointer, so
        // an unwritten `malloc` block satisfies it as it stands.
        //
        // SAFETY: `malloc(1)` returns null or a uniquely owned allocation of at
        // least one byte, which this `CVoidBox` adopts exactly once.
        let scalar =
            unsafe { CVoidBox::<LibcFree>::from_raw(ffi::malloc(1)) }.expect("malloc failed");
        drop(scalar);
    }

    #[test]
    fn drops_counted_buffer() {
        const VALUES: [u8; 4] = [1, 2, 3, 4];

        let buffer = malloc_filled(&VALUES);

        assert_eq!(buffer.as_slice(), &VALUES);
        drop(buffer);
    }

    #[test]
    fn promotes_a_filled_buffer_without_changing_its_extent() {
        const VALUES: [u8; 4] = [1, 2, 3, 4];

        let promoted = malloc_filled(&VALUES);

        // What the promotion in `malloc_filled` has to preserve, pinned so a
        // regression shows up here rather than as a silent invalid slice: every
        // slot reads back what was written, so the typed view covers
        // initialised elements only, and the count and byte extent are the ones
        // the uninitialised tier held, so `LibcFree::c_drop_len` still releases
        // exactly the allocation `malloc` handed out.
        assert_eq!(promoted.as_slice(), &VALUES);
        assert_eq!(promoted.count(), VALUES.len());
        assert_eq!(promoted.byte_len(), VALUES.len() * size_of::<u8>());
    }

    #[test]
    fn drops_an_unfilled_buffer_through_the_uninitialised_tier() {
        const LEN: usize = 4;

        // The other half of the escape hatch, and `malloc_filled`'s bail path:
        // a buffer that is never filled must still be owned and released, which
        // is only expressible while the element type is `MaybeUninit<T>`. The
        // byte extent matches the typed tier's, so the two are interchangeable
        // to `LibcFree` — a mismatch would reach `free` as an invalid free,
        // which the sanitiser run catches.
        //
        // SAFETY: the checked `malloc` result is a uniquely owned allocation of
        // `LEN` bytes, adopted exactly once as `LEN` `MaybeUninit<u8>` — a type
        // every bit pattern of which is valid, so the unwritten storage already
        // satisfies `from_raw_parts`.
        let unfilled = unsafe {
            CVec::<MaybeUninit<u8>, LibcFree>::from_raw_parts(ffi::malloc(LEN as _).cast(), LEN)
        }
        .expect("malloc failed");

        assert_eq!(unfilled.count(), LEN);
        assert_eq!(unfilled.byte_len(), LEN * size_of::<u8>());
        drop(unfilled);
    }

    #[test]
    fn drops_empty_counted_buffer() {
        // `malloc(0)` is allowed to return either NULL or a unique pointer, and
        // on this campaign's glibc it returns the pointer — so a zero-length
        // owner still owns storage. `c_drop_len` must therefore reach `free`
        // for it: a `Munmap`-style zero-length short-circuit would leak here,
        // which the sanitiser run turns into a failure.
        let empty = malloc_filled::<u8>(&[]);

        assert!(empty.is_empty());
        assert_eq!(empty.byte_len(), 0);
        drop(empty);
    }

    #[test]
    fn drops_terminated_string() {
        const TEXT: &[u8] = b"crustify\0";

        // SAFETY: the checked `malloc` result is a fresh allocation of exactly
        // `TEXT.len()` bytes owned by nobody else, and `TEXT` is a distinct,
        // fully initialised source of that many bytes, so the copy leaves a
        // uniquely owned NUL-terminated `malloc` string — which is what
        // `CrustifyStr` adopts, with `LibcFree` as its matching destructor.
        // Unlike the counted owner, this arm needs no `MaybeUninit` tier: the
        // copy precedes the adoption, so the string is already well-formed by
        // the time `from_raw` states its precondition.
        let string = unsafe {
            let raw = ffi::malloc(TEXT.len() as _).cast::<u8>();
            assert!(!raw.is_null(), "malloc failed");
            core::ptr::copy_nonoverlapping(TEXT.as_ptr(), raw, TEXT.len());
            CrustifyStr::<LibcFree>::from_raw(raw.cast()).expect("malloc failed")
        };

        assert_eq!(string.as_bytes(), b"crustify");
        assert_eq!(string.len(), 8);
        drop(string);
    }
}
