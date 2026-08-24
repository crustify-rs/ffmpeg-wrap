# crustify — `ffmpeg/ffmpeg` / `libavutil`

## Campaign

- **target repo** — `https://github.com/ffmpeg/ffmpeg` @ `1019f8f036602a8464185baa4857654337eeca14`
- **target** — `libavutil`
- **campaign objective** — `wrap`
- **`impl_files`** — `libavutil/`, `compat/`
- **`api_headers`** — the published `libavutil` headers (see `crustify/oracle/targets/libavutil/oracle-config.json`)
- **agent backend** — `codex`
- **model** — `openai/gpt-5.6-sol`
- **`--billing`** — `subscription`
- **`--max-types`** — `2`
- **`--max-syms`** — `50`
- **`--max-loc`** — `1000`
- **`--min-fields`** — `10`
- **`--parallel-max`** — `32`
- **branch** — `crustify/libavutil-gpt-5.6-sol`, last code commit `b19afcb3f0` (this report lands on top)
- **deps** — crustify-cli `4f93c88` (`fix/log-cost-campaigns-tier`), ffibox `600399f` (`main`)

## Review pass

`--objective review`, LLM-as-a-Judge over the landed waves. Run it under a
DIFFERENT model from the one being judged — a review by the author is
self-review, and any disagreement is what makes the pass informative.

- **agent backend** — `claude`
- **model** — `anthropic/claude-opus-5`
- **`--billing`** — `subscription`
- **`--max-types`** — `2`
- **`--max-syms`** — `50`
- **`--max-loc`** — `1000`
- **`--min-fields`** — `10`
- **`--parallel-max`** — `32`
- **branch** — `crustify/session/review-2026-08-23_22-14-28_82c3`, tip `414ff93355`
- **agents** — `28` spawned over `3` session(s); `24` landed a batch result, `4` were killed mid-flight (see Notes)

`rv`-prefixed columns below carry the review pass; the unprefixed ones remain
the campaign's.

## Legend

- `DAG layer` — the unit's own wrap DAG layer
- `kind` — `struct` / `union` / `enum` for a type; `callback`; `function` for
  every symbol, whatever linkage the C declaration carries
- `fields` — all declared fields
- `target fields` / `target ptr` — fields a target-section function touches / of
  those, pointers
- `wrapped fields` — fields given an accessor, counted as DISTINCT `type.field`
  paths; `—` = wrapped with no field accessor (opaque)
- `newtypes` — distinct Rust types carrying a `/// Wraps: <tag>` anchor; `1` is
  a plain 1:1 wrap, `>1` where one C type needs several representations (an
  owned handle beside a borrowed view, a by-value beside a by-pointer form)
- `target fns` — every target-section function needing the symbol, tree-wide
- `deps` — import types/callbacks the symbol needs
- `wrappers` — distinct safe fns emitted over the one C routine; `>1` where the
  signature forked (a slice-taking beside a `CStr`-taking form, a fallible
  beside an infallible one)
- `batch` — the agent that emitted it. Symbols pool, so their cost is per
  batch, not per symbol — see the batches table
- `$` / `wall` / `loc` — that agent's own cost, its elapsed time, and the `.rs`
  insertions of its landing commit. `wall` is `ended_at − started_at` from the
  agent's own `usage.json`, so it INCLUDES the per-worktree C rebuild
- `$/unit` / `$/loc` / `$/field` — that row's `$` over its units, its `loc`, or
  its declared fields
- `$/symbol` / `$/type` — a batch holds one kind or the other, so one of the
  two reads `—`; on a Σ row each divides that kind's own cost by its own count
- `↖ batched` — shares the row above's agent; one usage record covers both
- `rv $` / `rv wall` / `rv loc` — the REVIEW agent's own cost, elapsed time, and
  net `.rs` line delta (`+ins/-del`) of its landing commit
- `verdict` — what the judge concluded: `held` = analysis and code confirmed as
  emitted, `fixed` = a defect in the emitted Rust corrected, `record` = an
  ownership finding resubmitted through the oracle. Several may apply
- In a batches row, `wall` is the layer's LONGEST agent — what the layer would
  cost with every batch spawned at once — and the parenthetical is the
  serial-sum multiple. A Σ row sums the columns it can and carries the same
  longest-agent reading for `wall`

## Raw lifetime discovery

Goal: turn the untyped lifecycle primitives into Rust lifetime contracts before
any wrapper needs one. Oracle `schedule --lifetime-for void` then
`schedule --lifetime-for string`, one
agent each, objective `raw` (set by the tier, not `--objective`). `strategies`
counts the deleter/cloner ZSTs emitted; the four trait columns count the
`unsafe impl`s that bind them.

| tier | symbols submitted | strategies | CDropped | CCloned | CLenDropped | CLenCloned | $ | wall |
|---|---|---|---|---|---|---|---|---|
| void | `5` | `3` | `2` | `0` | `3` | `1` | `$4.57` | `10m04s` |
| string | `2` | `1` | `1` | `2` | `0` | `0` | `$3.30` | `7m05s` |
| **Σ** | **`7`** | **`4`** | **`3`** | **`2`** | **`3`** | **`1`** | **`$7.87`** | **`17m09s`** |

## Target set

What the campaign wrapped and in what order: types and callbacks first,
bottom-up by DAG layer, then the symbols over them.

### Types and callbacks

