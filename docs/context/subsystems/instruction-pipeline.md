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

`current pc/context → target/provenance classification → one instruction fetch → one raw-field decode → one identity classification → contextual and bootstrap source-knownness gates → ordinary-control-flow planning, no-effect/stopped/unsupported, aligned-Lw planning, or one CPU-local helper selection → classified action`.

Production does not apply machine mutation. Application does not refetch,
decode, or identify. The instruction word and decoded fields are fixed-width;
memory words are interpreted through explicit big-endian storage access.

Allowed dependencies flow from cartridge/storage and CPU address types into
Machine fetch, then pure decode/identify, then classification/application.
Forbidden dependencies include host paths, dynamic registries, probe policy,
private producer calls from inspection, and a generic all-future dispatcher.

Proof consists of source anchors, classification/fetch unit tests, focused step
tests, the fourteen-case step probe, and the bounded BOOT-2 trace. Read-only
current-instruction inspection exposes address, fields, identity, and Machine
source provenance without mutable state. Proof does not mean every recognized
identity executes. `Lw` is represented as one Machine-owned rule over direct
RDRAM and known SP IMEM, with a separate pre-mutation rejection when any SP
IMEM source byte is unknown. The authentic first frontier remains that
represented rejection at `0xA4000044`, not absent decode or load semantics.
Integrated provenance evidence identifies the missing source category as
retained IPL2 firmware content. Explicit profiled bootstrap materialization can
now satisfy the known-byte gate from generated or user-supplied bytes; external
source knowledge alone cannot bypass it. Synthetic `Lw` success proves the
represented composition, not authentic boot. A separate generated-only NTSC
cold-x105 test proves the Machine bootstrap plan can source the inherited t3
operand before `Machine::step`; that source gate still rejects every unstaged
GPR and does not imply IPL2 execution.

Ordinary `BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR` identities now select
one bounded Machine-owned action before sequential staging. A control-flow
identity inside a represented slot selects explicit unsupported rollback.
Application schedules or clears the CPU-owned slot context; it does not refetch
or re-identify.

Required validation: `./rust/verify-forward` and relevant focused filters.
Known unknowns include future public-step integration categories, branch-likely
and other control-flow families, nested control-flow behavior, broad fetch
mapping, and instruction timing.
