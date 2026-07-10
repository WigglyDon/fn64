# Known frontier

First unsupported frontier:

- Execution address: `0xA4000044`
- Instruction identity: `Lw`
- Decoded operands: `rs = 9`, `rt = 8`, immediate `0xF010`
- Source classification: cartridge-bootstrap instruction staged in SP DMEM;
  exact private artifact identity remains external
- Last committed instruction: `SpecialAdd` at `0xA4000040`
- Last committed represented mutation: `pc / next_pc` advanced to
  `0xA4000044 / 0xA4000048` and Count advanced from 0 to 1
- Frontier behavior: `Machine::step` restored control flow and did not advance
  Count

`Lw` is not the only missing fact. Register 9 was just derived from register 29,
while PIF/CIC-produced GPR state is deliberately unavailable in the current
`RepresentedResetSubset`. With the decoded negative immediate, an authentic
next address also pressures the currently absent SP IMEM range. Implementing a
word load against zeroed guessed registers would manufacture progress.

The smallest coherent future seam is therefore a source-backed, Machine-owned
PIF/CIC reset-state classification together with represented SP IMEM storage
and routing, followed by the complete aligned `Lw` semantic rule and synthetic
alias/read-before-write/zero-register/fault tests. That future work must not be
an imported emulator register dump, title/hash branch, or proprietary firmware
bundle.

Deliberately not added: generic bus, generalized memory map, device plugin
system, broad PIF HLE, ROM patch, instruction skip, silent MMIO behavior, or
direct cartridge-entry PC staging.
