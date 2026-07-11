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

`current pc → target/provenance classification → one instruction fetch → one raw-field decode → one identity classification → bootstrap source-knownness gate when active → no-effect/stopped/unsupported, aligned-Lw planning, or one CPU-local helper selection → classified action`.

Production does not apply machine mutation. Application does not refetch,
decode, or identify. The instruction word and decoded fields are fixed-width;
memory words are interpreted through explicit big-endian storage access.

Allowed dependencies flow from cartridge/storage and CPU address types into
Machine fetch, then pure decode/identify, then classification/application.
Forbidden dependencies include host paths, dynamic registries, probe policy,
private producer calls from inspection, and a generic all-future dispatcher.

Proof consists of source anchors, classification/fetch unit tests, focused step
tests, the eight-case step probe, and the bounded BOOT-2 trace. Read-only
current-instruction inspection exposes address, fields, identity, and Machine
source provenance without mutable state. Proof does not mean every recognized
identity executes. `Lw` is represented as one Machine-owned rule over direct
RDRAM and known SP IMEM, with a separate pre-mutation rejection when any SP
IMEM source byte is unknown. The authentic first frontier remains that
represented rejection at `0xA4000044`, not absent decode or load semantics.

Required validation: `./rust/verify-forward` and relevant focused filters.
Known unknowns include future public-step integration categories, complete
delay-slot behavior, broad fetch mapping, and instruction timing.
