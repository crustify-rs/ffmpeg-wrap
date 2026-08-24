# Miri is not usable on this crate

**Status: cleared as an instrument, not as a lead.**

`crustify/rust/libavutil` is a thin wrapper: essentially every public function
body is an `unsafe { ffi::av_* (...) }` call. Miri stops at every `extern "C"`
boundary, so a Miri run over the crate's own test suite terminates at the first
`av_frame_alloc`. There is no subset of the public API that does meaningful
work without crossing the seam (the only pure-Rust wrappers are
`av_make_q`, `av_inv_q`, `av_q2d` and `av_cmp_q` in `rational.rs`, which
reimplement C header inlines and were reviewed by reading).

> **Update (run 2):** the "what would change this" paragraph below has been
> acted on — see `rust-side-asan.md`. The Rust half *is* now instrumented, so
> the gap described here is narrower than it was: use-after-free, out-of-bounds
> and bad-page faults are caught on both sides of the seam. Pure Rust-side
> *aliasing* (two live `&mut`) is still uncovered, because ASan does not model
> Stacked Borrows and Miri still cannot run this crate. Miri remains unusable
> for the reason given here.

What this costs the audit: a *Rust-side* aliasing violation — two live handles
to one C object, a `CSlice` that outlives its owner — would not be caught by
anything here. The structural review below (`handle-lifetimes.md`) is the only
coverage that shape got, and it is reasoning, not a demonstration.

What would change this: a `-Zsanitizer=address` build of the Rust side, which
needs `libavutil.so` rebuilt with clang so that one LLVM ASan runtime serves
both halves. That is a tractable next step and is the single highest-value
thing a follow-up run could do.
