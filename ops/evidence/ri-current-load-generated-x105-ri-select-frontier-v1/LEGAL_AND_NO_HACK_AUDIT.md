# Legal and no-hack audit

- Private PIF or cartridge input searched/read/hashed/copied/executed: no.
- Private or proprietary bytes staged, committed, or packaged: no.
- Authentic instruction stream, assembly, or disassembly copied: no.
- Generated PIF-shaped and cartridge bytes only: yes.
- Instruction fields independently encoded: yes.
- Game identity, filename, digest, expected completion, hidden countdown,
  instruction skip, imported state, or trace replay selector: none.
- RI_CURRENT_LOAD consumes stored RI_CONFIG state; it is not a trace bypass.
- RI_SELECT write, RI_MODE, calibration, timing, RDRAM readiness, generic RI
  bank, MMIO, bus, map, BOOT-3, and compatibility claims: absent.
