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

None after verified Wave 3 integration. Master retains lane, queue, context,
branch, worktree, and integration ownership.

Master process worktrees are not worker lanes and do not receive active Worker
lane pages merely because they are registered Git worktrees.

## Planned but not active

- `pif-ipl2-profiled-copy-materialization-v1`: planned from the accepted
  variant-qualified mapping; registration and provisioning remain Master work.
- `pif-ipl2-handoff-state-mapping-v1`: planned evidence-only follow-on for the
  complete pre-IPL3 state boundary.

## Parked donor lanes

- `/home/don/dump/fn64`: `LIVE_REPO_FACT`, a separate clean local clone of the
  same upstream identity at an older two-commit state. It is donor/reference
  only, has no active packet, and must not be deleted or merged automatically.

## Blocked lanes

None established from live repository evidence.

## Retired lanes

- [`ordinary-control-flow-delay-slot-v1`](ordinary-control-flow-delay-slot-v1.md):
  **ACCEPTED — INTEGRATED** at candidate `01b06e5a`. All six ordinary
  identities, one explicit slot, link/alias/Count rules, branch-in-slot
  rollback, and selected slot exception EPC/BD lineage are accepted. BOOT-2 is
  unchanged; Worker branch/worktree remain preserved and unpushed.
- [`pif-ipl2-source-mapping-v1`](pif-ipl2-source-mapping-v1.md): **ACCEPTED —
  VARIANT-SPECIFIC SOURCE MAPPING** at candidate `2ee4b3c7`. NTSC uses raw
  `[0x0d4,0x71c)` and PAL/MPAL use `[0x0d4,0x720)`, all to SP IMEM offset
  zero. No private input or product behavior change occurred; Worker
  branch/worktree remain preserved and unpushed.
- [`user-supplied-pif-boot-source-v1`](user-supplied-pif-boot-source-v1.md):
  **ACCEPTED — SOURCE-BOUNDARY PRODUCT** at complete candidate `1fa8aa17`.
  Explicit no-search `--pif-rom` plumbing, Machine-owned structural validation,
  immutable input, lifecycle, and atomic rejection are accepted. No private PIF
  input, SP IMEM production, firmware execution, checkpoint advance, or
  compatibility fact was earned; Worker branch/worktree remain preserved and
  unpushed.
- [`sp-imem-bootstrap-provenance-v1`](sp-imem-bootstrap-provenance-v1.md):
  **PARTIAL — EVIDENCE INTEGRATED; PRODUCT SOURCE UNAVAILABLE** at candidate
  `8db1b57c`. Source-qualified evidence identifies retained IPL2 content as the
  observed x105 source, consuming `[0x000, 0x020)` and initially mutating
  `[0x000, 0x02c)`. No product behavior or checkpoint changed; Worker
  branch/worktree remain preserved and unpushed.
- [`boot-frontier-sp-imem-lw-v1`](boot-frontier-sp-imem-lw-v1.md):
  **PARTIAL — INTEGRATED** at candidate `dcb9f1bf`. SP IMEM and aligned `Lw`
  are accepted; the authentic trace remains BOOT-2 at unknown SP IMEM byte
  zero. Worker branch/worktree remain preserved and unpushed.
- [`real-cartridge-boot-spine-v1`](real-cartridge-boot-spine-v1.md): completed,
  integrated, and closed. BOOT-2 is accepted; BOOT-3 is not earned. Worker
  branch/worktree remain preserved and unpushed.
- [`rust-purity-repo-cleanup-v1`](rust-purity-repo-cleanup-v1.md): completed,
  integrated, and closed. Its accepted non-product consolidation scope is
  complete; worker branch/worktree remain preserved and unpushed.
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
- Authentic BOOT-2: worker commits `6f189716` and `8e5efc8e`; one
  cartridge-derived `SpecialAdd` committed before the `Lw` frontier.
- Rust-purity consolidation: worker commit `9cc16142`; current capability now
  has one detailed owner while durable history remains discoverable.

Worktree existence alone never proves an active lane. Activation requires a
Master-provisioned and verified branch/worktree, literal launch command, current
packet, authority, goal, and Context-SHA. See
[Master-Provisioned Worker Worktrees](../process/WORKTREE_PROVISIONING.md).
