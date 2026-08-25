# crustify — `ffmpeg/ffmpeg` / `libavutil`

## Campaign

- **target repo** — `https://github.com/ffmpeg/ffmpeg` @ `1019f8f036602a8464185baa4857654337eeca14`
- **target** — `libavutil`
- **campaign objective** — `wrap`
- **`impl_files`** — `libavutil/`, `compat/`
- **`api_headers`** — the published `libavutil` headers (see `crustify/oracle/targets/libavutil/oracle-config.json`)
- **agent backend** — `codex`
- **model** — `openai/gpt-5.6-sol`
- **`--billing`** — `api`
- **`--max-types`** — `4`
- **`--max-syms`** — `50`
- **`--max-loc`** — `1000`
- **`--min-fields`** — `20`
- **`--parallel-max`** — `16`
- **branch** — `crustify/libavutil-gpt-5.6-sol`, last code commit `c26528d27c`
- **deps** — crustify-cli `d756ae6` (`docs/results-template-ub`), ffibox `600399f` (`main`)

## Review pass

`--objective review`, LLM-as-a-Judge over the landed waves.

- **agent backend** — `claude`
- **model** — `anthropic/claude-opus-5`
- **`--billing`** — `subscription`
- **`--max-types`** — `15`
- **`--max-syms`** — `150`
- **`--max-loc`** — `3000`
- **`--min-fields`** — `60`
- **`--parallel-max`** — `16`
- **branch** — `crustify/session/review-2026-08-25_20-43-14_3d6e`, tip `c26528d27c`
- **agents** — `44` over `5` session(s) — `28` in the first pass, `16` in the second

`rv`-prefixed columns below carry the review pass; the unprefixed ones remain
the campaign's.

## UB pass

`crustify-audit ub`, an agentic hunt for undefined behaviour reachable from the
crate's SAFE APIs.

- **agent backend** — `claude`
- **model** — `anthropic/claude-opus-5`
- **`--billing`** — `subscription`
- **`--timeout`** — `60` min — a wall BUDGET, not a kill switch: agents are
  spawned one after another until it is reached and each finishes on its own,
  so the run overshoots by however long the last one takes. `0` runs exactly
  one agent
- **subject** — `libavutil-wrap` at `24cd1b5a06`
- **agents** — `2`, `1h11m30s` wall, `$54.40`
- **advisories** — `4` at `crustify/audit/advisories/`
- **patch** — `crustify/audit-gate-the-file-mapping-safe-code-cannot-keep-valid` at `b19afcb3f0`; `merged`

`ub`-prefixed columns carry this pass.

## Legend

- `objective` — what the batch's agents were told to do: `wrap`, `port`, or
  `raw lifetime`. The type tables are split by it, so it appears as a column
  only in `Batches — symbols`, which mixes the two
- `types` / `symbols` — scheduler units in the batch. Callbacks are scheduled
  in symbol batches and counted there
- `fields` — in-scope fields: the field accessors the oracle assigned to that
  type batch, not the type's full declared field count
- `lifecycle prims` — deleters, disposers and cloners the ownership store binds
  to that batch's types; raw-tier primitives that belong to no type are counted
  in `Raw lifetime discovery` instead
- `$` / `wall` / `loc` — that agent's computed cost, its elapsed time, and the
  `.rs` insertions of its landing commit. `wall` is `ended_at − started_at` from
  the agent's own `usage.json`, so it INCLUDES the per-worktree C rebuild
- `$/type` / `$/symbol` / `$/field` / `$/loc` — that row's `$` over its units,
  its in-scope fields, or its `loc`
- `$/type` / `$/sym` — in the Overview, a sub-campaign's cost over the types or
  symbols it was scheduled for; `—` where it was scheduled for none
- `rv $` / `rv wall` / `rv loc` — the REVIEW agent's cost, elapsed time, and net
  `.rs` line delta (`+ins/-del`) of its landing commit. Under subscription
  billing `rv $` is an API-equivalent comparison value, not a charged amount
- `ub $` / `ub wall` — the UB pass's cost and elapsed time; `—` where the
  optional pass did not run

Every table below is a heading, a model line and the table. All prose belongs
in Notes.

## Overview

- **Rust LoC, non-test** — `12,252`
- **Rust LoC, tests** — `8,201`
- **C LoC** — `6,072`
- **ported types** — `0`
- **ported symbols** — `0`
- **wrapped types** — `101` (`56%` of API)
- **wrapped symbols** — `334` (`50%` of API)
- **remaining types** — `80`
- **remaining symbols** — `337`

Implementation `openai/gpt-5.6-sol` via `codex`; review
`anthropic/claude-opus-5` via `claude`. Each row names the model that produced
it.

