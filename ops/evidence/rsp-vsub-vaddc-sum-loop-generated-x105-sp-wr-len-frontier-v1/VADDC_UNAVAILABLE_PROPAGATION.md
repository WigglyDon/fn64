# Vaddc Unavailable Propagation

Two Available operands produce an Available vector, eight Available
accumulator-low slices, and an Available carry byte.

If either operand is Unavailable, the whole destination, all accumulator-low
slices, and VCO carry/borrow become cause-known Unavailable. VCO not-equal is
still Available zero. High/middle accumulator slices, VCC, and VCE are
preserved.

Public v13 begins Unavailable and v14 is Available from each aligned `Lqv`, so
all 256 public `Vaddc` instructions commit and propagate exact unavailable
lineage without storing vector bytes or symbolic expressions.
