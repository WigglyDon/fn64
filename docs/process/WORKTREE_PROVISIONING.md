# Master-Provisioned Worker Worktrees

> Historical process: Supervisor GPT and Worker Codex lane roles are retired.
> This page preserves the former topology contract and grants no current launch
> or provisioning authority. See [Master Direct Workflow](MASTER_DIRECT_WORKFLOW.md).

Context role: canonical worker-topology provisioning process.
Scope: fn64 Worker Codex branches, worktrees, launch commands, cancellation, and reprovisioning.
Canonical for: who chooses, creates, verifies, reattaches, and integrates worker topology.
Not canonical for: product architecture, lane semantic goals, current lane state, Git history, or evidence results.
Inherits: [repository standing law](../../AGENTS.md) and [packet protocol](PACKET_PROTOCOL.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [lane registry](../lanes/lane-registry.md) and [handoff/respawn](HANDOFF_AND_RESPAWN.md).
Update triggers: worker-topology authority, persistent-path policy, launch ownership, cancellation, or respawn provisioning changes.

## Sole provisioning owner

Master Codex alone provisions Worker Codex branches and worktrees. Supervisors
own lane semantics and may recommend a topology, but their seeds and first
Worker packets become executable only after Master Codex has created or safely
reattached and verified that literal topology.

For every worker lane, Master Codex:

1. inspects canonical repository state and checks for stale, canceled, parked,
   overlapping, or conflicting lane state;
2. chooses the literal lane ID, branch, accepted base SHA, and persistent
   worktree path;
3. creates or safely attaches the branch and worktree with non-destructive Git
   mechanics;
4. verifies the exact path, branch, starting HEAD, clean worktree, clean index,
   absence of unknown prior commits/files, governing Context-SHA, lane registry,
   and integration queue;
5. gives Don one literal launch command for the verified topology; and
6. records the provisioned topology before a supervisor seed or first Worker
   packet is issued.

The normal policy pattern is non-executable documentation:

```text
cd /home/don/fn64-worktrees/&lt;lane-id&gt;
codex
```

An executable packet replaces the rendered lane token with one literal lane ID
and exact provisioned path. Don transports that complete command and packet; Don
does not perform branch or worktree mechanics.

## Persistent path and exceptions

Normal worker worktrees live under:

`/home/don/fn64-worktrees/LITERAL_LANE_ID`

`/tmp` is not the normal worker-lane location. A `/tmp` worker worktree requires
an explicit Master decision, a literal reason recorded as the provisioning
exception, and the same creation and verification checks. A reserved future
path or branch is planning data, not provisioned truth or an active lane.

A deferred lane has no launch command and no provisioned branch/worktree.

## Worker authority after provisioning

The first Worker packet contains the exact provisioned worktree, branch,
starting base, governing Context-SHA, lane ID, launch command, and
`Provisioning-State: MASTER_PROVISIONED_VERIFIED`.

Before modifying anything, Worker Codex verifies:

- its repository root equals the packet worktree;
- its current branch equals the packet branch;
- its current HEAD equals the packet starting base;
- the worktree and index are clean; and
- Context-SHA matches the packet.

On any mismatch, Worker Codex stops without modification.

Worker Codex commits only inside the assigned worktree and branch and returns
candidate commits and evidence to its supervisor. Worker Codex must not create,
remove, move, repair, attach, detach, or prune a worktree; create, rename,
delete, or switch branches; merge, rebase, reset, clean, or push; integrate into
canonical main; or repair topology.

Master Codex alone revalidates worker claims, reviews candidate commits, creates
integration branches when needed, integrates canonical main, runs final
validation, and pushes when authorized.

## Cancellation, context updates, and respawn

A canceled or stale branch/worktree is `UNKNOWN` until Master Codex inspects it.
Residue is preserved. No branch or worktree is automatically reused, reset,
cleaned, removed, moved, overwritten, or attached. The lane registry records
its disposition, and reuse requires a separate source-clear Master decision.

Context-only amendments never transfer provisioning authority to supervisors or
workers and do not change existing verified topology unless Master Codex
explicitly reprovisions it.

Respawning a worker requires new Master provisioning or verified reattachment.
The replacement supervisor and Worker receive the literal provisioned topology,
not a speculative path or private-chat assumption. Dirty or unknown prior work
remains preserved.
