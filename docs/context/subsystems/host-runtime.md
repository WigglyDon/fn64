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

The current product has only no-window inspection binaries. The bounded
`fn64_boot_probe` owns one ROM path/file read for explicit evidence and then
passes owned bytes to the core; it is not a game runtime. There is no
SDL/window/audio runtime, renderer, input loop, or host timing. The former CLI
and SDL host were intentionally retired without a current replacement; this
accepted absence creates no restoration prerequisite.

Product law now permits one explicit optional user-supplied PIF firmware path.
Host authority is limited to parsing that literal path, opening/reading it,
reporting failure, and transferring owned bytes. The existing no-window boot
probe implements this boundary as `--pif-rom`; it is still an inspection shell,
not a runtime host. Automatic search, hidden defaults, downloads, acquisition
instructions, bundled fallbacks, and host-owned validation or boot policy are
forbidden.

The probe also owns the exact `--pif-profile` spellings
`ntsc-pinned`, `pal-pinned`, and `mpal-pinned`. It transfers the parsed choice;
Machine owns profile meaning and copy layout. The host must not infer a profile
from firmware content, digest, filename, cartridge identity, platform, or any
expected trace.

For the coupled handoff, the probe additionally owns only the literal syntax
`--ipl3-family x105`, `--reset-kind cold`, `--boot-medium cartridge`, and
`--pif-version-bit 0|1`. These are separate emulated-machine inputs, not one
game profile. The host neither defaults nor derives them; Machine owns the
supported combination, complete plan, state production, lineage, and
fail-closed behavior.

Allowed future direction is thin host → public Machine/inspection surface.
Core → host, renderer → machine state, platform clock → stepping, and host-owned
emulator policy remain forbidden. User-local commercial assets stay outside
routine inspection and evidence.

Host runtime, rollback, performance, window, audio, and input evidence are
unavailable. Any future host needs an explicit product packet and separate
runtime observation. Current required validation is only
`./rust/verify-forward`.
