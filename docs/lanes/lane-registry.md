# fn64 Lane Registry

Context role: canonical lane coordination registry.
Scope: active, planned, parked, blocked, retired, and historical lanes.
Canonical for: lane classification and links to active lane pages.
Not canonical for: implementation truth or source acceptance.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: Git worktree/branch inventory and [integration queue](integration-queue.md).
Update triggers: lane creation, activation, blocking, parking, retirement, or integration.

## Active lanes

- `real-cartridge-boot-spine-v1`
  - Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
  - Coordination owner: [lane page](real-cartridge-boot-spine-v1.md)
  - No Worker implementation or candidate exists.
- `rust-purity-repo-cleanup-v1`
  - Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
  - Coordination owner: [lane page](rust-purity-repo-cleanup-v1.md)
  - No Worker implementation or candidate exists.

Master process worktrees are not worker lanes and do not receive active Worker
lane pages merely because they are registered Git worktrees.

## Planned but not active

None.

## Parked donor lanes

- `/home/don/dump/fn64`: `LIVE_REPO_FACT`, a separate clean local clone of the
  same upstream identity at an older two-commit state. It is donor/reference
  only, has no active packet, and must not be deleted or merged automatically.

## Blocked lanes

None established from live repository evidence.

## Retired lanes

- `cpp-reference-truth-reconstruction-v1`: canceled without provisioning. Its
  reserved branch/worktree were never created, no launch command was issued,
  and no inventory work ran. The void `/tmp` topology had
  [`NO_RESIDUE_FOUND`](../../ops/evidence/master-provisioned-worker-worktrees-v1/CANCELED_LANE_RESIDUE_AUDIT.md).
- Former seam-090/inventory-first C++ retirement sequencing: superseded by the
  direct Master-owned retirement decision. No Worker deletion lane exists.

## Historical milestones

- Rust workspace adoption: commit `8034b5085c5131e71a0192d8a18e061b075d570e`.
- Rust forward-gate promotion: commit `df0551f87506d136717e7c2b3673580adce5869a`.
- Frozen C++ lane retirement: direct Master operation; Git history is the only
  archive and no parity prerequisite was applied.

Worktree existence alone never proves an active lane. Activation requires a
Master-provisioned and verified branch/worktree, literal launch command, current
packet, authority, goal, and Context-SHA. See
[Master-Provisioned Worker Worktrees](../process/WORKTREE_PROVISIONING.md).
