# First-responder RDRAM_DEVICE_ID frontier

At the frontier:

- PC/next_pc: `0xA4000198/0xA400019C`
- Count/committed steps: `32167/32183`
- word: `0xAE2E0004`
- instruction: `Sw r14,4(r17)`
- r17: `0xFFFFFFFFA3F08000`
- r14: zero, with `KnownInstructionResult` from generated
  `Addu r14,r0,r0` at `0xA4000138`
- effective address: `0xFFFFFFFFA3F08004`
- CPU/physical: `0xA3F08004/0x03F08004`
- transfer word: `0x00000000`
- result: `MachineStoreWordRejectionReason::DirectTargetMiss`

The accepted global target is `0x03F80004`; this non-global target is not
matched by masking or ranges. Rejection preserves the complete Machine.
