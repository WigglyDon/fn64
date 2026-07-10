# Retired C++ History

Context role: retired C++ era and archival-boundary context.
Scope: the former C++ machine, host, proof, CLI, SDL, CMake, and launch lane.
Canonical for: retirement status and the boundary between current source and Git history.
Not canonical for: current Rust capability or semantic parity.
Inherits: [repository standing law](../../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [project history](../PROJECT_HISTORY.md), Git history, and the [historical boot checkpoint](../../boot_spine_checkpoint.md).
Update triggers: archival authority or an accepted decision about retired-source interpretation changes.

The former C++ source/build/host/proof lane is absent from the current product
tree. Git history is its only archive. No checked-in museum, compatibility
wrapper, fake CMake target, or replacement C++ gate exists.

`USER_DECISION` waived semantic parity and inventory prerequisites before
retirement. Unported C++ instruction behavior, cartridge/bootstrap experiments,
device behavior, CLI output, SDL/window/event-loop plumbing, and proof coverage
are `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT`. They were not migrated and are
not implied by Rust tests.

Historical documents and the detailed Rust ledger may name deleted paths,
targets, or commands as dated source anchors. Those names are not current files
or runnable instructions. Restoring retired C++ as a product/reference lane is
forbidden without a new explicit product decision; future required behavior is
earned independently in Rust under current architecture.

The required current validation is `./rust/verify-forward`. It proves the
bounded Rust gate only—not C++ parity, cartridge boot, game compatibility,
host runtime, or performance.
