# SP IMEM producer

- `INFERENCE` Creation event: IPL1 copies the IPL2 portion of console PIF ROM
  into SP IMEM starting at local offset `0x000` before jumping the CPU there.
- `INFERENCE` Event owner on hardware: CPU-executed IPL1 firmware. The hardware
  SP IMEM is the resulting state owner.
- `INFERENCE` Production time: after initial RCP/device quiescence and before
  CPU execution of IPL2.
- `INFERENCE` Source category: proprietary PIF-ROM IPL2 executable code bytes.
- `INFERENCE` Full produced range: `[0x000, IPL2_length)`, where `IPL2_length`
  is the exact word-aligned span between the source symbols `ipl2_rom` and
  `ipl2_rom_end` for the selected PIF firmware variant.
- `UNKNOWN` Numeric `IPL2_length` is deliberately not reconstructed in fn64 or
  its artifacts. The exact relevant guaranteed subrange is `[0x000, 0x020)`,
  because the x105 prelude consumes all eight words in that prefix.
- `INFERENCE` Persistence lifecycle: boot-stage residue. It is neither reset
  zero nor cartridge data, and it remains until overwritten by the x105 IPL3
  prelude or another explicit memory write.
- `UNKNOWN` Exact bytes and any PIF-revision-specific differences are
  unavailable without an explicitly supplied firmware image.

`WORKER_CLAIM` A Machine-owned creation event could honestly exist only after a
future architecture supplies the firmware bytes or executes their effects. A
constant table in `SpImem`, bootstrap staging, or the probe would falsely move
firmware ownership into fn64.
