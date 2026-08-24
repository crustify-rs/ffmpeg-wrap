# `av_file_map` is a safe `fn` that hands back a `&[u8]` over a file mapping, so shrinking or rewriting the file from safe code invalidates a live shared reference

* **Crate:** `libavutil` 0.0.0 (`crustify/rust/libavutil`), repo revision
  `4505beec42ca36c9e4993eecc5f11557d4a53bb1`, branch
  `crustify/audit-bound-c-alignment-and-comparison-arguments`.
* **Instrument:** AddressSanitizer, LLVM, instrumenting **both** halves — a
  `clang-asan-ubsan` build of `libavutil.so` and a
  `rustc -Zsanitizer=address` build of the Rust crate and the reproduction.
  How that build is made is in `../notes/rust-side-asan.md`; it is new in this
  run, and it is what puts the faulting frame in the caller's own Rust code
  rather than in an unsymbolised `SIGBUS`.
* **Reproduction:** `crustify/audit/tmp/repro-file-map/` — two binaries,
  both `#![forbid(unsafe_code)]`, depending on the audited crate and calling
  only its public API.
* **Lead note:** `../notes/file-mapping-validity.md`.
* **Severity:** memory unsafety — a live `&[u8]` reachable with no `unsafe` in
  the caller stops being dereferenceable, and separately changes value.

## The path from safe code

There is only one step. `libavutil::file::av_file_map` is safe:

```rust
// crustify/rust/libavutil/src/file.rs:56
pub fn av_file_map(
    filename: &CStr,
    log_offset: i32,
    log_context: Option<LogContextRef<'_>>,
) -> Result<Option<CVec<u8, AvFileUnmap>>, i32> {
```

and `ffibox::CVec::<u8, _>::as_slice` is safe (`/opt/ffibox/src/owned_refs.rs:751`):

```rust
pub fn as_slice(&self) -> &[T] {
    // SAFETY: `count` contiguous, initialised elements at `ptr` per
    // `from_raw_parts`, each valid by `T: CElem`; bound by `&self`.
    unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.count) }
}
```

So a caller writes `av_file_map(path, 0, None)?` and holds a `&[u8]`. What
backs those bytes is a `MAP_PRIVATE` mapping of a file on disk
(`libavutil/file.c:92`):

```c
ptr = mmap(NULL, *size, PROT_READ|PROT_WRITE, MAP_PRIVATE, fd, 0);
```

Nothing owns the file. Any process — including the caller's own
`std::fs`, which is also safe — may shrink it or rewrite it. Rust's
guarantees for `&[u8]` (dereferenceable for the whole extent, and immutable
for the borrow) both depend on something the wrapper cannot enforce and does
not document.

This is the reason `memmap2::Mmap::map` is an `unsafe fn` and has been since
the crate existed. The wrapper reaches the same construct through a safe door.

## What the instrument says

### 1. The file shrinks: the reference stops being dereferenceable

`crustify/audit/tmp/repro-file-map/src/bin/truncate.rs`, in full, is 8 KiB of
`0xAB`, a map, a `set_len(0)`, and a read:

```rust
#![forbid(unsafe_code)]
let mapping = libavutil::file::av_file_map(cpath, 0, None).unwrap().unwrap();
let bytes: &[u8] = mapping.as_slice();
println!("mapped {} bytes; bytes[8000] = {:#x}", bytes.len(), bytes[8000]);

// Safe std. No `unsafe` anywhere in this program.
fs::OpenOptions::new().write(true).open(path).unwrap().set_len(0).unwrap();

let bytes: &[u8] = mapping.as_slice();
let value = bytes[8000];                       // <- src/bin/truncate.rs:45
println!("bytes[8000] = {value:#x}");
```

The byte is loaded into a local rather than passed straight to `println!`, only
so that the faulting frame is the reproduction's own `main` instead of the
formatting machinery `println!` would otherwise hand a reference into. Either
spelling faults.