| sub-campaign | objective | nr types | nr symbols | session wall | total | $/type | $/sym | ub wall | ub $ |
|---|---|---:|---:|---|---:|---:|---:|---|---:|
| `lifetime-void` | raw lifetime | `0` | `1` | `10m04s` | `$4.57` (`openai/gpt-5.6-sol`) | — | `$4.57` | — | — |
| `review-void` | review | `0` | `4` | `33m12s` | `$16.52` (`anthropic/claude-opus-5`) | — | `$4.13` | — | — |
| `lifetime-string` | raw lifetime | `0` | `1` | `7m05s` | `$3.30` (`openai/gpt-5.6-sol`) | — | `$3.30` | — | — |
| `review-string` | review | `0` | `2` | `12m00s` | `$4.93` (`anthropic/claude-opus-5`) | — | `$2.46` | — | — |
| `libavutil-wrap` | wrap | `32` | `164` | `1h05m44s` | `$124.06` (`openai/gpt-5.6-sol`) | `$3.88` | `$0.76` | `1h11m30s` | `$54.40` (`anthropic/claude-opus-5`) |
| `review-final + review-final-continuation` | review | `32` | `164` | `2h11m17s` | `$230.95` (`anthropic/claude-opus-5`) | `$7.22` | `$1.41` | — | — |
| `api-50pct-wrap` | wrap | `69` | `171` | `1h35m39s` | `$185.59` (`openai/gpt-5.6-sol`) | `$2.69` | `$1.09` | — | — |
| `api-50pct-review` | review | `69` | `167` | `1h46m49s` | `$132.17` (`anthropic/claude-opus-5`) | `$1.92` | `$0.79` | — | — |
| orchestrator | orchestration | `101` | `335` | — | not recorded | — | — | — | — |
| **Σ recorded agents** | | **`202`** | **`674`** | **`7h41m50s`** | **`$702.09`** | **`$3.48`** | **`$1.04`** | | **`$54.40`** |

## Raw lifetime discovery

`openai/gpt-5.6-sol` via `codex`.

| tier | symbols submitted | strategies | CDropped | CCloned | CLenDropped | CLenCloned | $ | wall |
|---|---|---|---|---|---|---|---|---|
| void | `5` | `3` | `2` | `0` | `3` | `1` | `$4.57` | `10m04s` |
| string | `2` | `1` | `1` | `2` | `0` | `0` | `$3.30` | `7m05s` |
| **Σ** | **`7`** | **`4`** | **`3`** | **`2`** | **`3`** | **`1`** | **`$7.87`** | **`17m09s`** |

### Review, in-model

None ran; see Notes.

| tier | symbols | batches | $ | wall |
|---|---|---|---|---|
| **Σ** | **`0`** | **`0`** | **`$0.00`** | — |

### Review, independent

`anthropic/claude-opus-5` via `claude`.

| symbols | rv loc | rv $ | rv wall | rv $/symbol |
|---|---|---|---|---|
| `2` | `+66/-3` | `$4.28` | `8m46s` | `$2.14` |
| `1` | `+95/-14` | `$5.55` | `9m55s` | `$5.55` |
| `1` | `+87/-3` | `$6.69` | `14m28s` | `$6.69` |
| `2` | `+187/-27` | `$4.93` | `11m59s` | `$2.46` |
| **Σ `6`** | **`+435/-47`** | **`$21.45`** | — | **`$3.57`** |

## Target set

### Batches — types, wrap

`openai/gpt-5.6-sol` via `codex`.

