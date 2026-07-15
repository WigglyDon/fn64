# Atomicity and rollback

Unsupported values, unknown lineage, pending-transfer conflicts, nearby
targets, and direct misses preserve complete pre-step state. Ordinary and
delay-slot unaligned writes enter AdES without REF_ROW mutation or normal Count
cadence. Success performs no fallible operation after mutation begins and
preserves RDRAM_DELAY, MI/RI facts, bytes, SP memories, reservations, and host
state.
