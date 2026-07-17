# AdEL and atomicity

Alignment retains existing precedence. An unaligned address such as
`0xA4300005` enters the ordinary AdEL route with exact BadVAddr, EPC, BD,
exception vector, and no normal Count cadence.

Unknown base lineage, non-direct addresses, neighboring MI targets, and direct
target misses reject before mutation. Ordinary and delay-slot rejection proofs
preserve destination, all GPR lineage, MI, RI, RDRAM, memory, COP0, delay
context, reservations, cartridge, and host state.