| types | fields | lifecycle prims | $ | wall | $/type | $/field |
|---|---|---|---|---|---|---|
| `2` | `0` | `1` | `$7.81` | `11m28s` | `$3.91` | — |
| `2` | `0` | `0` | `$9.30` | `15m55s` | `$4.65` | — |
| `2` | `0` | `0` | `$3.57` | `7m45s` | `$1.79` | — |
| `2` | `0` | `0` | `$3.28` | `5m27s` | `$1.64` | — |
| `2` | `0` | `0` | `$4.17` | `7m07s` | `$2.09` | — |
| `2` | `5` | `1` | `$6.91` | `13m48s` | `$3.46` | `$1.38` |
| `2` | `2` | `0` | `$5.39` | `10m42s` | `$2.69` | `$2.69` |
| `2` | `0` | `0` | `$1.14` | `3m42s` | `$0.57` | — |
| `2` | `0` | `0` | `$5.23` | `9m10s` | `$2.62` | — |
| `2` | `4` | `0` | `$8.54` | `14m32s` | `$4.27` | `$2.14` |
| `2` | `0` | `0` | `$4.68` | `6m30s` | `$2.34` | — |
| `2` | `2` | `0` | `$4.68` | `9m48s` | `$2.34` | `$2.34` |
| `1` | `0` | `0` | `$4.84` | `8m22s` | `$4.84` | — |
| `2` | `6` | `2` | `$6.01` | `10m44s` | `$3.01` | `$1.00` |
| `1` | `14` | `0` | `$3.37` | `7m27s` | `$3.37` | `$0.24` |
| `1` | `7` | `0` | `$2.61` | `5m35s` | `$2.61` | `$0.37` |
| `2` | `11` | `2` | `$4.43` | `8m51s` | `$2.21` | `$0.40` |
| `1` | `40` | `4` | `$5.89` | `13m02s` | `$5.89` | `$0.15` |
| `4` | `12` | `0` | `$3.93` | `7m43s` | `$0.98` | `$0.33` |
| `4` | `21` | `0` | `$7.91` | `14m01s` | `$1.98` | `$0.38` |
| `4` | `13` | `0` | `$5.90` | `12m15s` | `$1.48` | `$0.45` |
| `4` | `17` | `0` | `$7.99` | `12m57s` | `$2.00` | `$0.47` |
| `2` | `21` | `0` | `$5.86` | `10m18s` | `$2.93` | `$0.28` |
| `1` | `21` | `0` | `$5.77` | `15m35s` | `$5.77` | `$0.27` |
| `3` | `16` | `0` | `$6.96` | `11m36s` | `$2.32` | `$0.43` |
| `4` | `9` | `0` | `$5.40` | `9m30s` | `$1.35` | `$0.60` |
| `4` | `0` | `0` | `$2.26` | `5m23s` | `$0.57` | — |
| `4` | `6` | `0` | `$6.81` | `11m00s` | `$1.70` | `$1.14` |
| `3` | `0` | `0` | `$4.08` | `8m58s` | `$1.36` | — |
| `3` | `24` | `0` | `$4.55` | `10m28s` | `$1.52` | `$0.19` |
| `2` | `23` | `0` | `$4.96` | `7m59s` | `$2.48` | `$0.22` |
| `4` | `14` | `0` | `$3.63` | `7m13s` | `$0.91` | `$0.26` |
| `1` | `7` | `0` | `$3.17` | `5m27s` | `$3.17` | `$0.45` |
| `2` | `21` | `0` | `$5.70` | `11m02s` | `$2.85` | `$0.27` |
| `1` | `23` | `0` | `$2.33` | `4m04s` | `$2.33` | `$0.10` |
| `2` | `18` | `0` | `$3.49` | `6m25s` | `$1.74` | `$0.19` |
| `3` | `25` | `0` | `$8.11` | `9m40s` | `$2.70` | `$0.32` |
| `4` | `27` | `0` | `$8.65` | `11m33s` | `$2.16` | `$0.32` |
| `2` | `11` | `0` | `$7.97` | `13m56s` | `$3.99` | `$0.72` |
| `4` | `17` | `0` | `$6.09` | `11m32s` | `$1.52` | `$0.36` |
| `1` | `6` | `0` | `$3.83` | `6m52s` | `$3.83` | `$0.64` |
| `2` | `21` | `0` | `$5.35` | `9m33s` | `$2.68` | `$0.25` |
| `1` | `4` | `0` | `$4.29` | `7m43s` | `$4.29` | `$1.07` |
| **Σ `101`** | **`468`** | **`10`** | **`$226.86`** | — | **`$2.25`** | **`$0.48`** |

### Batches — types, port

None ran; this is a wrap campaign. See Notes.

| types | fields | lifecycle prims | $ | wall | $/type | $/field |
|---|---|---|---|---|---|---|
| **Σ `0`** | **`0`** | **`0`** | **`$0.00`** | — | — | — |

### Batches — review types

`anthropic/claude-opus-5` via `claude`.

