# Legal and no-hack audit

- Private cartridge ROM searched/read/hashed/copied/staged/committed: no.
- Private PIF searched/read/hashed/copied/staged/committed: no.
- Firmware or cartridge bootcode bytes embedded: no.
- Copied assembly or disassembly: no.
- Copied external source tree: no.
- Game title, product code, filename, digest, expected PC, or expected value
  used as a behavior selector: no.
- Instruction skipping, direct game-entry staging, imported emulator state, or
  replayed trace: no.
- Generic bus, generalized memory map, device registry, PIF execution, RSP
  execution, DMA, SDL, window, or audio: no.

Tests independently encode only the small generated identities needed to prove
architectural semantics. External sources contribute paraphrased identity
order and architectural rules only.

The result makes no claim of authentic PIF, IPL1, IPL2, cartridge, or IPL3
execution; BOOT-3 and game compatibility remain unearned.
