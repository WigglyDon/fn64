# Project History

Context role: canonical architectural history.
Scope: major fn64 eras and direction changes.
Canonical for: chronology, superseded directions, and surviving invariants.
Not canonical for: current state or complete per-instruction provenance.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](CURRENT_STATE.md).
Related evidence: Git history, [boot checkpoint](../boot_spine_checkpoint.md), and [rust/PARITY.md](../../rust/PARITY.md).
Update triggers: a major architectural era begins, ends, or is reinterpreted by stronger evidence.

## Era 1 — repository skeleton (2026-03-16)

- Evidence: `LIVE_REPO_FACT`, commits `e425b35` and `88fae0a`.
- Prior direction: minimal repository identity and Fedora-oriented bootstrap.
- Revised direction: a runnable C++ machine/host path grew from the skeleton.
- Surviving law: Fedora is a workbench, not emulator identity.
- Remaining debt: architecture and proof ownership were not yet explicit.
- Status: superseded foundation.

## Era 2 — C++ machine and host growth (2026-03-16 through 2026-04-14)

- Evidence: `LIVE_REPO_FACT`, commits `4788b77` through `24fe880` and retained `src/` history.
- Direction: C++ owned ROM loading, machine execution, proof/bootstrap code, CLI plumbing, and SDL presentation.
- Trigger for revision: commit/source evidence shows monolith pressure and later boundary work; product meaning cannot be inferred from terse commit subjects alone.
- Surviving law: direct control, one small machine, and visible host plumbing.
- Remaining debt: product, proof, and host truth were coupled.
- Status: donor/reference.

## Era 3 — C++ boundary and no-window proof (2026-06-17 through 2026-07-04)

- Evidence: `LIVE_REPO_FACT`, commits `36dbee9` through `e66ad6e`.
- Revised direction: separate core, proof, CLI inspection/step probe, and SDL host; narrow public execution to `Machine::step`; harden no-ghost behavior; add explicit boot-adjacent boundaries.
- Trigger evidence: source and commits name core/proof/host separation, no-SDL inspection, step-only execution, and the boot-spine checkpoint.
- Surviving law: proof is an instrument, runtime smoke is not boot, machine truth stays machine-owned.
- Remaining debt at the time: C++ remained both historical implementation and comparison source.
- Status: retired source; preserved in Git history.

## Era 4 — Rust parallel machine construction (chronology partially unavailable)

- Evidence: `LIVE_REPO_FACT`, the complete Rust tree first enters Git in commit `8034b50`; older revisions of `rust/PARITY.md` in Git describe the preceding incremental passes as `WORKER_CLAIM` records.
- Direction: reconstruct represented machine ownership in Rust through small source-clear seams, culminating in public represented `Machine::step` and two no-window probes.
- Trigger evidence: the adopted source and tests prove the resulting ownership; Git does not preserve individual pre-adoption Rust seam commits.
- Surviving law: one owner per mutation, no generic future dispatcher, no host policy in the core.
- Remaining debt: historical per-seam authorship and timing before adoption are `UNKNOWN` in repository history.
- Status: superseded development method; resulting tree is current product truth.

## Era 5 — tracked Rust product and forward gate (2026-07-09)

- Evidence: `LIVE_REPO_FACT`, commits `8034b50` and `df0551f`.
- Revised direction: track the Rust workspace as product truth; make `rust/verify-forward` the default required lane; retain C++ checks only as optional frozen reference.
- Surviving law: deterministic no-window verification and narrow capability claims.
- Remaining debt at the time: C++ remained physically present.
- Status: current Rust product foundation; its transitional C++ retention policy is superseded by Era 7.

## Era 6 — repository-native project context and fleet infrastructure (2026-07-10)

- Evidence: `USER_DECISION`, packet `fn64-master-infrastructure-v1-2026-07-10-001`.
- Revised direction: suspend seam-090 deletion, make authority/context/packet/evidence state discoverable in the repository, and add small non-destructive fleet instruments.
- Surviving law: infrastructure must reduce cognitive load and must not become a second product.
- Remaining debt: context and tools require continued rent-paying review; no product feature is authorized by this era.
- Status: current infrastructure pass.

## Era 7 — direct C++ lane retirement (2026-07-10)