| types | rv loc | rv $ | rv wall | rv $/type |
|---|---|---|---|---|
| `2` | `+25/-2` | `$7.87` | `13m56s` | `$3.93` |
| `2` | `+36/-8` | `$5.13` | `9m33s` | `$2.57` |
| `2` | `+100/-13` | `$3.55` | `6m19s` | `$1.78` |
| `2` | `+156/-18` | `$4.46` | `7m57s` | `$2.23` |
| `2` | `+64/-0` | `$3.44` | `6m21s` | `$1.72` |
| `2` | `+0/-0` | `$12.58` | `23m01s` | `$6.29` |
| `2` | `+110/-25` | `$6.18` | `11m15s` | `$3.09` |
| `2` | `+174/-11` | `$4.07` | `8m34s` | `$2.03` |
| `2` | `+0/-0` | `$5.32` | `10m39s` | `$2.66` |
| `2` | `+18/-11` | `$7.40` | `13m21s` | `$3.70` |
| `2` | `+0/-0` | `$5.82` | `10m08s` | `$2.91` |
| `2` | `+0/-0` | `$4.54` | `10m07s` | `$2.27` |
| `1` | `+0/-0` | `$5.05` | `9m40s` | `$5.05` |
| `2` | `+462/-82` | `$11.73` | `20m14s` | `$5.87` |
| `1` | `+348/-31` | `$7.20` | `13m00s` | `$7.20` |
| `1` | `+186/-25` | `$6.15` | `11m40s` | `$6.15` |
| `2` | `+524/-64` | `$17.78` | `22m36s` | `$8.89` |
| `1` | `+526/-41` | `$17.51` | `24m37s` | `$17.51` |
| `15` | `+0/-0` | `$11.65` | `15m14s` | `$0.78` |
| `7` | `+88/-16` | `$8.69` | `12m48s` | `$1.24` |
| `15` | `+362/-29` | `$9.98` | `13m53s` | `$0.67` |
| `9` | `+1257/-11` | `$14.90` | `21m39s` | `$1.66` |
| `1` | `+217/-13` | `$6.40` | `11m32s` | `$6.40` |
| `5` | `+187/-0` | `$5.40` | `8m57s` | `$1.08` |
| `9` | `+653/-2` | `$9.49` | `13m17s` | `$1.05` |
| `5` | `+186/-87` | `$4.64` | `9m02s` | `$0.93` |
| `2` | `+67/-1` | `$4.45` | `8m28s` | `$2.23` |
| `1` | `+74/-4` | `$3.99` | `7m16s` | `$3.99` |
| **Σ `101`** | **`+5820/-494`** | **`$215.39`** | — | **`$2.13`** |

### Batches — symbols

`openai/gpt-5.6-sol` via `codex`.

| objective | symbols | loc | $ | wall | $/symbol | $/loc |
|---|---|---|---|---|---|---|
| wrap | `31` | `808` | `$5.46` | `12m40s` | `$0.18` | `$0.007` |
| wrap | `50` | `1135` | `$7.93` | `18m40s` | `$0.16` | `$0.007` |
| wrap | `32` | `1025` | `$4.71` | `9m24s` | `$0.15` | `$0.005` |
| wrap | `21` | `685` | `$7.64` | `11m50s` | `$0.36` | `$0.011` |
| wrap | `16` | `322` | `$3.60` | `7m43s` | `$0.22` | `$0.011` |
| wrap | `14` | `250` | `$2.85` | `6m12s` | `$0.20` | `$0.011` |
| wrap | `50` | `1270` | `$11.89` | `18m41s` | `$0.24` | `$0.009` |
| wrap | `36` | `765` | `$8.93` | `18m43s` | `$0.25` | `$0.012` |
| wrap | `49` | `1228` | `$11.35` | `18m31s` | `$0.23` | `$0.009` |
| wrap | `31` | `894` | `$7.59` | `15m26s` | `$0.24` | `$0.008` |
| wrap | `3` | `210` | `$4.12` | `7m08s` | `$1.37` | `$0.020` |
| wrap | `1` | `209` | `$3.44` | `8m41s` | `$3.44` | `$0.016` |
| wrap | `1` | `97` | `$3.28` | `7m07s` | `$3.28` | `$0.034` |
| **Σ** | **`335`** | **`8,898`** | **`$82.79`** | | **`$0.25`** | **`$0.009`** |

### Batches — review symbols

`anthropic/claude-opus-5` via `claude`.

| symbols | rv loc | rv $ | rv wall | rv $/symbol |
|---|---|---|---|---|
| `31` | `+733/-26` | `$19.47` | `28m29s` | `$0.63` |
| `50` | `+325/-27` | `$11.31` | `18m55s` | `$0.23` |
| `32` | `+0/-0` | `$19.95` | `29m01s` | `$0.62` |
| `21` | `+128/-9` | `$9.18` | `15m38s` | `$0.44` |
| `16` | `+355/-23` | `$12.95` | `19m31s` | `$0.81` |
| `14` | `+329/-15` | `$11.54` | `16m37s` | `$0.82` |
| `86` | `+210/-6` | `$21.10` | `30m13s` | `$0.25` |
| `47` | `+150/-24` | `$11.22` | `17m11s` | `$0.24` |
| `29` | `+119/-8` | `$11.66` | `16m48s` | `$0.40` |
| `3` | `+133/-0` | `$3.68` | `8m07s` | `$1.23` |
| `1` | `+43/-18` | `$2.37` | `6m09s` | `$2.37` |
| `1` | `+75/-1` | `$2.54` | `5m47s` | `$2.54` |
| **Σ `331`** | **`+2600/-157`** | **`$136.98`** | — | **`$0.41`** |

## Safety audit

Deterministic `crustify-audit unsafe`; no model.

### Snapshots

