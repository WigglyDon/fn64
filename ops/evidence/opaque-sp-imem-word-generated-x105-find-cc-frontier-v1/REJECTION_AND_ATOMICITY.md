# Rejection and atomicity

Non-exception rejection preserves CPU control flow, Count, delay context, all
GPR values and lineage, COP0, complete SP memories and knowledge, cartridge,
MI, RI, RDRAM, reservations, and host state. AdES/AdEL use existing exception
entry and do not partially install an opaque state or sentinel.

Malformed four-byte opaque markers are rejected as inconsistent owner state.
