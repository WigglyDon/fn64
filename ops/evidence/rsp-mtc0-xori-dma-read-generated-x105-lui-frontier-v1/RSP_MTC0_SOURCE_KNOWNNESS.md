# RSP MTC0 Source Knownness

Planning captures old `rt` before mutation and requires it Available; r0 is
valid architectural zero. Unavailable input rejects atomically. Mtc0 writes no
scalar register. Each immutable source record stores PC, IMEM provenance,
source index/value/lineage, and control destination; Sp states refer to it by a
stable source index without duplicating recursive scalar truth.