```
$ cd crustify/audit/tmp/repro-file-map
$ RUSTFLAGS="-Zsanitizer=address -Cdebuginfo=2 -Clink-arg=-Wl,--export-dynamic" \
    cargo +nightly build --target x86_64-unknown-linux-gnu
$ LD_LIBRARY_PATH=/tmp/ffsrc-clang/libavutil ASAN_OPTIONS=detect_leaks=0 \
    ./target/x86_64-unknown-linux-gnu/debug/truncate

mapped 8192 bytes; bytes[8000] = 0xab
file truncated to 0 while the safe slice is still live
about to read bytes[8000] from a safe &[u8]
AddressSanitizer:DEADLYSIGNAL
=================================================================
==44028==ERROR: AddressSanitizer: BUS on unknown address (pc 0x63b37d8a25ca bp 0x7ffcd6936040 sp 0x7ffcd6935de0 T0)
==44028==The signal is caused by a READ memory access.
==44028==HINT: this fault was caused by a dereference of a high value address (see register values below).  Disassemble the provided pc to learn which register was used.
    #0 0x63b37d8a25ca  (.../debug/truncate+0x2085ca) (BuildId: c71a0327d22d55f37148f6fe11f7cca50cd48ba9)
    #1 0x63b37d8a363a  (.../debug/truncate+0x20963a) (BuildId: c71a0327d22d55f37148f6fe11f7cca50cd48ba9)
    #2 0x63b37d8a4cad  (.../debug/truncate+0x20acad) (BuildId: c71a0327d22d55f37148f6fe11f7cca50cd48ba9)
    ...
SUMMARY: AddressSanitizer: BUS (.../debug/truncate+0x2085ca) (BuildId: c71a0327d22d55f37148f6fe11f7cca50cd48ba9)
==44028==ABORTING
```

This image has no `llvm-symbolizer`, so ASan prints bare `binary+0xoffset`
frames. GNU `addr2line` resolves them:

```
$ addr2line -f -C -e ./target/x86_64-unknown-linux-gnu/debug/truncate \
    0x2085ca 0x20963a 0x20acad
truncate::main
    /work/ffmpeg/crustify/audit/tmp/repro-file-map/src/bin/truncate.rs:45
<fn() as core::ops::function::FnOnce<()>>::call_once
    library/core/src/ops/function.rs:250
std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
    library/std/src/sys/backtrace.rs:166
```

Frame **#0** is the reproduction's own `main`, at the line that indexes the
slice. The faulting access is the caller reading a `&[u8]` it obtained with no
`unsafe`, and there is no libavutil frame on the stack at all — the mapping
libavutil established simply stopped being backed.

### 2. The file is rewritten: the reference's contents change

`src/bin/mutate.rs` is the quieter half — no fault, no diagnostic, and worse
for it:

```
$ ./target/x86_64-unknown-linux-gnu/debug/mutate
same &[u8], same index: before=0x11 after=0x99
MUTATED: a shared reference's contents changed underneath the holder
```

Between the two reads the program only did `OpenOptions::new().write(true)`,
`seek`, `write_all`, `sync_all` — safe `std`, same process, no second
thread. A `&[u8]` whose bytes change while it is live is undefined behaviour
independently of whether anything faults, and it is the variant a caller hits
by accident rather than on purpose.

## The case against this being a bug

Worth making properly, because two of these are real objections.

1. **"This is inherent to `mmap`; it is not the wrapper's defect."** Half
   right, and it is exactly the point. C's `av_file_map` hands back a
   `uint8_t *` and a size and makes no promise about them; a C caller who maps
   a file another process is rewriting has no contract to breach. The wrapper's
   job is to convert that into a Rust type, and `&[u8]` carries promises
   `mmap` cannot keep. The Rust ecosystem settled this: `memmap2::Mmap::map`
   is `unsafe fn` and its documentation gives these two reasons and no others.
2. **"The reproduction goes out of its way to truncate the file."** True of
   the first binary and not of the second: any concurrent writer produces
   variant 2, and a log file or a growing capture is an ordinary thing to map.
   And soundness is a claim about *all* safe programs, not about likely ones —
   `truncate.rs` contains no `unsafe`, which is the whole test.
3. **"`SIGBUS` is not undefined behaviour."** Correct in isolation, which is
   why variant 2 is here. Taken together they say the reference is neither
   dereferenceable nor immutable, which are the two things `&[u8]` asserts.
4. **"The crate documents the assumption somewhere."** It does not. `file.rs`
   never uses the words truncate, modify, shrink or SIGBUS; `lib.rs` carries
   no crate-level caveat; `crustify/status.md` and `crustify/wrappers-results.md`
   record `av_file_map` as wrapped and reviewed with no note attached. The one
   review that touched the module (`8037d43390`) reasoned carefully about the
   *unmap extent* — see `../notes/file-mapping-validity.md` — and never about
   the validity of the bytes.

