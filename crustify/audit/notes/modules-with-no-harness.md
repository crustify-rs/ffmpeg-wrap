# The four modules no harness ever listed

**Status: `file.rs` became an advisory; the other three are cleared, and this
note exists so a later run does not have to notice the gap again.**

`open-enum-values-into-c.md`'s coverage table names harnesses per module
family, and `unreachable-from-safe-code.md` accounts for `opt.rs`, `log.rs` and
`hwcontext.rs`. Cross-referencing those against `src/*.rs` leaves four modules
mentioned by neither:

| module | lines | verdict |
|---|---:|---|
| `file.rs` | 158 | **advisory** — `av-file-map-safe-mapping-of-a-mutable-file.md` |
| `side_data.rs` | 59 | cleared |
| `avutil.rs` | 207 | cleared |
| `version.rs` | 49 | cleared |

## `side_data.rs`

One function, `av_frame_side_data_name`. It calls C, null-checks the result,
and wraps a non-null one as `&'static CStr`. The `'static` is right: C returns
`side_data_props[type].name`, a `static const` table of string literals in
`libavutil/side_data.c`, so the lifetime is the process's. The open newtype
means an out-of-table `AVFrameSideDataType` can be passed, and C answers null
for it — which the module's own third test pins using
`ffi::AVFrameSideDataType::MAX`. Nothing to hammer.

## `avutil.rs`

Two `#[repr(transparent)]` integer newtypes, `AVPictureType` and `AVMediaType`,
with `const from_raw` / `as_raw` and `From` both ways. No FFI calls except in
its tests. This is the open-enum pattern `handle-lifetimes.md` §5 already
cleared, and the consumers of those values were hammered under
`open-enum-values-into-c.md` — `av_get_media_type_string` in `misc.rs`,
`pict_type` through `av_frame_copy_props` in the module's own test.

## `version.rs`

`av_version_info`, `avutil_configuration`, `avutil_license` — each returns a
compile-time string literal from C as `&'static CStr`, and `avutil_version`
returns a `u32`. No arguments, so nothing to fuzz.

## How all three were exercised anyway

They carry unit tests, and the whole `libavutil` test binary was run under the
combined Rust+C AddressSanitizer described in `rust-side-asan.md`, with
`detect_leaks=1` and `--test-threads=1`: 215 passed, 0 failed, 0 leaks, and the
only sanitiser output was the `rational.c:185` line the crate documents and
accepts. So these modules are covered by instrumented execution even though no
adversarial harness targets them — which is proportionate, because none of them
takes an argument C computes an extent from.

## What would change my mind

A new function in any of them that takes a caller-supplied integer or buffer.
All four are currently pure lookups.
