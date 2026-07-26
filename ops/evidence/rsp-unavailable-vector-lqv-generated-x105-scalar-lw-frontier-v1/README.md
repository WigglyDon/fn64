# Unavailable-Vector LQV And Scalar-LW Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This directory records one bounded RSP vector-memory addition. `SpDmem`
remains the singular DMEM byte/knowledge/provenance owner. `Sp::rsp` owns 32
individual vector-register slots. An aligned element-zero full-register `Lqv`
commits an available vector only when all sixteen DMEM bytes are available;
otherwise it commits one cause-known, value-unavailable register with no byte
array.

The public generated cold-x105 composition commits word `0xC80C2000` as
`Lqv v12[0],0(r0)` at local `0x008`. Low DMEM `0x000..0x010` remains
value-unavailable, so `v12` becomes unavailable with exact LQV and DMEM
knowledge cause. After one ordinary CPU step, public word `0x8C040040` at
local `0x00C` is identified as scalar `Lw r4,0x40(r0)` and rejects atomically.

This is public synthetic evidence. It is not private-firmware execution,
BOOT-3, vector arithmetic, scalar-LW execution, or compatibility.
