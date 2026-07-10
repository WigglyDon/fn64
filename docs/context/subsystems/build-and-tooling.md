# Build And Tooling

Context role: build and verification context.
Scope: Cargo forward gate and standalone repository tools.
Canonical for: verification-entry ownership and dependency separation.
Not canonical for: machine behavior or remote CI policy.
Inherits: [repository standing law](../../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and [fleet context](fleet-operations.md).
Update triggers: default gate, toolchain dependency, or build-lane authority changes.

`rust/verify-forward` is the sole repository-owned product gate. It runs
formatting, clippy with warnings denied, complete Rust tests, the machine probe,
and the step probe. It resolves its workspace from its own path and uses caller
`PATH`; it calls no CMake, C++, SDL, product runtime, Git mutation, or
`cargo clean`.

The retired CMake/C++ build graph is absent from the current tree and exists
only in Git history. No compatibility wrapper, no-op CMake project, or fake C++
gate replaces it. Fleet tools inspect repository/context state but remain
outside the product gate.

Allowed dependencies are Cargo workspace → current Rust crates and fleet
scripts → standard shell/Git/core utilities. Forbidden dependencies include a
second product build lane, new task framework, remote CI, package manager,
daemon, automatic installer, hidden Git mutation, or product dependency on
documentation/ops.

Green output proves only the named commands at an exact SHA. It does not prove
boot, compatibility, host runtime, performance, or equivalence to retired
source.
`rust/target` is generated and ignored; it is never evidence-bundle content.
Required product validation is `./rust/verify-forward`.
