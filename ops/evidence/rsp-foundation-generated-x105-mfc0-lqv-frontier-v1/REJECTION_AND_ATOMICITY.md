# Rejection And Atomicity

Explicit selected-RSP rejection covers:

- single-step execution;
- missing, unaligned, out-of-range, unknown, opaque, or inconsistent IMEM
  fetch truth;
- malformed MFC0;
- unsupported MFC0 control source;
- MTC0;
- another scalar/COP0/vector identity;
- identified but unrepresented LQV.

Planning completes before mutation. Rejection preserves processor turn,
run-start, higher-level task fact, current/next PC, delay context, RSP count,
all scalar/unit state, last instruction, semaphore, SP DRAM address, SP
control/DMA/memory, MI, CPU/COP0/Count, VI, devices, RDRAM, cartridge,
reservations, and host state. No selected-processor rejection falls back to
the other processor.
