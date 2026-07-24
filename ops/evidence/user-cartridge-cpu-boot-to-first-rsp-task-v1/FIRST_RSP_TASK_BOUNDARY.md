# First RSP task boundary

Start instruction PC: `0x800D5A98`

Instruction word: `0xADC40010`

Identity: `Sw`

Source GPR: `r4`

Source lineage: `KnownInstructionResult` from `Addiu` at `0x800CF978`

SP_STATUS command: `0x00000125`

SP halt: `true -> false`

SP PC: `0x000`

Final CPU PC / next PC: `0x800CF97C / 0x800CF980`

Final Count: `21,382,107`

Total attempted Machine steps: `21,382,817`

Total committed instructions: `21,382,123`

User-cartridge committed instructions: `13,988,271`

The final CPU PC differs from the store PC because the start store committed in
the already represented guest control-flow cadence. Its existing delay owner
selected `0x800CF97C` after the store.

No RSP instruction executed. The next subsystem is general RSP task execution
over the already represented SP memory and control truth.
