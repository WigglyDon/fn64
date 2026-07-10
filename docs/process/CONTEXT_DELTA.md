# Context Delta Protocol

Context role: canonical context-amendment contract.
Scope: safe propagation of accepted context to existing lanes.
Canonical for: amendment fields and update behavior.
Not canonical for: lane goals, product authority, or Git source truth.
Inherits: [packet protocol](PACKET_PROTOCOL.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [lane registry](../lanes/lane-registry.md) and [context law](../context/README.md).
Update triggers: context propagation mechanics or required delta fields change.

A `CONTEXT_DELTA_AMENDMENT` contains literal Packet-Version, Packet-ID,
Reply-To, From, To, Project-ID, Lane-ID, Goal-ID, Old-Context-SHA,
New-Context-SHA, Expected-Base, Worktree, Branch, Authority-State,
Changed-Context-Files, Branch-Update-Instruction,
Root-To-Local-Reread-Instruction, Contradiction-Report-Instruction,
Restart-Goal, Stale-When, and Allowed-Stop-Classes.

Normal behavior:

- clean lane: fast-forward when project convention permits;
- local commits: local merge or rebase according to established convention;
- dirty lane: preserve work and integrate context safely;
- context-only conflicts: resolve mechanically without losing lane work;
- semantic conflicts: stop and report;
- otherwise resume the existing goal with `Restart-Goal: false`.

A delta never erases lane work, expands authority, changes the goal, silently
changes product law, or asks Don to interpret Git mechanics. The reread order is
root AGENTS, every scoped AGENTS on the path, index, current state, detailed
ownership/migration ledger, relevant subsystem, active lane page, and relevant
contracts/evidence. Worker Codex confirms the new Context-SHA and reports
contradictions before resuming.

A context delta never grants branch/worktree management to Worker Codex and
never treats a supervisor path recommendation as provisioned truth. Existing
Master-provisioned topology remains unchanged unless Master Codex explicitly
reprovisions and verifies it. Dirty or unknown local work is preserved; Don does
not perform Git topology mechanics.
