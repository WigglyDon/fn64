# RI_SELECT write next frontier

At the final generated state, r8 is `0xFFFFFFFFA4700000`, r9 is `0x14`, and
independently encoded `Sw r9,0x0c(r8)` targets CPU `0xA470000C`, physical
`0x0470000C`, RI_SELECT. CPU writes to RI_SELECT remain unsupported, so the
step returns `MachineStoreWordRejectionReason::DirectTargetMiss`.

PC/next-PC remain `0xA40000E4 / 0xA40000E8`, Count remains 32,021, and total
committed steps remain 32,037. RI_SELECT cold-entry state, RI_CONFIG, the
committed RI_CURRENT_LOAD event, all CPU registers, COP0, memory, and
provenance are preserved. RI_MODE and RDRAM initialization remain later,
separate pressures.
