# MI_INIT_MODE field decision

Decision: `MI_INIT_MODE_EXACT_X105_WRITE_ONLY`.

The accepted input is exactly low word `0x0000010F`. It directly requests
initialization length 15 and set-initialization-mode. The persistent Machine
truth is therefore:

- initialization length: 15;
- initialization mode: true;
- source: the exact CPU store and its source-GPR lineage.

The command bit at position 8 is not persisted as though it were the readback
initialization-mode bit. EBUS test mode, RDRAM-register mode, DP interrupt
state, and all other MI facts remain unavailable because this word does not
directly establish new values for them.

Every other low word is an fn64 unsupported boundary and rejects before
mutation. This is not a hardware-trap claim and does not generalize
MI_INIT_MODE programming.
