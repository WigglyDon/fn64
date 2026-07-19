# Rejection and atomicity

Unavailable operand rejection and active-delay control-flow rejection preserve
PC, next_pc, Count, delay context, all GPR values and lineages, HI/LO, COP0,
MI, RI, RDRAM, SP memory and provenance, cartridge bytes, reservations, and
host state.

BEQL itself has no overflow or trap path. Taken-slot exceptions reuse existing
exception entry. A non-exception slot rejection preserves the already
committed branch and active slot context. Not-taken annul has no slot
rejection because no slot executes.
