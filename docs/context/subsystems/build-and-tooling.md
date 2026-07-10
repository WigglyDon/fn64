# Build And Tooling

Context role: build and verification context.
Scope: Cargo forward gate, optional CMake reference lane, and standalone tools.
Canonical for: verification-entry ownership and dependency separation.
Not canonical for: machine behavior or remote CI policy.
Inherits: [repository standing law](../../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and [fleet context](fleet-operations.md).
Update triggers: default gate, toolchain dependency, or build-lane authority changes.

`rust/verify-forward` is the repository-owned required gate. It runs formatting,
clippy with warnings denied, complete Rust tests, the machine probe, and the step
probe. It resolves its own workspace and uses caller `PATH`; it does not call
CMake, C++, SDL, product runtime, Git mutation, or `cargo clean`.

CMake and C++ targets remain optional frozen-reference tooling, invoked only by
an explicit reference/inventory task. The Rust gate must not consume CMake
output. Fleet tools may inspect repository/context state but are deliberately
not wired into the product gate in this infrastructure pass.

Allowed dependencies are Cargo workspace → current Rust crates and fleet scripts
→ standard shell/Git/core utilities. Forbidden dependencies include a new task
framework, remote CI, package manager, daemon, automatic installer, hidden Git
mutation, and product dependency on documentation/ops.

Green build output proves only its named commands at an exact SHA. It does not
prove boot, compatibility, host runtime, performance, or C++ parity. Build
directories and `rust/target` are generated/ignored and never evidence-bundle
content. Required validation is the Rust gate plus the narrow standalone tool
suite for infrastructure changes.
