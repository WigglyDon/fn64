# Address and target policy

Alignment is resolved before target or source-value consumption. A word store
requires both low address bits to be zero. Unaligned addresses select AdES.

Aligned CPU addresses must use the already represented direct KSEG0/KSEG1
translation. Only physical `[0x04001000,0x04002000)` is accepted, mapping to SP
IMEM local `[0x000,0x1000)`, with aligned word starts through `0xFFC`.
RDRAM, SP DMEM, other direct targets, and non-direct forms reject without
mutation. There is no mirroring, bus, target search, or generalized map.
