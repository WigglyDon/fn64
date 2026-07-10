# fn64-core Scope

Scope: `rust/crates/fn64-core/`.
Inherits: [root law](../../../AGENTS.md) and [Rust scope law](../../AGENTS.md).
Owner: represented cartridge, memory, CPU, exception, and `Machine` truth.

The crate may depend only on machine-owned state and Rust standard/core
facilities already accepted by the workspace. It must not depend on
`fn64-inspection`, host paths, terminal formatting, SDL, window/audio APIs,
platform clocks, Git/fleet policy, or probe assertion policy.

Read [the context index](../../../docs/INDEX.md), [current state](../../../docs/context/CURRENT_STATE.md),
[machine-core context](../../../docs/context/subsystems/machine-core.md),
[CPU context](../../../docs/context/subsystems/cpu-execution.md), and
[rust/PARITY.md](../../PARITY.md). Validate behavior with
`../../../rust/verify-forward` from repository root plus the narrow focused test
for the assigned seam.

Stop on a second owner for machine state, a generic future-facing execution
framework, host-policy leakage, or unearned behavior. Update this file only when
crate authority or dependency direction changes.
