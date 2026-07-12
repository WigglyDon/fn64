# Profiled PIF IPL2 Copy Materialization V1

Classification: `SOURCE_BACKED_PROFILED_COPY_MATERIALIZATION_SYNTHETIC_ONLY`.

`USER_DECISION`: fn64 accepts one explicit 1,984-byte raw-PIF-shaped input
without selecting a profile. It may materialize one exact pinned IPL1 copy
effect only when that accepted input is also paired with an explicit
`NTSC_PINNED`, `PAL_PINNED`, or `MPAL_PINNED` profile.

`LIVE_REPO_FACT`: `Machine::stage_cartridge_bootstrap` now constructs the
profile-selected SP IMEM range from Machine-owned PIF bytes, attaches a source
offset to every known byte, and leaves every byte outside that range Unknown.
Accepted firmware without a profile remains Machine-owned and non-materializing;
profile without firmware is representable but bootstrap rejects before
mutation. No default, inference, or hidden search exists.

This lane proves the effect with generated patterns only. It does not prove
firmware authenticity, IPL1 or IPL2 execution, complete pre-IPL3 handoff state,
BOOT-3, cartridge entry, game compatibility, timing, PIF devices, DMA, or a
general bus. Those facts remain `UNKNOWN` or explicitly unearned.

Related evidence: [profile and lifecycle](PROFILE_AND_LIFECYCLE.md), [CLI and
authority](CLI_AND_AUTHORITY.md), [synthetic coverage](SYNTHETIC_TEST_COVERAGE.md),
[source anchors](SOURCE_ANCHORS.md), [legal boundary](LEGAL_BOUNDARY.md), and
[validation](VALIDATION.md).
