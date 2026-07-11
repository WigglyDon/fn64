# Boot-chain causality

## IPL1

- `INFERENCE` Owner: CPU-executed code resident in the console PIF ROM.
- `INFERENCE` Execution location: PIF ROM at the reset vector.
- `INFERENCE` Source action: the pinned N64-IPL `pifrom.s` copy loop reads the
  source span delimited by `ipl2_rom` and `ipl2_rom_end` and writes it, one word
  at a time, starting at SP IMEM offset zero.
- `INFERENCE` Transfer of control: the same source sets the stack near the end
  of SP IMEM and jumps the CPU to SP IMEM start.
- `UNKNOWN` The complete copied byte count is a firmware-build property and is
  not derived or reproduced in this lane. The source proves that the full
  x105-consumed prefix `[0x000, 0x020)` lies inside the copied span.

## IPL2

- `INFERENCE` Owner: CPU-executed IPL2 firmware code now resident in SP IMEM.
- `INFERENCE` Execution location: CPU KSEG1 alias of SP IMEM start.
- `INFERENCE` Input sources: PIF RAM state and cartridge header/IPL3 bytes.
- `INFERENCE` Output action: IPL2 copies cartridge offsets `[0x040, 0x1000)`
  into SP DMEM at the same offsets, verifies the IPL3 checksum through the PIF
  protocol, and transfers CPU control to SP DMEM at `0xA4000040`.
- `INFERENCE` Persistence: no intervening source action clears SP IMEM before
  the transfer. IPL3 x105 then reads and writes the retained prefix directly.

## IPL3

- `RUNTIME_FACT` Current fn64 evidence reaches cartridge-derived IPL3 code in
  SP DMEM at `0xA4000040`; its first commit derives r9 from the represented
  stack pointer, and the next instruction targets SP IMEM offset zero.
- `INFERENCE` The pinned N64-IPL x105 entry prelude matches that instruction
  shape. It scans eight contiguous retained IPL2 words, combines each with
  cartridge-resident IPL3 data, writes each result back, then writes three
  following SP IMEM words before entering the common IPL3 body.
- `INFERENCE` The consumed values are firmware instruction bytes used as data.
  Their existence is hardware-visible state; their exact values are not a
  general hardware effect independent of the proprietary firmware image.

## Source tension and limitations

- `WORKER_CLAIM` n64docs describes an emulator-oriented whole-`0x1000`
  cartridge-to-DMEM copy. Pinned N64-IPL source instead shows the actual IPL2
  source and destination beginning at offset `0x040` for a `0x0fc0`-byte copy.
  Current fn64 follows the latter narrow span. The summary is useful
  corroboration, not the stronger copy-boundary owner.
- `WORKER_CLAIM` Mupen64Plus HLE materializes only the eight-word x105-required
  IMEM prefix. That corroborates the consumed range but is not specification
  evidence and cannot authorize importing its constants.
- `UNKNOWN` PIF hardware revision effects beyond the pinned NTSC/PAL/MPAL
  source variants are not established.
