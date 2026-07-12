# Profiled PIF IPL2 Copy Materialization V1

Classification: `SOURCE_BACKED_PROFILED_COPY_MATERIALIZATION_SYNTHETIC_ONLY`.

`USER_DECISION`: fn64 accepts one explicit 1,984-byte raw-PIF-shaped input
without selecting a profile. It may materialize one exact pinned IPL1 copy
effect only when that accepted input is also paired with an explicit
`NTSC_PINNED`, `PAL_PINNED`, or `MPAL_PINNED` profile. No private PIF or
private cartridge authority is granted. Root `AGENTS.md` alone owns the
copy-paste packet transport law.

`EXTERNAL_TECHNICAL_EVIDENCE`: accepted dependency `2ee4b3c7` establishes raw
source `[0x0d4,0x71c)` for pinned NTSC and `[0x0d4,0x720)` for pinned PAL and
MPAL, all to SP IMEM offset zero. It does not generalize to other revisions.

`LIVE_REPO_FACT`: `Machine::stage_cartridge_bootstrap` now constructs the
profile-selected SP IMEM range from Machine-owned PIF bytes, attaches a source
offset to every known byte, and leaves every byte outside that range Unknown.
Accepted firmware without a profile remains Machine-owned and non-materializing;
profile without firmware is representable but bootstrap rejects before
mutation. CLI spellings and their closed conversion live only in
`fn64-inspection`; profile meaning and copy ranges remain in `fn64-core`. No
default, inference, alias, or hidden search exists.

`RUNTIME_FACT`: [validation](VALIDATION.md) records exact commands executed
against product-source commit `4eaa33d9fc59182d8e69a24edb39ee3be9ff8797`.

`INFERENCE`: because the accepted external mapping supplies only numeric copy
ranges, current source performs only bounded copy planning/application, and
generated tests execute no PIF instruction, this lane proves materialization
but not IPL1 or IPL2 execution.

This lane proves the effect with generated patterns only. It does not prove
firmware authenticity, IPL1 or IPL2 execution, complete pre-IPL3 handoff state,
BOOT-3, cartridge entry, game compatibility, timing, PIF devices, DMA, or a
general bus. Those facts, along with unexamined physical PIF revisions, remain
`UNKNOWN`.

`WORKER_CLAIM`: the complete lane candidate satisfies the assigned product
contract. Acceptance remains pending independent Supervisor and Master review.

Related evidence: [profile and lifecycle](PROFILE_AND_LIFECYCLE.md), [CLI and
authority](CLI_AND_AUTHORITY.md), [synthetic coverage](SYNTHETIC_TEST_COVERAGE.md),
[source anchors](SOURCE_ANCHORS.md), [legal boundary](LEGAL_BOUNDARY.md), and
[validation](VALIDATION.md).
