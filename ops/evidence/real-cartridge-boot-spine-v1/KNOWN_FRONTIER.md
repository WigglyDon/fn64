# Known frontier

First unsupported frontier:

- Execution address: `0xA4000044`
- Instruction identity: `Lw`
- Decoded operands: `rs = 9`, `rt = 8`, immediate `0xF010`
- Source classification: cartridge-bootstrap instruction staged in SP DMEM;
  exact private artifact identity remains external
- Last committed instruction: `SpecialAdd` at `0xA4000040`
- Last committed represented mutation: GPR 9 changed from zero/unknown to
  `0xFFFFFFFFA4001FF0`/known, `pc / next_pc` advanced to
  `0xA4000044 / 0xA4000048`, and Count advanced from 0 to 1
- Decoded base: GPR 9, known from the committed `SpecialAdd`
- Effective address: `0xFFFFFFFFA4001000` (CPU address `0xA4001000`)
- Frontier behavior: `Machine::step` restored control flow and did not advance
  Count

The reset-state operand is no longer the blocker: GPR 29 and the derived GPR 9
are known with inspectable lineage. The unsupported facts are the complete
aligned `Lw` semantic rule plus represented storage/routing for its SP IMEM
target. The probe does not treat that absent target as zero and does not advance
past it.

The smallest coherent future seam is represented SP IMEM storage and narrow
address routing followed by the complete aligned `Lw` semantic rule and
synthetic alias/read-before-write/zero-register/fault tests.

Deliberately not added: generic bus, generalized memory map, device plugin
system, broad PIF HLE, ROM patch, instruction skip, silent MMIO behavior, or
direct cartridge-entry PC staging.
