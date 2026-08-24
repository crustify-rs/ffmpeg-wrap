# `av_frame_get_buffer` forwards an unbounded `alignment`, producing a frame whose planes point outside its own buffer

* **Crate:** `libavutil` 0.0.0 (`crustify/rust/libavutil`), repo revision
  `24cd1b5a0658e603c825f0f4c1e2ac88eb7569a0`, branch
  `crustify/libavutil-gpt-5.6-sol`.
* **Instrument:** AddressSanitizer + UndefinedBehaviorSanitizer, as compiled
  into this tree's `libavutil.so` (`FFMPEG_CONFIGURATION` =
  `--toolchain=gcc-asan-ubsan ...`), loaded with
  `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8`.
* **Reproduction:** `crustify/audit/tmp/repro-frame-align/` — a cargo crate that
  depends on the audited crate, calls only its public API, and carries
  `#![forbid(unsafe_code)]`.
* **Lead note:** `../notes/frame-invariant-gating.md`.
* **Severity:** memory unsafety — out-of-bounds read/write of ~2 GiB past a
  104-byte heap allocation, reached with no `unsafe` in the caller.

## The path from safe code

`libavutil::frame::av_frame_get_buffer` is safe and takes the alignment as a
plain `i32`:

```rust
// crustify/rust/libavutil/src/frame.rs:1786
pub fn av_frame_get_buffer(frame: &mut AVFrameMut<'_>, alignment: i32) -> Result<(), i32> {
    if !frame.as_ref().is_unallocated() || frame.as_ref().hardware_frames_context().is_some() {
        return Err(-22);
    }
    // SAFETY: the exclusive handle supplies a live initialized frame. Any
    // allocations installed on success become owned by the frame lifecycle,
    // and the check above proves the frame held no owner for them to displace.
    frame_status(unsafe { ffi::av_frame_get_buffer(frame.as_mut_ptr(), alignment) })
}
```

Both preconditions it does check are about the *frame*. `alignment` is passed
through untouched. Its doc comment considers the parameter only from below:

> An alignment of zero or below asks libavutil to choose its preferred value;
> `get_video_buffer` (`frame.c:89`) and `av_samples_get_buffer_size` both
> normalize or refuse one before any extent is computed from it.

There is no upper bound, and C does not impose one either.

Four lines of safe Rust are enough:

```rust
let mut frame = av_frame_alloc().unwrap();
frame.as_mut().set_width(2);
frame.as_mut().set_height(2);
frame.as_mut().set_format(0);                                  // AV_PIX_FMT_YUV420P
av_frame_get_buffer(&mut frame.as_mut(), 1_073_741_825).unwrap();   // Ok(())
```

`av_frame_get_buffer` returns `Ok(())`. The frame it produces has:

```
buf[0]: 104 bytes at 0x60c000000100
  data[0] = 0x60c000000100  (offset 0 into a 104-byte allocation)
  data[1] = 0x60c080000141  (offset 2147483713 into a 104-byte allocation)
  data[2] = 0x60bf80000152  (offset -2147483566 into a 104-byte allocation)
```

`data[1]` and `data[2]` are ~2 GiB either side of the allocation they are
supposed to live in. That directly contradicts the crate's own `AVFrame` type
invariant (`frame.rs:559`):

> **Planes and geometry.** `data[i]` is null, or a plane address inside one of
> those buffers [...] Every non-null plane is valid for the extent the geometry
> describes.

Every safe wrapper downstream of that invariant now walks a wild pointer.
`av_frame_copy` is the shortest route.

## What the instruments say

Running the reproduction:

```
$ cd crustify/audit/tmp/repro-frame-align
$ cargo build
$ LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 \
  ASAN_OPTIONS=detect_leaks=0 UBSAN_OPTIONS=print_stacktrace=1 \
  ./target/debug/repro-frame-align
```

UBSan, inside `av_frame_get_buffer` itself:

