# `AVBufferReferenceMut::truncate` / `advance` and `av_buffer_realloc`

**Status: cleared, by argument and by exhaustive small-input testing.**

`buffer.rs` exposes two safe operations that rewrite an `AVBufferRef` header's
`data` and `size` fields in place (`truncate`, `advance`, `buffer.rs:399` and
`:417`). Rewriting a live C object's window from safe code is exactly the shape
worth suspecting, and it interacts with `av_buffer_realloc`, which has three
different code paths keyed on `buf->data != buf->buffer->data`.

Reading `libavutil/buffer.c`:

* both operations only ever produce a **sub-range** of the current window
  (`truncate` refuses `new_size > size`, `advance` refuses `bytes > size`), so
  the type invariant "`size` allocated bytes at `data`" is preserved;
* `av_buffer_realloc`'s copy path does `memcpy(new->data, buf->data,
  FFMIN(size, buf->size))`, bounded by the *narrowed* `buf->size`;
* its in-place path is guarded by `buf->data == buf->buffer->data`, which
  `advance` breaks and `truncate` deliberately does not — and `truncate` only
  shrinks `buf->size`, so `av_realloc(buf->buffer->data, size)` still describes
  the underlying allocation, not the narrowed view;
* `av_buffer_alloc`/`allocz` do not set `BUFFER_FLAG_REALLOCATABLE`, so only
  buffers born from `av_buffer_realloc(None, n)` reach the in-place path at
  all.

Empirically: `../tmp/hammer/src/bin/buf.rs` walks the full cross product of
`{alloc, allocz, realloc-from-None}` x 8 sizes x 8 truncate lengths x 8 advance
offsets x 8 realloc sizes, writing the window through `write_all` and taking a
second reference at each step, i.e. 3 x 8^4 = 12288 sequences. Zero ASan or
UBSan reports.

What would change my mind: a `truncate`/`advance` that could *grow* the window,
or a new wrapper for `av_buffer_create`/`av_buffer_pool_*` that admits a
caller-supplied `data` pointer.
