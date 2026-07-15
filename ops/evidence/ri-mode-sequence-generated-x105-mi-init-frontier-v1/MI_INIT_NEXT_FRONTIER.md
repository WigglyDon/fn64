# MI_INIT_MODE next frontier

At the generated final state, r12 is `0xFFFFFFFFA4300000` and r9 is `0x10F`.
Independently encoded `Sw r9,0(r12)` at PC `0xA4000118` computes CPU address
`0xA4300000`, physical `0x04300000`, MI_INIT_MODE. The public header defines
`MI_SET_INIT` as `0x0100` and the low seven bits as init length, so the bounded
source word is `MI_SET_INIT | 15 = 0x0000010F`.

MI remains unrepresented. The store returns
`MachineStoreWordRejectionReason::DirectTargetMiss`; PC/next-PC, Count, GPRs,
all RI facts/provenance, memory, and delay context remain unchanged. No MI
owner, repeat state, RDRAM access, or bus effect is created.
