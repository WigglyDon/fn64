# RI_MODE next frontier

At the final generated state, r8 is `0xFFFFFFFFA4700000` and independently
encoded `Sw r0,0(r8)` targets CPU `0xA4700000`, physical `0x04700000`, RI_MODE.
RI_MODE has no represented state or write route, so the step returns
`MachineStoreWordRejectionReason::DirectTargetMiss`.

PC/next-PC remain `0xA40000E8 / 0xA40000EC`, Count remains 32,022, and total
committed steps remain 32,038. RI_SELECT stays `0x14` with CPU provenance;
RI_CONFIG, RI_CURRENT_LOAD, CPU, COP0, memory, and all provenance remain
unchanged. The later wait and RI_MODE `0x0E` write were not executed.