| | before review (`704fc88898`) | after review (`c26528d27c`) |
|---|---|---|
| unsafe loc | `3638` | `3780` |
| % of loc | `36.0%` | `35.6%` |
| blocks | `1818` | `1891` |
| % in `impl T` | `77.3%` | `75.7%` |
| `unsafe fn` | `421` | `429` |
| ...of which not sanctioned | `76` | `84` |
| raw-ptr smell | `10` | `10` |
| void-ptr smell | `2` | `2` |
| FFI calls | `336` | `374` |
| `&`/`&mut` on a wrapper | `0` | `0` |
| field proj outside an accessor | `0` | `0` |

### All metrics

| metric | before | after | Δ | reading |
|---|---|---|---|---|
| `code_lines` | `10101` | `10616` | `+515` | union of HIR definition spans (denominator); `cfg`-disabled items excluded |
| `total_stmts` | `1567` | `1658` | `+91` | statements |
| `unsafe_blocks` | `1818` | `1891` | `+73` | count of `unsafe { }` blocks, macro-expanded included |
| `unsafe_block_stmts` | `124` | `123` | `-1` | statements inside them |
| `unsafe_block_lines` | `3639` | `3781` | `+142` | their lines, every outermost block |
| `unsafe_block_code_lines` | `3638` | `3780` | `+142` | **36.0% → 35.6%** |
| `unsafe_blocks_wrapper_impl` | `1405` | `1431` | `+26` | inside `impl <wrapper T>` |
| `unsafe_blocks_ffi_export` | `8` | `8` | `0` | inside the C-ABI gateway |
| `unsafe_fns` | `421` | `429` | `+8` | `unsafe fn` declarations, post-expansion |
| `unsafe_fns_seam` | `345` | `345` | `0` | ...the sanctioned subset |
| **`unsafe fn` smell** | **`76`** | **`84`** | **`+8`** | the remainder — read each and accept or fix it |
| `unsafe_fns_pub` | `417` | `425` | `+8` | ...of `unsafe_fns`, exported from the crate |
| `unsafe_impls` / `unsafe_traits` | `130` / `0` | `136` / `0` | `+6` | lifecycle contracts asserted once per type |
| `ffi_calls` | `336` | `374` | `+38` | calls to a foreign item — the unsafe-FFI-call surface |
| `wrapper_newtypes` | `68` | `68` | `0` | LAYOUT newtypes — `repr(transparent)` over a `repr(C)` type by value, detected structurally |
| `wrapper_newtypes_declared` | `68` | `68` | `0` | the `CCell`-declared count, for comparison |
| `wrapper_declared_nonconformant` | `0` | `0` | `0` | declared but failing the structural test — **target 0** |
| `wrapper_newtypes_undeclared` | `0` | `0` | `0` | structural but undeclared — a hand-written layout newtype |
| `raw_ptr_args` | `219` | `219` | `0` | raw-ptr positions in arguments |
| `raw_ptr_rets` | `272` | `272` | `0` | raw-ptr positions in returns |
| **total positions** | **`491`** | **`491`** | `0` | args + rets; disjoint, so this is the surface |
| `raw_ptr_seam` | `481` | `481` | `0` | sanctioned: seam fn / `mod ffi_export` / `extern "C"` / ptr-to-own-`Self` |
| **smell (total − seam)** | **`10`** | **`10`** | `0` | the non-seam remainder |
| `raw_ptr_wrapped` | `0` | `0` | `0` | **of the smell**: pointee is a C type that HAS a wrapper — the actionable defect |
| `raw_ptr_in_wrapper` | `0` | `0` | `0` | **of the smell**: inside a wrapper impl — the least excusable placement |
| `raw_ptr_derefs` | `929` | `953` | `+24` | `*p` on a raw pointer (volume) |
| `ref_to_type_wrapper` | `0` | `0` | `0` | `&`/`&mut` on a layout newtype — **target 0** |
| `field_proj_wrapped` | `923` | `947` | `+24` | projection VOLUME — shares one HIR shape with `addr_of!`, not a violation |
| `field_proj_outside_impl` | `0` | `0` | `0` | projections outside any accessor — **target 0** |
| `field_ref_wrapped` | `0` | `0` | `0` | `&(*p).field` — forbidden by the translator playbook — **target 0** |
| `void_ptr_sanctioned` | `206` | `206` | `0` | `*c_void` in a seam / `ffi_export` / `extern "C"` signature |
| `void_ptr_smell` | `2` | `2` | `0` | `*c_void` elsewhere; `void_ptr_sites` names each one |

## Notes

The only prose outside the setup and legend above: pitfalls, findings, and the
context each table cannot carry. One `###` subsection per finding, titled by
what it is about. Describe the EXPERIMENT and its results — a fix made to
crustify-cli or ffibox along the way belongs in that repo's history, not here.

> Gate misses and anything the oracle and `translate` disagreed on; a wave that
> was superseded and why; what each wave's diff actually contained beyond its
> row counts; where a metric moved and what moved it; what the judge found and
> whether it held. Everything else stays in the tables.

