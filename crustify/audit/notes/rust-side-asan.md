# Instrumenting the Rust half too: how to build one ASan that spans the seam

**Status: built and validated. This supersedes the "what would change this"
paragraph of `miri-cannot-cross-the-seam.md`.**

The previous run recorded this as "the single highest-value thing a follow-up
run could do" and did not do it, because the tree's `libavutil.so` is a **GCC**
ASan build and GCC's `libasan` cannot coexist with the LLVM runtime that
`rustc -Zsanitizer=address` needs. It is doable in this image. Recipe, so the
next run does not rediscover it.

## The obstacle, and the trick

`clang-19` is installed but Debian ships no `libclang-rt-19-dev`, so
`clang -fsanitize=address` cannot link: it wants
`/usr/lib/llvm-19/lib/clang/19/lib/x86_64-pc-linux-gnu/libclang_rt.asan.a`,
which does not exist. `apt-get install libclang-rt-19-dev` has no candidate.

But the Rust **nightly** toolchain ships its own copy of the same compiler-rt
runtimes, and its ASan is ABI version `v8` — the same version clang-19 emits
(`__asan_version_mismatch_check_v8`). So clang can simply be pointed at Rust's:

```sh
RTLIB=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
D=/usr/lib/llvm-19/lib/clang/19/lib/x86_64-pc-linux-gnu
mkdir -p $D
ln -sf $RTLIB/librustc-nightly_rt.asan.a  $D/libclang_rt.asan.a
ln -sf $RTLIB/librustc-nightly_rt.ubsan.a $D/libclang_rt.ubsan_standalone.a
ln -sf $RTLIB/librustc-nightly_rt.ubsan.a $D/libclang_rt.ubsan_standalone_cxx.a
ar rc $D/libclang_rt.asan_static.a          # empty; clang asks for it, nothing needs it
```

## Building the C half

`configure` refuses an out-of-tree build while `config.h` sits in the source
dir, and the audit's hard rule forbids writing to the tree, so build from a
pristine copy outside it:

```sh
mkdir -p /tmp/ffsrc-clang
git -C /work/ffmpeg archive HEAD | tar -x -C /tmp/ffsrc-clang
cd /tmp/ffsrc-clang
./configure --toolchain=clang-asan-ubsan \
  --enable-shared --disable-static --disable-programs --disable-doc \
  --disable-network --disable-autodetect --disable-asm --disable-everything \
  --disable-avdevice --disable-avcodec --disable-avformat --disable-avfilter \
  --disable-swscale --disable-swresample --disable-stripping \
  --extra-cflags=-fno-omit-frame-pointer --extra-ldflags=-fno-omit-frame-pointer
make -j"$(nproc)" libavutil/libavutil.so
```

Same `SONAME` (`libavutil.so.61`) as the tree's GCC build, so it substitutes at
run time through `LD_LIBRARY_PATH` with no change to `libavutil-sys`, whose
`build.rs` hardcodes `-L <repo>/libavutil`.

## Building and running the Rust half

```sh
RUSTFLAGS="-Zsanitizer=address -Cdebuginfo=2 -Clink-arg=-Wl,--export-dynamic" \
  cargo +nightly build --target x86_64-unknown-linux-gnu
LD_LIBRARY_PATH=/tmp/ffsrc-clang/libavutil ASAN_OPTIONS=detect_leaks=0 \
  UBSAN_OPTIONS=print_stacktrace=1 ./target/x86_64-unknown-linux-gnu/debug/<bin>
```

`--export-dynamic` is **required** and the failure without it is opaque:

```
symbol lookup error: /tmp/ffsrc-clang/libavutil/libavutil.so.61:
    undefined symbol: __asan_register_elf_globals
```

The runtime lives in the executable, and a Rust binary does not export its
dynamic symbols by default, so the instrumented `.so` cannot find it.

Use `CARGO_TARGET_DIR` under `crustify/audit/tmp/` when building the workspace
crates, so nothing is written to `crustify/rust/target`.

## What this buys, and what it fixed

* **Rust code is instrumented.** The gap `miri-cannot-cross-the-seam.md` names
  — "a *Rust-side* aliasing violation ... would not be caught by anything here"
  — is now partly closed: ASan sees Rust-side loads and stores. It still does
  not model Stacked Borrows, so pure aliasing (two live `&mut`) remains
  uncovered; use-after-free, out-of-bounds and bad-page faults are covered on
  both sides of the seam for the first time.
* **`LD_PRELOAD` is gone**, and with it both traps in
  `asan-startup-flakiness.md`: no more one-run-in-three `DEADLYSIGNAL`, and
  `cargo` can be run normally, so `.rustc_info.json` never gets poisoned.
  Roughly 40 runs during this session, zero startup failures.
* It produced `advisories/av-file-map-safe-mapping-of-a-mutable-file.md`,
  whose fault (`AddressSanitizer: BUS`) is in the *caller's* Rust frame and
  would have been an unsymbolised `SIGBUS` under the old setup.

There is no `llvm-symbolizer` in this image, so ASan prints bare
`binary+0xoffset` frames. GNU `addr2line` resolves them:

```sh
addr2line -f -C -e ./target/x86_64-unknown-linux-gnu/debug/<bin> 0x<offset>
```

## Regression baseline established with it

Everything the previous run swept was re-run against this instrument, on the
patched tree (`4505beec42`):

| harness | result |
|---|---|
| `../tmp/hammer`: `img` `frm` `frmalign` `samples` `fifo` `buf` `dictmem` `misc` `imgtrace` `alignscan` | 0 ASan, 0 UBSan |
| `../tmp/hammer/rat` | 0 ASan; 1 UBSan, the `rational.c:185` the crate documents and accepts |
| `libavutil` unit tests, `--test-threads=1`, `detect_leaks=1` | 215 passed, 0 failed, 0 leaks, same single accepted `rational.c:185` |

So the three fixed advisories stay fixed under a stricter instrument, and no
Rust-side memory error exists anywhere those harnesses reach.
