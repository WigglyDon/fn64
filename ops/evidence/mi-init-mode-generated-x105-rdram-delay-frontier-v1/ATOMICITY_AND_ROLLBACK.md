# Atomicity and rollback

Planning completes address translation, alignment, target selection, source
knownness, low-word extraction, and exact-value validation before mutation.
There is no fallible operation after MI application begins.

Unsupported words, unknown sources, neighboring targets, and ordinary misses
preserve PC, next_pc, Count, delay context, GPR values and lineage, COP0,
memory, RI, prior MI, reservations, and host state.

Unaligned ordinary and delay-slot stores use the existing AdES entry with
exact BadVAddr/EPC/BD and exception-vector transition, no Count advance, and
no MI mutation.
