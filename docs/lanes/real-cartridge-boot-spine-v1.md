# Real Cartridge Boot Spine V1

Context role: active lane coordination memory.
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
- Accepted base source: canonical `main` after this lane-registration commit is integrated.
- Registration Context-SHA: `4924a0da1cc1bf36e5044e49127518a7e93ed7f0a08513c8359f9b2777d429ff`; the literal post-registration Context-SHA belongs in the Master provisioning report and first executable packet.
- Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Launch state: not started; Don must not launch Worker Codex before Master GPT issues the matching supervisor seed.

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

Integration requires source-scope review, authentic lineage evidence, the Rust
forward gate, lane-specific proof, current-context revalidation, and Master
acceptance. Stop before modification on topology or Context-SHA mismatch; stop
on ROM/legal leakage, game-specific behavior, authority expansion, direct
overlap, or contradictory accepted law. Retire after accepted integration or an
explicit Master decision. The next milestone is a complete supervisor seed;
no boot milestone or implementation progress is currently claimed.
