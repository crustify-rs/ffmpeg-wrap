# `file.rs` — what keeps the bytes behind `av_file_map`'s slice valid

**Status: hole found, promoted to
`advisories/av-file-map-safe-mapping-of-a-mutable-file.md`.**

`file.rs` is 158 lines and was reached by none of the previous run's
harnesses — `../tmp/hammer` has no `file` binary, and
`open-enum-values-into-c.md`'s coverage table does not list the module. It was
reviewed once, in `8037d43390`, and that review is worth reading because of
what it does and does not cover.

## What the existing review settled

The commit message for `8037d43390` says:

> `AvFileUnmap`'s `CLenDropped` comment asserted how the strategy happens to be
> used rather than what it requires. It now states the obligation `munmap` adds
> over the trait's "at least `byte_len` bytes" and names the ffibox bounds that
> keep a safe path from breaking it.

That analysis is correct and I re-derived it:

* `CVec` is the only owner reaching `CLenDropped`; its sole constructor
  `from_raw_parts` is `unsafe`.
* No safe `CVec` method changes `count`. I checked the whole impl surface in
  `/opt/ffibox/src/owned_refs.rs:684-838`: `as_ptr`, `count`, `is_empty`,
  `byte_len`, `into_raw_parts`, `as_slice`, `as_mut_slice`, `as_handles`,
  `as_handles_mut`, `try_clone`, `Clone`, `Debug`. Only `try_clone`/`Clone`
  could produce a second owner, and both need `S: CLenCloned`, which
  `AvFileUnmap` deliberately does not implement — so they do not compile for
  this element/strategy pair.
* `Debug` prints `ptr`/`count`/`byte_len` and does not read the elements
  (`owned_refs.rs:829`), so `{:?}` on a mapping touches no mapped page.

So the `(ptr, byte_len)` pair `av_file_unmap` receives is exactly the pair
`av_file_map` produced. The drop half is sound.

## The half nobody stated

The review reasoned about the *extent* the mapping is released with. It never
asks what keeps the mapped **bytes** readable and unchanging for as long as the
`CVec<u8, AvFileUnmap>` lives — and nothing does. `av_file_map` is a safe `fn`,
`CVec::as_slice` is a safe method, and the `&[u8]` it yields is backed by a
`MAP_PRIVATE` mapping of a file any process, including the caller's own safe
`std::fs` code, may shrink or rewrite.

Two demonstrated consequences, both from `#![forbid(unsafe_code)]` programs:

* shrink the file, then read the slice — `AddressSanitizer: BUS`;
* rewrite a byte of the file, then read the same index of the same `&[u8]` —
  the value changes.

See the advisory for the commands and output.

## Other things in this module I looked at and cleared

* **The empty-file path.** `libavutil/file.c:88` returns success with
  `*bufptr = NULL` and `*size = 0` before ever calling `mmap`, so
  `Ok(None)` is faithful and no zero-length `munmap` is constructible. The
  `debug_assert_eq!(size, 0)` beside it is consistent with C.
* **`log_offset` / `log_context`.** Both are forwarded into a `FileLogContext`
  C builds on its own stack (`file.c:59`) and drops before returning; the
  handle is only borrowed for the call. `LogContextRef::from_ptr` is `unsafe`,
  so a caller cannot manufacture a dangling one.
* **`size` vs. the page-rounded mapping.** `mmap` rounds the length up
  internally but `munmap` takes the same unrounded `*size` C reported, and
  `munmap` rounds identically. The tail of the last page is inside the
  mapping, so `as_slice`'s `size` bytes are all mapped. This is not the bug.
* **A file larger than `SIZE_MAX`.** `file.c:78` refuses it. Unreachable here
  anyway on a 64-bit target.

## Every other place the crate hands out a view, and why none is affected

The advisory claims `file.rs` is the only wrapper that views memory outside
libavutil's own heap. That claim is this survey. Each `CSlice` / `CSliceMut` /
`CVec` construction site in the crate, and what it views:

| site | views | reachable by another process? |
|---|---|---|
| `file.rs:85` | an `mmap` of a file | **yes — this is the finding** |
| `buffer.rs:344`, `:374`, `:471`, `:493` | an `AVBufferRef`'s `av_malloc` payload | no |
| `frame.rs:270`, `:300`, `:368`, `:390` | a side-data entry's `av_buffer_alloc` payload | no |
| `channel_layout.rs:820`, `:867` | `AVChannelLayout.u.map`, an `av_malloc` array | no |
| `pixdesc.rs:287`, `:365` | the inline `comp[4]` array of a descriptor | no |
| `imgutils.rs:651` | an `av_malloc` image buffer | no |
| `samplefmt.rs:378` | an `av_malloc` sample buffer | no |
| `mem.rs:257`, `:277`, `:383`, `:612`, `:682` | `av_malloc` allocations and dynarray tables | no |
| `hwcontext.rs:710`, `:755` | an `av_malloc` format list | no |

Everything but the first views process-private heap that only this crate's own
`unsafe` can reach. So the defect is specific to `av_file_map` rather than a
pattern repeated across the crate.

Two adjacent things also checked and cleared:

* **Frame planes are not handed out as slices at all.** `AVFrameRef::data_plane`
  and `extended_data_plane` return `AVFramePlane<'a>`, which is a bare
  `NonNull<u8>` with one `as_non_null` accessor and no way to read through it
  (`frame.rs:672`). Its doc says why: "Dereferencing remains unsafe because the
  plane's extent depends on the media type, format, dimensions and signed
  stride." So the extent computation that would be the obvious place for an
  out-of-bounds slice does not exist in safe code.
* **`MaybeUninit<u8>` does not help here.** The side-data and buffer views are
  `CSlice<MaybeUninit<u8>>` precisely so that safe code cannot read
  uninitialised bytes as `u8`. That is the right call for uninitialised heap,
  but it would not have rescued `file.rs`: a `MaybeUninit<u8>` read still
  touches the page, so it would raise `SIGBUS` identically. Gating the
  constructor is the only thing that covers it.

## What would change my mind about the finding

A documented crate-level position that files under a mapping are assumed
immutable for the mapping's lifetime — there is none in `file.rs`, in
`lib.rs`, or in the campaign notes — or a change to `ffibox` that stopped
`CVec<u8, _>` from yielding a safe `&[u8]`.
