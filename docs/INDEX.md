# Project Context Index

Context role: root context boot index.
Scope: the complete fn64 repository.
Canonical for: discovery order and links to canonical context owners.
Not canonical for: mutable project state, subsystem detail, or runtime evidence.
Inherits: [repository standing law](../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](context/CURRENT_STATE.md).
Related evidence: [evidence and release process](process/EVIDENCE_AND_RELEASE.md).
Update triggers: a canonical owner, stable boundary, or discovery step changes.

## Agent Discovery Order

1. Read repository-root [AGENTS.md](../AGENTS.md).
2. Read every more-specific `AGENTS.md` from project root to the current working directory.
3. Read this index.
4. Read [the canonical current state](context/CURRENT_STATE.md).
5. Read the canonical represented-machine capability ledger: [rust/PARITY.md](../rust/PARITY.md).
6. Read the subsystem page linked by the nearest scoped `AGENTS.md`.
7. Read the active lane page when operating in a lane.
8. Read relevant contracts, decisions, fixture manifests, reports, and validation surfaces.
9. When documents conflict, stop and report the conflict rather than silently choosing.
10. Treat stronger current source, Git, or runtime evidence as capable of correcting stale shared memory.

## Canonical owners

- Standing product law: [root AGENTS.md](../AGENTS.md)
- Context-system law: [docs/context/README.md](context/README.md)
- Current project state: [CURRENT_STATE.md](context/CURRENT_STATE.md)
- Project history: [PROJECT_HISTORY.md](context/PROJECT_HISTORY.md)
- Accepted decisions: [DECISION_LOG.md](context/DECISION_LOG.md)
- Boundary map: [CONTEXT_MATRIX.md](context/CONTEXT_MATRIX.md)
- Context digest nodes: [CONTEXT_MANIFEST.json](context/CONTEXT_MANIFEST.json)
- Detailed represented-machine capability: [rust/PARITY.md](../rust/PARITY.md)
- Lane registry: [lane-registry.md](lanes/lane-registry.md)
- Integration queue view: [integration-queue.md](lanes/integration-queue.md)
- Packet protocol: [PACKET_PROTOCOL.md](process/PACKET_PROTOCOL.md)
- Worker worktree provisioning: [WORKTREE_PROVISIONING.md](process/WORKTREE_PROVISIONING.md)
- Context deltas: [CONTEXT_DELTA.md](process/CONTEXT_DELTA.md)
- Handoff and respawn: [HANDOFF_AND_RESPAWN.md](process/HANDOFF_AND_RESPAWN.md)
- Evidence and release: [EVIDENCE_AND_RELEASE.md](process/EVIDENCE_AND_RELEASE.md)
- Runtime evidence schemas: [ops/evidence/README.md](../ops/evidence/README.md)
- Current verification entry point: [`./rust/verify-forward`](../rust/verify-forward)

## Subsystem context

- [Machine core](context/subsystems/machine-core.md)
- [CPU execution](context/subsystems/cpu-execution.md)
- [Memory and cartridge](context/subsystems/memory-and-cartridge.md)
- [Instruction pipeline](context/subsystems/instruction-pipeline.md)
- [Exceptions and COP0](context/subsystems/exceptions-and-cop0.md)
- [Inspection and evidence](context/subsystems/inspection-and-evidence.md)
- [Host runtime](context/subsystems/host-runtime.md)
- [Build and tooling](context/subsystems/build-and-tooling.md)
- [Fleet operations](context/subsystems/fleet-operations.md)