State where the LoC figures come from, and note that all of them exclude
comments and blank lines.

`C LoC` is `crustify-oracle query dag --name <every scheduled entity> --loc`:
the oracle's translated-LoC view, a function seed valued at its body LoC and a
type seed at its field and op count. It reports the seeds only, with no closure
expansion, so it is the C the campaign translated rather than the surface it
was drawn from. Give the defining files' and the whole target's raw totals
beside it for scale.

`Rust LoC` is counted from source over the authored `.rs` files under
`crustify/rust`, excluding anything generated into `target/`, and split by
`#[cfg(test)]` module.

Say why that non-test figure differs from the `code_lines` the Safety audit
reports. The audit measures the union of HIR definition spans, so it counts
only what sits inside an item — no `use`, `mod` or free-standing attribute
lines — and by construction cannot see `cfg`-disabled code, which is why it
yields no test figure. Its number is the right denominator for the unsafe
ratios and the wrong one for how much Rust was written; the two must not be
added.

Say plainly that the Rust-to-C ratio is not like-for-like in any measure,
because the Rust carries tests, `// SAFETY:` justifications, `ffi_export`
gateways and scaffolding with no C counterpart.

The four unit counts are de-duplicated over ENTITIES, not scheduled units: an
entity appears once, under the last objective it ran, so a type wrapped and
later ported counts as ported and never in both. Name the entities that took
both paths. Callbacks count with symbols.

Some of it is structural and belongs here every time: that a review pass is a
sub-campaign of its own because the oracle re-batches the units it judges, so
its rows never line up with the wave underneath; which units a review schedule
dropped and why; which sub-campaigns the Overview lists but no table details,
and the cost that leaves unaccounted; and any column a campaign could not fill,
said once rather than left as a field of em-dashes.

### What this campaign could not fill, and what the tables leave unaccounted

**A review pass is a sub-campaign of its own.** The oracle re-batches the
units it judges under the review schedule's own budgets, so review rows never
line up with the wave underneath. Both waves here demonstrate the two ends of
that. `review-final` ran at the *same* budgets as `libavutil-wrap` and
re-derived the identical `24` batches over the identical `5` layers — a `1:1`
mapping that is an artifact of equal budgets over an equal unit set, not a
property of the pass. `api-50pct-review` ran at the wide budgets and packed
the same units into `16` batches against the wrap wave's `32`; its layer 0
alone judged `123` units in `4` batches where writing them took `13`.

**Units a review schedule dropped.** `api-50pct-review` judged `236` of the
wave's `240`: `av_bprint_finalize`, `av_fifo_freep2`, `av_hash_freep` and
`av_opt_freep_ranges` are lifecycle primitives the oracle emits with their
owning type, so they are reviewed there rather than as units of their own.
The first review pass dropped none of its `196`.

**Sub-campaigns the Overview lists but no table details.**
`review-void-correction` and `review-void-free-correction` are tracked wave
plans that landed two focused ownership corrections — `av_memdup` and `free` —
as agent branches with no session log directory. They have no `usage.json`, no
recorded cost and no wall, and they appear in no table, including the
Overview. Their landed line delta is `+115/-37` and `+165/-11`. The cost is
unrecorded rather than zero.

**Cost the Target set tables leave unaccounted.** `Batches — types, wrap` plus
`Batches — symbols` sum to `$309.65`, which is both wrap waves exactly
(`$124.06` + `$185.59`). The review batch tables sum to `$352.37` against
`$363.12` on the two review Overview rows — **`$10.75` short**, which is the
spend on agents whose output was discarded and redone; it is charged to no
batch because no batch kept it. See *Two layer-3 review agents were killed
mid-flight*.

**Columns this campaign could not fill.** `Batches — types, port` is empty:
wrap-only campaign, no unit escalated. `Review, in-model` is empty for a
different reason — every review here, both raw tiers included, ran under
`anthropic/claude-opus-5` against an `openai/gpt-5.6-sol` implementation, so
all of it is independent review and none is in-model. The `orchestrator` row
carries no figure: that session writes no `usage.json`, so its supervision
cost is not recorded anywhere and is not estimated here. Every total in this
document is agents-only.

**One batch holds both kinds.** Layer 0's twelfth `api-50pct-wrap` batch is a
symbol batch carrying the `AVFifoCB` callback beside `49` functions. Callbacks
are scheduled in symbol batches and counted with symbols, which is why the
wave reads `69` types and `171` symbols against `240` units while its
`plan_items` split `69`/`170`.

### The wide review budgets, and what they bought

The second review is the first test of the split budgets: wrap at `4` types
per agent, review at `15`. The mechanical effect is exactly as intended —
`236` units in `16` batches instead of `32`, with layer 0 judging `123` units
in `4` agents where the wrap wave needed `13`.