| DAG layer | unit | kind | fields | target fields | target ptr | wrapped fields | newtypes | $ | wall | loc | rv $ | rv wall | rv loc | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `0` | `AVAlphaMode` | enum | `0` | `—` | `—` | `—` | `1` | `$7.81` | `11m28s` | `120` | `$7.87` | `13m56s` | `+25/-2` | fixed |
| `0` | `AVAudioFifo` | struct | `7` | `7` | `1` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVBuffer` | struct | `7` | `7` | `3` | `—` | `1` | `$9.30` | `15m55s` | `181` | `$5.13` | `9m33s` | `+36/-8` | fixed |
| `0` | `AVChannel` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVChannelOrder` | enum | `0` | `—` | `—` | `—` | `1` | `$3.57` | `7m45s` | `156` | `$3.55` | `6m18s` | `+100/-13` | fixed |
| `0` | `AVChromaLocation` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVColorPrimaries` | enum | `0` | `—` | `—` | `—` | `1` | `$3.28` | `5m26s` | `141` | `$4.46` | `7m57s` | `+156/-18` | fixed |
| `0` | `AVColorRange` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVColorSpace` | enum | `0` | `—` | `—` | `—` | `1` | `$4.17` | `7m07s` | `161` | `$3.44` | `6m20s` | `+64/-0` | fixed |
| `0` | `AVColorTransferCharacteristic` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVComponentDescriptor` | struct | `5` | `5` | `0` | `5` | `1` | `$6.91` | `13m48s` | `187` | `$12.58` | `23m00s` | `+0/-0` | held |
| `0` | `AVDictionary` | struct | `2` | `2` | `1` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | held |
| `0` | `AVDictionaryEntry` | struct | `2` | `2` | `2` | `2` | `1` | `$5.39` | `10m42s` | `208` | `$6.18` | `11m14s` | `+110/-25` | fixed |
| `0` | `AVFrameSideDataType` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVHWDeviceType` | enum | `0` | `—` | `—` | `—` | `1` | `$1.14` | `3m42s` | `136` | `$4.07` | `8m34s` | `+174/-11` | fixed |
| `0` | `AVHWFrameTransferDirection` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVMD5` | struct | `3` | `3` | `0` | `—` | `1` | `$5.23` | `9m09s` | `223` | `$5.32` | `10m38s` | `+0/-0` | held |
| `0` | `AVMediaType` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | held |
| `0` | `AVOptionArrayDef` | struct | `4` | `4` | `1` | `4` | `1` | `$8.54` | `14m32s` | `226` | `$7.40` | `13m20s` | `+18/-11` | fixed |
| `0` | `AVOptionType` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed |
| `0` | `AVPictureType` | enum | `0` | `—` | `—` | `—` | `1` | `$4.68` | `6m30s` | `439` | `$5.82` | `10m08s` | `+0/-0` | held |
| `0` | `AVPixelFormat` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | held |
| `0` | `AVRational` | struct | `2` | `2` | `0` | `2` | `1` | `$4.68` | `9m47s` | `192` | `$4.54` | `10m06s` | `+0/-0` | held |
| `0` | `AVRounding` | enum | `0` | `—` | `—` | `—` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | held |
| `0` | `AVSampleFormat` | enum | `0` | `—` | `—` | `—` | `1` | `$4.84` | `8m22s` | `116` | `$5.05` | `9m39s` | `+0/-0` | held |
| `1` | `AVBufferRef` | struct | `3` | `3` | `2` | `3` | `1` | `$6.01` | `10m43s` | `372` | `$11.73` | `20m13s` | `+462/-82` | fixed · record |
| `1` | `AVChannelCustom` | struct | `3` | `2` | `0` | `3` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed · record |
| `1` | `AVOption` | struct | `14` | `9` | `3` | `14` | `1` | `$3.37` | `7m27s` | `325` | `$7.20` | `12m59s` | `+348/-31` | fixed |
| `1` | `AVPixFmtDescriptor` | struct | `7` | `7` | `2` | `7` | `1` | `$2.61` | `5m34s` | `242` | `$6.15` | `11m39s` | `+186/-25` | fixed · record |
| `2` | `AVChannelLayout` | struct | `6` | `4` | `1` | `6` | `1` | `$4.43` | `8m51s` | `438` | `$17.78` | `22m35s` | `+524/-64` | fixed · record |
| `2` | `AVFrameSideData` | struct | `5` | `5` | `3` | `5` | `1` | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | ↖ batched | fixed · record |
| `3` | `AVFrame` | struct | `40` | `40` | `10` | `40` | `1` | `$5.89` | `13m01s` | `650` | `$17.51` | `24m37s` | `+526/-41` | fixed · record |
| **Σ `32`** | | | **`110`** | **`102`** | **`29`** | **`91`** | **`32`** | **`$91.87`** | | **`4513`** | **`$135.79`** | | **`+2729/-331`** | **`32`/`32` reviewed** |

### Batches — types

| DAG layer | units | loc | $ | wall (longest) | wall (actual) | serial Σ | $/unit | $/loc |
|---|---|---|---|---|---|---|---|---|
| `0` | `25` | `2486` | `$69.55` | `15m55s` | **`15m56s`** | `2h04m17s` (`7.8`x) | `$2.78` | `$0.03` |
| `1` | `4` | `939` | `$12.00` | `10m43s` | **`18m42s`** | `23m45s` (`2.2`x) | `$3.00` | `$0.01` |
| `2` | `2` | `438` | `$4.43` | `8m51s` | **`11m51s`** | `8m51s` (`1.0`x) | `$2.21` | `$0.01` |
| `3` | `1` | `650` | `$5.89` | `13m01s` | **`13m02s`** | `13m01s` (`1.0`x) | `$5.89` | `$0.01` |
| **Σ** | **`32`** | **`4513`** | **`$91.87`** | — | **`1h05m44s`** | **`2h49m55s`** (**`2.6`x**) | **`$2.87`** | **`$0.02`** |

### Symbols

| DAG layer | symbol | kind | target fns | deps | wrappers | batch | rv batch | verdict |
|---|---|---|---|---|---|---|---|---|
| `0` | `av_chroma_location_from_name` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_dynarray_add` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_file_map` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_file_unmap` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_freep` | function | `62` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_gettime` | function | `4` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_gettime_relative` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_gettime_relative_is_monotonic` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_log_get_flags` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_log_get_level` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_log_set_callback` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_log_set_flags` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_log_set_level` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_malloc` | function | `47` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_malloc_array` | function | `4` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_mallocz` | function | `80` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_match_name` | function | `2` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_md5_sum` | function | `1` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_opt_set` | function | `3` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_opt_set_bin` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_opt_set_double` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_opt_set_image_size` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_opt_set_int` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_realloc` | function | `15` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_reduce` | function | `5` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_strerror` | function | `1` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_usleep` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `av_version_info` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `avutil_configuration` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `avutil_license` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `0` | `avutil_version` | function | `0` | — | `1` | `L0·b13` | `L0·b13` | fixed · record |
| `1` | `av_add_q` | function | `1` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_alpha_mode_from_name` | function | `0` | `{'name': 'AVAlphaMode', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_alpha_mode_name` | function | `0` | `{'name': 'AVAlphaMode', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_alloc` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}`, `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_drain` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_free` | function | `1` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_peek` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_peek_at` | function | `1` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_read` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_realloc` | function | `1` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_reset` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_size` | function | `1` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_space` | function | `1` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_audio_fifo_write` | function | `0` | `{'name': 'AVAudioFifo', 'defined_in': 'libavutil/audio_fifo.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_channel_description` | function | `0` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_channel_from_string` | function | `2` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_channel_name` | function | `0` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_chroma_location_enum_to_pos` | function | `1` | `{'name': 'AVChromaLocation', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_chroma_location_name` | function | `0` | `{'name': 'AVChromaLocation', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_chroma_location_pos_to_enum` | function | `0` | `{'name': 'AVChromaLocation', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_cmp_q` | function | `5` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_color_primaries_name` | function | `0` | `{'name': 'AVColorPrimaries', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_color_range_name` | function | `0` | `{'name': 'AVColorRange', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_color_space_name` | function | `0` | `{'name': 'AVColorSpace', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_color_transfer_name` | function | `0` | `{'name': 'AVColorTransferCharacteristic', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_d2q` | function | `5` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_copy` | function | `5` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_count` | function | `1` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_free` | function | `11` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_get` | function | `1` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}`, `{'name': 'AVDictionaryEntry', 'defined_in': 'libavutil/dict.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_get_string` | function | `1` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_iterate` | function | `5` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}`, `{'name': 'AVDictionaryEntry', 'defined_in': 'libavutil/dict.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_parse_string` | function | `2` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_set` | function | `4` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_dict_set_int` | function | `0` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_div_q` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_frame_side_data_name` | function | `0` | `{'name': 'AVFrameSideDataType', 'defined_in': 'libavutil/frame.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_gcd_q` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_bytes_per_sample` | function | `3` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_media_type_string` | function | `0` | `{'name': 'AVMediaType', 'defined_in': 'libavutil/avutil.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_packed_sample_fmt` | function | `0` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_pix_fmt` | function | `1` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_pix_fmt_name` | function | `3` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_planar_sample_fmt` | function | `0` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_sample_fmt` | function | `1` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_get_sample_fmt_name` | function | `2` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_hwdevice_find_type_by_name` | function | `0` | `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_hwdevice_get_type_name` | function | `0` | `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_hwdevice_iterate_types` | function | `0` | `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_image_alloc` | function | `0` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b3` | `L1·b3` | fixed |
| `1` | `av_image_copy_to_buffer` | function | `0` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_image_fill_arrays` | function | `0` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_image_fill_black` | function | `0` | `{'name': 'AVColorRange', 'defined_in': 'libavutil/pixfmt.h'}`, `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_image_fill_pointers` | function | `3` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_image_get_buffer_size` | function | `1` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_inv_q` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_make_q` | function | `3` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_md5_alloc` | function | `2` | `{'name': 'AVMD5', 'defined_in': 'libavutil/md5.c'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_md5_final` | function | `3` | `{'name': 'AVMD5', 'defined_in': 'libavutil/md5.c'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_md5_init` | function | `3` | `{'name': 'AVMD5', 'defined_in': 'libavutil/md5.c'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_md5_update` | function | `4` | `{'name': 'AVMD5', 'defined_in': 'libavutil/md5.c'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_mul_q` | function | `2` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_nearer_q` | function | `1` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_array` | function | `0` | `{'name': 'AVOptionType', 'defined_in': 'libavutil/opt.h'}` | `2` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_dict` | function | `0` | `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_pixel_fmt` | function | `0` | `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_q` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_sample_fmt` | function | `0` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_opt_set_video_rate` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_q2d` | function | `1` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_q2intfloat` | function | `0` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_rescale_q` | function | `2` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_rescale_q_rnd` | function | `2` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}`, `{'name': 'AVRounding', 'defined_in': 'libavutil/mathematics.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_rescale_rnd` | function | `6` | `{'name': 'AVRounding', 'defined_in': 'libavutil/mathematics.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_sample_fmt_is_planar` | function | `9` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_alloc` | function | `1` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_alloc_array_and_samples` | function | `0` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_copy` | function | `1` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_fill_arrays` | function | `1` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_get_buffer_size` | function | `5` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_samples_set_silence` | function | `1` | `{'name': 'AVSampleFormat', 'defined_in': 'libavutil/samplefmt.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `1` | `av_sub_q` | function | `1` | `{'name': 'AVRational', 'defined_in': 'libavutil/rational.h'}` | `1` | `L1·b4` | `L1·b4` | held |
| `2` | `av_buffer_alloc` | function | `8` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_allocz` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_get_ref_count` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_is_writable` | function | `3` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_make_writable` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_realloc` | function | `1` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_ref` | function | `10` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_buffer_unref` | function | `23` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_get_bits_per_pixel` | function | `1` | `{'name': 'AVPixFmtDescriptor', 'defined_in': 'libavutil/pixdesc.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwdevice_ctx_alloc` | function | `2` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwdevice_ctx_create` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}`, `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwdevice_ctx_create_derived` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwdevice_ctx_create_derived_opts` | function | `1` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVDictionary', 'defined_in': 'libavutil/dict.c'}`, `{'name': 'AVHWDeviceType', 'defined_in': 'libavutil/hwcontext.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwdevice_ctx_init` | function | `2` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwframe_ctx_alloc` | function | `1` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwframe_ctx_init` | function | `0` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_hwframe_transfer_get_formats` | function | `1` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVHWFrameTransferDirection', 'defined_in': 'libavutil/hwcontext.h'}`, `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_opt_find2` | function | `12` | `{'name': 'AVOption', 'defined_in': 'libavutil/opt.h'}` | `2` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_pix_fmt_desc_get` | function | `18` | `{'name': 'AVPixFmtDescriptor', 'defined_in': 'libavutil/pixdesc.h'}`, `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_pix_fmt_desc_get_id` | function | `0` | `{'name': 'AVPixFmtDescriptor', 'defined_in': 'libavutil/pixdesc.h'}`, `{'name': 'AVPixelFormat', 'defined_in': 'libavutil/pixfmt.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `2` | `av_pix_fmt_desc_next` | function | `0` | `{'name': 'AVPixFmtDescriptor', 'defined_in': 'libavutil/pixdesc.h'}` | `1` | `L2·b1` | `L2·b1` | fixed · record |
| `3` | `av_channel_layout_channel_from_index` | function | `5` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}`, `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_channel_from_string` | function | `0` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}`, `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_check` | function | `3` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_compare` | function | `2` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_copy` | function | `6` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_default` | function | `1` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_describe` | function | `1` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_from_mask` | function | `2` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_from_string` | function | `3` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_index_from_channel` | function | `2` | `{'name': 'AVChannel', 'defined_in': 'libavutil/channel_layout.h'}`, `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_index_from_string` | function | `1` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_retype` | function | `1` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}`, `{'name': 'AVChannelOrder', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_standard` | function | `0` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_subset` | function | `0` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_channel_layout_uninit` | function | `7` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `3` | `av_opt_set_chlayout` | function | `0` | `{'name': 'AVChannelLayout', 'defined_in': 'libavutil/channel_layout.h'}` | `1` | `L3·b1` | `L3·b1` | fixed · record |
| `4` | `av_frame_alloc` | function | `6` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_clone` | function | `0` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_copy` | function | `2` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_copy_props` | function | `1` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_free` | function | `7` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_get_buffer` | function | `3` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_get_side_data` | function | `1` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}`, `{'name': 'AVFrameSideData', 'defined_in': 'libavutil/frame.h'}`, `{'name': 'AVFrameSideDataType', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_is_writable` | function | `1` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_make_writable` | function | `0` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_new_side_data` | function | `11` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}`, `{'name': 'AVFrameSideData', 'defined_in': 'libavutil/frame.h'}`, `{'name': 'AVFrameSideDataType', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_remove_side_data` | function | `0` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}`, `{'name': 'AVFrameSideDataType', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_frame_unref` | function | `8` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_hwframe_get_buffer` | function | `3` | `{'name': 'AVBufferRef', 'defined_in': 'libavutil/buffer.h'}`, `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| `4` | `av_hwframe_transfer_data` | function | `2` | `{'name': 'AVFrame', 'defined_in': 'libavutil/frame.h'}` | `1` | `L4·b0` | `L4·b0` | fixed · record |
| **Σ `164`** | | | **`497`** | | **`166`** | **`6` batches** | **`6` batches** | **`32` held · `132` fixed** |

### Batches — symbols

| DAG layer | units | loc | $ | wall | $/unit | $/loc |
|---|---|---|---|---|---|---|
| `0` | `31` | `808` | `$5.46` | `12m39s` (`1.0`x) | `$0.18` | `$0.01` |
| `1` | `82` | `2160` | `$12.64` | `18m40s` (`1.5`x) | `$0.15` | `$0.01` |
| `2` | `21` | `685` | `$7.64` | `11m49s` (`1.0`x) | `$0.36` | `$0.01` |
| `3` | `16` | `322` | `$3.60` | `7m43s` (`1.0`x) | `$0.22` | `$0.01` |
| `4` | `14` | `250` | `$2.85` | `6m11s` (`1.0`x) | `$0.20` | `$0.01` |
| **Σ** | **`164`** | **`4225`** | **`$32.19`** | **`1h05m44s`** (**`1.0`x**, session wall) | **`$0.20`** | **`$0.01`** |

### Batches — review

One agent per judged batch, same split as the wave it judges. `rv loc` is the
net `.rs` delta of the landing commit; a review that confirms without changing
code reads `+0/-0`.

| session | batch | units | rv loc | rv $ | rv wall | $/symbol | $/type |
|---|---|---|---|---|---|---|---|
| `82c3` | `L3·b0` | `1 types` | `+526/-41` | `$17.51` | `24m37s` | — | `$17.51` |
| `82c3` | `L3·b1` | `16 symbols` | `+355/-23` | `$12.95` | `19m31s` | `$0.81` | — |
| `82c3` | `L4·b0` | `14 symbols` | `+329/-15` | `$11.54` | `16m37s` | `$0.82` | — |
| `f119` | `L0·b0` | `2 types` | `+25/-2` | `$7.87` | `13m56s` | — | `$3.93` |
| `f119` | `L0·b1` | `2 types` | `+36/-8` | `$5.13` | `9m33s` | — | `$2.57` |
| `f119` | `L0·b2` | `2 types` | `+100/-13` | `$3.55` | `6m18s` | — | `$1.78` |
| `f119` | `L0·b3` | `2 types` | `+156/-18` | `$4.46` | `7m57s` | — | `$2.23` |
| `f119` | `L0·b4` | `2 types` | `+64/-0` | `$3.44` | `6m20s` | — | `$1.72` |
| `f119` | `L0·b5` | `2 types` | `+0/-0` | `$12.58` | `23m00s` | — | `$6.29` |
| `f119` | `L0·b6` | `2 types` | `+110/-25` | `$6.18` | `11m14s` | — | `$3.09` |
| `f119` | `L0·b7` | `2 types` | `+174/-11` | `$4.07` | `8m34s` | — | `$2.03` |
| `f119` | `L0·b8` | `2 types` | `+0/-0` | `$5.32` | `10m38s` | — | `$2.66` |
| `f119` | `L0·b9` | `2 types` | `+18/-11` | `$7.40` | `13m20s` | — | `$3.70` |
| `f119` | `L0·b10` | `2 types` | `+0/-0` | `$5.82` | `10m08s` | — | `$2.91` |
| `f119` | `L0·b11` | `2 types` | `+0/-0` | `$4.54` | `10m06s` | — | `$2.27` |
| `f119` | `L0·b12` | `1 types` | `+0/-0` | `$5.05` | `9m39s` | — | `$5.05` |
| `f119` | `L0·b13` | `31 symbols` | `+733/-26` | `$19.47` | `28m29s` | `$0.63` | — |
| `f119` | `L1·b0` | `2 types` | `+462/-82` | `$11.73` | `20m13s` | — | `$5.87` |
| `f119` | `L1·b1` | `1 types` | `+348/-31` | `$7.20` | `12m59s` | — | `$7.20` |
| `f119` | `L1·b2` | `1 types` | `+186/-25` | `$6.15` | `11m39s` | — | `$6.15` |
| `f119` | `L1·b3` | `50 symbols` | `+325/-27` | `$11.31` | `18m55s` | `$0.23` | — |
| `f119` | `L1·b4` | `32 symbols` | `+0/-0` | `$19.95` | `29m01s` | `$0.62` | — |
| `f119` | `L2·b0` | `2 types` | `+524/-64` | `$17.78` | `22m35s` | — | `$8.89` |
| `f119` | `L2·b1` | `21 symbols` | `+128/-9` | `$9.18` | `15m37s` | `$0.44` | — |
| **Σ** | **`24` agents** | **`32` types · `164` symbols** | **`+4599/-431`** | **`$220.19`** | **`29m01s`** (longest; **`6h01m07s`** serial, **`12.4`x**) | **`$0.51`** | **`$4.24`** |

## Safety audit

`crustify-audit <crate> unsafe`, unseeded — tree-wide, not
per seed. Two snapshots: the tree the review pass judged, and the tree it
produced.

| | before review (`5fd5c3a7d7`) | after review (`414ff93355`) |
|---|---|---|
| unsafe loc | `1016` | `1056` |
| % of loc | `24.7%` | `24.2%` |
| blocks | `501` | `519` |
| % in `impl T` | `58.5%` | `59.3%` |
| `unsafe fn` | `103` | `111` |
| ...of which not sanctioned | `26` | `33` |
| raw-ptr smell | `10` | `8` |
| void-ptr smell | `1` | `1` |
| FFI calls | `184` | `188` |
| `&`/`&mut` on a wrapper | `0` | `0` |
| field proj outside an accessor | `0` | `0` |

### All metrics

| metric | before | after | Δ | reading |
|---|---|---|---|---|
| `code_lines` | `4114` | `4372` | `+258` | union of HIR definition spans (denominator); `cfg`-disabled items excluded |
| `total_stmts` | `554` | `663` | `+109` | statements |
| `unsafe_blocks` | `501` | `519` | `+18` | count of `unsafe { }` blocks, macro-expanded included |
| `unsafe_block_stmts` | `9` | `10` | `+1` | statements inside them |
| `unsafe_block_lines` | `1016` | `1057` | `+41` | their lines, every outermost block |
| `unsafe_block_code_lines` | `1016` | `1056` | `+40` | **24.7% → 24.2%** |
| `unsafe_blocks_wrapper_impl` | `293` | `308` | `+15` | inside `impl <wrapper T>` |
| `unsafe_blocks_ffi_export` | `0` | `0` | `0` | inside the C-ABI gateway |
| `unsafe_fns` | `103` | `111` | `+8` | `unsafe fn` declarations, post-expansion |
| `unsafe_fns_seam` | `77` | `78` | `+1` | ...the sanctioned subset |
| **`unsafe fn` smell** | **`26`** | **`33`** | **`+7`** | the remainder — read each and accept or fix it |
| `unsafe_fns_pub` | `102` | `109` | `+7` | ...of `unsafe_fns`, exported from the crate |
| `unsafe_impls` / `unsafe_traits` | `34` / `0` | `35` / `0` | `+1` | lifecycle contracts asserted once per type |
| `ffi_calls` | `184` | `188` | `+4` | calls to a foreign item — the unsafe-FFI-call surface |
| `wrapper_newtypes` | `15` | `15` | `0` | LAYOUT newtypes — `repr(transparent)` over a `repr(C)` type by value, detected structurally |
| `wrapper_newtypes_declared` | `15` | `15` | `0` | the `CCell`-declared count, for comparison |
| `wrapper_declared_nonconformant` | `0` | `0` | `0` | declared but failing the structural test — **target 0** |
| `wrapper_newtypes_undeclared` | `0` | `0` | `0` | structural but undeclared — a hand-written layout newtype |
| `raw_ptr_args` | `53` | `53` | `0` | raw-ptr positions in arguments |
| `raw_ptr_rets` | `63` | `61` | `-2` | raw-ptr positions in returns |
| **total positions** | **`116`** | **`114`** | `-2` | args + rets; disjoint, so this is the surface |
| `raw_ptr_seam` | `106` | `106` | `0` | sanctioned: seam fn / `mod ffi_export` / `extern "C"` / ptr-to-own-`Self` |
| **smell (total − seam)** | **`10`** | **`8`** | `-2` | the non-seam remainder |
| `raw_ptr_wrapped` | `2` | `0` | `-2` | **of the smell**: pointee is a C type that HAS a wrapper — the actionable defect |
| `raw_ptr_in_wrapper` | `0` | `0` | `0` | **of the smell**: inside a wrapper impl — the least excusable placement |
| `raw_ptr_derefs` | `164` | `170` | `+6` | `*p` on a raw pointer (volume) |
| `ref_to_type_wrapper` | `0` | `0` | `0` | `&`/`&mut` on a layout newtype — **target 0** |
| `field_proj_wrapped` | `164` | `170` | `+6` | projection VOLUME — shares one HIR shape with `addr_of!`, not a violation |
| `field_proj_outside_impl` | `0` | `0` | `0` | projections outside any accessor — **target 0** |
| `field_ref_wrapped` | `0` | `0` | `0` | `&(*p).field` — forbidden by the translator playbook — **target 0** |
| `void_ptr_sanctioned` | `45` | `45` | `0` | `*c_void` in a seam / `ffi_export` / `extern "C"` signature |
| `void_ptr_smell` | `1` | `1` | `0` | `*c_void` elsewhere; `void_ptr_sites` names each one |

### What the review moved

**The one metric that is a defect count went to zero.** `raw_ptr_wrapped` —
a raw pointer whose pointee is a C type that already has a wrapper — was `2`
before the review and is `0` after. Those were the only positions where the
crate held a pointer it had a safe type for.

**Every target-`0` metric held at `0`, and an unchanged `0` here is a result.**
`ref_to_type_wrapper` is `0` against `15` layout newtypes — read as a pair, so
it is a real zero and not the vacuous one you get when `wrapper_newtypes` is
itself `0`. `field_ref_wrapped`, `field_proj_outside_impl`,
`raw_ptr_derefs_outside_impl` and `wrapper_declared_nonconformant` were `0`
before and after. `wrapper_newtypes_declared` equals `wrapper_newtypes` at
`15`, with `0` undeclared, so no hand-written layout newtype escaped the
`CCell` declaration.

**The raw-pointer surface shrank.** `raw_ptr_rets` fell `63 → 61` while
`raw_ptr_args` and `raw_ptr_seam` held, taking the non-seam smell from `10` to
`8`. The `8` that remain are `ffibox` handle-construction seams.

**The unsafe surface grew, and that is the review working rather than
regressing.** `unsafe_blocks` `501 → 519`, `unsafe_fns` `103 → 111`,
`ffi_calls` `184 → 188`, against `code_lines` `4114 → 4372`. The review added
accessors, falsifiable layout assertions and two documented
`assume_init` routines; folding those into fewer, larger blocks would improve
the count and make the crate worse. The ratio actually fell, `24.7% → 24.2%`.
The `unsafe fn` smell rose `26 → 33`; every one of the `18` `pub unsafe fn` in
the tree carries a `# Safety` section, and they are raw-pointer constructors
(`from_raw`, `from_ptr`, `from_raw_parts`) and invariant-bearing setters
(`set_opaque`, `set_line_size`, `replace_buffer`) whose obligation the type
system cannot carry.

**`void_ptr_smell` stayed at `1`, deliberately.** It is `freep_raw` at
`mem.rs:697` — a private helper behind a sealed trait, taking `*mut c_void`
because `av_freep` takes `void**` and needs a real pointer slot to null out.
The safe surface over it is `AvFreepTarget::free_with_av_freep`, implemented
only for `CVoidBox<AvFree>` and `CVec<T, AvFree>`. A necessary seam, left in
place with its justification, which is what the playbook asks for.

## Notes

The only prose outside the setup and legend above: pitfalls, findings, and the
context each table cannot carry. One `###` subsection per finding, titled by
what it is about.

### The agentic UB pass found four soundness bugs the deterministic pass cannot see

Run with the user's standing approval (`audit-ub: at campaign end`) on the
promoted tree `24cd1b5a06`, under `anthropic/claude-opus-5` — codex has no
credentials in this environment, and an independent model is what the review
principle above asks for anyway. Two agents, `71.5m` against a `60m` budget:
the runner spawns agents until the budget is spent and never kills one, so it
overshoots by however long the last one takes.

**4 advisories, 14 lead notes.** Every advisory is a reproduction that
`#![forbid(unsafe_code)]`, depends on the audited crate, and calls only its
public API.

Cost `$54.40` over `71.5m` wall (agent 1 `$42.25`/`48.8m`, agent 2
`$12.15`/`22.7m`), which takes the campaign to **`$438.73`**. That figure is
computed here rather than by `crustify-log-cost`: `crustify-audit ub` writes a
different usage schema — a `records` list with epoch `started_at`/`ended_at` and
no `provider`/`model` — which the campaign parser reads as `$0.00`. Priced from
the same Anthropic rate table the campaign agents were priced from, so the
numbers remain comparable. A second, smaller tooling gap alongside the one
below.

| advisory | reachable by | instrument | consequence |
|---|---|---|---|
| `av-frame-get-buffer-unbounded-alignment` | a large positive `alignment` | ASan + UBSan | **memory unsafety** — planes ~2 GiB outside a 104-byte allocation |
| `av-nearer-q-cmp-multiply-overflow` | `av_nearer_q(1/1, 1/1, 0/0)` | UBSan | UB, and a return outside the documented range |
| `av-file-map-safe-mapping-of-a-mutable-file` | mapping a file another process writes | Rust+C ASan | **memory unsafety** — a live `&[u8]` stops being dereferenceable |
| `imgutils-unbounded-alignment` | `align = i32::MAX` | UBSan | UB only; the agent argues no memory error is reachable |

**One defect class, applied inconsistently.** Three of the four are the same
mistake: a *safe* wrapper forwarding an unvalidated integer into a C
precondition. Each module reasoned about its alignment parameter **from below**
and never from above. `imgutils.rs` is the sharpest case — it rejects
`align <= 0` with a written-out argument that ends *"C never range-checks the
parameter, so the wrappers do"*, and then does not range-check the top. The
crate already knew the pattern: `opt.rs` refuses `AV_OPT_SEARCH_FAKE_OBJ`
before the call for exactly this reason. It was not applied uniformly.

**This is the case for the two audit verbs being separate.** The deterministic
pass scored this same tree clean: `raw_ptr_wrapped` `0`, `ref_to_type_wrapper`
`0`, every target-`0` metric holding, `519` unsafe blocks each carrying a
`SAFETY:` comment, clippy silent under a denying lint. None of that could see
these bugs, because in none of them is an `unsafe` block wrong. The defect is a
**safe** function accepting an argument it should refuse. A syntax-and-types
pass cannot ask whether an `i32` is a legal `align` for a C routine three
call-levels away; that judgement is what the agentic half is for. Neither
number is redundant, and neither substitutes for the other.

#### Independent verification before promotion

The playbook's gate is that the orchestrator reruns the evidence rather than
trusting the report. What I ran myself, on my own builds:

| check | result |
|---|---|
| `repro-frame-align` on the **unpatched** tree | UBSan `frame.c:115: signed integer overflow: 1073741825 + 1073741825`; ASan `SEGV on 0x60c080000141` via `av_frame_copy`, the exact address the run printed |
| `repro-frame-align` on the **patched** tree | `REFUSED: av_frame_get_buffer(.., 1073741825) -> Err(-22)` |
| `repro-nearer-q` **unpatched** | `Ok(-2147483648)` with UBSan overflow at `rational.c:141` |
| `repro-nearer-q` **patched** | `Err(UndefinedComparison)`, **0** UBSan runtime errors |
| `repro-file-map` **patched** | both binaries fail to compile, `E0133` — the gate blocks safe code at compile time |
| cited source lines | `frame.c:115`/`:135`, `frame.rs:1786`, `imgutils.rs:20` read directly; all match |

`imgutils-unbounded-alignment` was not re-run end to end: it shares its root
cause and its fix with the frame advisory, and the agent's own argument is that
it produces no memory error. Stated so the promotion rests on what was checked.

#### Gates on the patched tree, and the promotion decision

`crates validate` clean; `cargo build` clean; `cargo clippy --workspace
--all-targets` **0** warnings; **215** libavutil tests (up from `211` — four new
regression tests) and **8** ffibox tests pass under ASan+LSan with
`detect_leaks=1`, **0** failures; the new `compile_fail,E0133` doctest passes;
`make -j64 fate-libavutil` **60/60**, exit `0`. The patch touches **no C file**.

The deterministic scan across the patch:

| metric | after review | after UB patch | Δ |
|---|---|---|---|
| `code_lines` | `4372` | `4381` | `+9` — doc comments and `cfg(test)` items are excluded, so +377 source lines move this by 9 |
| `unsafe_blocks` | `519` | `519` | `+0` — unchanged |
| `unsafe_fns` | `111` | `112` | `+1` — **the deliberate `av_file_map` gate** |
| `unsafe_fns_seam` | `78` | `78` | `+0` — unchanged |
| `ffi_calls` | `188` | `188` | `+0` — unchanged |
| `raw_ptr_wrapped` | `0` | `0` | `+0` — held at 0 |
| `ref_to_type_wrapper` | `0` | `0` | `+0` — held at 0 |
| `field_proj_outside_impl` | `0` | `0` | `+0` — held at 0 |
| `void_ptr_smell` | `1` | `1` | `+0` — the `freep_raw` seam, unchanged |

Confined to four files, one per advisory; every gate green; three of four
findings reproduced by hand. **Promoted** by fast-forward onto
`crustify/libavutil-gpt-5.6-sol`, tip `b19afcb3f0`.

#### What the pass could not reach, and one hole it closed

`hwcontext.rs` was recorded mid-run as a genuine coverage hole — a fully safe
chain (`av_hwdevice_ctx_create` → `av_hwframe_ctx_alloc` → `av_hwframe_get_buffer`
→ `av_hwframe_transfer_data`) that nobody had exercised, because this campaign
configures FFmpeg `--disable-autodetect` with every backend off. The agent
closed it rather than accepting it: POCL is a **CPU** OpenCL implementation
needing no device, so it built a `--enable-opencl` `libavutil.so`, substituted
it through `LD_LIBRARY_PATH` (the public headers are config-independent, so no
Rust rebuild), constructed **8 live device contexts** and swept the surface —
**0 ASan, 0 UBSan, 0 leaks**.

The structural result is worth more than the clean sweep: the chain stops one
step in **for every caller**, not because of this build. `av_hwframe_ctx_init`
needs `format`, `sw_format`, `width` and `height` set between alloc and init,
and `hwcontext.rs` exposes **no setter for any of them** — so
`av_hwframe_get_buffer`, `av_hwframe_transfer_get_formats` and the
transfer-formats view are unreachable from safe code today. That is a
*reachability* clearance, not a correctness one. If a configuration setter is
ever added, this surface becomes reachable at once, and `initial_pool_size` is
an `int` that C multiplies into allocation sizes — the same shape as the three
alignment advisories.

Two limits remain. **Miri cannot audit this crate at all**: essentially every
public body is an `extern "C"` call and Miri stops at the seam, so a Rust-side
aliasing violation would not be caught by it — the four classic wrapper shapes
(lending `Mut` handles, lending iterators, `Deref` exposure, asserted
`Send`/`Sync`) were cleared **by reading**, not by demonstration. And `opt.rs`,
the largest module at 2,462 lines, is clean only because it is unreachable:
every constructor of an `OptionObjectMut` is a `pub unsafe fn`.

### The review branch named in the recovery snapshot was the wrong one

`crustify/status.md`, written when the interrupted session was recovered,
recorded the complete review as *25 commits on `crustify/review-final-continuation`,
ending at `49242b39bb`*. That branch is the **base** the continuation campaign
forked from, and it carries only session `f119`'s layers 0–2. The three commits
session `82c3` produced for layers 3–4 — `AVFrame`, the frame allocators and the
channel-layout symbol batch — were never merged into it.

The complete review is `crustify/session/review-2026-08-23_22-14-28_82c3` at
`414ff93355`: **28 commits, 22 files, +6,682/−809**, not the 25 commits and
+5,461/−719 the snapshot recorded. Promoting the branch the snapshot named
would have silently dropped 31 of the 196 reviewed units — the two layers that
cost the most per unit, including every `AVFrame` finding. Both figures in the
snapshot were internally consistent, which is what made the error survivable:
nothing about `49242b39bb` looks truncated until you ask whether `82c3`'s
commits are ancestors of it.

The lesson is narrow and mechanical. A session that continues an interrupted
one lands on its **own** session branch; the campaign's `-continuation` branch
is an input to it, never an output. Reconcile a resumed campaign against
`crustify/session/*` refs and the `session.log` layer lines, not against a
branch name that reads like a result.

### Two layer-3 review agents were killed mid-flight, and their spend is real

Session `f119` was scheduled over all 196 units but stopped after layer 2. Its
log ends at `layer 2` with no failure line, and two layer-3 stage records —
`review-type_AVFrame` and `review-symbol_av_channel_layout_channel_from_index` —
carry `started_at 22:00:01` and `ended_at 22:09:26`, the session's own end
stamp. `AVFrame`'s transcript closes on `[result: error_during_execution]`.
They were spawned, ran ~9m24s each, and were cut off with the session.

The first continuation, `958e`, then died after 21 seconds having spawned the
same two stages. `82c3` redid all three batches successfully.

So the campaign paid for that work three times, twice for nothing:

| discarded | $ | agent wall |
|---|---|---|
| `f119` `review-type_AVFrame` | `$5.14` | `9m24s` |
| `f119` `review-symbol_av_channel_layout_channel_from_index` | `$5.13` | `9m24s` |
| `958e` (both stages) | `$0.49` | `0m36s` |
| **Σ discarded** | **`$10.75`** | **`19m24s`** |

The per-batch review table sums to `$220.19`; the review campaigns' total
agent spend is `$230.94`. The `$10.75` gap is exactly this, and it is charged
to no batch because no batch kept the output. Any per-unit review cost quoted
from the table is therefore the cost of the work that **landed**, not the cost
the campaign incurred.

### `crustify-log-cost` could not account for any current campaign

The accounting tool globbed `crustify/targets/**/logs/**/*.usage.json`. No
current campaign writes there — the artifact tiers put agent logs under
`crustify/campaigns/<campaign>/logs/<session>/`. Against this repository it
exited `1` with `no agent logs`, and would do the same against any tree driven
by the current CLI, so the playbook's accounting step could not be performed at
all.

Patched on `fix/log-cost-campaigns-tier` in the `crustify-cli` checkout
(`4f93c88`), with a worktree at `/opt/crustify-cli-fix-log-cost`: search both
tiers and union the results, so a tree written by either layout accounts
identically. Every dollar figure in this report comes from that patched tool
over the per-agent `usage.json` records, never from provider-reported dollars.

The per-wave view remains empty for this campaign for an unrelated reason: it
keys on commit subjects matching `crustify: L<n> `, a convention these waves do
not use. The per-agent-kind view is unaffected and is what the tables here rest
on.

### Four `Replaces:` anchors in a wrap campaign are correct

The tree carries `305` `/// Wraps:` anchors and `4` `/// Replaces:` — a native
Rust translation anchor, which normally belongs to a port. All four are in
`rational.rs`: `av_make_q`, `av_inv_q`, `av_q2d` and `av_cmp_q`. Each is
`static inline` in `rational.h`, so it has no linkable symbol for a `-sys`
crate to bind and nothing to wrap. Re-implementing them natively is the only
available translation, not an escalation of the objective. `av_cmp_q` carries
its own argument that the widened arithmetic cannot overflow, so the
reimplementation is total over every `AVRational` pair.

### The fake-object option gap, opened and closed across batches

The `AVChannelLayout` review agent flagged, explicitly outside its worklist,
that `av_opt_set_chlayout` still forwarded a raw `search_flags`, so safe code
could pass `AV_OPT_SEARCH_FAKE_OBJ` and reach the NULL class load in
`opt_set_init`. It left the fix for the batch that owned the symbol rather
than reaching across the worklist boundary.

That batch closed it. In the promoted tree all `12` public option setters that
take `search_flags` call `reject_fake_object` first; `av_opt_find2` rejects the
flag inline; and the one legitimate fake-object caller is a separately named
`av_opt_find2_fake`, which supplies the flag itself and omits the target
object — the conventions' rule for a C function with several valid contracts.
No public function taking `search_flags` reaches C unguarded.

This is the review pass behaving as designed: a cross-batch defect surfaced by
the agent that could see it and repaired by the agent that owned it.

### Reading the wall-clock columns

`wall (actual)` in the types batches table is the **layer's** measured elapsed
time from `session.log`, and a layer runs its type and symbol batches
concurrently. It is therefore not attributable to the type batches alone, and
the `Σ` of that column is the whole wrap session (`1h05m44s`), not the sum of
type-batch time. `wall (longest)` and `serial Σ` are per-kind and do sum
cleanly. Layer 0 is where concurrency paid: `2h04m17s` of agent time in
`15m56s` of wall, `7.8`x.

### Gate results, and what the deterministic scan is evidence of

On the final tree (`b19afcb3f0`, review + UB patch): `crates validate` clean;
`cargo build --workspace` clean; `cargo clippy --workspace --all-targets` clean
with zero warnings under a workspace lint that **denies**
`clippy::undocumented_unsafe_blocks`, so every one of the `519` unsafe blocks
carries a `SAFETY:` comment; `215` libavutil tests and `8` `ffibox` tests pass,
`0` failures, with the ASan+UBSan-built `libavutil.so` loaded and
`detect_leaks=1`; `1` doctest, the `compile_fail,E0133` gate on `av_file_map`;
`0` surviving `crustify:todo` anchors.

`make -j64 fate-libavutil` returns `60/60`, exit `0`, matching the recorded
baseline of *60/60 under ASan+UBSan, disabled tests: none*. Neither the review
nor the UB patch changed a C file, so this gate was expected to hold and does;
it is evidence that the wrapper tree does not perturb the C library, not that
the wrappers are correct. The UB pass is what produced evidence about the
wrappers, and it found four bugs.

Three environment notes for anyone re-running these, all of which cost time here.

**Never run `cargo` itself under `LD_PRELOAD=libasan.so`.** It crashes `rustc`
mid-build and leaves the target directory in a state where *every later* cargo
invocation dies with `rustc -vV (signal: 11, SIGSEGV)` — including plain
`cargo check`, and surviving `cargo clean`. The failure looks like a broken
toolchain and is not. Build in a clean environment, then run the **test binary**
under the preload:

```
cargo test --no-run                      # clean env
LD_LIBRARY_PATH=<repo>/libavutil \
LD_PRELOAD=$(gcc -print-file-name=libasan.so) \
ASAN_OPTIONS=verify_asan_link_order=0:detect_leaks=1 \
  target/debug/deps/libavutil-<hash>
```

**Expect roughly one run in three to die before `main`** with repeated
`AddressSanitizer:DEADLYSIGNAL` and no output — a GCC-ASan-via-`LD_PRELOAD`
shadow-mapping collision with ASLR. `setarch -R` is not permitted in this
container, so retry. Redirect to a file rather than piping to `head`: SIGPIPE
lands inside ASan's own reporting path and produces the same spam.

**The crate has `0` doctests before the UB patch and `1` after**, so a doctest
run that reports `0 passed` on the pre-patch tree is correct, not a failure to
collect.
