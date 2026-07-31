# Rejection and atomicity

Full-Machine snapshots prove atomic rejection for:

- unavailable source scalar;
- malformed Mtc0 encoding;
- unsupported control index;
- zero, single-clear, pipe-clear, command-clear, mode-bit, and reserved-bit
  command words;
- malformed 24-bit counter invariant;
- exact RSP Break.

Rejection changes no counter, provenance, scalar/vector/control state, SP
state, DMA record, memory, MI state, PC, committed count, or processor token.
Selected RSP rejection receives no CPU fallback. Break rejection preserves the
already committed DPC clear.
