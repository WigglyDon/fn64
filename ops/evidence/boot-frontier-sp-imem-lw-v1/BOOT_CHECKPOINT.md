# Boot checkpoint

- `LIVE_REPO_FACT` BOOT-2 means at least one authentic ROM-derived instruction
  commits its represented effect through `Machine::step`.
- `RUNTIME_FACT` Starting checkpoint: BOOT-2.
- `RUNTIME_FACT` Final checkpoint: BOOT-2.
- `RUNTIME_FACT` The bounded private run attempted 2 instructions and
  committed 1. `SpecialAdd` at `0xA4000040` was the last commit; r9 became
  `0xFFFFFFFFA4001FF0`, `pc` became `0xA4000044`, `next_pc` became
  `0xA4000048`, and Count became 1.
- `RUNTIME_FACT` The next `Lw` was fully decoded, routed, and rejected before
  mutation at unknown SP IMEM offset `0x000`.
- `RUNTIME_FACT` Cartridge entry was not reached, post-handoff game code did
  not run, and no graphics or externally visible game behavior appeared.
- `WORKER_CLAIM` BOOT-3 and compatibility are not earned.

Private-input identity and the complete bounded transcript are retained only
in the external artifact.
