# Atomicity and rollback

Planning captures old base/source values and lineage, alignment, exact target,
word, and pending transfer before mutation. Missing transfer, wrong word,
unknown source, pending-transfer misuse, and target misses preserve PC,
next_pc, Count, delay context, GPRs/lineage, COP0, reservations, all memories,
RI, MI, prior delay state, and host state. Existing AdES precedence applies and
does not consume the transfer. No fallible work remains after mutation begins.

