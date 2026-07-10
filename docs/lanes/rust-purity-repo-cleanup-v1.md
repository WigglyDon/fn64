# Rust Purity Repository Cleanup V1

Context role: active lane coordination memory.
Scope: non-product cleanup of obsolete C++-to-Rust transition scaffolding and current repository language.
Canonical for: the Rust-purity cleanup lane's purpose, topology, authority boundary, overlap contract, and integration conditions.
Not canonical for: accepted product behavior, product source, history itself, or candidate acceptance.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [build and tooling](../context/subsystems/build-and-tooling.md), [project history](../context/PROJECT_HISTORY.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, candidate creation, context propagation, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `rust-purity-repo-cleanup-v1`
- Purpose: remove obsolete transition scaffolding and stale current language so the repository presents one coherent Rust product while durable C++ history remains historical.
- Supervisor role: Rust Purity Repository Cleanup Supervisor GPT
- Worker Codex worktree: `/home/don/fn64-worktrees/rust-purity-repo-cleanup-v1`
- Branch: `worker/rust-purity-repo-cleanup-v1`
- Accepted base source: canonical `main` after this lane-registration commit is integrated.
- Registration Context-SHA: `4924a0da1cc1bf36e5044e49127518a7e93ed7f0a08513c8359f9b2777d429ff`; the literal post-registration Context-SHA belongs in the Master provisioning report and first executable packet.
- Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Launch state: not started; Don must not launch Worker Codex before Master GPT issues the matching supervisor seed.

## Authority and ownership

Allowed cleanup scope:

- `README.md`, `AGENTS.md`, `docs/**` except Master-owned `docs/lanes/**`
- `tools/fleet/**`, `ops/AGENTS.md`, `tools/AGENTS.md`, and `docs/AGENTS.md`
- `rust/README.md`, `rust/PARITY.md`, `rust/AGENTS.md`
- `.gitignore` and narrow repository metadata when source-clear
- `ops/evidence/rust-purity-repo-cleanup-v1/**`
- external logs and artifacts

Prohibited authority:

- no edits to Rust product or test source, any Cargo manifest or lockfile, or `rust/verify-forward`
- no edits to `docs/lanes/**`, `ops/fleet/integration-queue.json`, another lane's evidence, user content, topology, or canonical `main`
- no C++/CMake restoration, product behavior change, historical erasure, duplicate status owner, new documentation framework, or fleet expansion without current pressure
- no move of the Cargo workspace from `rust/` to repository root in this concurrent lane

Master Codex alone owns branch/worktree topology, lane registry, integration
queue, context propagation, candidate integration, and canonical push. Worker
authority is
`WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION`.

## Proof, dependencies, and overlap

Required proof includes a source-clear stale-reference inventory, preserved
history/decision ownership, link and context verification, fleet tests when
affected, the unchanged Rust forward gate, and one explicit workspace-location
recommendation: `KEEP_RUST_WORKSPACE_UNDER_RUST`,
`MOVE_WORKSPACE_TO_ROOT_IN_FUTURE_LANE`, or
`UNKNOWN_PENDING_REAL_PRESSURE`.

Direct path overlap with `real-cartridge-boot-spine-v1`: none under the
reserved scope contract. Indirect risk: this lane may change Context-SHA. Its
preferred integration order is first; Master then sends the boot lane a
`CONTEXT_DELTA_AMENDMENT` with `Restart-Goal: false` unless a semantic
contradiction exists. This lane may propose coordination updates but may not
make them.

Integration requires source-scope review, preserved durable history, current
link/context/fleet proof, Rust forward verification, and Master acceptance.
Stop before modification on topology or Context-SHA mismatch; stop on product
source overlap, authority expansion, historical loss, or contradictory accepted
law. Retire after accepted integration or explicit Master decision. The next
milestone is a complete supervisor seed; no repository cleanup is currently
claimed.
