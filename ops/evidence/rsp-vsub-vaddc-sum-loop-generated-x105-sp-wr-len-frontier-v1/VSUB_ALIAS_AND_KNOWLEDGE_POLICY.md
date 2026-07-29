# Vsub Alias And Knowledge Policy

Planning captures old sources before destination mutation.

When `vs == vt` under element zero, the two references name the same exact
vector state, so vector bits cancel and are not consumed. With an Available
borrow byte, each result lane is exactly zero or minus one even if the aliased
vector is Unavailable.

An Unavailable borrow makes the whole destination and all accumulator-low
slices cause-known Unavailable. A non-aliased Unavailable vector input does the
same. No unavailable backing bytes or partially available vector results are
stored.
