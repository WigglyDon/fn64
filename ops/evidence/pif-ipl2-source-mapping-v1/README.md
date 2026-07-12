# PIF IPL2 source mapping V1

## Purpose and authority

This evidence lane maps the accepted 1,984-byte raw PIF Boot ROM shape to the
IPL2 bytes retained in SP IMEM for the currently examined x105 entry. It owns
source anchors, arithmetic, variant qualification, causal analysis, and a
bounded product recommendation. It does not own Rust product code, Machine
behavior, firmware loading or execution, private inputs, integration, or
compatibility policy.

Exact result classification:
`VARIANT_SPECIFIC_MAPPING_REQUIRES_EXPLICIT_MACHINE_PROFILE`.

Concise finding: three pinned matching reconstructions have a common raw source
start of `0x0D4`, but their end differs by region. NTSC copies
`[0x0D4, 0x71C)` (1,608 bytes) to SP IMEM `[0x000, 0x648)`. PAL
and MPAL copy `[0x0D4, 0x720)` (1,612 bytes) to SP IMEM
`[0x000, 0x64C)`. Both mappings fully cover the currently consumed
`[0x000, 0x020)` range and the currently required mutation range through
`[0x000, 0x02C)`. The accepted raw shape alone does not identify one mapping.

## Primary mission answers

1. Raw source offset corresponding to SP IMEM zero: `0x0D4` for all three
   pinned regional reconstructions; `UNKNOWN` for unexamined revisions.
2. IPL2 copy end exclusive: NTSC `0x71C`; PAL and MPAL `0x720`; other
   revisions `UNKNOWN`.
3. Copy length: NTSC 1,608 bytes (`0x648`); PAL and MPAL 1,612 bytes
   (`0x64C`).
4. SP IMEM destination: NTSC `[0x000, 0x648)`; PAL and MPAL
   `[0x000, 0x64C)`.
5. Copy event: the CPU executing the IPL1 aligned copy loop.
6. Boot stage: IPL1, after the RSP DMA-idle wait and before transfer to IPL2 at
   `0xA4001000`.
7. Lifecycle: yes, the copied low range survives unchanged until IPL3 begins
   in the three pinned builds; unexamined revisions remain `UNKNOWN`.
8. Coverage of `[0x000, 0x020)`: fully proven for all three pinned mappings.
9. Coverage of `[0x000, 0x02C)`: fully proven for all three pinned mappings.
10. Variation: the mapping differs by region in the pinned builds; physical
    PIF revision and other IPL-variant rules remain `UNKNOWN`.
11. Current structural validation can distinguish variants: no. It validates
    byte shape only.
12. Source-backed materialization can reproduce the selected IPL1 copy effect,
    but copy alone cannot reproduce the complete pre-IPL3 machine state.
13. A complete shortcut consisting only of the copy would omit relevant
    dynamic effects, including x105 inputs `t3` and `ra`; a copy-only lane is
    honest only if later unavailable state remains fail-closed.
14. Minimal IPL1/IPL2 execution required: not proven. Public implementations
    demonstrate both HLE materialization and supplied-byte execution, and the
    evidence does not make execution uniquely necessary.
15. Governing result:
    `VARIANT_SPECIFIC_MAPPING_REQUIRES_EXPLICIT_MACHINE_PROFILE`.

## Accepted starting truth

- `LIVE_REPO_FACT`: Rust is the sole current implementation, and `Machine`
  owns emulated truth.
- `LIVE_REPO_FACT`: exactly 1,984 bytes (`0x7C0`) are structurally accepted as
  the raw PIF Boot ROM shape; 2,048 bytes are recognized as the full address-map
  shape and rejected; other lengths are malformed.
- `LIVE_REPO_FACT`: structural acceptance proves no authenticity, revision,
  region, compatibility, or executability.
- `LIVE_REPO_FACT`: accepted bytes survive reset but currently produce no SP
  IMEM content and execute nothing.
- `LIVE_REPO_FACT`: IPL1 places IPL2 in SP IMEM, IPL2 stages IPL3 in SP DMEM,
  and CPU control enters IPL3 at `0xA4000040`.
- `LIVE_REPO_FACT`: the observed x105 path consumes retained IPL2 bytes in
  `[0x000, 0x020)` and has a mutation input/output frontier through
  `[0x000, 0x02C)`.
- `USER_DECISION`: no private PIF or cartridge input is authorized, and no
  firmware content may be copied, reconstructed, hashed, or packaged.

