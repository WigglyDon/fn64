# Scalar LW Provenance

`MachineRspScalarRegisterSource::Lw(Box<MachineRspScalarLwSource>)` records:

- local instruction PC;
- exact four-byte `SpImem` provenance;
- base GPR index, old base value, and old base source;
- signed 16-bit offset;
- resolved local DMEM address;
- four exact Available `SpDmem` knowledge descriptors.

The scalar register owns the single loaded `u32`. Provenance does not duplicate
the result word or DMEM ownership.
