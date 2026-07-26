# RSP Scalar LW, Two NOPs, And MTC0 Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This bounded increment represents one aligned scalar RSP `Lw` identity and the
exact raw-zero RSP `Nop`. `Sp::rsp` remains the scalar-register owner,
`SpDmem` remains the DMEM value/knowledge/provenance owner, and `SpImem`
remains the fetched-instruction owner.

The public generated composition commits `0x8C040040` at local `0x00C` as
`Lw r4,0x40(r0)`, producing `r4 = 0x03A04820` from available bootstrap-owned
DMEM bytes. It then commits the distinct zero words at `0x010` and `0x014`.
After each RSP commit, one ordinary CPU-selected instruction rotates the
accepted processor token. The selected RSP word `0x40800000` at local `0x018`
is identified as `Mtc0 r0,SP_MEM_ADDR` and rejects atomically.

This is public synthetic proof. It does not execute `Mtc0`, SP DMA, scalar J,
private firmware, private cartridge input, BOOT-3, or compatibility.