## Repository reconciliation

Relevant ownership rules were revalidated from current source and context:
host code may parse an explicit path and read only that path; `Machine` owns
accepted bytes, SP IMEM, reset, policy, and represented stepping; inspection
may report but not invent state; `Machine::step` remains the public execution
entrance. No product source currently maps accepted PIF bytes into SP IMEM.

| Earlier statement | Revalidation result |
| --- | --- |
| The prior SP-IMEM provenance report described IPL1 as the producer and recorded eight aligned retained-word reads in `[0x000, 0x020)` with mutation through `[0x000, 0x02C)`. | The existence and exact wording of that report are `LIVE_REPO_FACT`; current lane/context pages retain those ranges, and the pinned x105 source independently corroborates the read/write causality. The earlier report's missing raw-source mapping is superseded only by this lane's qualified external evidence. |
| The prior user-supplied-PIF report said 1,984-byte acceptance was shape-only, survived reset, and produced no SP IMEM. | Revalidated as `LIVE_REPO_FACT` in current `pif_firmware.rs`, Machine bootstrap/reset behavior, boot-probe output, and tests. |
| Earlier evidence cited public reverse-engineering and emulator behavior. | Those statements began as `WORKER_CLAIM`; the exact revisions and anchors in this lane were independently reopened and are now recorded as `EXTERNAL_TECHNICAL_EVIDENCE`. |
| A convenient isolated retained word would be sufficient. | Contradicted by accepted `[0x000, 0x020)` consumption and `[0x000, 0x02C)` mutation coverage, and rejected. |

No accepted repository law contradicts the exact pinned regional arithmetic.
The unresolved ambiguity is hardware revision scope and profile selection, not
the three pinned build ranges.

## Evidence language

The lane uses the required labels literally: `USER_DECISION` for supplied
authority, `LIVE_REPO_FACT` for revalidated repository state, `RUNTIME_FACT`
for exact command results, `WORKER_CLAIM` for earlier unvalidated reports,
`EXTERNAL_TECHNICAL_EVIDENCE` for public sources, `INFERENCE` only when its
supporting facts are named, and `UNKNOWN` instead of a guess. External evidence
does not grant product authority, and one emulator is corroboration rather than
specification.

## File index

- [SOURCE_MAPPING.tsv](SOURCE_MAPPING.tsv): exact regional mappings and
  arithmetic-qualified limitations.
- [VARIANT_MATRIX.tsv](VARIANT_MATRIX.tsv): examined region and revision scope.
- [IPL1_COPY_CAUSALITY.md](IPL1_COPY_CAUSALITY.md): copy event, addresses, and
  lifecycle.
- [IPL2_EXECUTION_EFFECTS.md](IPL2_EXECUTION_EFFECTS.md): dynamic effects that a
  byte copy does not represent.
- [REQUIRED_PRE_IPL3_STATE.md](REQUIRED_PRE_IPL3_STATE.md): current range and
  state requirements.
- [MATERIALIZATION_VS_EXECUTION.md](MATERIALIZATION_VS_EXECUTION.md): bounded
  classification and alternatives.
- [EXTERNAL_SOURCE_REGISTER.tsv](EXTERNAL_SOURCE_REGISTER.tsv): pinned public
  source inventory.
- [COPYRIGHT_BOUNDARY.md](COPYRIGHT_BOUNDARY.md): no-copy and private-input
  audit.
- [PRODUCT_RECOMMENDATION.md](PRODUCT_RECOMMENDATION.md): smallest later lane.
- [SOURCE_ANCHORS.md](SOURCE_ANCHORS.md): claim-to-anchor map.
- [VALIDATION.md](VALIDATION.md): repository and artifact validation record.

## Boundary and future work

No private PIF search, read, or hash occurred. No private cartridge ROM was
read. No proprietary byte or word sequence, copied source, assembly,
disassembly, or external source tree entered Git. The lane retains only public
anchors and paraphrased technical findings.

A later Master-owned product lane may add an explicit Machine profile and
materialize the selected IPL1 copy effect from user-supplied bytes. That future
work must remain fail-closed for unselected or unsupported variants and must not
describe materialization as firmware execution. It must not infer a profile
from content, filename, region heuristics, or a compatibility database.

This lane does not earn BOOT-3, cartridge compatibility, PIF authenticity,
firmware execution, or support for any commercial title.
