# `imgutils`'s four alignment-taking wrappers bound `align` from below only, so a large `align` overflows `FFALIGN` in C

* **Crate:** `libavutil` 0.0.0 (`crustify/rust/libavutil`), repo revision
  `24cd1b5a0658e603c825f0f4c1e2ac88eb7569a0`.
* **Instrument:** UndefinedBehaviorSanitizer, as compiled into this tree's
  `libavutil.so`, loaded with
  `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8`.
* **Reproduction:** `crustify/audit/tmp/repro-imgutils-align/`
  (`#![forbid(unsafe_code)]`), plus the sweep in
  `crustify/audit/tmp/hammer/src/bin/img.rs`.
* **Lead note:** `../notes/open-enum-values-into-c.md`.
* **Severity:** undefined behaviour (signed integer overflow) inside libavutil,
  reachable with no `unsafe` in the caller. **No memory error was observed**
  and I argue below that none is reachable through this particular overflow —
  see "How bad is it".

## The path from safe code

`imgutils.rs` is the module that is *most* explicit about validating `align`,
and it validates exactly one side of it. All four wrappers begin:

```rust
if align <= 0 {
    return Err(ImageError::NonPositiveAlignment);
}
```

with a long, correct rationale attached (`imgutils.rs:14`):

> `FFALIGN(x, a)` is `(x + a - 1) & ~(a - 1)`, which is `>= x` only for
> `a >= 1`. At `align == 0` the mask is zero, so the whole image reports an
> extent of zero bytes while C still copies or fills a full unaligned
> `linesize` per row [...] C never range-checks the parameter, so the wrappers
> do.

The same expression overflows at the top of the range, and that is not checked.
One safe call is enough:

```rust
use libavutil::imgutils::av_image_get_buffer_size;
use libavutil::pixfmt::AVPixelFormat;

let _ = av_image_get_buffer_size(AVPixelFormat::GRAY8, 2, 2, i32::MAX);
```

## What the instrument says

```
$ cd crustify/audit/tmp/repro-imgutils-align
$ cargo build
$ LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 \
  ASAN_OPTIONS=detect_leaks=0 UBSAN_OPTIONS=print_stacktrace=1 \
  ./target/debug/repro-imgutils-align

libavutil/imgutils.c:486:31: runtime error: signed integer overflow: 2147483647 + 2 cannot be represented in type 'int'
    #0 0x7fe11416ad4f in av_image_get_buffer_size libavutil/imgutils.c:486
    #1 0x55a740201383 in _RNvNtCsj7wNU239tvn_9libavutil8imgutils24av_image_get_buffer_size src/imgutils.rs:334
    #2 0x55a74020104e in _RNvCskHwEX3jC2AN_7img_min4main src/bin/img_min.rs:6
```

`imgutils.c:486` is `aligned_linesize[i] = FFALIGN(linesize[i], align);`.

The full sweep (`hammer/src/bin/img.rs`: all 270 pixel-format table entries x
12 widths x 12 heights x 8 alignments, then `copy_to_buffer` with a second
alignment) reaches three distinct sites, through all four public wrappers:

```
libavutil/imgutils.c:461:27  <- av_image_fill_arrays  / av_image_fill_black
libavutil/imgutils.c:486:31  <- av_image_get_buffer_size
libavutil/imgutils.c:530:20  <- av_image_copy_to_buffer  (dst += FFALIGN(...))
```

Line 530 is inside the row loop, i.e. it is reached *after* C accepted the
computed size, on a frame it is actively copying.

## How bad is it — the honest version

I tried to turn this into memory corruption and could not, and I believe it
cannot be, for this expression. Write `X = (x + a - 1) mod 2^32` interpreted as
`int` and `M = ~(a - 1)`:

* `a` is positive, so `a - 1 < 2^31` and bit 31 of `M` is set;
* therefore bit 31 of `X & M` is bit 31 of `X`;
* `x, a < 2^31`, so `x + a - 1 < 2^32`; if the sum overflowed, `X` has bit 31
  set, the result is negative, and every downstream consumer
  (`av_image_fill_plane_sizes`, then `sizes[i] > INT_MAX - ret`) rejects it
  with `EINVAL`;
* the one way to overflow and still come out positive is `x + a == 2^31`
  exactly, where the two-step wrap `(x+a) -> INT_MIN`, `-1 -> INT_MAX` lands on
  `0x7FFFFFFF & M`, which works out to exactly `x`.

So the *value* is either rejected or correct; only the intermediate overflow is
undefined. Under GCC — which this campaign builds with, and which documents
that it does not exploit the latitude C99/C11 give it for signed wraparound —
the observable behaviour is benign.

I checked that argument two ways rather than trusting it. Over three million
random `(x, a)` pairs drawn from `x < 1 << 27` (the widest linesize that can
reach this call) and `1 <= a < 1 << 31`, restricted to those where the `int`
arithmetic actually overflows, **every** wrapped result was negative; and over
the whole `x + a == 2^31` family for `x` up to `1 << 20` — the only shape that
can overflow and come out positive — the result was exactly `x` every time.
The empirical side agrees: 18 UBSan reports and zero ASan reports over the
whole 270x12x12x8 sweep.

That is why this is a separate, lower-severity advisory rather than part of
`av-frame-get-buffer-unbounded-alignment.md`, which is the same root cause with
a genuine out-of-bounds pointer at the end of it.

