# `av_nearer_q`'s overflow guard misses the `* av_cmp_q(...)` at `rational.c:141`

* **Crate:** `libavutil` 0.0.0 (`crustify/rust/libavutil`), repo revision
  `24cd1b5a0658e603c825f0f4c1e2ac88eb7569a0`.
* **Instrument:** UndefinedBehaviorSanitizer, as compiled into this tree's
  `libavutil.so`, loaded with
  `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8`.
* **Reproduction:** `crustify/audit/tmp/repro-nearer-q/`
  (`#![forbid(unsafe_code)]`).
* **Lead note:** `../notes/rational-overflow-guards.md`.
* **Severity:** undefined behaviour (signed integer overflow) inside libavutil,
  reachable with no `unsafe` in the caller, on entirely ordinary inputs. No
  memory error; the observable damage under GCC is a wrong return value.

## The path from safe code

```rust
use libavutil::rational::{av_make_q, av_nearer_q};

let q  = av_make_q(1, 1);
let q1 = av_make_q(1, 1);
let q2 = av_make_q(0, 0);          // C's "undefined rational"
let _ = av_nearer_q(q.as_ref(), q1.as_ref(), q2.as_ref());
```

Nothing extreme is required — `1/1`, `1/1`, `0/0`. `0/0` is a value C itself
defines and that `av_cmp_q`'s own documentation calls out, and `av_make_q` is
a safe total constructor.

## What the instrument says

```
$ cd crustify/audit/tmp/repro-nearer-q
$ cargo build
$ LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 \
  ASAN_OPTIONS=detect_leaks=0 UBSAN_OPTIONS=print_stacktrace=1 \
  ./target/debug/repro-nearer-q

nearer = Ok(-2147483648)
libavutil/rational.c:141:48: runtime error: signed integer overflow: -1 * -2147483648 cannot be represented in type 'int'
    #0 0x7b6c94fd7a38 in av_nearer_q libavutil/rational.c:141
    #1 0x55ab4a987913 in _RNvNtCsj7wNU239tvn_9libavutil8rational11av_nearer_q src/rational.rs:285
    #2 0x55ab4a98740b in _RNvCs1KBbKktHtFN_7nearer34main src/bin/nearer3.rs:6
```

Note the return value: `Ok(-2147483648)`. The wrapper's own doc says the
function "Reports which of `q1` and `q2` lies nearer to `q`: 1 for `q1`, -1 for
`q2`, 0 when they are equidistant." `i32::MIN` is none of those, so the UB also
leaks a value that violates the wrapper's stated contract.

## Why

`libavutil/rational.c`:

```c
129: int av_nearer_q(AVRational q, AVRational q1, AVRational q2)
130: {
132:     int64_t a = q1.num * (int64_t)q2.den + q2.num * (int64_t)q1.den;
133:     int64_t b = 2 * (int64_t)q1.den * q2.den;
136:     int64_t x_up   = av_rescale_rnd(a, q.den, b, AV_ROUND_UP);
139:     int64_t x_down = av_rescale_rnd(a, q.den, b, AV_ROUND_DOWN);
141:     return ((x_up > q.num) - (x_down < q.num)) * av_cmp_q(q2, q1);
142: }
```

With `q2 = 0/0`: `b == 0`, so both `av_rescale_rnd` calls hit their
`if (c <= 0 ...) return INT64_MIN;` guard and `x_up == x_down == INT64_MIN`.
The left factor is therefore `(false) - (true) == -1` for any `q.num`.
Meanwhile `av_cmp_q(0/0, q1)` returns its "undefined" sentinel `INT_MIN`
(cross-difference is zero, one denominator is zero, one numerator is zero).
`-1 * INT_MIN` overflows `int`.

The wrapper does guard this function — but only the other overflow in it:

```rust
// crustify/rust/libavutil/src/rational.rs:275
pub fn av_nearer_q(q: ..., q1: ..., q2: ...) -> Result<i32, RationalError> {
    if q1.den() == i32::MIN && q2.den() == i32::MIN {
        return Err(RationalError::DenominatorProductOverflow);
    }
    ...
}
```

and its doc comment states the analysis as complete:

> `2 * (int64_t)q1.den * q2.den` reaches `1 << 63` exactly when both
> denominators are `i32::MIN` [...] Every other pair of `int` denominators
> keeps `2 * d1 * d2` at `(1 << 63) - (1 << 32)` or below, so the rejection is
> exactly one pair wide.

That paragraph is correct about lines 132-133 and silent about line 141. The
guard is one line short, not wrong.

Empirically (`crustify/audit/tmp/hammer/src/bin/nearer2.rs`), over triples
drawn from `{i32::MIN, -2, -1, 0, 1, 2, i32::MAX}`, 4753 of the 7^6 = 117 649
triples have `av_cmp_q(q2, q1) == i32::MIN`; every one of them whose `q2` is
`0/0` (which forces the left factor to `-1`) is a trigger.

## The case against this being a bug

* It is signed overflow in C, and on GCC the wrapped result is simply
  `i32::MIN` again — no memory is touched. A caller who ignores the number sees
  nothing.
* One could argue `0/0` is a garbage input and the caller deserves garbage out.
  But `av_make_q(0, 0)` is safe, total and documented as meaningful (`0/0` is
  the "undefined rational" that `av_q2intfloat` encodes as a quiet NaN), and
  the wrapper's whole design premise — visible in the five other
  `RationalError` variants — is that C's overflow-prone inputs get rejected at
  the boundary rather than passed through.

## Suggested fix

Reject the inputs that make the multiplication overflow. The clean statement is
"refuse when `av_cmp_q(q2, q1)` would return its `i32::MIN` sentinel", and
`av_cmp_q` is already reimplemented in Rust in this very module
(`rational.rs:485`), so the check is local and needs no FFI call:

```rust
if av_cmp_q(q2, q1) == i32::MIN {
    return Err(RationalError::UndefinedComparison);
}
```

placed alongside the existing denominator-pair guard. That is a new
`RationalError` variant, so **it is a breaking change** for anyone matching
`RationalError` exhaustively — though the enum is not `#[non_exhaustive]`
today, so adding *any* variant would be. Reusing
`DenominatorProductOverflow` would avoid that at the cost of a misleading name;
`#[non_exhaustive]` on `RationalError` is the better long-term move and is
itself breaking. Given the crate is `publish = false` at version `0.0.0`, I
would take the new variant.

## What I did not check

* Whether `av_find_nearest_q_idx` (not wrapped by this crate) has the same
  exposure — it calls `av_nearer_q` in a loop, so it would.
* Whether any *other* consumer of `av_cmp_q`'s `INT_MIN` sentinel inside
  libavutil is reachable from this crate's safe surface. `av_nearer_q` is the
  only wrapped one.

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
