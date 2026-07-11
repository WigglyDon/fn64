# SP IMEM And Aligned-Lw Partial Integration Evidence

Evidence classes: `USER_DECISION`, `LIVE_REPO_FACT`, and `RUNTIME_FACT` only.

## Product lineage

- Canonical and candidate parent:
  `5f77d2df6005fe34ebb20f4751c2980ff73c57f1`.
- Accepted worker candidate:
  `dcb9f1bfac971a5a637f4c168aa57c9d0228ea0c`.
- Classification: `PARTIAL PRODUCT INCREMENT — INTEGRATED`.
- Merge strategy: fast-forward the complete one-commit candidate unchanged,
  then create a separate Master capability/lane reconciliation commit.
- The exact reconciliation SHA, final Context-SHA, canonical push state, and
  tested-SHA manifest belong to the external Master artifact created after the
  final commit exists; this tracked file cannot name its own containing commit.

## Candidate artifact

- Source: `/tmp/UPLOAD_ME_fn64_boot_frontier_sp_imem_lw_v1.tar.gz`.
- Durable verified copy:
  `/tmp/fn64-final-artifacts/UPLOAD_ME_fn64_boot_frontier_sp_imem_lw_v1_fca9c7e0.tar.gz`.
- SHA-256:
  `fca9c7e0608617490da38b8054a56716de16372e00929cb584b85fe5de88debb`.
- Shape: 41 regular files; 40 manifest-owned payloads plus
  `MANIFEST.sha256`; safe relative regular/directory entries only.
- The binary-safe patch applies to the stated parent and reproduces candidate
  tree `7a2a3a1e90ddee96affde5f09ecfc0645ca62c5b` exactly.

## Accepted machine truth

- Every Machine privately owns 4 KiB of SP IMEM.
- Construction, reset, and cartridge-bootstrap restaging produce concrete zero
  backing with per-byte `Unknown` provenance.
- Four known bytes are required for one N64 big-endian word.
- Direct RDRAM and the narrow SP IMEM physical range share one complete aligned
  `Lw` plan/application rule.
- Sign extension, zero-register behavior, base/destination aliasing,
  `KnownInstructionResult` lineage, data AdEL/BadVAddr, success cadence, and
  rejection/blocked-entry rollback are represented.
- Test-only known-word staging is unavailable to production and inspection.

## Runtime boundary

- Authorized private input SHA-256:
  `c916ab315fbe82a22169bff13d6b866e9fddc907461eb6b0a227b82acdf5b506`.
- Size: `33554432` bytes.
- Input content committed or packaged: no.
- Result: BOOT-2; two attempted instructions, one committed `SpecialAdd`, PC
  `0xA4000044`, next PC `0xA4000048`, Count 1.
- First frontier: aligned `Lw` at `0xA4000044` routes CPU address
  `0xA4001000` to SP IMEM offset zero and rejects before mutation because the
  first byte is `Unknown`.

Unavailable and unclaimed: the source creation event for SP IMEM bytes
`0x000..0x003`, authentic SP-IMEM-backed `Lw` commit, BOOT-3, bootstrap
handoff, cartridge-entry/game execution, PIF emulation, SP DMA/registers/status,
RSP execution, graphics, host runtime, compatibility, performance, generic bus,
and generalized memory map.
