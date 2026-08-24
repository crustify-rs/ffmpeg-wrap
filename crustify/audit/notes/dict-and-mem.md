# `dict.rs` owner slot, cursor identity, and `mem.rs` dynarray capacity

**Status: cleared.**

Three things in these two modules are load-bearing and worth recording as
checked rather than re-derived.

## `Dictionary::with_owner_slot`

`av_dict_set` frees the header and stores NULL when the last entry goes, so an
owner that held a stable `AVDictionary *` could not model the type. The wrapper
moves the pointer out into a local slot, runs the C entry point on that slot,
and **unconditionally** re-adopts (`dict.rs:190`). The unconditional re-adopt
is what makes the early returns safe. `av_dict_copy`/`set`/`set_int`/
`parse_string` all clear `AV_DICT_DONT_STRDUP_{KEY,VAL}` first, which is
necessary because a `&CStr` can never be transferred to C's ownership.

## Cursor identity

`av_dict_get`/`av_dict_iterate` take a `DictionaryEntry<'a>` that carries the
`AVDictionaryRef` it came from, and `checked_previous` (`dict.rs:238`) compares
the two dictionary pointers before the call. That is not decoration:
`av_dict_iterate` recovers the cursor index by pointer subtraction, so a
foreign cursor would be UB. The check is present and correct.

## Aliasing `destination` and `source` in `av_dict_copy`

Ruled out by the signature: `&mut Dictionary` and `Option<AVDictionaryRef<'_>>`
cannot both be obtained, because `Dictionary::as_ref` borrows `&self`.

## `AvDynArray` capacity

`FF_DYNARRAY_ADD` reallocates only at powers of two, so a table holding `count`
elements is backed by `count.next_power_of_two()` slots and the next append
writes into the spare one *without* asking the allocator. Handing C a buffer
sized to its element count is therefore a one-element heap overflow. The crate
makes that a type invariant (`AvDynArray`, `mem.rs:551`) rather than a doc
sentence: the only safe constructors are `new()` and `av_dynarray_add` itself,
`from_raw_parts` is `unsafe` and states the capacity obligation, and
`into_table` is one-way. Correct.

Empirically: `../tmp/hammer/src/bin/dictmem.rs` runs the dictionary separators
x flags x inputs cross product (6 x 6 x 8 x 9 = 2592 sequences, each with
set/set_int/count/get_string/iterate/get/copy/remove) and grows an
`AvDynArray` to 5000 elements, cloning the resulting `CVec`. Zero ASan/UBSan
reports.
