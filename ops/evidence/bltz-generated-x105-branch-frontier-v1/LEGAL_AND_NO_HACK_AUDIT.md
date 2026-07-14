# Legal and no-hack audit

- Private PIF searched/read/hashed/copied/staged/committed/packaged: no.
- Private ROM searched/read/hashed/copied/staged/committed/packaged: no.
- Firmware or commercial cartridge bootcode bytes/words embedded: no.
- Copied assembly, disassembly, instruction stream, or external source: no.
- Game title, cartridge ID, filename, digest, expected PC, expected value, or
  hidden default used as behavior selection: no.
- Link/likely variants, instruction skipping, imported state, trace replay,
  COP0 execution, generic REGIMM execution, broad mode framework, bus, map,
  device system, PIF/IPL execution, SDL, window, or audio: none.
- BOOT-3 or compatibility claim: none.

All durable instruction words are independently encoded from semantic fields,
and all PIF/cartridge bytes used by tests are generated.
