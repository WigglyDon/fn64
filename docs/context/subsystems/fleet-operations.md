# Fleet Operations

Context role: fleet coordination architecture context.
Scope: context, packets, lane diagnostics, integration queue, and evidence manifests.
Canonical for: fleet-tool authority and non-destructive operating rules.
Not canonical for: product state or source implementation truth.
Inherits: [repository standing law](../../../AGENTS.md) and [tools scope law](../../../tools/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [packet protocol](../../process/PACKET_PROTOCOL.md), [lane registry](../../lanes/lane-registry.md), and [evidence process](../../process/EVIDENCE_AND_RELEASE.md).
Update triggers: fleet tool, packet, lane, integration, or evidence ownership changes.

Fleet infrastructure reduces transport and repository ambiguity, but the fleet
operation is retired. The current loop is owned by
[Master Direct Workflow](../../process/MASTER_DIRECT_WORKFLOW.md): Master GPT
owns architecture/sequencing and Master Codex directly owns bounded repository
work, integration, authorized normal push, evidence, and reporting. Don remains
product authority and the physical manual packet boundary.

Supervisor/Worker branches, worktrees, packets, and provisioning rules are
historical only. They are preserved without automatic repair, reset, cleanup,
deletion, merge, or reuse.

Tools inspect context, packets, Git metadata, queues, and evidence. Default mode
is read-only. Explicit repair may only configure an already-authenticated forge
CLI and must report mutation. Reset, clean, force-push, canonical-main mutation
from workers, secret output, hidden background work, and product-source mutation
are forbidden.

The machine-readable integration queue remains the owner for active Worker
candidate coordination and is empty while Worker execution is retired; its
Markdown is generated. Git/source remain code truth. The local
`/home/don/fn64-fleet` spool is transport convenience only and is never
canonical or committed. Queue, lane, and tool output never create launch or
product authority.

Accepted proof includes deterministic fixture tests, valid/invalid packet cases,
Context-SHA repeatability, no-mutation lane-doctor samples, generated-view
equality, and exact artifact checksums. Text checks cannot prove semantic product
truth. Performance and resource cost must be measured before claimed.

Fleet tools remain dormant diagnostics. Their validation is
`tools/fleet/test-fleet` plus relevant check commands when a direct pass uses
them. They remain separate from `rust/verify-forward`; principal risk is tools
becoming a second project rather than paying rent.
