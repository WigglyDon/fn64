# Instruction Pipeline

Context role: fetch/decode/identify/classify context.
Scope: the represented Rust current-PC instruction production path.
Canonical for: pipeline fact ownership and ordering.
Not canonical for: exhaustive instruction semantics.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and `machine.rs` source/tests.
Update triggers: fetch targets, decode/identity ownership, selection, or action production changes.

The source-clear path is:

`current pc → target classification → one instruction fetch → one raw-field decode → one identity classification → no-effect/stopped/unsupported or one CPU-local helper selection → classified action`.

Production does not apply machine mutation. Application does not refetch,
decode, or identify. The instruction word and decoded fields are fixed-width;
memory words are interpreted through explicit big-endian storage access.

Allowed dependencies flow from cartridge/storage and CPU address types into
Machine fetch, then pure decode/identify, then classification/application.
Forbidden dependencies include host paths, dynamic registries, probe policy,
private producer calls from inspection, and a generic all-future dispatcher.

Proof consists of source anchors, classification/fetch unit tests, focused step
tests, and the eight-case step probe. It does not prove that every recognized
identity executes through public `Machine::step`; unselected helpers/readiness
remain distinct. Fetch rejection and rollback are observable only for sealed
paths.

Required validation: `./rust/verify-forward` and relevant focused filters.
Known unknowns include future public-step integration categories, complete
delay-slot behavior, broad fetch mapping, and instruction timing.
