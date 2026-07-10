# BOOT-2 And Rust-Purity Integration Evidence

Evidence class: `LIVE_REPO_FACT`, `RUNTIME_FACT`, and identified
`USER_DECISION` only.

## Integration lineage

- Original and actual canonical base:
  `42321bd07d4e2fa0182bd0aeee8d4bceb10f10f5`
- Starting Context-SHA:
  `b3869517214d4e6869b0af245ddbcc8088ae569db2228e7c2b082b7e2b43f536`
- Boot commits, integrated first:
  `6f189716ad401cbc9996ad57a23cef4a7c3da196` and
  `8e5efc8eab87e11e78f66cdef0542fe43bcd0e3f`
- Cleanup candidate, merged second:
  `9cc1614228397a2aad7d7bb6298fb88e5f0f4bf4`
- Cleanup candidate tree:
  `011435ca6b89b414f2db20f035c21a3485194e54`
- Merge strategy: fast-forward through both boot commits, then a two-parent
  no-fast-forward merge preserving the cleanup candidate.
- Cleanup merge:
  `d7e1da9648c463d9794d0817b73e3db8426c537c`
- Final reconciliation commit, final Context-SHA, canonical push result, and
  exact tested-SHA manifest are owned by the external Master artifact created
  after the final commit exists; this tracked file cannot contain its own
  containing commit hash.

## Candidate artifacts

- Boot artifact source:
  `/tmp/UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1.tar.gz`
- Durable boot artifact:
  `/tmp/fn64-final-artifacts/UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1_d4ceb596.tar.gz`
- Boot artifact SHA-256:
  `d4ceb59640722afbb1a86c5e4c1329487f6ffa6a6ee689ddc4a555104e9e8511`
- Cleanup artifact source:
  `/tmp/UPLOAD_ME_fn64_rust_purity_repo_cleanup_v1.tar.gz`
- Durable cleanup artifact:
  `/tmp/fn64-final-artifacts/UPLOAD_ME_fn64_rust_purity_repo_cleanup_v1_repaired_def244e.tar.gz`
- Cleanup artifact SHA-256:
  `def244e3639e64279f5e21f65d92768859d648e599f1deb0655df357de0c7b54`

## Runtime evidence

Authorized private input identity:

- SHA-256:
  `c916ab315fbe82a22169bff13d6b866e9fddc907461eb6b0a227b82acdf5b506`
- Size: `33554432` bytes
- Content committed or packaged: no

Available:

- bounded no-window BOOT-2 trace;
- exact input digest and size;
- Machine-owned cartridge/SP-DMEM/GPR source lineage;
- one committed `SpecialAdd` result and deterministic first frontier;
- focused bootstrap, rollback, inspection, machine-step, forward-gate, and
  clean-checkout proof.

Unavailable:

- authentic bootstrap handoff and cartridge-entry execution;
- game-program execution after handoff;
- BOOT-3;
- SP IMEM storage/routing and complete aligned `Lw`;
- graphics, audio, window, game compatibility, and performance
  characterization.

## Authority and operational state

- Product authority: Rust only.
- Product behavior change: the accepted boot candidate only.
- Cleanup behavior change: none; non-product consolidation only.
- Master reconciliation: capability/context/coordination only.
- Lane closure: both worker lanes completed; worktrees/branches preserved.
- Rollback state: unknown-source and unrepresented-`Lw` paths are proved
  pre-mutation for their represented cases.
- Observability state: public read-only Machine inspection plus no-window text.
- Performance/resource state: `UNKNOWN`; not measured.
- Deploy state: not performed.
- Known unknowns: BOOT-3, real handoff, broad hardware behavior, compatibility,
  and performance remain unavailable.
