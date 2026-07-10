# Host Runtime

Context role: host/runtime boundary context.
Scope: platform plumbing present or absent around fn64 cores.
Canonical for: host authority and current host-capability boundary.
Not canonical for: machine semantics or frozen C++ implementation detail.
Inherits: [repository standing law](../../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [historical C++ context](historical-cpp-reference.md) and [build context](build-and-tooling.md).
Update triggers: a host is added, retired, integrated, or changes authority.

Hosts may own file paths, CLI parsing, windows, input, audio-device plumbing,
presentation, platform event loops, shutdown, and platform error reporting.
They must not own emulated state, execution policy, ROM normalization truth,
machine timing, or cross-instance globals.

The forward Rust product currently has only no-window inspection binaries; it
has no SDL/window/audio/game runtime. The retained C++ lane contains CLI and SDL
host source, but it is frozen optional reference and is not a forward dependency.
The absence of a Rust SDL replacement is not itself a blocker under current law.

Allowed direction is thin host → public Machine/inspection surface. Core → host,
renderer → machine state, platform clock → stepping, and host-owned emulator
policy are forbidden. File I/O evidence must not read user-local commercial
assets during infrastructure work.

No host runtime evidence, rollback state, performance, window, audio, or input
behavior is claimed by the current required gate. Any future host needs an
explicit product packet and separate runtime observation. Required validation
today is only `./rust/verify-forward`; do not launch the retained product runtime.
