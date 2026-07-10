# Host Runtime

Context role: host/runtime boundary context.
Scope: platform plumbing currently absent or later earned around fn64-core.
Canonical for: host authority and current host-capability boundary.
Not canonical for: machine semantics or retired host implementation detail.
Inherits: [repository standing law](../../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [project history](../PROJECT_HISTORY.md) and [build context](build-and-tooling.md).
Update triggers: a host is added, retired, integrated, or changes authority.

Hosts may own file paths, CLI parsing, windows, input, audio-device plumbing,
presentation, platform event loops, shutdown, and platform error reporting.
They must not own emulated state, execution policy, ROM-normalization truth,
machine timing, or cross-instance globals.

The current product has only no-window inspection binaries. It has no ROM-path
shell, SDL/window/audio/game runtime, renderer, input loop, or host timing. The
former CLI and SDL host were intentionally retired without a current
replacement; this accepted absence creates no restoration prerequisite.

Allowed future direction is thin host → public Machine/inspection surface.
Core → host, renderer → machine state, platform clock → stepping, and host-owned
emulator policy remain forbidden. User-local commercial assets stay outside
routine inspection and evidence.

Host runtime, rollback, performance, window, audio, and input evidence are
unavailable. Any future host needs an explicit product packet and separate
runtime observation. Current required validation is only
`./rust/verify-forward`.
