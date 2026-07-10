# fn64 Packet Protocol V1

Context role: canonical packet-envelope and causality contract.
Scope: packets crossing fn64's manual isolated-agent boundary.
Canonical for: required fields, packet types, staleness, replies, and stop classes.
Not canonical for: product state or lane implementation progress.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [context-delta protocol](CONTEXT_DELTA.md) and [handoff protocol](HANDOFF_AND_RESPAWN.md).
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
truth. Workers never push, deploy, mutate canonical main, discard unknown work,
or expand authority.

Red-line stops are product-purpose conflict, authority expansion, security risk,
unknown destructive operation, irreducible scope conflict, contradictory
accepted law, data-loss risk, or need for user judgment. Compile failures,
ordinary local conflicts, stale refs, and missing routine SHAs are not red-line
by themselves.

## Literal non-executable historical example

The following demonstrates the envelope and is intentionally stale because its
base predates the current infrastructure context:

```text
BEGIN PROJECT PACKET
Packet-Version: 1.0
Packet-ID: fn64-historical-example-2026-07-09-001
Reply-To: NONE
From: Master GPT
To: Worker Codex
Project-ID: fn64
Lane-ID: historical-forward-gate-example
Goal-ID: demonstrate-packet-v1
Context-SHA: UNKNOWN
Expected-Base: df0551f87506d136717e7c2b3673580adce5869a
Worktree: /tmp/fn64-historical-example
Branch: historical/example
Authority-State: NON_EXECUTABLE_HISTORICAL_EXAMPLE
Expected-Reply-Type: WORKER_FINAL_REPORT
Stale-When: always; this is a non-executable historical example
Allowed-Stop-Classes: STOP

Purpose: demonstrate a complete literal fn64 packet envelope only.
Execution: forbidden.

END PROJECT PACKET
```

Structural validation uses `tools/fleet/packet-verify`. Validation proves
completeness and consistency, not the truth of claims.
