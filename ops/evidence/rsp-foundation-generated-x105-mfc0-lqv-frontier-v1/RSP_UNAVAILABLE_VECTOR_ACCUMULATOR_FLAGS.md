# Unavailable Vector, Accumulator, And Flags

The RSP vector unit is represented only as explicitly unavailable.

The accumulator and the VCC, VCO, and VCE state are represented together as
explicitly unavailable. There are no fabricated zero arrays, vector register
values, accumulator lanes, carry bits, compare bits, or clip bits.

Scalar MFC0 neither consumes nor changes this boundary. LQV is decoded only
far enough to name the next vector frontier; it creates no vector state and
performs no load.