- Evidence: `USER_DECISION`, packet `fn64-master-remove-frozen-cpp-lane-v1-2026-07-10-001`, plus the resulting tracked-tree deletion.
- Prior direction: keep C++ as optional frozen reference until another inventory/deletion-readiness pass completed.
- Trigger: the product chose the more fundamental end state—one current implementation and an honestly smaller Rust machine—without requiring semantic parity.
- Revised direction: remove C++ machine, host, proof, CMake, and C++-only launch source from the current tree; use Git history as the only archive.
- Surviving law: one owner per fact, machine/host separation, explicit absence, lawful synthetic proof, and no compatibility claim from green gates.
- Accepted loss: unported C++ instruction, host, SDL, cartridge/bootstrap, device, CLI, and proof behavior is intentionally absent rather than migrated.
- Remaining debt: Rust is incomplete and headless; future behavior must be earned independently under current architecture.
- Status: current source-ownership era.

The retirement decision intentionally waived semantic comparison and inventory
prerequisites. Git history is the only retired-source archive. In particular,
unported instruction families, cartridge/bootstrap experiments, device and DMA
behavior, CLI output, window/event-loop plumbing, and proof coverage are absent
from the current product. Green Rust tests do not imply that those historical
behaviors were carried forward. Restoring the retired implementation as a
product or reference lane requires a new explicit product decision.

## Era 8 — Rust-purity consolidation and authentic BOOT-2 (2026-07-10)

- Evidence: `USER_DECISION`, packet
  `fn64-master-integrate-boot2-and-rust-purity-v1-2026-07-10-001`;
  `LIVE_REPO_FACT`, worker commits `6f189716`, `8e5efc8e`, and `9cc16142`;
  `RUNTIME_FACT`, the bounded private no-window BOOT-2 trace.
- Prior direction: current capability remained buried inside a transition
  transcript while no authentic cartridge-derived instruction had a complete
  represented effect.
- Revised direction: consolidate current capability under one ledger, preserve
  C++ chronology only in history/decisions/Git, and accept one authentic
  cartridge-derived `SpecialAdd` commit as BOOT-2.
- Repair lesson: concrete zero storage is not architectural knownness. The
  first instruction became complete only after r29 and r0 gained source-backed
  known state, all other unstaged PIF GPRs stayed unknown, and rejection was
  proved pre-mutation.
- Exact boundary: BOOT-3 was not reached. The next frontier is `Lw` at
  `0xA4000044` targeting CPU address `0xA4001000`; SP IMEM storage/routing and
  complete aligned-load semantics remain absent.
- Surviving law: Machine owns bytes and state; the inspection shell owns paths
  and reporting; no game-specific patch, direct-entry bypass, generic bus, or
  compatibility claim was introduced.
- Status: current Rust product/capability era.

## Era 9 — SP IMEM and aligned-Lw partial frontier (2026-07-11)

- Evidence: `USER_DECISION`, packet
  `fn64-master-integrate-sp-imem-lw-partial-and-provision-wave-2-2026-07-11-001`;
  `LIVE_REPO_FACT`, worker commit `dcb9f1bf`; `RUNTIME_FACT`, the repeated
  bounded private no-window trace.
- Prior boundary: the authentic `Lw` frontier lacked SP IMEM representation,
  data routing, and aligned-load semantics.
- Accepted increment: each Machine owns 4 KiB of SP IMEM with separate backing
  bytes and knownness, and one complete aligned `Lw` rule covers direct RDRAM
  plus known SP IMEM with exception, lineage, cadence, and rollback proof.
- Honest partial result: the authentic `Lw` still does not commit. Offset zero
  remains unknown, so BOOT-2 remains the highest checkpoint.
- Next source question: the Machine-owned creation event for SP IMEM bytes
  `0x000..0x003` is `UNKNOWN`; history does not relabel it as reset, PIF, DMA,
  transfer, or firmware behavior without evidence.
- Surviving law: concrete storage value is distinct from represented
  architectural knowledge, and a truthful partial machine increment may land
  without a higher boot claim.
- Status: accepted current capability; the provenance question is resolved by
  Era 10 without a product-behavior change.

## Era 10 — retained IPL2 provenance boundary (2026-07-11)

- Evidence: `LIVE_REPO_FACT`, evidence-only worker commit `8db1b57c`;
  `INFERENCE`, pinned source reconstruction plus independent corroboration;
  `USER_DECISION`, explicit user-supplied PIF input authority.
