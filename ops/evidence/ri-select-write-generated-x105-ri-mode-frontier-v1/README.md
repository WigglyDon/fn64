# Exact RI_SELECT write and generated x105 RI_MODE frontier

Frontier decision: `RI_SELECT_SW_FRONTIER_CONFIRMED`.

Write decision: `RI_SELECT_EXACT_X105_VALUE_ONLY`.

The generated cold-x105 composition reaches aligned `Sw r9,0x0c(r8)` at
`PC=0xA40000E4`. Its old r8 is `0xFFFFFFFFA4700000`, its old r9 is
`0x0000000000000014`, and the resulting CPU/physical addresses are
`0xA470000C` / `0x0470000C`, the exact R/W RI_SELECT word. Before this pass the
store returns `MachineStoreWordRejectionReason::DirectTargetMiss` and preserves
the complete pre-step state.

The public register header does not unambiguously separate receive and transmit
field positions. The bounded x105 source does establish one exact write of
`0x00000014`, described as enabling TX/RX select. Product work is therefore
limited to storing that one exact word with CPU-store lineage, reading it
through the existing RI_SELECT `Lw` route, and stopping at the following
RI_MODE store.

All proof bytes and instruction fields are generated independently. No private
PIF firmware or cartridge ROM is used. The authentic checkpoint remains
BOOT-2.
