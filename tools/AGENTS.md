# Fleet Tooling Scope

Scope: `tools/`.
Inherits: [repository standing law](../AGENTS.md).
Owner: small repository diagnostics and deterministic context/packet tooling.

Tools may inspect Git metadata, context, packets, queues, and evidence. Default
behavior must be non-destructive. They must not own product truth, mutate
machine state, launch runtime hosts, hide branch mutations, expose secrets, or
become a second project-state database.

Read [the context index](../docs/INDEX.md), [current state](../docs/context/CURRENT_STATE.md),
[fleet context](../docs/context/subsystems/fleet-operations.md), and
[packet protocol](../docs/process/PACKET_PROTOCOL.md). The canonical provisioning
owner is [WORKTREE_PROVISIONING.md](../docs/process/WORKTREE_PROVISIONING.md);
fleet tools may validate provisioned facts but must not provision a lane.
Validate with the fleet fixture suite and the relevant tool's check mode.

Stop when a tool would require a new product dependency, destructive Git
operation, or semantic product judgment. Update this file only when tooling
authority or safety policy changes.
