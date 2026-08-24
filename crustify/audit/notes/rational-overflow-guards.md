# `rational.rs`'s integer-overflow guards: where they hold and where they don't

**Status: one gap found (`av_nearer_q`, promoted to an advisory); the rest
cleared, including one diagnostic the crate knowingly accepts.**

`rational.rs` is unusual for this crate in that it *already* treats C's signed
overflow as a soundness obligation and rejects the offending inputs. That makes
it the right place to check whether the analysis is complete. It is nearly so.

## Guards that hold

I re-derived each one against `libavutil/rational.c` at this revision:

* `av_add_q` rejects `(i32::MIN/i32::MIN, i32::MIN/i32::MIN)`. C computes
  `b.num*(int64_t)c.den + c.num*(int64_t)b.den`; an `i32 x i32` product lies in
  `[-(2^62 - 2^31), 2^62]`, so the sum reaches `2^63` only when both products
  are `2^62`, i.e. exactly that pair. Correct and exactly one pair wide.
* `av_sub_q` needs no guard *at this revision*: the difference of two such
  products lies in `[-(2^63 - 2^31), 2^63 - 2^31]`, inside `i64`. (Older
  FFmpeg wrote `av_add_q(b, (AVRational){-c.num, c.den})`, which would have
  negated `i32::MIN`. It does not any more — `rational.c:180`.)
* `av_mul_q` / `av_div_q` need none: the products stay at `2^62`, and
  `av_reduce`'s `FFABS` never sees `i64::MIN`.
* `av_q2intfloat` rejects `i32::MIN` in either half, because C normalizes the
  sign with `q.num *= -1`.
* `av_reduce` rejects a non-positive bound and `i64::MIN` inputs.
* `av_gcd_q` rejects both denominators zero.

`../tmp/hammer/src/bin/rat.rs` walks all 9^4 pairs drawn from the `i32`
extremes through every arithmetic wrapper plus `av_rescale_q{,_rnd}`; the only
two UBSan sites it reaches are the two below.

## The diagnostic the crate accepts on purpose

`rational.c:185` — `sign<<31` in `av_q2intfloat` — is UB for *every* negative
numerator. `av_q2intfloat`'s doc comment says so explicitly and argues that
refusing it would refuse every negative rational, that GCC documents it does
not exploit the latitude there, and that the encoding produced is correct.
The crate's own test suite trips it on every run (visible as a `runtime error`
line during `cargo test`). I agree with the judgement and am not reporting it;
recording it here so the next run does not spend budget on it either. It is an
upstream FFmpeg issue, not a wrapper one — a C caller hits it identically.

## The gap

`av_nearer_q`'s guard covers only the doubled-denominator product
(`rational.c:133`). It misses `rational.c:141`,
`((x_up > q.num) - (x_down < q.num)) * av_cmp_q(q2, q1)`, where `av_cmp_q`
returns `INT_MIN` as its "undefined" sentinel and the left factor can be `-1`.
See `advisories/av-nearer-q-cmp-multiply-overflow.md`.
