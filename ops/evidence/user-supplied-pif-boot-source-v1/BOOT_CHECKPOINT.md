# Boot checkpoint

- `LIVE_REPO_FACT` Starting accepted checkpoint: BOOT-2.
- `LIVE_REPO_FACT` This lane changes input ownership and validation only; it does
  not claim PIF-backed trace advancement.
- `LIVE_REPO_FACT` Accepted firmware input is inert and cannot make SP IMEM
  known.
- `RUNTIME_FACT` Generated accepted-input probes still stop at the represented
  unknown-SP-IMEM `Lw` frontier after one generated cartridge-derived commit.
- `RUNTIME_FACT` The authorized private-ROM no-firmware regression attempted 2
  steps, committed 1, and stopped at BOOT-2 with `pc=0xA4000044`,
  `next_pc=0xA4000048`, and Count 1.
- `UNKNOWN` Authentic firmware-backed checkpoint because no private PIF path
  is authorized.
- `WORKER_CLAIM` Cartridge entry, post-handoff game code, graphics, and
  compatibility remain unearned.

The external artifact owns the final authorized private-ROM no-firmware
regression output and exact candidate SHA.