- Causal finding: IPL1 places proprietary IPL2 content in SP IMEM, CPU control
  enters IPL2 there, and IPL2 stages cartridge IPL3 before entering it at
  `0xA4000040`. The observed x105 prelude consumes retained SP IMEM
  `[0x000, 0x020)` and initially mutates `[0x000, 0x02c)`.
- Product boundary: external observability does not authorize embedding code,
  constants, tables, or a firmware-derived profile. No Machine behavior or
  checkpoint changed; current bytes remain `Unknown` and BOOT-2 remains highest.
- Revised direction: fn64 may accept an explicit user-supplied PIF firmware
  file. Host authority ends at path/read/owned-byte transfer; the Machine owns
  validation, lifecycle, supported/unsupported state, SP IMEM production, and
  provenance.
- Remaining decision: source inspection must choose between source-backed state
  materialization, minimal firmware execution, an input-boundary partial, or an
  evidence-only partial. Broad PIF architecture is not preselected.
- Status: evidence boundary accepted; product implementation follows in a
  separately provisioned lane.

## Era 11 — explicit user-supplied PIF input boundary (2026-07-11)

- Evidence: `LIVE_REPO_FACT`, Worker commits `7d0fd689`, `63aea796`, and
  `1fa8aa17`; `USER_DECISION`, accepted classification
  `INPUT_BOUNDARY_ONLY_PRODUCT`; `RUNTIME_FACT`, focused and complete Rust gates
  plus the unchanged private no-firmware BOOT-2 regression.
- Accepted increment: the existing no-window boot probe accepts one optional
  literal `--pif-rom` path, performs no search or fallback, and transfers owned
  bytes into Machine-owned structural validation.
- Machine boundary: one 1,984-byte raw-Boot-ROM-shaped candidate is accepted
  structurally, a 2,048-byte full-map shape is explicitly unsupported, and
  other lengths are malformed. Accepted immutable bytes persist across reset
  and repeated bootstrap; rejected replacement has no partial mutation.
- Honest limitation: structural shape is not authenticity, firmware revision,
  executability, or compatibility. Accepted bytes execute nothing, produce no
  SP IMEM state, and do not advance the authentic trace beyond BOOT-2.
- Next evidence pressure: identify the exact numeric and variant-qualified
  mapping from accepted input bytes to retained IPL2 content before selecting
  source-backed materialization or minimal execution.
- Status: accepted current input-boundary product capability.

## Era 12 — ordinary control flow and profile-specific IPL2 mapping (2026-07-12)

- Evidence: `LIVE_REPO_FACT`, control-flow commits `60cfc832`, `e46816b9`, and
  `01b06e5a`; `RUNTIME_FACT`, focused tests, direct step probe, and full Rust
  gate; `EXTERNAL_TECHNICAL_EVIDENCE`, mapping commit `2ee4b3c7` and pinned
  public-source anchors.
- Accepted product increment: `BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR`
  execute through `Machine::step` with one CPU-owned delay slot, explicit link
  and Count rules, complete branch-in-slot rollback, and EPC/BD exception
  lineage. A region-crossing shared-helper test proves J/JAL use PC+4 high
  bits.
- Accepted evidence increment: pinned NTSC raw `[0x0d4,0x71c)` maps to SP
  IMEM `[0x000,0x648)`; pinned PAL and MPAL raw `[0x0d4,0x720)` map to
  `[0x000,0x64c)`. Structural input shape cannot select a profile, and
  unexamined physical PIF revisions remain unknown.
- Honest limitation: accepted firmware still produces no SP IMEM state. A
  profile-specific copy is only one IPL1 effect and does not establish the
  complete IPL2 handoff; minimal execution remains unearned. BOOT-2 is
  unchanged.
- Status: both independent Wave 3 candidates accepted and integrated without
  rewriting Worker history.

## Era 13 — explicit profiled IPL2 copy materialization (2026-07-12)

- Evidence: `LIVE_REPO_FACT`, eight Worker commits from `0a095487` through
  `a2a8ca51`; `RUNTIME_FACT`, focused tests, patch-tree reproduction, artifact
  verification, clean-checkout proof, and the complete Rust gate;
  `USER_DECISION`, accepted classification `PROFILED COPY MATERIALIZATION
  PRODUCT`.
