# MI_INIT_MODE write and generated x105 RDRAM-delay frontier

Decision: `MI_INIT_MODE_EXACT_X105_WRITE_ONLY`.

The generated cold-x105 composition reaches aligned `Sw r9,0(r12)` at
`PC=0xA4000118`, CPU address `0xA4300000`, and physical address
`0x04300000`. fn64 accepts only the exact low word `0x0000010F`, stores
initialization length 15 plus initialization mode true with CPU-store
provenance, and then advances through the represented `Lui`/`Ori` pair.

The next exact frontier is aligned `Sw r9,8(r10)` at `PC=0xA4000124`, CPU
address `0xA3F80008`, physical address `0x03F80008`, and transfer word
`0x18082838`. That global/broadcast RDRAM_DELAY store remains a closed
`DirectTargetMiss` with complete preservation.

No MI read, other MI register, bus replication, RDRAM-register state, timing,
private input, authentic execution, BOOT-3, or compatibility claim is added.
The authentic checkpoint remains BOOT-2.
