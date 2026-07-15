# Legal and no-hack audit

- Private PIF or cartridge input searched/read/hashed/copied/executed: no.
- Private or proprietary bytes staged, committed, or packaged: no.
- Authentic instruction stream, assembly, or disassembly copied: no.
- Generated PIF-shaped and cartridge bytes only: yes.
- Instruction fields independently encoded: yes.
- Game identity, filename, digest, expected hardware completion, hidden default,
  instruction skip, imported state, or trace replay selector: none.
- RI_CONFIG and RI_CURRENT_LOAD are not hidden write authorization inputs.
- General RI_SELECT fields, RI_MODE, electrical behavior, timing, RDRAM
  initialization, generic RI bank/MMIO/bus/map, BOOT-3, and compatibility
  claims: absent.
