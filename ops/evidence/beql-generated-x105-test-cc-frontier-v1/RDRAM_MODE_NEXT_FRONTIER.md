# RDRAM_MODE next frontier

Exact rejected frontier:

- pre-state: `pc=0xA4000BB8`, `next_pc=0xA4000BC4`;
- Count `32243`, committed steps `32259`;
- active delay owner: `0xA4000BB4`;
- word `0xAEAF0000`, `Sw r15,0(r21)`;
- base `r21=0xFFFFFFFFA3F0000C`, known `Addiu` result from `0xA400019C`;
- source `r15=0x0000000046C0C0C0`, known `Or` result from `0xA4000BAC`;
- effective `0xFFFFFFFFA3F0000C`;
- CPU `0xA3F0000C`, physical `0x03F0000C`;
- low transfer word `0x46C0C0C0`;
- rejection `MachineStoreWordRejectionReason::DirectTargetMiss`.

The rejected step preserves the entire post-BNE pre-slot Machine, including
the active delay owner. No RDRAM_MODE request or physical effect exists.
