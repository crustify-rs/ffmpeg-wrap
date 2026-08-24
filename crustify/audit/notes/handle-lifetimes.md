# Borrowed-handle lifetimes, lending iterators, `Deref`, `Send`/`Sync`

**Status: cleared by reading. No demonstration attempted (see
`miri-cannot-cross-the-seam.md` for why none was possible).**

These are the four classic wrapper shapes. I went looking for each of them
specifically and did not find one.

## 1. `Mut` handles that lend their own lifetime

The dangerous shape is `impl<'a> FooMut<'a> { fn get(&self) -> Bar<'a> }`,
which lets a caller keep a shared view alive while continuing to use the
exclusive handle. Every `Mut` impl block in the crate is anonymous in the
lifetime and therefore cannot express it:

```
$ grep -n "^impl" src/*.rs | grep -i mut
buffer.rs:391:impl AVBufferReferenceMut<'_> {
channel_layout.rs:346:impl AVChannelCustomMut<'_> {
channel_layout.rs:849:impl AVChannelLayoutMut<'_> {
frame.rs:342:impl AVFrameSideDataMut<'_> {
frame.rs:1087:impl AVFrameMut<'_> {
pixdesc.rs:78:impl AVComponentDescriptorMut<'_> {
pixdesc.rs:338:impl AVPixFmtDescriptorMut<'_> {
rational.rs:67:impl AVRationalMut<'_> {
md5.rs:66:impl<'a> AVMD5Mut<'a> {          <- named, but its two methods return
                                              `*mut ffi::AVMD5` and `AVMD5Ref<'_>`
opt.rs:24:impl<'a> OptionObjectMut<'a> {   <- named; `as_mut_ptr(&mut self)` only
```

`Ref` handles *do* hand out `'a`-bound views (`AVFrameRef<'a>::buffer` returns
`AVBufferReferenceRef<'a>`, `AVFrameSideDataRef<'a>::data` returns
`CSlice<'a, MaybeUninit<u8>>`). That is the same rule `ffibox::CSlice::get`
uses and is sound for the same reason: the `Ref` is itself the shared-borrow
token, it is `Copy`, and nothing reachable from it writes.

The macro-generated `FooMut::as_ref(&self) -> FooRef<'_>` is bound to `&self`,
not to `'a` — `/opt/ffibox/src/macros.rs:224`. `ffibox` documents why
(`define_ctype!`, "Reading through an exclusive handle") and even ships a
`compile_fail` test for it.

## 2. Lending iterators

Two iterators exist. `AVChannelLayoutStandards`
(`channel_layout.rs:1270`) has `type Item = AVChannelLayoutRef<'static>`, which
looks alarming but is correct: `av_channel_layout_standard` walks
`channel_layout_map[]`, a `static const` table in `libavutil/channel_layout.c`
with process lifetime, and the item is a read-only handle. `collect()`ing them
yields many shared handles to immutable static storage.
`HWFrameTransferFormatIter<'a>` (`hwcontext.rs:652`) yields `AVPixelFormat`,
a `Copy` scalar.

## 3. `Deref` / `DerefMut` exposing an inner value

There are none. `grep -n "impl.*Deref" src/*.rs` is empty; ffibox deliberately
reaches handles through `as_ref`/`as_mut` instead, and documents that choice.
So `mem::swap`/`mem::replace` have nothing to reach through.

## 4. `Send` / `Sync` asserted over thread-affine state

The crate asserts neither: `grep -n "unsafe impl Send\|unsafe impl Sync"
src/*.rs` is empty. `ffibox`'s owning handles hold `NonNull`, so they are
`!Send + !Sync` by inference. `ffibox::Owner` is the one trait that claims
`Send + Sync`, and nothing in this crate implements it.

## 5. Unvalidated integer -> enum transmutes

Also absent, and deliberately so. Every C enum crossing the seam is a
`#[repr(transparent)]` newtype over the bindgen integer with a **safe**
`from_raw` (`AVPixelFormat`, `AVSampleFormat`, `AVChannel`, `AVChannelOrder`,
`AVColorRange`, `AVMediaType`, `AVFrameSideDataType`, `AVHWDeviceType`, ...).
Every value round-trips; nothing is turned into a Rust `enum`. The modules'
tests pin this against `T::MAX`/`T::MIN` of the bindgen type. I spent effort
trying to break the *consumers* of those open values instead — see
`open-enum-values-into-c.md`.

## What would change my mind

A Rust-side ASan or Miri run that actually executes the crate. Everything above
is a reading of the source and of `/opt/ffibox`, not an experiment.
