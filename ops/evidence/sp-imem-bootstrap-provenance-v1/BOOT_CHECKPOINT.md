# Boot checkpoint

- `RUNTIME_FACT` Starting checkpoint: BOOT-2.
- `RUNTIME_FACT` Current integrated behavior attempts two steps and commits one
  authentic cartridge-derived `SpecialAdd` before the represented `Lw` rejects
  at unknown SP IMEM offset zero.
- `WORKER_CLAIM` Machine behavior change in this lane: none.
- `WORKER_CLAIM` Final checkpoint remains BOOT-2 because publishing the loaded
  word would require proprietary IPL2 content.
- `UNKNOWN` BOOT-3, cartridge-entry handoff, post-handoff game execution, and
  game compatibility remain unearned.

Exact private-input identity and the complete bounded probe output are external
only.