- Accepted product increment: inspection owns exact `--pif-profile` spellings,
  while Machine owns distinct NTSC/PAL/MPAL meanings and atomically copies the
  complete selected firmware slice at `stage_cartridge_bootstrap`. Copied bytes
  become known with source-offset/profile provenance; untouched bytes remain
  `Unknown`.
- Lifecycle: firmware and profile installation remain independent and
  non-materializing. Both persist across reset; repeated bootstrap constructs a
  replacement image, including stale PAL/MPAL-tail clearing when NTSC is later
  selected.
- Proof boundary: generated bytes prove exact ranges and one source-backed
  `Machine::step` `Lw`. No private PIF or cartridge ROM was read, no PIF
  execution or complete handoff was represented, and BOOT-2 remains highest.
- Workflow record: the raw report sealed in the artifact passed line-integrity
  checks, while the chat-delivered packet was terminal-wrapped and
  noncompliant. This is a `WORKFLOW_DELIVERY_DEFECT`, not a product semantic
  defect.
- Handoff continuation: evidence candidate `c24ab78c` remains unaccepted. Its
  first focused repair must be retried after quota/context recovery and must
  profile-qualify retained IPL2 r31/ra.
- Status: profiled-copy lane accepted and integrated; handoff evidence lane
  remains active and blocked for `FOCUSED_REPAIR_1_RETRY`.

## Era 14 — direct Master workflow and fleet-operation retirement (2026-07-13)

- `USER_DECISION`: the active loop is now Don -> Master GPT -> Master Codex ->
  Master GPT. Supervisor GPT and Worker Codex lane roles are retired; Master
  Codex directly owns bounded implementation, validation, integration,
  authorized normal push, evidence, and reporting.
- Preserved history: prior branches, worktrees, commits, pages, and artifacts
  remain intact. Fleet tools remain dormant diagnostics rather than active
  delivery architecture.
- Unaccepted donor: `pif-ipl2-handoff-state-mapping-v1` candidate `c24ab78c`
  and context merge `96840e99` remain historical only. Their stale artifact is
  not accepted, and the known profile-qualified r31/ra defect remains explicit.
- Coordination state: no active supervisor/Worker lane and no queued Worker
  candidate remain.

## Era 15 — coupled cold x105 handoff materialization (2026-07-13)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned N64-IPL reconstruction,
  R4300 documentation, and independent emulator corroboration;
  `LIVE_REPO_FACT`, direct Master evidence and product commits;
  `RUNTIME_FACT`, generated-only focused tests and the complete Rust gate.
- Donor handling: historical candidate `c24ab78c` supplied hypotheses and
  source leads only. Independent review retained its useful t3/sp/s3-s7
  direction but rejected its universal ra claim.
- Accepted product increment: one explicit `NTSC_PINNED`, x105, cold,
  cartridge path stages the bounded inherited GPRs, Status, PC/next-PC, and
  completed-transfer lineage through one Machine-owned plan and atomic
  bootstrap application. The PIF-version bit is a separate explicit input.
- Fail-closed boundary: PAL_PINNED and MPAL_PINNED retained-link arithmetic is
  not independently corroborated for product use, so coupled handoff requests
  for those profiles reject without partial CPU or memory mutation.
- Proof boundary: all bytes and instructions were generated. No private PIF or
  cartridge ROM was accessed; no firmware or IPL execution, authentic
  advancement, BOOT-3, or compatibility fact was earned. BOOT-2 remains the
  highest authentic checkpoint.

## Era 16 — generated x105 SP-DMEM load frontier (2026-07-13)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned x105 identity order and
  architectural load rules; `LIVE_REPO_FACT`, direct Master commits and narrow
  target/provenance ownership; `RUNTIME_FACT`, generated focused tests, direct
  step probe, and complete Rust gate.
- Frontier correction: aligned `Sw` was not the first missing capability. The
  earlier `Lw` at the generated third semantic position already decoded and
  identified, but lacked a data route to cartridge-staged SP DMEM.
- Accepted product increment: the existing aligned-`Lw` planner now routes
  direct SP-DMEM words only when the current bootstrap span supplies exact
  cartridge provenance. Unclassified concrete backing rejects before mutation;
  AdEL, delay-slot EPC/BD, cadence, and GPR lineage retain their existing owners.
