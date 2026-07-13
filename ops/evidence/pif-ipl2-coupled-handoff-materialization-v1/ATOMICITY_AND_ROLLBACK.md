# Atomicity

All fallible source and selector checks precede mutation. Replacement CPU and
memories are built privately, then assigned as one Machine transition.

Rejected cases preserve the complete prior state: missing PIF bytes, missing
profile, partial handoff inputs, unsupported PAL/MPAL coupled handoff,
unsupported family, malformed or unsupported firmware replacement, and short
cartridge input. Preservation includes cartridge and PIF inputs, profile and
handoff inputs, GPR values and sources, COP0, PC pair, delay context, Count,
SP DMEM, SP IMEM bytes and provenance, RDRAM, and prior bootstrap state.

No rollback-after-visible-mutation path is used.