## The case against this being a bug

* It is signed overflow in C, not a Rust-level unsoundness, and its practical
  consequence here is nil.
* The value `i32::MAX` for an alignment is nonsense that no caller would write.

Against that: it *is* undefined behaviour, it *is* reachable from safe Rust
with no `unsafe`, and the module in question already treats this exact
parameter as the wrapper's responsibility to range-check. Leaving the upper
half unchecked while writing three paragraphs about the lower half is an
inconsistency worth closing, and the fix is two comparisons.

## Suggested fix

Extend the existing guard in all four wrappers from `align <= 0` to a range
check. `FFALIGN` is only meaningful for powers of two anyway, so the tightest
honest form is:

```rust
if align <= 0 || !align.is_power_of_two() || align > (1 << 16) {
    return Err(ImageError::NonPositiveAlignment /* -> rename */);
}
```

If preserving today's acceptance of non-power-of-two alignments matters, the
minimal version is `align <= 0 || align > i32::MAX - MAX_LINESIZE`, but the
power-of-two form is what the parameter actually means.

**Not a breaking change** to any signature; it adds a rejection on inputs that
today invoke UB. Renaming `ImageError::NonPositiveAlignment` to
`InvalidAlignment` *would* be breaking and is optional.

## What I did not check

* Whether a *non*-overflowing but absurd `align` (say `1 << 20`) can make
  `av_image_copy_to_buffer` mis-stride in a way ASan would catch. The sweep
  covered `1, 2, 4, 32, i32::MAX` only; intermediate powers of two were not
  swept for this module (they were for `frame`, where they produced the other
  advisory).
* The bitstream and palette formats' interaction with a huge `align` beyond
  what the 270-format sweep happened to exercise.

---

## Remediation

* **Branch:** `crustify/audit-bound-c-alignment-and-comparison-arguments`,
  taken from the audited revision `24cd1b5a0658e603c825f0f4c1e2ac88eb7569a0`.
* **Commit:** `4505beec42` — *crustify: bound the C arguments three safe
  wrappers passed through*. Not merged, not pushed.
* All three advisories in this directory are fixed by that one commit, since
  they are the same defect shape in three modules.

### Commands run, and what they said

Note the shape of the test command: **cargo must not be run under
`LD_PRELOAD`**, because cargo shells out to `rustc -vV`, which crashes under
the preloaded ASan runtime and — worse — gets that crash *cached* in
`target/.rustc_info.json`, after which every later cargo invocation in the
workspace fails. Build with cargo, then run the test binary with the preload.
(I hit this and had to delete the cache file; see
`../notes/asan-startup-flakiness.md`.)

```
$ cd crustify/rust
$ cargo build --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s)

$ cargo clippy --workspace --all-targets
    Finished `dev` profile [unoptimized + debuginfo] target(s)      # no lints

$ cargo fmt --all -- --check
Diff in .../libavutil/src/hwcontext.rs
Diff in .../libavutil/src/opt.rs                # both pre-existing on the
                                                # base branch; the three files
                                                # this commit touches are clean

$ cargo test --workspace --no-run
$ for b in target/debug/deps/lib*; do
>   LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 ASAN_OPTIONS=detect_leaks=0 ./$b
> done
libavutil : test result: ok. 215 passed; 0 failed
libc      : test result: ok.   8 passed; 0 failed
(the other eight test binaries hold no tests)
```

The only sanitiser output left across all 223 tests is the diagnostic
`rational.rs` documents and deliberately accepts:

```
libavutil/rational.c:185:16: runtime error: left shift of 1 by 31 places cannot be represented in type 'int'
```

C side (unchanged by this commit, run to confirm no regression):

```
$ make -j32 fate-libavutil
... 60 TEST lines ...
FATE_RC=0                     # matches the recorded 60/60 baseline
```

### The reproductions, before and after

Each reproduction under `../tmp/` was rebuilt against the patched crate and
re-run under `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8
ASAN_OPTIONS=detect_leaks=0`:

| reproduction | before | after |
|---|---|---|
| `repro-frame-align` | `ACCEPTED`, three out-of-bounds plane addresses, then `AddressSanitizer: SEGV on unknown address 0x60c080000141` in `image_copy_plane` | `REFUSED: av_frame_get_buffer(.., 1073741825) -> Err(-22)`, exit 0, no sanitiser output |
| `repro-imgutils-align` | six UBSan `signed integer overflow` reports across `imgutils.c:461`, `:486`, `:530` | all four entry points return `Err(AlignmentTooLarge)`, no sanitiser output |
| `repro-nearer-q` | `Ok(-2147483648)` plus `rational.c:141:48: runtime error: signed integer overflow: -1 * -2147483648` | `Err(UndefinedComparison)`, no sanitiser output |

`repro-frame-align` was made branch-agnostic afterwards — it prints `REFUSED`
and returns instead of panicking — and the before/after pair above was
produced by stashing the three patched files, rebuilding, running, and
restoring them, so both halves are the same binary source against the two
revisions.

The whole discovery sweep in `../tmp/hammer` was also re-run against the
patched crate: `img`, `frmalign`, `alignscan`, `hammer` (channel layouts),
`samples`, `fifo`, `buf`, `dictmem`, `misc` and `rat` all reach their final
`done` with zero ASan reports and zero UBSan reports, except `rat`, which
still reaches the accepted `rational.c:185`.
