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

Active worker lanes: none found from live repository evidence.

The dedicated `master/infrastructure-v1` worktree is Master integration work for
the current packet, not a worker lane, and therefore has no active-lane page.

## Planned but not active

- C++ deletion / former seam-090: `USER_DECISION` superseded the drafted packet.
  It has no branch, worktree, candidate SHA, or active worker. A new explicit
  product packet is required before activation.

## Parked donor lanes

- `/home/don/dump/fn64`: `LIVE_REPO_FACT`, a separate clean local clone of the
  same upstream identity at an older two-commit state. It is donor/reference
  only, has no active packet, and must not be deleted or merged automatically.

## Blocked lanes

None established from live repository evidence.

## Retired lanes

None represented as durable lane records. Historical C++ and Rust migration
passes are milestones, not currently active lanes.

## Historical milestones

- Rust workspace adoption: commit `8034b5085c5131e71a0192d8a18e061b075d570e`.
- Rust forward-gate promotion: commit `df0551f87506d136717e7c2b3673580adce5869a`.

Worktree existence alone never proves an active lane. Activation requires a
literal current packet, authority, branch/worktree, goal, and Context-SHA.
