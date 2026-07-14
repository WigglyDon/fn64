# RI_CONFIG next frontier

After 33 generated commits, r8 contains `0xFFFFFFFFA4700000` and r9 contains
`0x40`. The next independently encoded identity is `Sw r9,4(r8)` at
`PC=0xA40000C4`:

- effective address: `0xFFFFFFFFA4700004`;
- represented CPU address: `0xA4700004`;
- physical address: `0x04700004`;
- RI offset: `0x04`;
- register: RI_CONFIG;
- current result: `MachineStoreWordRejectionReason::DirectTargetMiss`;
- mutation and Count delta: none.

RI_CONFIG write semantics, other RI state, RDRAM initialization, and a general
MMIO route are not earned by this pass.
