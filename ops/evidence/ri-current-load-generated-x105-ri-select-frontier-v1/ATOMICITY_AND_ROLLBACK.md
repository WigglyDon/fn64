# Atomicity and rollback

Planning resolves identity, old base, exact address, alignment, target, old
known source, low transfer word, stored RI_CONFIG snapshot, CPU provenance, and
control-flow context before mutation. Application has no remaining fallible
work: it assigns one event and applies existing cadence.

Unknown operands, non-direct or neighboring targets, missing RI_CONFIG, and
target misses preserve all GPR values/sources, HI/LO, COP0, memory and SP-IMEM
provenance, all RI facts, PC/next-PC, Count, and delay context. Existing AdES
owns unaligned state and leaves RI and memory unchanged. Complete Machine
snapshots discriminate the bounded rejection and failed-bootstrap paths.
