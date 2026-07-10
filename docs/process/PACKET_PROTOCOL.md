# fn64 Packet Protocol V1

Context role: canonical packet-envelope and causality contract.
Scope: packets crossing fn64's manual isolated-agent boundary.
Canonical for: required fields, packet types, staleness, replies, and stop classes.
Not canonical for: product state or lane implementation progress.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [worktree provisioning](WORKTREE_PROVISIONING.md), [context-delta protocol](CONTEXT_DELTA.md), and [handoff protocol](HANDOFF_AND_RESPAWN.md).
Update triggers: packet schema, authority role, cadence, or stop classification changes.

## Envelope

Every executable packet begins with `BEGIN PROJECT PACKET`, ends with
`END PROJECT PACKET`, and contains literal values for: Packet-Version,
Packet-ID, Reply-To, From, To, Project-ID, Lane-ID, Goal-ID, Context-SHA,
Expected-Base, Worktree, Branch, Authority-State, Expected-Reply-Type,
Stale-When, and Allowed-Stop-Classes. `Project-ID` is always `fn64`.

`UNKNOWN` is valid only as an explicit evidence state. Angle-bracket or other
unresolved placeholders make an executable packet invalid. Template fragments
are not packets. IDs are unique; every reply names its source packet. Context-
SHA names governing project memory; Expected-Base names intended Git lineage;
worktree, branch, authority, staleness, and allowed final classes are literal.

Assertions never promote themselves to live truth. Contradictions and stale
conditions are reported, not silently bypassed. Manual copy/paste transport
does not weaken causality or imply a direct-agent messaging layer.

## Packet types

- `MASTER_TO_MASTER_CODEX_WORK`
- `MASTER_CODEX_FINAL_REPORT`
- `SUPERVISOR_SEED`
- `SUPERVISOR_TO_WORKER_WORK`
- `WORKER_FINAL_REPORT`
- `FOCUSED_REPAIR`
- `FINAL_LANE_PACKET`
- `RED_LINE_DECISION_REQUEST`
- `CONTEXT_DELTA_AMENDMENT`
- `RESPAWN_SEED`
- `INTEGRATION_RESULT`

## Normal lane autonomy

An implementation-authorized lane performs one comprehensive work pass rather
than stopping after discovery. There is no routine progress packet or branch/auth
troubleshooting routed through Don. One focused repair is normal when genuinely
needed; a second is permitted only after material progress with a bounded
remainder. The lane returns merge-ready, partial-salvage, red-line, or respawn
truth. Master Codex provisions and verifies topology before launch. Workers
never create, attach, repair, move, remove, prune, or switch worktrees/branches;
merge, rebase, reset, clean, push, deploy, mutate canonical main, discard unknown
work, or expand authority.

Red-line stops are product-purpose conflict, authority expansion, security risk,
unknown destructive operation, irreducible scope conflict, contradictory
accepted law, data-loss risk, or need for user judgment. Compile failures,
ordinary local conflicts, stale refs, and missing routine SHAs are not red-line
by themselves.

## Master-provisioned Worker packets

A `SUPERVISOR_TO_WORKER_WORK` packet is executable only after Master Codex has
provisioned and verified its literal topology under
[WORKTREE_PROVISIONING.md](WORKTREE_PROVISIONING.md). Supervisor seeds may carry
semantic recommendations, but they do not reserve or create executable Git
topology.

In addition to the common envelope, a Worker work packet contains:

- `Provisioning-State: MASTER_PROVISIONED_VERIFIED`;
- `Provisioning-Exception: NONE` for a normal persistent lane, otherwise one
  literal Master-authorized reason;
- `Launch-Command`, as `cd` into the exact provisioned worktree followed by
  `codex`, with no placeholder;
- `Lane-State: ACTIVE_PROVISIONED`;
- `Worker-Worktree-Management: forbidden`;
- `Worker-Branch-Management: forbidden`; and
- `Worker-Integration: forbidden`.

The normal authority is exactly
`WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION`. Normal
worker paths are `/home/don/fn64-worktrees/LITERAL_LANE_ID`; `/tmp` requires a
non-`NONE` provisioning exception.

The packet instructs Worker Codex to verify repository root, branch, starting
HEAD, clean worktree, clean index, and Context-SHA before modification. Any
mismatch causes a no-modification stop. The packet must not authorize worktree
or branch creation/removal/repair/switching, merge, rebase, reset, clean, or
push. Master Codex alone owns actual topology and integration.

## Literal non-executable historical example

The following demonstrates an already provisioned persistent worker packet. It
is non-executable historical documentation because no such lane is registered:

```text
BEGIN PROJECT PACKET
Packet-Version: 1.0
Packet-ID: fn64-historical-provisioned-worker-example-2026-07-10-001
Reply-To: NONE
From: Supervisor GPT
To: Worker Codex
Project-ID: fn64
Lane-ID: historical-forward-gate-example
Goal-ID: demonstrate-packet-v1
Context-SHA: 95ba787a2c7adad5d95341de12015d14c5bcb68acb224783bcfc757da686ee3f
Expected-Base: 5fedaf8fc9257faa43566febb14ff8a3aa79d28c
Worktree: /home/don/fn64-worktrees/historical-forward-gate-example
Branch: worker/historical-forward-gate-example
Authority-State: WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION
Expected-Reply-Type: WORKER_FINAL_REPORT
Stale-When: this documented example is not an active registered lane
Allowed-Stop-Classes: ACCEPTED,NEEDS_FIX,PARTIAL,STOP
Provisioning-State: MASTER_PROVISIONED_VERIFIED
Provisioning-Exception: NONE
Launch-Command: cd /home/don/fn64-worktrees/historical-forward-gate-example && codex
Packet-Type: SUPERVISOR_TO_WORKER_WORK
Lane-State: ACTIVE_PROVISIONED
Worker-Worktree-Management: forbidden
Worker-Branch-Management: forbidden
Worker-Integration: forbidden

Goal: demonstrate a complete literal provisioned Worker envelope only.
Worker-Preflight: verify exact root, branch, HEAD, clean worktree/index, and Context-SHA; stop without modification on mismatch.
No-Push: required
No-Deploy: required

END PROJECT PACKET
```

Structural validation uses `tools/fleet/packet-verify`. Validation proves
completeness and consistency, not the truth of claims.
