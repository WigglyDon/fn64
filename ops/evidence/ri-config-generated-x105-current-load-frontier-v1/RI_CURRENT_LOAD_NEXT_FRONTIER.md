# RI_CURRENT_LOAD next frontier

At the generated final state, independently encoded `Sw r0,8(r8)` computes CPU
address `0xA4700008`, physical address `0x04700008`, RI_CURRENT_LOAD. It returns
`MachineStoreWordRejectionReason::DirectTargetMiss` with the complete pre-step
state preserved: PC/next-PC remain `0xA40000DC / 0xA40000E0`, Count remains
32,019, total committed steps remain 32,035, and both represented RI facts and
all memory remain unchanged.

No RI_CURRENT_LOAD state, update role, calibration result, RDRAM usability, or
ignored-write shortcut is represented.
