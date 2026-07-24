# Public synthetic regression coverage

All committed tests remain independent of the user ROM.

The complete 594-test core suite preserves:

- public fixed-profile RDRAM bring-up;
- BEQL taken and not-taken annul semantics;
- primary cache initialization and relocation;
- PI DMA and final x105 handoff;
- public runtime-v1 and runtime-v2 cartridge fixtures;
- functional D-cache stores and dirty writeback;
- exact synthetic checksum and stable success loop;
- delay-slot and exception ownership;
- opaque SP-IMEM truth;
- bootstrap rollback and independent Machines.

New focused proofs cover TLB operations, ERET, ordinary and delay-owned
interrupt entry, COP1 FCR31 transfers, integer divide/multiply edges,
halfword/doubleword cached and uncached access, cache writeback/invalidation,
variable PI DMA, AI/VI/SI state, SP DMA lifecycle, atomic rejection, and
independent owners.

The local user-ROM probe is a separate optional validation and is not a CI
dependency.