```
libavutil/frame.c:115:36: runtime error: signed integer overflow: 1073741825 + 1073741825 cannot be represented in type 'int'
    #0 0x759470f20469 in get_video_buffer libavutil/frame.c:115
    #1 0x759470f20469 in av_frame_get_buffer libavutil/frame.c:212
    #2 0x56c88c25fae5 in _RNvNtCsj7wNU239tvn_9libavutil5frame19av_frame_get_buffer src/frame.rs:1793
    #3 0x56c88c25e890 in _RNvCs2y29ZRVwgN1_17repro_frame_align4main src/main.rs:28

libavutil/frame.c:115:36: runtime error: signed integer overflow: -2147483646 * 4 cannot be represented in type 'int'
    #0 0x759470f2043b in get_video_buffer libavutil/frame.c:115
```

and ASan, on the first safe use of the resulting frame:

```
==15706==ERROR: AddressSanitizer: SEGV on unknown address 0x60c080000141 (pc 0x75947098b85a bp 0x7ffe5be87ae0 sp 0x7ffe5be87a88 T0)
==15706==The signal is caused by a READ memory access.
    #0 0x75947098b85a  (/lib/x86_64-linux-gnu/libc.so.6+0x16d85a)
    #1 0x759470f61713 in image_copy_plane libavutil/imgutils.c:353
    #2 0x759470f689e7 in image_copy libavutil/imgutils.c:415
    #3 0x759470f689e7 in av_image_copy libavutil/imgutils.c:434
    #4 0x759470f251f3 in av_image_copy2 libavutil/imgutils.h:188
    #5 0x759470f251f3 in frame_copy_video libavutil/frame.c:684
    #6 0x759470f251f3 in av_frame_copy libavutil/frame.c:717
    #7 0x56c88c25fa30 in _RNvNtCsj7wNU239tvn_9libavutil5frame13av_frame_copy src/frame.rs:1708
    #8 0x56c88c25ee97 in _RNvCs2y29ZRVwgN1_17repro_frame_align4main src/main.rs:53
SUMMARY: AddressSanitizer: SEGV (/lib/x86_64-linux-gnu/libc.so.6+0x16d85a)
```

Note frame #7/#8: the faulting address was produced by the audited crate and
consumed by the audited crate. `0x60c080000141` is exactly the `data[1]` the
first call printed.

## Why C does this

`get_video_buffer` (`libavutil/frame.c`):

```c
 89:    if (align <= 0)
 90:        align = ALIGN;                       /* 64 */
 91:    plane_padding = FFMAX(ALIGN, align);     /* == align for align > 64 */
...
104:            frame->linesize[i] = FFALIGN(frame->linesize[i], align);
...
115:    total_size = 4 * plane_padding + 4 * align;   /* int arithmetic */
116:    for (int i = 0; i < 4; i++) {
117:        if (sizes[i] > SIZE_MAX - total_size)
118:            return AVERROR(EINVAL);
119:        total_size += sizes[i];
120:    }
122:    frame->buf[0] = av_buffer_alloc(total_size);
...
132:    for (int i = 1; i < 4; i++) {
133:        if (frame->data[i])
134:            frame->data[i] += i * plane_padding;
135:        frame->data[i] = (uint8_t *)FFALIGN((uintptr_t)frame->data[i], align);
136:    }
```

Line 115 is `8 * align` in `int`, so it wraps for any `align > INT_MAX / 8`.
With `align = 2^30 + 1` it wraps to `8`, and the padding the planes are about
to be pushed into is simply not allocated: `total_size` comes out 104 bytes.
Line 134 then overflows too (`i * plane_padding` for `i` = 2, 3), and line 135
rounds each plane address up to a `2^30 + 1` boundary using 64-bit
`uintptr_t` arithmetic that does *not* wrap. The plane pointers end up a
gigabyte or two outside the allocation while `av_frame_get_buffer` returns 0.

