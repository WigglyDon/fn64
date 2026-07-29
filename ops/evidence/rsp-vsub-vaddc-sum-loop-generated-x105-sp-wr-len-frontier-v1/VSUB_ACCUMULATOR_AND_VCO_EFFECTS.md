# Vsub Accumulator And VCO Effects

A successful commit replaces exactly `vd` and the eight accumulator-low
slices. It preserves every accumulator high/middle slice, VCC, and VCE.

Both VCO halves become Available zero with exact `Vsub` clear provenance. The
public pre-Vsub borrow is Unavailable, so public v13 and all accumulator-low
slices become cause-known Unavailable while high/middle remain
`ConstructionOrReset` Unavailable.

The commit advances PC/next once, increments only the RSP count once, and
selects CPU. CPU Count, CPU committed count, and VI do not advance.
