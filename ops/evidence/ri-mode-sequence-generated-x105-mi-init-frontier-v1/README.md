# RI_MODE sequence and generated x105 MI init frontier

Frontier decision: `RI_MODE_SW_FRONTIER_CONFIRMED`.

Field decision: `RI_MODE_DEFINED_FIELDS_REPRESENTABLE`.

The generated cold-x105 composition reaches aligned `Sw r0,0(r8)` at
`PC=0xA40000E8`. Its CPU/physical addresses are `0xA4700000` /
`0x04700000`, the exact R/W RI_MODE word. Before this pass the store returns
`MachineStoreWordRejectionReason::DirectTargetMiss` and preserves the complete
pre-step state.

Public RI definitions make bits 1:0 the operating-mode field, bit 2 the
stop-transmit-active field, and bit 3 the stop-receive-active field. fn64
stores only those fields with exact CPU-store lineage and rejects nonzero
undefined high bits before mutation. The bounded x105 sequence writes zero,
executes a four-iteration CPU loop, writes `0x0E`, executes a 32-iteration CPU
loop, and stops at MI_INIT_MODE with word `0x10F`.

No RI electrical effect, hardware timing, MI behavior, RDRAM initialization,
private input, authentic execution, BOOT-3, or compatibility claim is added.
The authentic checkpoint remains BOOT-2.
