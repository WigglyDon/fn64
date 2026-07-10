# Machine lineage

## Cartridge ingress and normalization

Host-selected owned file bytes
-> content-classified N64 source byte order
-> `fn64_core::load_cartridge`
-> normalized Machine-core cartridge representation
-> `Machine::from_cartridge`
-> Machine-owned read-only cartridge state
-> synthetic byte-order tests and external private-input metadata.

## Bootstrap payload

Named `Machine::stage_cartridge_bootstrap` request
-> normalized cartridge-bootstrap source classification
-> cartridge range `0x40..0x1000`
-> Machine-owned SP DMEM range `0x40..0x1000`
-> atomic replacement bootstrap state
-> `MachineCartridgeBootstrapState`
-> exact-boundary, all-layout, reset, and rollback tests.

## Bootstrap CPU entry

Completed payload materialization
-> public documented general PIF reset fact
-> Machine-owned bootstrap CPU state
-> architectural GPR zero known as zero and GPR 29 known as `0xFFFFFFFFA4001FF0`
-> all other unstaged PIF-produced GPRs explicitly unknown
-> represented IPL3 execution-entry fact
-> Machine-owned CPU control flow
-> `pc = 0xA4000040`, `next_pc = 0xA4000044`
-> explicit `RepresentedResetSubset`
-> reset-lineage, unknown-state, rollback, and reset-clearing tests.

## First committed cartridge instruction

Public `Machine::step`
-> SP DMEM fetch at `0xA4000040`
-> cartridge-bootstrap provenance at normalized offset `0x40`
-> decode and `SpecialAdd` identity
-> known source GPR 29 plus known architectural GPR zero
-> existing CPU-local executed-helper selection
-> destination GPR 9 changes from zero/unknown to
   `0xFFFFFFFFA4001FF0`/known
-> `pc / next_pc` become `0xA4000044 / 0xA4000048` and Count becomes 1
-> `cpu-local-committed`
-> complete known-SpecialAdd synthetic test and private bounded trace.

## Unknown bootstrap operand rejection

Generated bootstrap instruction
-> cartridge-bootstrap provenance
-> decoded CPU-local operation consuming an unstaged PIF-produced GPR
-> `UnknownPifProduced` source classification
-> rejection before CPU helper invocation
-> no GPR, HI, LO, COP0, RDRAM, SP DMEM, `pc`, or `next_pc` mutation
-> dedicated no-partial-mutation synthetic proof.

## First frontier

Next public `Machine::step`
-> SP DMEM fetch at `0xA4000044`
-> cartridge-bootstrap provenance at normalized offset `0x44`
-> `Lw` identity with `rs = 9`, `rt = 8`, immediate `0xF010`
-> known base `0xFFFFFFFFA4001FF0`
-> computed effective address `0xFFFFFFFFA4001000` (CPU address `0xA4001000`)
-> no represented Machine step category
-> control-flow rollback with no Count advance
-> explicit unsupported frontier and successful represented-stop probe policy.
