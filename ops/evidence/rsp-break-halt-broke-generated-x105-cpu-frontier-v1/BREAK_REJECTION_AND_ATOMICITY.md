# Break rejection and atomicity

Planning completes before mutation. Focused proofs cover:

- nonzero Break code;
- adjacent/unrepresented SPECIAL identity;
- a BREAK function under a non-SPECIAL major opcode;
- Break in an active RSP delay slot;
- selected-RSP single-step rejection before fetch;
- selected-RSP fetch rejection.

Each rejection preserves the complete `Machine`: no halt, broke, MI pending,
provenance, PC, count, turn, DPC, register, DMA, or memory mutation occurs.
Selected RSP rejection receives no CPU fallback.

