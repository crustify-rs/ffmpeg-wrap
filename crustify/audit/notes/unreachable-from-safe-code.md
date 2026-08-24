# Surfaces that safe code cannot reach in this crate

**Status: cleared, with the reachability argument spelled out so a later run
does not re-derive it.**

Three modules look dangerous and are not reachable without the caller writing
`unsafe` of their own. A soundness bug needs a safe path; these have none.

## `opt.rs` (2462 lines, the largest module)

Every setter takes `&mut OptionObjectMut<'_>` or an `AVOptionMatch`, and the
only constructors of an `OptionObjectMut` are:

* `OptionObjectMut::from_raw` — `pub unsafe fn` (`opt.rs:32`), and
* `av_opt_find2`, which itself takes an `&mut OptionObjectMut` (`opt.rs:1309`
  builds the result's target from the caller's own handle).

`FakeOptionObjectRef::from_raw` is likewise `unsafe`. Nothing in `frame.rs`,
`hwcontext.rs` or elsewhere manufactures one — `grep -rn "OptionObjectMut" src/`
outside `opt.rs` is empty. So the whole `av_opt_set*` family is behind an
`unsafe` door the caller opens. The module's own tests build the object with an
`unsafe` block, which is exactly right.

Note in passing: the module already refuses `AV_OPT_SEARCH_FAKE_OBJ` in every
setter, with a written-out argument for why C would otherwise NULL-deref. That
is the same class of defensive validation the three confirmed advisories say is
missing elsewhere; the author clearly knew the pattern.

## `log.rs`

`av_log_set_callback` is `pub unsafe fn`, correctly — the callback is
process-global, called from any thread, and takes a `va_list`.
`LogContextRef::from_raw` is `unsafe`. `av_log_{get,set}_{level,flags}` are safe
and are atomic loads/stores.

## `hwcontext.rs`

Reachable in principle (`av_hwdevice_ctx_create` is safe), but this campaign's
FFmpeg build is configured `--disable-autodetect` with every hardware backend
off, so `av_hwdevice_iterate_types` yields nothing and every `create` returns
an error before touching a backend. The module's own test acknowledges this
("This campaign configures every hardware backend off, so the loop covers
nothing here"). **This was a genuine coverage hole, not a clearance — it has since been closed;
see `hwcontext-with-a-real-backend.md`, which builds a backend, exercises the
reachable surface clean, and shows the rest is not reachable from safe code at
all.**
`HWFramesContext::from_reference` is `unsafe` and crate-internal, and
`AVFrameMut::replace_hardware_frames_context` is `unsafe`, so the sharpest
edges are gated — but `av_hwdevice_ctx_create` -> `av_hwframe_ctx_alloc` ->
`av_hwframe_get_buffer` -> `av_hwframe_transfer_data` is a safe chain that
nobody has exercised. A run with a backend compiled in (vaapi/vulkan/drm are
the cheapest on Linux) should start here.
