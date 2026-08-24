# Lead notes — libavutil safe wrapper (`crustify/rust/libavutil`)

Two runs so far, both on 2026-08-24.

* **Run 1** — revision `24cd1b5a0658e603c825f0f4c1e2ac88eb7569a0` on
  `crustify/libavutil-gpt-5.6-sol`. Three advisories, fixed on
  `crustify/audit-bound-c-alignment-and-comparison-arguments` (`4505beec42`).
* **Run 2** — revision `4505beec42`, i.e. run 1's fixes. One advisory
  (`av-file-map-safe-mapping-of-a-mutable-file.md`), fixed on
  `crustify/audit-gate-the-file-mapping-safe-code-cannot-keep-valid`
  (`b19afcb3f0`). It also built the instrument run 1 said it wanted and closed
  the `hwcontext` coverage hole run 1 recorded.

Crate version `0.0.0` (unpublished).

Instruments available in this image and how they were used:

* **ASan + UBSan on the C side.** `libavutil.so` in this tree is already built
  with `--toolchain=gcc-asan-ubsan` (see `FFMPEG_CONFIGURATION` in `config.h`).
  A Rust binary that links it must preload the runtime:
  `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8`. This is the instrument
  that produced every confirmed finding — it sees exactly the seam Miri cannot.
* **Miri:** not used. Every interesting operation in this crate is an
  `extern "C"` call, which Miri refuses to execute. See
  `miri-cannot-cross-the-seam.md`.
* **Rust-side ASan (`-Zsanitizer=address`) — run 1 could not use it, run 2
  can.** The obstacle was that the tree's `libavutil.so` is a GCC ASan build
  and GCC's `libasan` cannot coexist with the LLVM runtime rustc needs. The
  way through is to build a second `libavutil.so` with clang and point clang at
  the compiler-rt runtimes the Rust nightly toolchain already ships. The full
  recipe, and what it caught, is in `rust-side-asan.md`. **Prefer it to the
  `LD_PRELOAD` setup above** — it instruments both halves, and it makes the two
  traps in `asan-startup-flakiness.md` disappear.

Scratch harnesses live in `../tmp/hammer` (one binary per module family).