The specific value `2^30 + 1` matters only because `align - 1` is a power of
two, which lets `FFALIGN(linesize, align)` at line 104 leave the linesizes
small so the allocation stays tiny; larger round alignments instead fail with
`ENOMEM`. It is not a lone magic number. A scan
(`crustify/audit/tmp/hammer/src/bin/alignscan2.rs`) over 200 000 pseudo-random
odd alignments found 27 hits in the first sample it printed, e.g.

```
align=536870913  buf_size=104         bad=[(1, 1073741889), (2, 1073741906)]
align=1073741825 buf_size=104         bad=[(1, 2147483713), (2, -2147483566)]
align=1119657057 buf_size=367321960   bad=[(1, 2202351873), (2, -2012923630)]
align=1615927145 buf_size=42515368    bad=[(1, 2152802321)]
```

(`bad` lists `(plane_index, byte_offset_from_buffer_start)`.) The smallest
alignment observed to produce an out-of-bounds plane is `536870913` = 2^29 + 1.

## The case against this being a bug

Worth stating, because it is not weak:

1. **C's own documentation warns you off.** `av_frame_get_buffer`'s header
   says "It is highly recommended to pass 0 here unless you know what you are
   doing." A C caller passing `2^30 + 1` gets what they asked for.
2. **The UB is in libavutil, not in Rust.** Nothing in the wrapper's own
   `unsafe` block is wrong on its face; the block's SAFETY comment is about
   frame ownership and is accurate.
3. **Nobody would pass this value.** True, and irrelevant to soundness: a safe
   Rust function must not admit UB for *any* argument a caller can write, and
   `alignment: i32` admits all of them.

Points 1 and 2 are why this belongs to the wrapper rather than to FFmpeg. The
wrapper is what turns "a documented C footgun" into "a safe Rust function", and
that conversion is precisely the job it did not finish. The crate holds itself
to exactly this standard elsewhere and says so in `imgutils.rs:20`:

> C never range-checks the parameter, so the wrappers do.

and in `opt.rs:110`, where `AV_OPT_SEARCH_FAKE_OBJ` is refused before the call
because "reaching C with the flag would dereference the NULL target it
produces". The same reasoning applied to `alignment` yields the fix below.

## Suggested fix

Bound `alignment` in the wrapper, before the call. The safe ceiling is set by
line 115: `4 * plane_padding + 4 * align` must stay inside `int`, and
`plane_padding == max(64, align)`, so `align <= i32::MAX / 8` is sufficient and
also covers the `i * plane_padding` overflow at line 134 (`3 * align`). A
practical, more conservative choice is to require a power of two no larger than
`1 << 16`, which is what every real caller uses and what keeps
`FFALIGN(linesize, align)` meaningful at line 104 — `FFALIGN` is only an
alignment operation for powers of two; for other values it is a bit-clearing
operation whose result happens to be `>= x`.

**Not a breaking change** in the API-signature sense: the function already
returns `Result<(), i32>`, so the new rejection is a new error value on inputs
that previously produced a corrupt frame. It is a behaviour change for a caller
who passes a huge alignment today and (accidentally) relies on it succeeding.

The same treatment is owed to the `imgutils` alignment parameters — see
`imgutils-unbounded-alignment.md`, which is the same root cause in a different
module and has a strictly weaker observed consequence.

## What I did not check

* **The audio path.** `get_audio_buffer` reads `align` too, but an audio frame
  cannot be constructed through this crate at all: there is no safe way to set
  `AVFrame.ch_layout`, and `av_frame_get_buffer` therefore always takes the
  video branch. See `../notes/frame-invariant-gating.md`.
* **Whether a chosen alignment can be made to produce a *write* out of bounds
  rather than the read above.** `av_frame_copy` writes through the
  destination's planes as well, and the destination in the reproduction is
  equally corrupt, so a write fault is very likely reachable; I stopped at the
  first ASan report rather than tuning for it.
* **Rust-side memory errors.** The Rust half of this build is not
  ASan-instrumented (see `../notes/miri-cannot-cross-the-seam.md`).

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
