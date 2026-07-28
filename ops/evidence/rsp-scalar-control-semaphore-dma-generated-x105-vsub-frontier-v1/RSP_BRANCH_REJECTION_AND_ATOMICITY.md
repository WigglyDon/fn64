# RSP Branch Rejection And Atomicity

Focused snapshots prove atomic rejection for:

- unavailable `Bltz` source;
- unavailable `Bne` source A or B;
- unsupported REGIMM selector;
- control flow in an active RSP delay slot;
- selected-RSP fetch failure with an active delay context;
- represented delay-slot rejection.

Pre-commit branch rejection creates no delay context or partial PC change.
Post-branch slot rejection preserves the committed branch and complete
post-branch state. Selected-RSP rejection receives no CPU fallback.

Scalar/vector state, accumulator/flags, SP registers, semaphore, DMA records,
DMEM/IMEM/RDRAM, CPU/COP0, Count, VI, devices, cartridge, reservations, and
host state are unchanged by the rejected operation.
