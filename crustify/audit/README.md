# UB audit of `crustify/rust/libavutil`

Two runs, both 2026-08-24. Run 2 audited run 1's own fixes.

| run | audited revision | advisories | fix branch (not merged, not pushed) |
|---|---|---:|---|
| 1 | `24cd1b5a06` on `crustify/libavutil-gpt-5.6-sol` | 3 | `crustify/audit-bound-c-alignment-and-comparison-arguments` @ `4505beec42` |
| 2 | `4505beec42` | 1 | `crustify/audit-gate-the-file-mapping-safe-code-cannot-keep-valid` @ `b19afcb3f0` |

Crate `libavutil` 0.0.0, unpublished.

## Confirmed (`advisories/`)

| advisory | run | instrument | consequence |
|---|---:|---|---|
| `av-frame-get-buffer-unbounded-alignment.md` | 1 | ASan (SEGV) + UBSan | **memory unsafety**: `av_frame_get_buffer` returns `Ok(())` with plane pointers ~2 GiB outside a 104-byte buffer; the next safe `av_frame_copy` dereferences them |
| `av-file-map-safe-mapping-of-a-mutable-file.md` | 2 | ASan (BUS), both halves instrumented | **memory unsafety**: a safe `&[u8]` over an `mmap`; shrinking the file faults on a read in the caller's own frame, and rewriting it changes the bytes a live shared reference yields |
| `imgutils-unbounded-alignment.md` | 1 | UBSan | signed overflow at three sites in `imgutils.c`; no memory error follows, and run 1 argues none can |
| `av-nearer-q-cmp-multiply-overflow.md` | 1 | UBSan | signed overflow at `rational.c:141` on `av_nearer_q(1/1, 1/1, 0/0)`, plus a return value outside the documented range |

The first three are one shape — a caller-supplied integer forwarded to C whose
arithmetic on it is not total, fixed by a bound. The fourth is not about an
argument at all: the call is well formed for every input and the defect is the
type the result is handed back as, fixed by a safety obligation.

## Instruments — read this before running anything

**`rust-side-asan.md` is the current instrument.** It builds one LLVM ASan that
spans the seam: a clang-built `libavutil.so` plus a `-Zsanitizer=address` Rust
half, using the compiler-rt runtimes the Rust nightly toolchain already ships,
since Debian has no `libclang-rt-19-dev`. It supersedes the `LD_PRELOAD` setup
run 1 used, and it makes both traps in `asan-startup-flakiness.md` — the
one-run-in-three `DEADLYSIGNAL` and the cargo `.rustc_info.json` poisoning —
simply not happen. `miri-cannot-cross-the-seam.md` still stands: Miri cannot
execute this crate at all.

## Chased and cleared (`notes/`)

Run 1: `handle-lifetimes.md` (lending `Mut` handles, lending iterators,
`Deref`, `Send`/`Sync`, enum transmutes) · `frame-invariant-gating.md` ·
`buffer-window-mutation.md` · `dict-and-mem.md` ·
`rational-overflow-guards.md` · `open-enum-values-into-c.md` (the harness
sweep, module by module) · `unreachable-from-safe-code.md` (`opt.rs`,
`log.rs`, `hwcontext.rs`).

Run 2: `file-mapping-validity.md` (the lead behind the fourth advisory, plus a
survey of every slice-producing site in the crate) ·
`hwcontext-with-a-real-backend.md` · `modules-with-no-harness.md` ·
`rust-side-asan.md`.

## Coverage holes

Run 1 recorded two. Both are closed:

1. ~~Nothing on the Rust side is sanitised.~~ Closed by `rust-side-asan.md`.
   Residual: ASan does not model Stacked Borrows, so a pure Rust-side
   *aliasing* violation — two live `&mut` — is still uncovered by anything
   here, and Miri cannot supply it.
2. ~~`hwcontext.rs` is untested by construction.~~ Closed by
   `hwcontext-with-a-real-backend.md`, which compiles the OpenCL backend
   against a POCL CPU device. Everything safe code reaches is clean; everything
   past `av_hwframe_ctx_alloc` turns out to be unreachable from safe code,
   because no setter exists for the frames-context configuration fields.

What is left, in rough order of value:

1. **Rust-side aliasing.** The one shape with no instrument.
   `handle-lifetimes.md` is a careful reading, not an experiment.
2. **`opt.rs`, 2462 lines, gated behind `unsafe` constructors.** Correctly
   gated, so not a soundness bug — but it is the largest module in the crate
   and nothing has executed most of it. If a safe constructor for
   `OptionObjectMut` is ever added, start there.
3. **`HWFrameTransferFormats`' terminator-excluding view**, only ever fed a
   list the module's own test fabricates, never one from a backend.

## Scratch (`tmp/`)

`hammer/` is the discovery harness — one binary per module family, each a cross
product of adversarial arguments; run 2 added `hwctx.rs`. `repro-frame-align/`,
`repro-imgutils-align/`, `repro-nearer-q/` and `repro-file-map/` are one
minimal reproduction per advisory; each depends on the audited crate, calls
only its public API, and carries `#![forbid(unsafe_code)]`.
`logs-rustasan/` holds this run's harness and test output.

Note that `repro-file-map` **no longer compiles** against the patched crate,
and that is its "after" state: the fix is a gate, so the reproductions fail
with `E0133` rather than returning an error. The `compile_fail` doctest inside
`file.rs` is what stands in for them as a regression test.
