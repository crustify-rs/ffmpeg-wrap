# Feeding libavutil's open enums arbitrary values from safe Rust

**Status: cleared for every module hammered except `imgutils`/`frame`
alignment, which became advisories of their own.**

Because every C enum is an open transparent newtype with a safe `from_raw`
(see `handle-lifetimes.md` §5), safe Rust can hand libavutil any `i32`/`u32`
where C expects an enumerator. I built harnesses that do exactly that and ran
them under the tree's ASan+UBSan `libavutil.so`.

Harnesses (all in `../tmp/hammer/src/bin/`, run with
`LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 ASAN_OPTIONS=detect_leaks=0`):

| binary | surface | result |
|---|---|---|
| `main.rs` | `channel_layout`: 22 raw `AVChannel` ids x 6 orders x 4 retype flag combos, custom-map mutation, names with `@` and 15-byte payloads, `describe` into 0/8/256-byte buffers, defaults for +/-`i32::MAX` counts, masks incl. `u64::MAX`, 14 layout strings, the standard-layout iterator | clean |
| `img.rs` / `imgtrace.rs` | `imgutils`: all 270 table formats x 12 widths x 12 heights x 8 aligns, plus `fill_pointers` with adversarial strides | **UB found** — see `advisories/imgutils-unbounded-alignment.md` |
| `samples.rs` | `samplefmt`: 20 formats x 12 channel counts x 10 sample counts x 8 aligns; `alloc`, `alloc_array_and_samples`, `fill_arrays`, `copy`, `set_silence` with out-of-range offsets | clean |
| `fifo.rs` | `audio_fifo`: 14 formats x 8 channel counts x 4 initial sizes x 6 sample counts through write/peek/peek_at/read/drain/realloc/reset | clean |
| `buf.rs` | `buffer`: cross product of alloc/allocz/realloc-from-`None` x truncate x advance x write_all x ref/make_writable/realloc | clean |
| `frm.rs`, `frmalign.rs`, `alignscan*.rs` | `frame`: geometry x format x alignment, side data, clone/copy/copy_props/make_writable, mismatched-geometry copies | **memory unsafety found** — see `advisories/av-frame-get-buffer-unbounded-alignment.md` |
| `dictmem.rs` | `dict` separators/flags/inputs cross product, iteration, `get_string`, `copy`; `mem` dynarray to 5000 elements, allocators | clean |
| `rat.rs`, `nearer*.rs` | `rational`/`mathematics` over the `i32` extremes | **UB found** — see `advisories/av-nearer-q-cmp-multiply-overflow.md` |
| `misc.rs` | `avstring`, `md5`, `error`, `pixdesc`/`pixfmt` name tables at their bounds, `utils`, `time` | clean |

"Clean" means: zero `runtime error:` lines from UBSan and zero
`ERROR: AddressSanitizer` reports across the whole run, with the harness
reaching its final `println!`.

The three surfaces that were *not* covered this way are recorded separately in
`unreachable-from-safe-code.md`.