The trade is parallelism for context. The same layer 0 took `18m49s` to write
across `13` concurrent agents and `30m14s` to judge across `4`; the review
wave ran `1h46m49s` against the wrap wave's `1h35m39s` despite doing strictly
less work, because fewer, longer agents cannot fill a `parallel-max` of `16`.
Per unit it is cheaper than the narrow pass — `$0.56` against `$1.18` — since
one agent amortises its context load over fifteen types rather than two.

What it did **not** buy, on this evidence, is a visible defect-finding
advantage. The pass reported `0` failures over `16` batches and moved
`+3,879/-234` lines of Rust; the narrow pass over a comparable unit count
moved `+6,682/-809`. Whether the wider window catches cross-type
inconsistencies the narrow one missed is not answerable from batch counts, and
this campaign has no instrument that would settle it. Recorded as an open
question rather than a result.

### The second wave, and where the coverage went

`api-50pct-wrap` took the public API closure from `18%`/`24%` to `56%`/`50%`:
`240` units, `69` types and `171` symbols, `32` batches over `7` layers,
`1h35m39s`, `0` failures, `$185.59`.

The selection is composed rather than swept, because the remaining types and
symbols do not live together. Nearly every unwrapped **type** sat in a
metadata header family and nearly every unwrapped **symbol** in a `.c` module,
so neither a deepen-what-we-have nor an add-new-families selection reaches
`50%` on both axes: deepening alone gets `22%`/`53%`, the metadata families
alone `65%`/`30%`. Types therefore come from `dovi_meta`, `iamf`, the two HDR
dynamic-metadata headers, `stereo3d`, `film_grain_params` and `csp`; symbols
from `opt`, `common`, `avstring`, `mem`, `fifo`, `pixdesc`, `imgutils`, `hash`
and `bprint`.

Five public-API dependencies the first pass had left behind were pulled in —
`AVBPrint`, `AVClass`, `AVOptionRanges`, `tm` and the `AVFifoCB` callback —
and scheduling those surfaced two more, `AVClassCategory` and `AVOptionRange`.
Only `FF_PAD_STRUCTURE` remains unsatisfied, and it is a macro: a generated
`-sys` constant, never a schedulable unit. `AVClass` was the load-bearing one,
since `opt.c`'s `40` symbols sit on top of it.

### A landed wave failed clippy, and the fix was placement not reasoning

`api-50pct-wrap` landed with an `unsafe` block in `opt.rs` whose `// SAFETY:`
comment sat above the `assert_eq!` wrapper rather than immediately above the
block inside it. `clippy::undocumented_unsafe_blocks` is **denied** by the
workspace lints, so the wave did not pass its own gate.

The justification was already written and correct; only its position was
wrong. Moving it is an integration fix, which is the orchestrator's to make —
as against retranslating a scheduled worklist, which is not. It landed as its
own commit (`704fc88898`) so it reads as orchestrator work rather than
agent output. Worth noting that a translator's own gate did not catch it:
clippy's rule is positional, and prose that reads correctly to a human sits in
the wrong place for the lint.

### The agents adopted the new anchor convention unprompted

`conventions.md` gained `/// Field: <name>.<field>` between the two waves, and
the first wave's anchors were migrated to it mechanically. The second wave's
agents emitted `Field:` correctly without being told, because they read the
current conventions at run time. The tree now carries `404` whole-item
`Wraps:`, `471` `Field:` and `4` `Replaces:` over `455` distinct `type.field`
paths.

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

### What the first review moved

The Safety audit table above brackets the **second** review
(`704fc88898` → `c26528d27c`). This note is about the first
(`5fd5c3a7d7` → `414ff93355`), whose snapshots are not in that table.


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

**The second review moved almost nothing measurable, and that is the finding.**
Across `704fc88898` → `c26528d27c` every target-`0` metric held at `0` —
`ref_to_type_wrapper` against `68` layout newtypes now, `field_ref_wrapped`,
`field_proj_outside_impl`, `wrapper_declared_nonconformant`,
`raw_ptr_wrapped`. Raw-pointer smell held at `10` and void-pointer smell at
`2`, both unchanged. What grew — `unsafe_blocks` `1818` → `1891`,
`unsafe_fns` `421` → `429`, `ffi_calls` `336` → `374` — grew because the pass
added accessors and falsifiable assertions, not because it loosened anything;
the unsafe ratio fell, `36.0%` → `35.6%`. A review that finds no defect and
still adds `+3,879` lines of tests and justification is doing the job the
deterministic pass cannot.

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

### Where the LoC figures come from

All of them exclude comments and blank lines.