- Generated composition: four public `Machine::step` commits reach
  PC/next-PC `0xA4000050 / 0xA4000054`, Count `4`, then stop atomically at
  aligned `Sw`, the next unrepresented frontier.
- Proof boundary: no private input, copied instruction stream, store family,
  bus, or generalized memory map was added. BOOT-2 remains the highest
  authentic checkpoint.

## Era 17 — aligned SP-IMEM Sw and generated store frontier (2026-07-13)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, bounded x105 identity/operand order
  and primary VR4300 store rules; `LIVE_REPO_FACT`, direct Master product and
  inspection commits; `RUNTIME_FACT`, generated focused tests, expanded step
  probe, clean checkout, and complete Rust gates.
- Accepted product increment: `Sw` stores old `rt` low 32 bits to aligned SP
  IMEM only, using direct aliases, four-byte big-endian mutation, exact
  instruction/source provenance, read-before-write aliasing, and one committed
  cadence. Unaligned access enters AdES code 5 through existing COP0 ownership.
- Fail-closed boundary: unknown operands, RDRAM, SP DMEM, non-direct forms,
  target misses, bounds failures, and blocked exception entry preserve state.
  No other store identity, generic store path, bus, or map was added.
- Generated composition: thirteen public `Machine::step` commits reach
  PC/next-PC `0xA4000074 / 0xA4000078`, Count `13`, then stop atomically at
  recognized but unrepresented `RegimmBltz`.
- Proof boundary: all inputs and instruction fields are generated. No private
  input or authentic execution was used; BOOT-2 remains highest.

## Era 18 — BLTZ and generated x105 branch frontier (2026-07-13)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, the primary VR4300 BLTZ rule and
  pinned bounded x105 identity/operand order; `LIVE_REPO_FACT`, direct Master
  product and inspection commits; `RUNTIME_FACT`, generated focused tests,
  expanded step probe, clean checkout, and complete Rust gates.
- Signed-width decision: BLTZ reuses the existing full-GPR signed comparator
  already owned by SLT/SLTI. Discriminating positive-low-word-negative and
  negative-low-word-zero values prevent accidental 32-bit interpretation. No
  CPU-mode framework was added.
- Accepted product increment: only non-linking/non-likely `RegimmBltz` joins
  ordinary control-flow planning/application. Known source, signed condition,
  branch target, untaken successor, one slot, Count, nested-control-flow
  rejection, and slot exception EPC/BD are exact. Every other REGIMM identity
  remains unrepresented.
- Generated composition: BLTZ commits as step 14 and its aligned r0 `Sw` slot
  writes SP IMEM local `0x00C` as step 15. PC/next-PC become
  `0xA400007C / 0xA4000080`, Count is `15`, and `Cop0Mtc0` to Cause is the next
  explicit unrepresented frontier.
- Proof boundary: all inputs and instruction fields are generated. No private
  input, COP0 execution, authentic execution, BOOT-3, or compatibility fact was
  added. BOOT-2 remains highest.

## Era 19 — bounded MTC0 boot trio and RI frontier (2026-07-14)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, primary VR4300 MTC0 and
  Cause/Count/Compare rules plus pinned bounded identity order;
  `LIVE_REPO_FACT`, direct Master implementation and inspection commits;
  `RUNTIME_FACT`, generated focused tests, step probe, clean checkout, and
  complete Rust gates.
- Accepted product increment: `Cop0Mtc0` transfers a known GPR low word only
  to Cause IP1/IP0, Count, or Compare on the source-backed cold-x105 path.
  Cause preserves read-only and timer state; Count writes before cadence;
  Compare clears timer pending before cadence and equality may relatch it.
- Generated composition: the trio commits as steps 16-18, represented address
  construction commits as step 19, and the RI_SELECT `Lw` at represented CPU
  address `0xA470000C` (effective GPR address `0xFFFFFFFFA470000C`) rejects as
  a direct target miss. Final PC/next-PC are
  `0xA400008C / 0xA4000090` and Count is `3`.
- Workflow audit: the accepted BLTZ report's branch name was a report-only
  `MASTER_BRANCH_LABEL_OR_TOPOLOGY_DRIFT`; its worktree was registered to the
  assigned branch, and no historical topology or accepted product was changed.
- Proof boundary: all inputs and instruction fields are generated. RI,
  interrupt delivery, general COP0, authentic execution, BOOT-3, and
  compatibility remain absent. BOOT-2 remains highest.

