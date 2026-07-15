# Atomicity and rollback

Planning resolves identity, old base, exact address, alignment, target, known
old source, low transfer word, supported-value policy, CPU provenance, and
control-flow context before mutation. Application has no remaining fallible
work: it assigns one RI_SELECT state and applies existing cadence.

Unknown operands, non-direct or neighboring targets, unsupported words, and
target misses preserve all GPR values/sources, HI/LO, COP0, RDRAM, SP DMEM,
SP IMEM/provenance, all RI facts, PC/next-PC, Count, and delay context. Existing
AdES owns unaligned state and leaves RI/memory unchanged. Failed bootstrap
planning likewise exposes no partial replacement.
