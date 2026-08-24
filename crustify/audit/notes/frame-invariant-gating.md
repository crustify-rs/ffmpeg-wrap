# The `AVFrame` geometry/plane invariant and what actually guards it

**Status: mostly cleared; one hole found and promoted to
`advisories/av-frame-get-buffer-unbounded-alignment.md`.**

`frame.rs` states a long type invariant (`frame.rs:559`+) whose load-bearing
half is:

> Every non-null plane is valid for the extent the geometry describes:
> `format`, `width`, `height` and `linesize` for video [...]  That is why
> `set_width` and its siblings refuse a frame that already holds planes, and
> why `set_line_size` is `unsafe`.

I went through every safe writer that could break that relation.

## Gated, correctly

* `set_width` / `set_height` / `set_format` / `set_sample_count` go through
  `frame_geometry_scalar!` (`frame.rs:721`), which returns `false` without
  writing unless `AVFrameRef::is_unallocated()` holds. Verified empirically:
  `frm.rs` prints `set_width after alloc -> false` for every allocated frame.
* `set_line_size` is `pub unsafe fn` with the stride obligation written out.
* `replace_buffer`, `replace_extended_buffer`,
  `replace_hardware_frames_context`, `set_opaque` are all `unsafe`.
* `av_frame_get_buffer` additionally refuses a frame that is not unallocated
  or that already carries a hardware frames context — a precondition C states
  but does not check.

## Ungated but inert

`AVFrameMut::channel_layout_mut()` (`frame.rs:1242`) hands out an
`AVChannelLayoutMut` on the frame's embedded `ch_layout` with **no**
`is_unallocated` gate, even though the audio half of the invariant depends on
`ch_layout.nb_channels`. I tried to turn that into a bug and could not:

* the only safe mutator reachable through an `AVChannelLayoutMut` is
  `av_channel_layout_retype`, plus `custom_map_mut`/`clear_opaque`;
* `av_channel_layout_retype` never changes `nb_channels` — every branch in
  `libavutil/channel_layout.c:887` either preserves it explicitly or, for the
  `NATIVE` branch, rebuilds from a mask `masked_description` derived from the
  same channel count;
* and in any case an audio frame cannot be built through this crate at all:
  there is no safe way to *set* `ch_layout`, `av_frame_alloc` leaves it zeroed,
  `av_channel_layout_check` rejects `nb_channels == 0`, so
  `av_frame_get_buffer` always takes the video path.

So the gate is missing but currently unreachable. It is worth adding anyway if
a `replace_channel_layout` is ever introduced — flagged here rather than as a
finding.

`crop_left`/`crop_right`/`crop_top`/`crop_bottom` and `sample_rate` are
ungated `frame_scalar!`s. Nothing in the wrapped surface consumes them
(`av_frame_apply_cropping` is not wrapped), so they cannot reach an extent
computation.

## The hole

`av_frame_get_buffer(frame, alignment)` forwards `alignment: i32` to C with no
upper bound. C's `get_video_buffer` computes the allocation size and the plane
offsets from it in `int`, and overflows. The result is a frame that satisfies
none of the invariant above while `av_frame_get_buffer` reports `Ok(())`.
See the advisory.
