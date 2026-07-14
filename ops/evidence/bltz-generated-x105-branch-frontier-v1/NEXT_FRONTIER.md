# Next frontier

The next explicit generated frontier is `Cop0Mtc0` at CPU address
`0xA400007C`, with source r0 and destination COP0 register 13 (Cause).
Decode and identity are present; execution and its exact COP0 mutation contract
are absent. `Machine::step` returns `UnrepresentedInstruction` and preserves
PC, next-PC, Count, GPRs, COP0, memories, and provenance.

This pass adds no COP0 instruction execution. Any future `MTC0` work must earn
its own bounded register, privilege, exception, cadence, and source-knownness
contract rather than generalizing from this stop.
