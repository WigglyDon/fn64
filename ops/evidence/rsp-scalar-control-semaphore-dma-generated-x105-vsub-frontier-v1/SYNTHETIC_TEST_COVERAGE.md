# Synthetic Test Coverage

Focused fn64-core proofs cover:

- exact Lui/Addi decode, arithmetic, aliasing, r0, provenance, and rejection;
- exact Bltz/Bne conditions, targets, taken/not-taken slots, and source
  knownness;
- branch context ownership, CPU interleave, slot success, and slot/fetch
  rejection;
- existing semaphore read-and-set and guest CPU clear;
- second-DMA full-range success and preflight atomic rejection;
- Mfc0 SP_DMA_BUSY idle zero and SP_DMA_FULL rejection;
- exact Vsub identity and atomic rejection;
- complete public generated x105 composition through the frontier.

Inspection adds a no-window public-step composition with guest CPU semaphore
clear, second DMA, busy read, branch delay, and Vsub rejection.

All instruction words and byte patterns are independently encoded public
synthetic data. No user task or private input is used.