## Era 20 — cold-entry RI_SELECT read and RI_CONFIG frontier (2026-07-14)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, the pinned RI register definition,
  bounded cold/NMI source relation, and official CPU-only NMI reset note;
  `LIVE_REPO_FACT`, direct Master evidence, product, and inspection commits;
  `RUNTIME_FACT`, generated focused tests and public-step proof.
- Source decision: public evidence proves RI_SELECT zero at the bounded cold
  x105 entry but does not state a generic hardware power-on reset value.
  Therefore the complete coupled bootstrap atomically creates Machine-owned
  RI_SELECT zero with `ColdX105Entry` provenance; construction and general
  reset leave it unavailable.
- Accepted product increment: exact direct KSEG0/KSEG1 aligned `Lw` aliases of
  physical `0x0470000C` read the stored RI_SELECT word without side effects.
  Neighboring RI registers, all RI writes, NMI, a register bank, MMIO, and a bus
  remain absent.
- Generated composition: RI_SELECT loads zero at step 20, BNE takes the cold
  fall-through through one NOP slot, five stores save s3-s7 at SP-IMEM locals
  `0xFD8..0xFE8`, and address/immediate construction reaches step 33. Final
  PC/next-PC are `0xA40000C4 / 0xA40000C8`, Count is `17`, and the `Sw` to
  RI_CONFIG at physical `0x04700004` rejects as a direct target miss.
- Proof boundary: all inputs and instruction fields are generated. RI_CONFIG,
  RI initialization, NMI, authentic execution, BOOT-3, and compatibility
  remain absent. BOOT-2 remains highest.

## Era 21 — RI_CONFIG write and generated current-load frontier (2026-07-14)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned RI_CONFIG field definitions
  and bounded x105 store/loop order; `LIVE_REPO_FACT`, direct Master evidence,
  product, and inspection commits; `RUNTIME_FACT`, generated focused tests,
  public-step probe, clean checkout, and complete Rust gates.
- Field decision: RI_CONFIG represents only current-control input bits 5:0 and
  enable bit 6. Undefined high bits reject before mutation as an explicit fn64
  boundary; no raw register value or hardware trap behavior is invented.
- Accepted product increment: exact direct KSEG0/KSEG1 aligned `Sw` aliases of
  physical `0x04700004` create Machine-owned RI_CONFIG field state with exact
  CPU-store lineage. Reset and repeated cold bootstrap clear stale state;
  RI_SELECT and memory remain unchanged. No RI_CONFIG read exists.
- Generated composition: commit 34 stores `0x40`, commit 35 installs wait count
  8,000, and 8,000 generated loop iterations commit 32,000 more instructions.
  Final PC/next-PC are `0xA40000DC / 0xA40000E0`, Count is `32019`, total
  commits are 32,035, and `Sw r0` to RI_CURRENT_LOAD at physical `0x04700008`
  rejects atomically.
- Proof boundary: the loop proves CPU composition, not RCP time or analog
  calibration. RI_CURRENT_LOAD, RDRAM initialization, generic MMIO, authentic
  execution, BOOT-3, and compatibility remain absent. BOOT-2 remains highest.

## Era 22 — RI_CURRENT_LOAD event and generated RI_SELECT frontier (2026-07-14)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned RI definitions naming
  RI_CURRENT_LOAD write-only and any-write update behavior plus bounded x105
  source order; `LIVE_REPO_FACT`, direct Master evidence, product, and
  inspection commits; `RUNTIME_FACT`, generated focused tests and public-step
  proof.
- Source decision: `RI_CURRENT_LOAD_UPDATE_EVENT_REPRESENTABLE`. The event
  records which stored RI_CONFIG input/enable fields the known CPU store
  consumed, plus low-word store evidence and source lineage. It does not invent
  a current-control output or calibration result.
- Accepted product increment: exact direct KSEG0/KSEG1 aligned `Sw` aliases of
  physical `0x04700008` create the Machine-owned event only when RI_CONFIG is
  available. Reset and repeated cold bootstrap clear stale state; RI_SELECT,
  RI_CONFIG, and all memory remain otherwise unchanged.
