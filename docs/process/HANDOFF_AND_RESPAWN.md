# Handoff And Respawn

> Historical process: supervisor/Worker respawn is retired. This page preserves
> donor and non-destructive retirement law only; current work follows
> [Master Direct Workflow](MASTER_DIRECT_WORKFLOW.md).

Context role: canonical lane handoff and respawn contract.
Scope: stale supervisors/workers, donor work, lane retirement, and restart seeds.
Canonical for: when delta, respawn, parking, or retirement is appropriate.
Not canonical for: product state or acceptance of candidate code.
Inherits: [packet protocol](PACKET_PROTOCOL.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [lane registry](../lanes/lane-registry.md) and [context delta](CONTEXT_DELTA.md).
Update triggers: role, staleness, respawn, donor, or retirement rules change.

A supervisor or worker is stale when its governing Context-SHA/base/authority is
superseded and a delta cannot reconcile it safely, its worktree/branch no longer
matches its literal packet, or semantic contradictions make continuation unsafe.
Missing private chat alone is not staleness when repository and packet evidence
are complete.

Use a context delta when goal, authority, and implementation remain valid. Use a
full respawn when the lane lacks a recoverable complete packet, its semantic
model is invalid, the worktree is irreconcilable, or responsibility is replaced.
Master Codex creates or verifies replacement worktrees from accepted canonical
main under [WORKTREE_PROVISIONING.md](WORKTREE_PROVISIONING.md); supervisors and
workers never provision them and useful donor work is never deleted.

A respawn preserves project identity, new Context-SHA, lane page, accepted base,
literal worktree and branch, latest candidate HEAD, latest packet ID, artifacts,
goal, risks, exact next action, authority/prohibitions, expected proof, donor
work, and contradictions. It is complete without private chat and validates as
a `RESPAWN_SEED`. It is not launch-ready until Master Codex has provisioned or
verified that literal branch/worktree and supplied its provisioning state,
exception, and launch command to the replacement supervisor and Worker.

Park an old worktree as donor/reference when it contains useful or unknown work
but no current authority. Retire a lane only after its result is integrated,
superseded with preserved evidence, or explicitly abandoned by authority.
Deactivate stale sessions in lane memory without pretending an inaccessible
chat was modified. Never reset, clean, delete, or overwrite unknown work to make
a handoff look tidy.

A stale or canceled worktree is never automatically removed. Preserved residue
is recorded in the lane registry; Master Codex alone decides reuse after a
separate source-clear audit. Don transports the complete respawn packet and
literal launch command but performs no branch/worktree mechanics.
