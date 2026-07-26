# RSP Rejection And Atomicity

Immutable planning precedes application. Rejections cover unavailable scalar
base, malformed encoding or knowledge, nonzero element, misalignment, other
vector loads, vector stores, unrepresented consumers, fetch/single-step
boundaries, and scalar Lw.

A rejection preserves processor turn, run-start, current/next PC, RSP count,
all scalar/vector/accumulator state, last instruction, all SP/DMEM/IMEM/MI
truth, complete CPU/COP0 state and counts, VI, devices, memory, cartridge,
reservations, and host state. No selected-RSP rejection falls back to CPU.
