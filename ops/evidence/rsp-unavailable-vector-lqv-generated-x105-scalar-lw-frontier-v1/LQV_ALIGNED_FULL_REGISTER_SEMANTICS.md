# LQV Aligned Full-Register Semantics

The represented subset is element zero at a sixteen-byte-aligned local DMEM
address. It replaces the entire destination and consumes no old destination
bits.

One successful commit:

- replaces exactly one vector slot;
- advances singular `Sp::pc` to prior `rsp.next_pc`;
- advances `rsp.next_pc` by one local word;
- increments only the RSP committed count;
- records LQV as last instruction;
- preserves run-start, scalars, accumulator/flags, memory, devices, CPU Count,
  CPU committed count, and VI;
- selects CPU for the next public `Machine::step`.
