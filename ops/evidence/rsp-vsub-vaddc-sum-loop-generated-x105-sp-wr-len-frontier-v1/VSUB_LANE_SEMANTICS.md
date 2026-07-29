# Vsub Lane Semantics

Each vector lane is one big-endian signed 16-bit value. For lane `i`:

`wide = signed(vs[i]) - signed(vt[i]) - VCO.low[i]`

The destination receives `wide` clamped to `[-32768, 32767]`. Accumulator-low
receives the low 16 result bits before clamping. Positive and negative
saturation, borrow zero and one, byte/lane order, and all 256 borrow masks are
covered by independent synthetic tests. No architectural exception is raised.
