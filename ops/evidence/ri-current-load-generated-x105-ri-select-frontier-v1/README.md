# RI_CURRENT_LOAD update event and generated x105 RI_SELECT frontier

Frontier decision: `RI_CURRENT_LOAD_SW_FRONTIER_CONFIRMED`.

Source decision: `RI_CURRENT_LOAD_UPDATE_EVENT_REPRESENTABLE`.

The generated cold-x105 composition reaches aligned `Sw r0,8(r8)` at
`PC=0xA40000DC`. Its represented CPU address is `0xA4700008`, physical
`0x04700008`, and public RI definitions identify the exact word as write-only
RI_CURRENT_LOAD: any write updates current control.

The bounded product represents that write as one Machine-owned event which
snapshots the already stored RI_CONFIG input and enable fields and preserves
the CPU-store lineage. It does not represent a current-control result,
calibration completion, elapsed hardware time, RDRAM readiness, or any other
RI register action.

All proof bytes and instruction fields are generated independently. No private
PIF firmware or cartridge ROM is used. The authentic checkpoint remains
BOOT-2.
