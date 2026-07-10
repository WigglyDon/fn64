# Project Context System

Context role: context-system law.
Scope: durable repository memory and its discovery graph.
Canonical for: context ownership, inheritance, staleness, and digest rules.
Not canonical for: product behavior or current lane implementation state.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](CURRENT_STATE.md).
Related evidence: [EVIDENCE_AND_RELEASE.md](../process/EVIDENCE_AND_RELEASE.md).
Update triggers: context ownership, evidence language, or integration duties change.

## One fact, one owner

Standing law belongs in scoped `AGENTS.md`; mutable state in
[CURRENT_STATE.md](CURRENT_STATE.md); chronology in
[PROJECT_HISTORY.md](PROJECT_HISTORY.md); accepted choices in
[DECISION_LOG.md](DECISION_LOG.md); contracts in process pages; detailed
represented-machine capability in [rust/PARITY.md](../../rust/PARITY.md);
measured proof in identified evidence artifacts. Other pages link instead of
copying mutable facts.

## Inheritance and discovery

Context inherits from repository root toward the working directory. Every agent
reads root law, each scoped law on that path, [docs/INDEX.md](../INDEX.md), the
current-state owner, the relevant subsystem page, and any active lane page.
Child context may specialize but never weaken parent law.

## Evidence language

- `USER_DECISION`: Don's explicit product or process judgment.
- `LIVE_REPO_FACT`: directly observed repository, Git, source, mode, or history state.
- `RUNTIME_FACT`: directly measured behavior tied to a command and artifact.
- `WORKER_CLAIM`: a prior report or packet not revalidated in the current pass.
- `INFERENCE`: a conclusion whose source evidence is named.
- `UNKNOWN`: unavailable, ambiguous, contradictory, or unverified.

Claims do not promote themselves. Source, Git, and runtime evidence may correct
stale memory; product meaning remains governed by user decision and accepted
law. Conflicting canonical owners stop integration until reconciled. Stale text
is labeled or moved into history, never silently preferred.

## Context-SHA

[`CONTEXT_MANIFEST.json`](CONTEXT_MANIFEST.json) enumerates canonical context
nodes. `tools/fleet/context-sha` sorts paths bytewise, hashes exact repository
bytes with SHA-256, and hashes the path/digest records. Missing, duplicate, or
escaping paths fail. The tool reports dirty context separately. A digest from a
dirty tree is not accepted committed context.

The manifest must not list transient evidence, generated build output, private
chat, or a local packet spool. Documents do not embed their own Git blob or
commit SHA; Git owns revision history.

## Update and integration duties

Update context when source ownership, current authority, a decision, a lane,
verification policy, or a canonical evidence owner changes. Master Codex
revalidates live facts, updates the smallest canonical owner, verifies links and
the manifest, records the new Context-SHA, and propagates a context delta to
active lanes. Master Codex also provisions and verifies literal worker topology
under [WORKTREE_PROVISIONING.md](../process/WORKTREE_PROVISIONING.md).
Supervisors seed semantic lane context only after provisioning; Worker Codex
discovers live root-to-local law and verifies its assigned topology before
acting.

Packets carry context across isolated manual transport. They do not replace
durable repository context, confer live truth by assertion, or imply direct
agent messaging. Future automation may check structure and consistency; human
Master review remains responsible for semantic contradiction.