- Generated composition: commit 32,036 stores r0 to RI_CURRENT_LOAD and
  snapshots input zero/enable true. Commit 32,037 executes `Ori r9,r0,0x14`.
  Final PC/next-PC are `0xA40000E4 / 0xA40000E8`, Count is `32021`, and the
  next `Sw` to RI_SELECT at physical `0x0470000C` rejects atomically.
- Proof boundary: no current-control output, hardware timing, RDRAM readiness,
  RI_SELECT write, RI_MODE, authentic execution, BOOT-3, or compatibility is
  represented. BOOT-2 remains highest.

## Era 23 — exact RI_SELECT write and generated RI_MODE frontier (2026-07-14)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned R/W RI_SELECT address and
  bounded x105 value/order; `LIVE_REPO_FACT`, direct Master evidence, product,
  and inspection commits; `RUNTIME_FACT`, generated focused tests and public
  step proof.
- Write decision: `RI_SELECT_EXACT_X105_VALUE_ONLY`. The public header's
  duplicated receive/transmit bit range is ambiguous, while the bounded source
  directly establishes word `0x14` and its enable-TX/RX-select purpose. Other
  words reject as fn64's unsupported boundary, not a hardware-trap claim.
- Accepted product increment: exact direct KSEG0/KSEG1 aligned `Sw` aliases of
  physical `0x0470000C` replace RI_SELECT value/source with `0x14` and exact
  CPU-store lineage. The existing `Lw` reads the updated state. Reset and
  repeated cold bootstrap clear stale CPU provenance; RI_CONFIG,
  RI_CURRENT_LOAD, and memory remain unchanged.
- Generated composition: commit 32,038 stores `0x14` to RI_SELECT. Final
  PC/next-PC are `0xA40000E8 / 0xA40000EC`, Count is `32022`, and the next
  `Sw r0` to RI_MODE at physical `0x04700000` rejects atomically.
- Proof boundary: no general RI_SELECT fields, RI_MODE, hardware timing,
  RDRAM initialization, authentic execution, BOOT-3, or compatibility is
  represented. BOOT-2 remains highest.

## Era 24 — RI_MODE fields, bounded waits, and MI init frontier (2026-07-15)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned RI_MODE field definitions
  and bounded x105 instruction/order comments; `LIVE_REPO_FACT`, direct Master
  evidence, product, and inspection commits; `RUNTIME_FACT`, focused core tests
  and the public-step composition.
- Field decision: `RI_MODE_DEFINED_FIELDS_REPRESENTABLE`. Bits 1:0 are stored
  as numeric operating-mode bits, bit 2 as stop-transmit-active, and bit 3 as
  stop-receive-active. Undefined high bits reject as fn64's unsupported
  boundary; operating-mode values 1 and 3 receive no undocumented names.
- Accepted product increment: exact KSEG0/KSEG1 aligned `Sw` aliases of physical
  `0x04700000` create or replace RI_MODE fields and CPU-store provenance. Reset
  and repeated cold bootstrap clear stale mode state; failed bootstrap and
  every rejection preserve all Ri siblings and Machine state.
- Generated composition: commit 32,039 stores zero. The first wait executes
  four NOP/Addi/Bne/NOP iterations. Commit 32,057 constructs `0x0E`, commit
  32,058 replaces RI_MODE, and the second wait executes 32 Addi/Bne/Ori
  iterations with ORI in every BNE delay slot. At 32,155 commits, PC/next-PC
  are `0xA4000118 / 0xA400011C`, Count is `32139`, s1 is zero, and r9 is
  `0x10F`; the MI_INIT_MODE store rejects atomically.
- Proof boundary: the waits prove CPU composition only. RI electrical effects,
  hardware time, MI, RDRAM initialization/readiness, authentic execution,
  BOOT-3, and compatibility remain absent. BOOT-2 remains highest.

## Era 25 — exact MI initialization state and RDRAM delay frontier (2026-07-15)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned MI_INIT_MODE command/read
  definitions and bounded x105 source order; `LIVE_REPO_FACT`, the exact aligned
  `Sw` planner/application and one private per-Machine `Mi` owner; `RUNTIME_FACT`,
  deterministic generated public-step proof.
- Product decision: `MI_INIT_MODE_EXACT_X105_WRITE_ONLY`. Exact low word
  `0x0000010F` at physical `0x04300000` stores initialization length 15 and
  initialization mode true with CPU-store provenance. Other words reject
  atomically; no MI read or general command surface exists.