**`C LoC` — `6,072`.** From
`crustify-oracle query dag --name <every scheduled entity> --loc`: the
oracle's translated-LoC view, a function seed valued at its body LoC and a
type seed at its field and op count. It reports the seeds only, with no closure
expansion, so it is the C this campaign translated rather than the surface it
was drawn from. For scale: the `51` files the `436` seeds are defined in hold
`15,747` raw SLOC, and `libavutil/` as a whole is `268` files and `54,961` raw
SLOC — so the campaign has translated roughly `11%` of the library by raw
source while covering half its published API.

One trap worth recording: `query dag --loc` appends a `TOTAL` row to its own
output. Summing every row it prints double-counts, and the first figure this
report carried was exactly `2×` the truth until the row was noticed.

**`Rust LoC` — `12,252` non-test and `8,201` tests.** Counted from source over
the `53` authored `.rs` files under `crustify/rust`, excluding anything
generated into `target/`, and split by `#[cfg(test)]` module. Nearly half the
Rust written is test code, which is the shape a wrap campaign should have: the
tests are where a layout assertion or a lifetime claim becomes falsifiable.

**That non-test figure is not the Safety audit's `code_lines` (`10,616`), and
the two must not be added.** The audit measures the union of HIR definition
spans, so it counts only what sits inside an item — no `use`, `mod` or
free-standing attribute lines — and by construction cannot see `cfg`-disabled
code, which is why it yields no test figure at all. Its number is the right
denominator for the unsafe ratios in that section and the wrong one for how
much Rust was written.

**The Rust-to-C ratio is not like-for-like in any measure.** `20,453` Rust
against `6,072` C is not a `3.4×` expansion of the same work: the Rust carries
tests, `// SAFETY:` justifications, `ffi_export` gateways and scaffolding with
no C counterpart, while the C figure is seeds only. Quote it as a size, never
as an expansion factor.

**The four unit counts are de-duplicated over entities, not scheduled units.**
An entity appears once, under the last objective it ran. This campaign makes
that trivial: it is wrap-only, no unit was ever escalated, so `0` types and `0`
symbols are ported and no entity took both paths. Callbacks count with
symbols; the oracle scheduled none separately here.

**The API percentages and the remaining counts are against the public API
closure**, which is what `api_headers` publishes: `181` types (`132` struct,
`49` enum) and `671` linkable symbols (`589` exported functions, `67`
header-inline functions, `10` `extern` globals, `5` callbacks). The `374`
macros the headers also publish are excluded — they become generated constants
in the `-sys` crate and are never scheduled as units. That closure is
self-contained: taking the transitive dependency closure of all `181` types and
`671` symbols adds exactly one node, the macro `FF_PAD_STRUCTURE`.

Every scheduled unit sits inside that set, so coverage is a clean subset rather
than an overlap. After both waves what remains is `80` types and `336` symbols
over `106` headers, and the shape has changed: the first wave left the big
type families and the `AVOption` system standing, and the second took them, so
the tail is now flat — the heaviest cluster is `iamf.c` at `12`, then
`timecode.c`, `refstruct.c`, `threadmessage.c` and `aes_ctr.c` at `10`-`11`
each. There is no third wave that reaches a further `25` points as cheaply as
the second did. Note also that the API closure is itself a slice of the
library: `libavutil/` implements `228` targeted types and `2,014` targeted
symbols, so most of what it contains is internal and out of scope by
construction.

### The tree was migrated to the current schemas after the campaign ran

Everything above describes work that ran against the previous artifact
schemas; the tree and its artifacts were migrated afterwards, and the tables
report the migrated state. Three things moved.

**Campaign artifacts are scoped by target.** Nine per-campaign directories,
each holding a fixed `campaign.json`, became named wave plans under
`crustify/campaigns/libavutil/`, and eight session log directories merged into
that target's shared `logs/` namespace. No session name collided. The
sub-campaign names in the Overview are those wave-plan basenames.

**Wave documents moved to schema version 2**, whose only structural change
here was the top-level `waves` key becoming `steps`. `crustify.wave.load` is
strict — there is no `waves` alias — so all nine were re-validated through the
real loader rather than by inspection.

**Field accessors carry a new anchor.** `conventions.md` gained
`/// Field: <name>.<field>`, and campaign coverage now counts distinct
`type.field` paths that reached one. All `107` owner-qualified anchors in this
tree were field accessors — getters, getter/setter macro pairs, and window
views — and were rewritten from `Wraps:` to `Field:`, checked against
representative sites first rather than renamed blind. The tree now carries
`198` whole-item `Wraps:`, `107` `Field:` and `4` `Replaces:`. The coverage
figure did not move: the same `91` distinct `type.field` paths, which is also
the `fields` column of `Batches — types, wrap`, since the oracle's
`field_anchors` and the emitted accessors agree exactly.

The migrated tree was re-gated: `crates validate` clean, `cargo build` clean,
clippy `0` warnings, `215` libavutil tests passing under ASan+LSan.
