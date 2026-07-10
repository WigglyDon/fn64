# fn64-inspection Scope

Scope: `rust/crates/fn64-inspection/`.
Inherits: [root law](../../../AGENTS.md) and [Rust scope law](../../AGENTS.md).
Owner: deterministic no-window probe construction, formatting, assertions, and
process exit status.

This crate may consume public `fn64-core` machine-owned APIs. It must not own
emulated state, call private production/applicator seams, create host emulator
policy, launch SDL/window/audio, or claim cartridge boot or compatibility.

Read [the context index](../../../docs/INDEX.md), [current state](../../../docs/context/CURRENT_STATE.md),
[inspection context](../../../docs/context/subsystems/inspection-and-evidence.md),
and [rust/PARITY.md](../../PARITY.md). Validate through
`../../../rust/verify-forward` from repository root.

Stop when a probe requires a broad debug backdoor or behavior outside the core's
accepted public surface. Update this file only when inspection ownership or
probe authority changes.