## How this differs from the three advisories already here

All three existing findings are one shape: a safe wrapper forwards a
caller-supplied integer into C, and C's arithmetic on it is not total
(`av_frame_get_buffer`, the four `imgutils` alignments, `av_nearer_q`). The
fix in each case is a bound derived from the C expression.

This one is not about an argument at all. The call is well-formed for every
input; the defect is in the *type* the result is handed back as, and the fix is
a safety obligation rather than a check. It is also the first finding in this
directory whose faulting frame is in Rust rather than in libavutil.

## Suggested fix

Make the constructor `unsafe`, with the obligation stated, exactly as
`memmap2` does and exactly as this crate already does for the other API it
cannot make safe (`log.rs`'s `av_log_set_callback`, and the whole
`from_ptr`/`from_raw` family):

```rust
/// # Safety
///
/// For as long as the returned mapping lives, no process may change the
/// length or the contents of `filename`. [...]
pub unsafe fn av_file_map(...) -> Result<Option<CVec<u8, AvFileUnmap>>, i32>
```

Gating the constructor rather than the accessor is deliberate: it covers
`as_slice`, `as_mut_slice` and any future safe `CVec` method in one move,
without this crate having to re-audit `ffibox`'s surface each time it changes.

**This is a breaking change** — the signature changes and every caller needs an
`unsafe` block. The crate is `publish = false` at `0.0.0`, and the only
callers in the tree are its own four tests.

Two alternatives, for the maintainer to weigh:

* **A newtype around the `CVec` exposing only an `unsafe fn as_slice`.**
  Tighter — holding a mapping really is harmless, only reading it is not — and
  it keeps `av_file_map` on the safe surface. It is more code, it has to keep
  pace with `ffibox`, and it needs `CVec`'s `Debug` to stay non-reading
  (it is, `owned_refs.rs:829`).
* **Copy the mapping into an `av_malloc` buffer and unmap before returning.**
  Keeps the function safe and genuinely is safe, but it stops wrapping
  `av_file_map` and starts reimplementing "read a file", against the
  campaign's stated "wrap, not port" premise, and throws away the zero-copy
  property that is the only reason to call it.

## What I did not check

* **Whether `SIGBUS` is reachable on a *write* through `as_mut_slice`** as
  well as on the read above. It plainly should be — the mapping is
  `PROT_READ|PROT_WRITE` and a copy-on-write fault on a page past a truncated
  file's end faults identically — but I stopped at the first report rather
  than tuning for a second.
* **The non-`mmap` branches of `file.c`.** `HAVE_MAPVIEWOFFILE` (Windows) and
  the `av_realloc`-and-read fallback are not compiled in this configuration.
  The fallback branch would not have this defect at all, which means the
  soundness of the wrapper is configuration-dependent — itself an argument for
  the `unsafe` marking rather than for a check.
* **Whether any *other* wrapper in the crate hands out a view over memory
  outside libavutil's own heap.** I grepped every `CSlice`/`CSliceMut`/`CVec`
  construction site (`../notes/file-mapping-validity.md` lists them); `file.rs`
  is the only one. Everything else views `av_malloc` storage, which no other
  process can reach.

---

## Remediation

* **Branch:** `crustify/audit-gate-the-file-mapping-safe-code-cannot-keep-valid`,
  taken from the audited revision `4505beec42ca36c9e4993eecc5f11557d4a53bb1`
  (itself the branch carrying the three earlier fixes).
* **Commit:** `b19afcb3f0` — *crustify: gate the file mapping safe code cannot
  keep valid*. One file, `crustify/rust/libavutil/src/file.rs`, +87/-12. Not
  merged, not pushed.
* **What it does:** `av_file_map` becomes `pub unsafe fn` with the obligation
  written out; the module doc says the mapping and release halves carry
  different obligations and only the first needs the caller; the four existing
  tests route through one `unsafe fn map_this_file()` helper that names the
  assumption they rely on; and a `compile_fail,E0133` doctest — the crate's
  first doctest — pins the gate.

### Commands run, and what they said

No `LD_PRELOAD` anywhere: the instrument in `../notes/rust-side-asan.md`
puts the ASan runtime in the executable, so `cargo` runs normally and the
`.rustc_info.json` poisoning described in `../notes/asan-startup-flakiness.md`
cannot happen.

