# Lifecycle and rollback

Cold construction and complete bootstrap establish idle PI, default MI
interrupt truth, no SP semaphore command, reset cache ownership, and no stale
SP-DMEM CPU-store provenance. Generated execution alone creates PI programming,
DMA completion, cache fills, interrupt transitions, SP control, and teardown
provenance.

Repeated complete bootstrap clears those mutable facts while preserving the
immutable current RDRAM profile and cartridge bytes. The same bootstrap
replacement seam clears PI state alongside CPU, caches, MI, RI, SP, and RDRAM
initialization state.

When bootstrap preflight fails, complete snapshots prove no owner changes:
PI, MI interrupts, caches, SP, cartridge, RDRAM, CPU, COP0, reservations, and
provenance remain identical. Two `Machine` instances retain independent PI,
cache, interrupt, SP, and memory state.
