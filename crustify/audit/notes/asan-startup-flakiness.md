# `AddressSanitizer:DEADLYSIGNAL` at process start is an artifact, not a finding

**Status: cleared. Do not chase it.**

Running any of the reproduction binaries under
`LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8` fails roughly one run in
three with:

```
AddressSanitizer:DEADLYSIGNAL      (x ~20)
Segmentation fault (core dumped)
```

and **no program output at all** — not even the first `eprintln!` of `main`.
The same binary, same arguments, succeeds on the next attempt. This is the
well-known GCC-ASan-via-`LD_PRELOAD` shadow-mapping collision with ASLR; the
process dies before `main`. `setarch -R` is not permitted in this container, so
the workaround used throughout is simply to retry until the run starts.

A second, unrelated trap: piping such a binary into `head` produces the same
DEADLYSIGNAL spam, because SIGPIPE lands inside ASan's own reporting path.
Redirect to a file and grep the file instead.

Both were mistaken for real crashes early in this run. They are not.

## Do not run `cargo` itself under `LD_PRELOAD`

A second, worse trap from the same setup, and the one that cost the most time
in this run.

`cargo` shells out to `rustc -vV` to identify the toolchain. Under the
preloaded ASan runtime that child SIGSEGVs, and cargo **caches the failure**
in `<target>/.rustc_info.json`:

```json
{"rustc_fingerprint":2266397237075984366,
 "outputs":{"17228521274749693413":
   {"success":false,"status":"signal: 11, SIGSEGV: invalid memory reference", ...}},
 "successes":{}}
```

From then on *every* cargo command in that workspace fails with

```
error: process didn't exit successfully:
  `.../bin/rustc -vV` (signal: 11, SIGSEGV: invalid memory reference)
```

even without the preload, and even from another directory via
`--manifest-path`. `rustc -vV` run by hand works fine, which makes it look like
a toolchain problem rather than a cache problem. The cure is
`rm <target>/.rustc_info.json`.

The correct pattern is: **build with cargo, run the binary with the preload.**

```
cargo test -p libavutil --lib --no-run
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 ASAN_OPTIONS=detect_leaks=0 \
    ./target/debug/deps/libavutil-<hash>
```

The crate's test binary genuinely needs the preload — without it, it exits 1
with "ASan runtime does not come first in initial library list" — so plain
`cargo test` cannot be the gate for this workspace either way.
