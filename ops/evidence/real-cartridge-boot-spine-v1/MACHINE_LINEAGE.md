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
-> represented IPL3 execution-entry fact
-> Machine-owned CPU control flow
-> `pc = 0xA4000040`, `next_pc = 0xA4000044`
-> explicit `RepresentedResetSubset` with PIF-produced state unavailable
-> bootstrap state and reset-clearing tests.

## First committed cartridge instruction

Public `Machine::step`
-> SP DMEM fetch at `0xA4000040`
-> cartridge-bootstrap provenance at normalized offset `0x40`
-> decode and `SpecialAdd` identity
-> existing CPU-local executed-helper selection
-> represented destination write semantics plus cadence commit
-> `pc / next_pc` become `0xA4000044 / 0xA4000048` and Count becomes 1
-> BOOT-2 report, synthetic ROM-derived commit test, and private bounded trace.

## First frontier

Next public `Machine::step`
-> SP DMEM fetch at `0xA4000044`
-> cartridge-bootstrap provenance at normalized offset `0x44`
-> `Lw` identity with `rs = 9`, `rt = 8`, immediate `0xF010`
-> no represented Machine step category
-> control-flow rollback with no Count advance
-> explicit unsupported frontier and successful represented-stop probe policy.
