# SP IMEM and aligned Lw frontier v1

- `USER_DECISION` Lane scope is Machine-owned SP IMEM, narrow direct CPU data
  routing, complete aligned `Lw`, and trace-driven continuation without a
  generic bus, proprietary firmware, or game-specific behavior.
- `LIVE_REPO_FACT` `Machine::step` remains the public represented execution
  entrance. `fn64-core` owns storage, routing, planning, application,
  exception entry, and cadence; `fn64-inspection` only presents outcomes.
- `RUNTIME_FACT` Synthetic proof covers known SP IMEM and direct-RDRAM `Lw`
  commits, sign extension, aliasing, zero-register behavior, lineage,
  alignment exceptions, and complete rollback on every represented rejection.
- `RUNTIME_FACT` The bounded private trace still earns BOOT-2: one authentic
  `SpecialAdd` commits, then `Lw` at `0xA4000044` rejects before mutation
  because SP IMEM offset `0x000` is unknown.
- `UNKNOWN` No source-clear represented fact in this lane establishes the four
  post-PIF SP IMEM bytes consumed at offset `0x000`.
- `WORKER_CLAIM` Result is PARTIAL. The product semantics are sound and
  independently proved, but authentic trace continuation would require
  inventing or importing unavailable bootstrap state.

The external artifact owns the private-input identity, bounded runtime
transcript, full command logs, patch series, and final Git facts. No ROM bytes,
ROM-like file, PIF/BIOS blob, raw bootcode dump, or compatibility claim are in
this directory.