```
$ cd crustify/rust
$ cargo build --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s

$ cargo clippy --workspace --all-targets
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s   # no lints

$ cargo fmt --all -- --check
Diff in .../libavutil/src/hwcontext.rs:752
Diff in .../libavutil/src/opt.rs:756  (and 11 more in opt.rs)
                                # all pre-existing on the base branch, as
                                # 4505beec42 recorded; file.rs is clean
```

Tests, built with the Rust half instrumented and run against the
clang-instrumented `libavutil.so`, single-threaded, with leak detection on:

```
$ CARGO_TARGET_DIR=<audit>/tmp/target-rustasan \
  RUSTFLAGS="-Zsanitizer=address -Cdebuginfo=2 -Clink-arg=-Wl,--export-dynamic" \
  cargo +nightly test --workspace --target x86_64-unknown-linux-gnu --no-run

$ for B in <target>/debug/build/*/*/out/*; do
>   LD_LIBRARY_PATH=/tmp/ffsrc-clang/libavutil ASAN_OPTIONS=detect_leaks=1 \
>   UBSAN_OPTIONS=print_stacktrace=1 "$B" --test-threads=1
> done

libavutil : test result: ok. 215 passed; 0 failed   asan=0  ubsan=1
libc      : test result: ok.   8 passed; 0 failed   asan=0  ubsan=0
(the other eight test binaries hold no tests)
```

The single UBSan line is the `rational.c:185` `sign << 31` that `rational.rs`
documents and deliberately accepts (`../notes/rational-overflow-guards.md`).
`detect_leaks=1` reported nothing.

The doctest gate, run without the sanitiser since `compile_fail` never links:

```
$ cargo test --doc -p libavutil
running 1 test
test libavutil/src/file.rs - file::av_file_map (line 98) - compile fail ... ok
test result: ok. 1 passed; 0 failed
```

It passes only because the call fails with **E0133** specifically. Against the
old signature the snippet compiled, so the doctest would have failed — which is
what makes it a regression pin rather than decoration.

C side, unchanged by this commit and run to confirm no regression:

```
$ make -j64 fate-libavutil
... 60 TEST lines ...
FATE_RC=0                     # matches the recorded 60/60 baseline
```

### The reproduction, before and after

| reproduction | before | after |
|---|---|---|
| `tmp/repro-file-map` `truncate` | `AddressSanitizer: BUS` on a `&[u8]` read, frame #0 in the reproduction's own `main` | **does not compile:** `error[E0133]: call to unsafe function 'libavutil::file::av_file_map' is unsafe and requires unsafe block --> src/bin/truncate.rs:23:19` |
| `tmp/repro-file-map` `mutate` | `before=0x11 after=0x99`, a live shared reference's contents changing | **does not compile:** the same `E0133` at `src/bin/mutate.rs:20:19` |

To re-run the "before" halves, check out the audited revision `4505beec42`
(or `git checkout 4505beec42 -- crustify/rust/libavutil/src/file.rs`) and
rebuild the reproduction; nothing else about it needs to change. The `mutate`
binary is written to say `not observed on this kernel/page state` rather than
to assert, so a kernel that behaves differently reports that instead of a
false positive — on this one it printed `MUTATED`.

A compile error is the correct "after" for a fix of this shape, and it is
stronger than a refused call: the reproductions carry `#![forbid(unsafe_code)]`,
so there is no edit to them that both keeps them safe and reaches the mapping.
The cost is that they can no longer be run as regression binaries — the
`compile_fail` doctest inside the crate is what stands in for them.

### No regression elsewhere

Rebuilt against the patched crate and re-run under the same instrument:

* `tmp/repro-frame-align` → `REFUSED: av_frame_get_buffer(.., 1073741825) -> Err(-22)`, 0 ASan, 0 UBSan.
* `tmp/repro-imgutils-align` → all four entry points `Err(AlignmentTooLarge)`, 0/0.
* `tmp/repro-nearer-q` → `Err(UndefinedComparison)`, 0/0.
* `tmp/hammer`: `img` `frm` `frmalign` `samples` `fifo` `buf` `dictmem` `misc`
  `imgtrace` `alignscan` all reach their final `done` with 0 ASan and 0 UBSan;
  `rat` reaches `done` with 0 ASan and the one accepted `rational.c:185`.
