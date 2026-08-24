# `hwcontext.rs` with a backend actually compiled in

**Status: the coverage hole `unreachable-from-safe-code.md` recorded is now
closed. Everything safe code can reach was exercised against a real device and
is clean; everything past `av_hwframe_ctx_alloc` turns out to be unreachable
from safe code, for a structural reason, and that is the more useful half of
this note.**

`unreachable-from-safe-code.md` ends:

> **This is a genuine coverage hole, not a clearance.** [...] but
> `av_hwdevice_ctx_create` -> `av_hwframe_ctx_alloc` -> `av_hwframe_get_buffer`
> -> `av_hwframe_transfer_data` is a safe chain that nobody has exercised. A run
> with a backend compiled in (vaapi/vulkan/drm are the cheapest on Linux)
> should start here.

## Getting a backend without a GPU

None of vaapi, vulkan or drm is the cheapest — **OpenCL** is, because POCL is a
CPU implementation and needs no device at all:

```sh
apt-get update && apt-get install -y pocl-opencl-icd ocl-icd-opencl-dev opencl-headers clinfo
clinfo -l
#   Platform #0: Portable Computing Language
#    `-- Device #0: pthread-skylake-avx512-INTEL(R) XEON(R) PLATINUM 8570
```

Then a `libavutil.so` with the backend in it, built the way
`rust-side-asan.md` describes but with `--enable-opencl` added, into
`/tmp/ffsrc-ocl`. The public headers are config-independent, so
`libavutil-sys`'s bindings are unaffected and the library substitutes through
`LD_LIBRARY_PATH` with no Rust rebuild.

## What was exercised

`../tmp/hammer/src/bin/hwctx.rs` (`#![forbid(unsafe_code)]`), run under the
combined Rust+C ASan with `detect_leaks=1`:

* `av_hwdevice_iterate_types` walked to exhaustion; every reported type's name
  round-tripped through `av_hwdevice_get_type_name` /
  `av_hwdevice_find_type_by_name`.
* Four non-type name strings, and six raw `AVHWDeviceType` values outside the
  table (`-1`, `0`, `1`, `1000`, `i32::MAX`, `i32::MIN`) into
  `get_type_name`, `iterate_types` and `ctx_alloc`.
* `av_hwdevice_ctx_create` over every compiled-in type x 4 device strings
  (including `""` and a bogus one) x 4 flag values x {options, no options} —
  **8 live OpenCL device contexts were genuinely constructed**, each cloned
  twice and released.
* The two-step `av_hwdevice_ctx_alloc` + `av_hwdevice_ctx_init` path,
  including its failure arm: OpenCL cannot initialize without a device
  selection, so C answered `-5` and the wrapper handed the
  construction-phase context back. Releasing it is clean.
* `av_hwdevice_ctx_create_derived` and `av_hwdevice_ctx_create_derived_opts`,
  every live source x every type x 3 flag values x {options, none}, including
  the self-derivation case where C answers with another reference to the
  existing context rather than a new device.
* `av_hwframe_ctx_alloc` and `av_hwframe_ctx_init` against a real device, plus
  allocate-and-drop without initializing.
* `av_hwframe_transfer_data` on frames carrying no hardware context.

Result: **0 ASan reports, 0 UBSan reports, 0 leaks**, reaching the final
`hwctx done`. Log: `../tmp/logs-rustasan/hwctx-opencl.log`.

## Why the rest of the chain is not reachable at all

The chain the previous note worried about stops one step in, and not because
of the build configuration. `av_hwframe_ctx_init` fails for **every** safe
caller:

```
[AVHWFramesContext @ 0x...] The hardware pixel format '(null)' is not supported
                            by the device type 'OpenCL'
```

`AVHWFramesContext` requires `format`, `sw_format`, `width` and `height` to be
set on the context between `av_hwframe_ctx_alloc` and `av_hwframe_ctx_init`.
Those live in the buffer's `data`, and `hwcontext.rs` exposes **no setter for
any of them** — `grep -n "sw_format\|initial_pool_size" hwcontext.rs` finds only
two prose mentions in doc comments. `HWFramesContextUninit` is an opaque
newtype over `CBox<AVBufferReference>` with no methods at all.

So an `HWFramesContext` can only be obtained by:

* `av_hwframe_ctx_init`, which cannot succeed on a context safe code cannot
  configure; or
* `HWFramesContext::from_reference`, which is `pub(crate) unsafe fn`
  (`hwcontext.rs:419`); its only caller is the frame module, and the frame side
  is gated too — `AVFrameMut::replace_hardware_frames_context` is `unsafe`.

`av_hwframe_transfer_get_formats`, `av_hwframe_get_buffer` and the
`HWFrameTransferFormats` indexed view all take `&HWFramesContext`, so all three
are unreachable from safe code today. That is a *reachability* clearance, not a
correctness one: the terminator-excluding logic in `HWFrameTransferFormats`
(`hwcontext.rs:612-710`) is still only covered by the module's own
hand-built test, which fabricates a list with `av_malloc_array` rather than
receiving one from a backend.

## What would change my mind

A setter for the frames-context configuration fields. If one is ever added,
this whole surface becomes reachable at once and needs re-hammering — with a
configured OpenCL context the harness above already has the shape for it, only
the four assignments are missing. `initial_pool_size` in particular is an
`int` that C multiplies into allocation sizes, which is exactly the shape of
the three alignment advisories in this directory.
