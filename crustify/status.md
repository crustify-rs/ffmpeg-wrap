# Crustify libavutil wrapper campaign status

Snapshot taken 2026-08-24, after the end-of-campaign review was validated and
promoted. Supersedes the 2026-08-24 recovery snapshot, which named the wrong
review branch — see "Correction" below.

## What we set out to do

- Repository: `https://github.com/ffmpeg/ffmpeg`.
- Resolved source revision: `1019f8f036602a8464185baa4857654337eeca14`.
- Objective: wrap, not port. Keep the C implementation and expose a safe Rust
  wrapper surface.
- Target: `libavutil` only. All other FFmpeg libraries and `fftools` are
  explicitly outside the campaign.
- Coverage: the subset of the public `libavutil` API reached by established
  safe Rust wrapper crates, plus its deterministic transitive dependencies.
- Selection mode: public API graph (`api_headers_only: true`).
- Translator backend/model: Codex, `gpt-5.6-sol`, subscription billing.
- Review and UB backend/model: Anthropic, `claude-opus-5`, subscription.
- Scheduling: dependency ordered, `max_types: 2`, `max_syms: 50`,
  `max_loc: 1000`, `min_fields: 10`, CLI `parallel_max: 32`.
- Review policy: one agentic review at campaign end.
- Audit policy: deterministic `crustify-audit unsafe` is the normal regression
  gate. The user approved the optional agentic `crustify-audit ub` pass at
  campaign end.

## Correction to the previous snapshot

The recovery snapshot recorded the complete review as **25 commits on
`crustify/review-final-continuation` ending at `49242b39bb`**. That was wrong,
and acting on it would have dropped review work.

`crustify/review-final-continuation` is the *base branch* the continuation
campaign forked from. It carries only session `f119`'s layers 0-2. The three
commits session `82c3` produced for layers 3-4 were never merged into it.

The complete review is `crustify/session/review-2026-08-23_22-14-28_82c3` at
`414ff93355`: **28 commits, 22 files, +6,682/-809**.

Reconcile a resumed campaign against `crustify/session/*` refs and the
`session.log` layer lines, never against a branch name that reads like a result.

## Current repository state

- Canonical branch: `crustify/libavutil-gpt-5.6-sol`.
- Canonical HEAD: `b19afcb3f0` (review promoted to `414ff93355`, review
  campaign plans committed at `24cd1b5a06`, then the two UB patch commits
  promoted by fast-forward).
- The review branch is fully promoted. `crustify/review-final-continuation` and
  every `crustify/agent/*` and `crustify/session/*` ref are retained as history.
- `crustify/wrappers-results.md` is written, in the requested template layout.

## Completion matrix

| Stage | Execution | Landed on canonical | Remaining action |
|---|---:|---:|---|
| Setup and baseline | Complete | Yes | None |
| Raw `void` lifetime discovery/review | Complete | Yes | None |
| Raw string lifetime discovery/review | Complete | Yes | None |
| Main 196-unit wrap wave | Complete | Yes | None |
| Final 196-unit review | Complete | Yes | None |
| Canonical Rust integration gates | Complete | Yes | None |
| FFmpeg 60/60 regression gate | Complete | n/a | None |
| Fresh tree-wide `unsafe` scan | Complete | n/a | None |
| Approved agentic UB pass | Complete | Yes | None — 4 advisories, patch verified and promoted |
| Final results and usage accounting | Complete | Yes | None |

The campaign is complete. Nothing is outstanding.

## Gate results on the final tree (`b19afcb3f0`)

- `crustify-cli crates validate` — clean.
- `cargo build --workspace` — clean.
- `cargo clippy --workspace --all-targets` — clean, zero warnings, under a
  workspace lint that denies `clippy::undocumented_unsafe_blocks`.
- `cargo test` — 215 libavutil tests and 8 ffibox tests pass, 0 failures, with
  the ASan+UBSan-built `libavutil.so` loaded and `detect_leaks=1`, plus the
  `compile_fail,E0133` doctest gating `av_file_map`.
- `make -j64 fate-libavutil` — 60/60, exit 0, matching the recorded baseline.
- 0 surviving `crustify:todo` anchors; 305 `/// Wraps:` and 4 `/// Replaces:`.
- Deterministic `crustify-audit unsafe`, unseeded, before and after the review:
  `raw_ptr_wrapped` 2 -> 0; `ref_to_type_wrapper` 0 -> 0 against 15 layout
  newtypes; `field_ref_wrapped`, `field_proj_outside_impl` and
  `wrapper_declared_nonconformant` held at 0.

## Reproducing the Rust test gate

The Rust tests load an ASan+UBSan-instrumented `libavutil.so`, so they need the
ASan runtime preloaded:

```
LD_LIBRARY_PATH=<repo>/libavutil \
LD_PRELOAD=$(gcc -print-file-name=libasan.so) \
ASAN_OPTIONS=verify_asan_link_order=0:detect_leaks=1 \
cargo test -p libavutil -p libavutil-sys
```

That preload kills non-instrumented host binaries — `rustdoc` and the zero-test
placeholder `-sys` harnesses die with repeated `AddressSanitizer:DEADLYSIGNAL`.
Run the rest of the workspace, and the doctests, without it.

## Tooling repair made during this campaign

`crustify-log-cost` globbed `crustify/targets/**/logs`, a path no current
campaign writes; the artifact tiers put agent logs under
`crustify/campaigns/<campaign>/logs/<session>/`. It exited 1 with `no agent
logs` against any current tree, so the playbook's accounting step could not run.
Patched on branch `fix/log-cost-campaigns-tier` in the crustify-cli checkout
(`4f93c88`), worktree at `/opt/crustify-cli-fix-log-cost`. The crustify-cli
checkout itself was restored to `scope-refactor`, so the installed editable
package still carries the original behaviour; run the fix with
`PYTHONPATH=/opt/crustify-cli-fix-log-cost/src python3 -m crustify.log_cost`.

## Resume invariants

- Treat the manifests, ownership store, landed commits, and logs as campaign
  state; do not regenerate completed waves.
- Keep all non-`libavutil` FFmpeg libraries outside the translation surface.
- The normal regression audit is `crustify-audit unsafe`.
- A UB patch stays unmerged until independently verified under the playbook's
  promotion gate. This campaign's was verified and promoted; see
  `crustify/audit/advisories/` and the report's UB section.
- Never run `cargo` itself under `LD_PRELOAD=libasan.so`: it crashes `rustc`
  mid-build and poisons the target directory for every later invocation,
  surviving `cargo clean`. Build clean, then run the test binary under preload.
