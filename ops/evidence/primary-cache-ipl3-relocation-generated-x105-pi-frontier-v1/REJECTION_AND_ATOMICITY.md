# Rejection and atomicity

Focused proofs cover unavailable CACHE bases, unavailable TagLo or TagHi,
unsupported CACHE operation fields, unsupported tag state, non-KSEG0 cache
addresses, unsupported SP status commands, unsupported SP_PC values, opaque or
unavailable fetch backing, and the unimplemented PI target.

Each rejection preserves PC, next-PC, Count, delay context, GPR values and
lineage, HI/LO, COP0, both cache arrays, SP control and memories, RDRAM and its
module state, MI, RI, cartridge bytes, reservations, and host facts. Successful
MTC0, CACHE, SP stores, relocation loads/stores, branches, jumps, and fills use
the established one-instruction cadence and delay owner.
