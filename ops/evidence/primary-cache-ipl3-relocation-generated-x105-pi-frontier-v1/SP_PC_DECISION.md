# SP_PC decision

At PC 0xA40004CC, the generated store writes raw low field zero to CPU
0xA4080000 / physical 0x04080000 from r0 ArchitecturalZero. The Sp owner
records exact instruction PC, source, lineage, effective address, CPU address,
and physical address.

No status or PC read route, arbitrary SP register bank, semaphore behavior, or
RSP execution is implied.
