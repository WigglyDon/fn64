# Real Cartridge Boot Spine V1

Context role: completed lane coordination memory.
Scope: authentic private-ROM boot-spine work in the Rust machine and inspection crates.
Canonical for: the real-cartridge boot lane's purpose, topology, authority boundary, overlap contract, and integration conditions.
Not canonical for: accepted product behavior, runtime milestones, private ROM content, or candidate acceptance.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [memory and cartridge](../context/subsystems/memory-and-cartridge.md), [machine core](../context/subsystems/machine-core.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, candidate creation, context propagation, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `real-cartridge-boot-spine-v1`
- Purpose: earn the first honest vertical path from one private user-provided N64 ROM through machine-owned cartridge, bootstrap, CPU, memory, and step semantics.
- Supervisor role: Real Cartridge Boot Supervisor GPT
- Worker Codex worktree: `/home/don/fn64-worktrees/real-cartridge-boot-spine-v1`
- Branch: `worker/real-cartridge-boot-spine-v1`
- Accepted base source: `42321bd07d4e2fa0182bd0aeee8d4bceb10f10f5`
- Provisioning Context-SHA: `b3869517214d4e6869b0af245ddbcc8088ae569db2228e7c2b082b7e2b43f536`
- Status: **COMPLETED — INTEGRATED AND CLOSED**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Launch state: completed locally; no Worker branch push occurred.

## Authority and ownership

Allowed implementation scope:

- `rust/crates/fn64-core/src/**`
- `rust/crates/fn64-inspection/src/**`
- narrow tests owned by those crates
- `rust/Cargo.toml` and `rust/Cargo.lock` only for a source-proven accepted product need
- `ops/evidence/real-cartridge-boot-spine-v1/**`
- external logs and artifacts

Prohibited authority:

- no edits to root/current documentation, `docs/**`, `tools/fleet/**`, `ops/fleet/**`, `rust/README.md`, `rust/PARITY.md`, `rust/AGENTS.md`, or `rust/verify-forward`
- no edits to lane registry, integration queue, another lane's evidence, worker topology, canonical `main`, or user ROM files
- no per-game patch, title/name branch, direct-entry bypass, arbitrary state pre-seeding, host-written success marker, ROM-byte patch, copied PC sequence, or synthetic substitute for the selected ROM
- no checked-in ROM, proprietary PIF/BIOS blob, SDL/window/audio presentation, plugin system, compatibility database, or unearned bus/memory-map framework

Master Codex alone owns branch/worktree topology, context propagation,
candidate integration, and canonical push. Worker authority is
`WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION`.

## Proof, dependencies, and overlap

Required proof preserves the lineage `ROM or represented input bytes -> address
and owning subsystem -> classified operation -> machine mutation -> observable
outcome`. The lane reports only the highest source-clear `BOOT-0` through
`BOOT-5` checkpoint actually earned; header parsing, loading bytes, staging
IPL3, or reaching one address alone is not boot. One private ROM may be read
only after a valid supervisor/Worker packet grants the runtime-input authority;
no ROM content enters Git or evidence artifacts.

Direct path overlap with `rust-purity-repo-cleanup-v1`: none under the reserved
scope contract. Indirect risk: the cleanup lane may change Context-SHA. If it
integrates first, Master sends this lane a `CONTEXT_DELTA_AMENDMENT` with
`Restart-Goal: false` unless a semantic contradiction exists.

Integration required source-scope review, authentic lineage evidence, the Rust
forward gate, lane-specific proof, current-context revalidation, and Master
acceptance. Those conditions passed; the lane is closed and its worker
worktree/branch remain preserved without push.

## Completion record

- First worker commit: `6f189716ad401cbc9996ad57a23cef4a7c3da196`
- Accepted repaired candidate: `8e5efc8eab87e11e78f66cdef0542fe43bcd0e3f`
- Product-truth integration SHA: `8e5efc8eab87e11e78f66cdef0542fe43bcd0e3f`
- Combined cleanup merge: `d7e1da9648c463d9794d0817b73e3db8426c537c`
- Final canonical reconciliation SHA: recorded in the external Master
  integration artifact because a context document cannot embed its containing
  commit hash.
- Accepted checkpoint: **BOOT-2 — ROM-DERIVED INSTRUCTION COMMITTED**
- Rejected checkpoints: BOOT-3 and above
- Verified worker artifact source:
  `/tmp/UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1.tar.gz`
- Durable verified artifact:
  `/tmp/fn64-final-artifacts/UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1_d4ceb596.tar.gz`
- Artifact SHA-256:
  `d4ceb59640722afbb1a86c5e4c1329487f6ffa6a6ee689ddc4a555104e9e8511`
- Authorized private input SHA-256:
  `c916ab315fbe82a22169bff13d6b866e9fddc907461eb6b0a227b82acdf5b506`
- Authorized private input size: `33554432` bytes; no content was committed or
  packaged.
- No game-specific patch, title/cartridge selection, direct-entry bypass, ROM
  mutation, imported trace, or host-authored success marker was found.
- First frontier: `Lw` at `0xA4000044`, known r9 base, CPU address
  `0xA4001000`; SP IMEM storage/routing and complete aligned-load semantics are
  absent.
