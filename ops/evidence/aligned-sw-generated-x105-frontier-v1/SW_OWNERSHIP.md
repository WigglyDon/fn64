# Sw ownership

- Decode and identity remain owned by `cpu/instruction.rs`; opcode `0x2B`
  already identifies `Sw`.
- `Machine::step` remains the sole public execution entrance.
- Machine owns base/source source checks, effective-address planning, target
  classification, immutable action selection, application, and cadence.
- The CPU address layer owns write alignment and AdES selection.
- COP0 owns exception entry, BadVAddr, EPC, BD, EXL, and vectoring.
- SP IMEM owns its four-byte mutation and per-byte provenance.
- The host owns no store policy and receives no mutable CPU or memory surface.

No RDRAM, SP-DMEM, device, MMIO, bus, or generalized-map store owner is added.