- Lifecycle: construction, reset, and complete cold bootstrap leave the MI fact
  unavailable. Repeated complete bootstrap clears stale state/provenance, failed
  bootstrap preserves them, and Machines remain independent.
- Generated composition: commit 32,156 performs the MI store at
  `0xA4000118`; generated `Lui`/`Ori` construct `0x18082838`. At 32,158 commits,
  PC/next-PC are `0xA4000124`/`0xA4000128`, Count is 32,142, and the global
  RDRAM_DELAY store to CPU `0xA3F80008` rejects atomically.
- Boundary: MI next-write replication, other MI state, RDRAM control-register
  access, RDRAM initialization/readiness, hardware timing, authentic
  advancement, BOOT-3, and game compatibility remain absent.

## Era 26 — bounded MI transfer and global RDRAM delay fact (2026-07-15)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned RDRAM/global definitions and
  bounded x105 next-write relationship; `LIVE_REPO_FACT`, one pending transfer
  in the existing `Mi` owner and one delay fact in the existing `Rdram` owner;
  `RUNTIME_FACT`, deterministic public-step proof.
- Product decisions: `EXACT_X105_MI_INIT_TO_RDRAM_DELAY_PAIR_ONLY`,
  `RDRAM_DELAY_BROADCAST_CONFIGURATION_FACT_ONLY`, and
  `POST_TRANSFER_MI_READBACK_UNAVAILABLE_UNLESS_PRIMARY_SOURCE_PROVES_EXACT_STATE`.
- Accepted increment: the exact MI command arms length 15 / 16 repeated bytes.
  Only global physical `0x03F80008`, exact low word `0x18082838`, and known
  lineage consume it. The resulting broadcast fact stores logical fields
  5/7/3/1 and packed value `0x28381808` with complete CPU and MI provenance;
  post-transfer current MI state becomes unavailable.
- Generated composition: commit 32,159 performs the exact delay store at
  `0xA4000124`. PC/next-PC become `0xA4000128`/`0xA400012C`, Count is 32,143,
  and the global RDRAM_REF_ROW store at CPU `0xA3F80014` rejects atomically.
- Boundary: general MI replication, arbitrary RDRAM registers, per-module
  state, timing/readiness, authentic advancement, BOOT-3, and compatibility
  remain absent.

## Era 27 — raw global RDRAM REF_ROW fact and DEVICE_ID frontier (2026-07-15)

- Evidence: `EXTERNAL_TECHNICAL_EVIDENCE`, pinned global RDRAM definitions and
  bounded x105 zero-write order/comment; `LIVE_REPO_FACT`, one raw REF_ROW fact
  in the existing sole `Rdram` owner; `RUNTIME_FACT`, deterministic public-step
  proof.
- Product decisions: `RDRAM_REF_ROW_EXACT_X105_ZERO_WRITE_ONLY`,
  `RDRAM_REF_ROW_GLOBAL_APERTURE_WRITE_FACT_ONLY`,
  `RDRAM_REFRESH_ENGINE_EFFECT_UNAVAILABLE`, and
  `RDRAM_DEVICE_ID_GLOBAL_WRITE_NEXT_FRONTIER_ONLY`.
- Accepted increment: exact direct physical `0x03F80014` accepts only low word
  zero with known CPU-store lineage and stores raw word/global-aperture/address
  provenance. It does not require RDRAM_DELAY as hidden authorization, preserves
  the accepted delay fact, and changes no RDRAM byte.
- Generated composition: commit 32,160 performs the REF_ROW store at
  `0xA4000128`; commit 32,161 executes `Lui r9,0x8000` and produces
  `0xFFFFFFFF80000000`. PC/next-PC are `0xA4000130`/`0xA4000134`, Count is
  32,145, and the global RDRAM_DEVICE_ID store at CPU `0xA3F80004` rejects
  atomically.
- Boundary: REF_ROW fields, refresh-engine behavior, physical broadcast
  completion, DEVICE_ID behavior, per-module state, timing/readiness, authentic
  advancement, BOOT-3, and compatibility remain absent.

## Unresolved history

The stale local donor clone preserves an earlier two-commit repository shape but
does not establish accepted product authority. The unrecovered prior C++
inventory never became a deletion prerequisite. Private chat chronology and
unavailable earlier bundles are not reconstructed as live truth.
