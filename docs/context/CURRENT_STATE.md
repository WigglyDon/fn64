# Current Project State

Context role: canonical current-state owner.
Scope: fn64 at the accepted repository head.
Canonical for: current phase, authority, verification, lanes, blockers, and capability boundaries.
Not canonical for: detailed machine behavior, full history, or individual test evidence.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: this document.
Related evidence: [rust/PARITY.md](../../rust/PARITY.md), [project history](PROJECT_HISTORY.md), and [lane registry](../lanes/lane-registry.md).
Update triggers: accepted authority, capability, verification, lane, or retirement state changes.

## Current phase and authority

- `LIVE_REPO_FACT`: fn64 is one Git repository whose sole current product implementation is the tracked Cargo workspace under `rust/`.
- `LIVE_REPO_FACT`: no active C/C++ source, header, CMake build owner, C++ proof binary, C++ host shell, or C++-only launch script remains in the current tree.
- `USER_DECISION`: the former C++ machine, host, proof, and build lane is retired without semantic-equivalence or inventory prerequisites. Git history is its only archive; unported behavior is intentionally absent.
- `LIVE_REPO_FACT`: `./rust/verify-forward` is the sole required product gate and has no CMake, C++, ROM, SDL/window/audio, or Git dependency.
- `USER_DECISION`: the active delivery loop is Don -> Master GPT -> Master
  Codex -> Master GPT. Master GPT owns architecture, sequencing, scope, and
  interpretation; Master Codex directly owns repository inspection,
  implementation, validation, commits, canonical integration, authorized normal
  push, evidence, and reporting.
- `USER_DECISION`: Supervisor GPT and Worker Codex lane roles are retired. Their
  branches, worktrees, commits, pages, and artifacts remain preserved historical
  state and carry no current execution authority.
- `USER_DECISION`: fn64 may accept one explicitly selected user-supplied PIF
  firmware file as local runtime input. The host owns only the path, file read,
  read failure, and owned-byte transfer; the Machine must own accepted bytes,
  validation, lifecycle, state production, provenance, and rejection.
- `LIVE_REPO_FACT`: the existing no-window `fn64_boot_probe` now accepts one
  optional literal `--pif-rom` path. It performs no search or fallback; the host
  reads that path and transfers owned bytes into Machine validation.

## Forward machine truth

- `LIVE_REPO_FACT`: `fn64-core` owns the represented machine and public
  `Machine::step`; `fn64-inspection` owns deterministic no-window proof
  plumbing only.
- `RUNTIME_FACT`: BOOT-2 is the highest earned cartridge checkpoint. One
  authentic private-ROM-derived `SpecialAdd` committed through `Machine::step`
  with complete represented value, provenance, `pc` / `next_pc`, and Count
  lineage.
- `LIVE_REPO_FACT`: each Machine now owns 4 KiB of SP IMEM with explicit
  construction/reset, byte knownness independent of zero backing, and a narrow
  CPU-data route for the represented physical range. Complete aligned `Lw`
  semantics cover direct RDRAM, known SP IMEM, and cartridge-bootstrap-staged
  SP DMEM, including sign extension, alias/zero-register behavior, data AdEL,
  source lineage, cadence, and rollback. Concrete SP-DMEM backing outside the
  staged cartridge span remains explicitly unclassified and unreadable.
- `LIVE_REPO_FACT`: aligned `Sw` now executes through one Machine-owned plan
  and applicator for direct KSEG0/KSEG1 aliases of SP IMEM only. It stores old
  `rt` low 32 bits as four known big-endian bytes with instruction-PC,
  source-GPR, and source-lineage provenance. Unknown operands and every other
  target reject before mutation. Unaligned stores enter AdES code 5 through
  the existing COP0 owner, including exact BadVAddr and delay-slot EPC/BD;
  success advances Count once and faults advance it zero times.
- `RUNTIME_FACT`: the authentic trace still stops at `Lw` at `0xA4000044`.
  Known r9 produces CPU address `0xA4001000`, but SP IMEM offset zero is
  `Unknown`, so the load rejects before mutation.
- `INFERENCE`: integrated source-qualified evidence identifies the hardware
  producer chain: IPL1 copies proprietary IPL2 firmware content into SP IMEM,
  CPU control enters IPL2 there, IPL2 stages cartridge IPL3 in SP DMEM, and the
  observed x105 entry consumes `[0x000, 0x020)` and initially mutates
  `[0x000, 0x02c)`. External observability does not authorize embedding the
  values or make them current Machine truth.
- `LIVE_REPO_FACT`: represented execution remains incomplete and headless.
  BOOT-3, authentic handoff, cartridge-entry/game execution, compatibility,
  graphics, window, and audio are not claimed.
- `LIVE_REPO_FACT`: ordinary `BEQ`, `BNE`, non-linking/non-likely `BLTZ`,
  `J`, `JAL`, `JR`, and `JALR`
  execute through `Machine::step` with one CPU-owned delay-slot context.
  Taken and untaken branches both execute one slot; link, alias, Count,
  branch-in-delay-slot rejection, and delay-slot EPC/BD behavior are explicit.
  BLTZ reuses the established full-GPR signed comparison used by SLT/SLTI;
  every other REGIMM identity remains unrepresented. This is bounded ordinary
  control flow, not complete MIPS control flow.
- `LIVE_REPO_FACT`: `Cop0Mtc0` executes only for Cause software-pending bits,
  Count, and Compare while the source-backed cold-x105 kernel state is active.
  It transfers the known source GPR's low word, preserves the source, and
  rejects malformed encodings, other destinations, unavailable sources, or
  other access contexts before mutation. Cause writes only IP1/IP0 and does
  not clear timer pending; Count writes before normal committed cadence;
  Compare clears timer pending before that cadence, whose post-increment
  equality check may relatch it. Interrupt delivery and general COP0 access
  remain absent.
- `LIVE_REPO_FACT`: Machine structurally accepts an explicitly supplied
  1,984-byte raw-Boot-ROM-shaped input, rejects the 2,048-byte full-map shape as
  unsupported, and classifies other lengths as malformed. Acceptance proves no
  authenticity, revision, executability, or compatibility. Accepted bytes are
  immutable Machine input that survives reset and repeated bootstrap staging.
- `LIVE_REPO_FACT`: the no-window probe accepts a separate explicit
  `--pif-profile` value of `ntsc-pinned`, `pal-pinned`, or `mpal-pinned`.
  Inspection owns those spellings; Machine owns the three distinct profile
  meanings. Neither firmware nor profile installation alone materializes
  state, and there is no default or inference.
- `LIVE_REPO_FACT`: `Machine::stage_cartridge_bootstrap` atomically copies the
  selected generated or user-supplied firmware slice into a replacement SP
  IMEM image: NTSC raw `[0x0d4,0x71c)` to `[0x000,0x648)`, or PAL/MPAL raw
  `[0x0d4,0x720)` to `[0x000,0x64c)`. Copied bytes become known with
  user-supplied-PIF source provenance; all other bytes remain `Unknown`.
  Reset preserves both immutable inputs while clearing runtime SP IMEM, and
  repeated bootstrap rematerializes only the selected range.
- `LIVE_REPO_FACT`: the no-window probe also accepts four separate explicit
  cold-handoff selectors: `--ipl3-family x105`, `--reset-kind cold`,
  `--boot-medium cartridge`, and `--pif-version-bit 0|1`. Inspection owns only
  those spellings. Machine owns their meanings and rejects an incomplete set.
- `LIVE_REPO_FACT`: with structurally accepted PIF-shaped bytes, explicit
  `NTSC_PINNED`, and the complete cold cartridge x105 selector set,
  `Machine::stage_cartridge_bootstrap` atomically materializes the bounded
  coupled handoff. Known GPRs are t3=`0xFFFFFFFFA4000040`,
  sp=`0xFFFFFFFFA4001FF0`, ra=`0xFFFFFFFFA4001550`, s3=0, s4=1, s5=0,
  s6=`0x91`, and s7 equal to the explicit PIF-version bit. Status is
  `0x34000000`, PC/next-PC are `0xA4000040 / 0xA4000044`, and no delay-slot
  context is active. Every staged fact has a narrow Machine-owned source;
  other inherited GPRs remain unknown. Product tests use generated bytes only.
- `LIVE_REPO_FACT`: PAL_PINNED and MPAL_PINNED coupled handoff requests reject
  before mutation. Their pinned source layouts imply different retained-link
  arithmetic, but independent matching corroboration is insufficient for
  product authority. The three copy profiles remain supported independently;
  only the coupled handoff is NTSC-only.
- `LIVE_REPO_FACT`: one private per-Machine `Ri` owner stores optional
  RI_SELECT and RI_CONFIG facts. Construction and general reset leave both
  unavailable; the complete supported NTSC cold-cartridge x105 bootstrap
  atomically creates RI_SELECT zero with `ColdX105Entry` provenance and leaves
  RI_CONFIG unavailable. Exact aligned `Lw` reads only RI_SELECT at physical
  `0x0470000C`. Exact aligned `Sw` writes only RI_CONFIG at physical
  `0x04700004`, representing current-control input bits 5:0, enable bit 6, and
  CPU-store lineage. Undefined high bits reject before mutation. RI_CONFIG has
  no read route; RI_SELECT has no write route. Other RI registers, calibration,
  timing, NMI, a register bank, MMIO framework, and bus remain absent.
- `LIVE_REPO_FACT`: generated-only public-step composition now commits
  32,035 bounded x105-shaped instructions. The accepted 33-step prefix reaches
  RI_CONFIG; commit 34 stores `0x40`, producing input zero and enable true, and
  commit 35 installs wait counter 8,000. Exactly 8,000 generated loop
  iterations commit 32,000 instructions: 7,999 taken BNEs, one untaken BNE,
  and 8,000 ordinary NOP slots. Final PC/next-PC are
  `0xA40000DC / 0xA40000E0`, Count is `32019`, s1 is zero, and RI_CONFIG is
  unchanged. The next `Sw` computes RI_CURRENT_LOAD at represented CPU address
  `0xA4700008`, physical `0x04700008`, and rejects as a direct target miss
  without mutation. Every instruction and byte is independently generated.
  This CPU-composition proof establishes neither RI elapsed time nor
  calibration and does not change BOOT-2.
- `EXTERNAL_TECHNICAL_EVIDENCE`: pinned NTSC, PAL, and MPAL IPL
  reconstructions share raw source start `0x0d4` and SP IMEM destination zero,
  but NTSC ends at `0x71c` (`0x648` bytes) while PAL and MPAL end at `0x720`
  (`0x64c` bytes). The current 1,984-byte structural shape cannot select that
  profile, and unexamined physical PIF revisions remain `UNKNOWN`.

The single detailed owner for represented capability and explicit absence is
the [represented-machine capability ledger](../../rust/PARITY.md). Stable
architecture boundaries live in the active subsystem pages; retirement
chronology lives in [project history](PROJECT_HISTORY.md).

## Verification and lanes

- Required gate: `./rust/verify-forward`.
- `real-cartridge-boot-spine-v1`: completed, integrated through Master
  authority, and closed at accepted worker candidate `8e5efc8e`.
- `rust-purity-repo-cleanup-v1`: completed, integrated through Master
  authority, and closed at accepted worker candidate `9cc16142`.
- Active C++ lanes: none.
- `cpp-reference-truth-reconstruction-v1`: canceled without provisioning; its reserved branch/worktree remain absent.
- The earlier seam-090 and inventory-first sequences are superseded by the direct Master retirement decision.
- `boot-frontier-sp-imem-lw-v1`: candidate `dcb9f1bf` was independently
  verified, integrated as a truthful partial product increment, and closed
  **PARTIAL — INTEGRATED**. It earned SP IMEM plus aligned `Lw`; it did not
  advance the authentic trace beyond BOOT-2.
- `sp-imem-bootstrap-provenance-v1`: evidence-only candidate `8db1b57c` was
  independently verified and integrated **PARTIAL — EVIDENCE INTEGRATED;
  PRODUCT SOURCE UNAVAILABLE**. It changed no product behavior and earned no
  higher checkpoint.
- `user-supplied-pif-boot-source-v1`: complete three-commit candidate
  `1fa8aa17` was independently verified and integrated **ACCEPTED —
  SOURCE-BOUNDARY PRODUCT**. No private PIF input was used and BOOT-2 did not
  advance.
- `ordinary-control-flow-delay-slot-v1`: complete three-commit candidate
  `01b06e5a` was independently verified and integrated **ACCEPTED**. The shared
  J/JAL target helper passed a region-crossing PC+4 discriminator, the direct
  step probe passed, and no boot checkpoint changed.
- `pif-ipl2-source-mapping-v1`: evidence-only candidate `2ee4b3c7` was
  independently verified and integrated **ACCEPTED — VARIANT-SPECIFIC SOURCE
  MAPPING**. It changed no product behavior and used no private input.
- `pif-ipl2-profiled-copy-materialization-v1`: complete eight-commit candidate
  `a2a8ca51` was independently verified and integrated **ACCEPTED — PROFILED
  COPY MATERIALIZATION PRODUCT**. The artifact raw report is compliant; the
  malformed chat delivery is recorded separately as a
  `WORKFLOW_DELIVERY_DEFECT`. No private input was used and BOOT-2 did not
  advance.
- `master-direct-cold-x105-coupled-handoff-v1`: direct Master product operation;
  source reconstruction and generated proof support one NTSC cold cartridge
  x105 handoff. It creates no Worker lane and leaves the integration queue
  empty.
- `master-direct-generated-x105-frontier-v1`: direct Master product operation;
  bounded source order selected the missing SP-DMEM data target for existing
  aligned `Lw` before `Sw`. Generated proof reaches `Sw` without creating a
  Worker lane or queue entry.
- `master-direct-aligned-sw-x105-store-frontier-v1`: direct Master product
  operation; generated proof represents aligned SP-IMEM-only `Sw`, AdES, and
  CPU-store provenance, then reaches `RegimmBltz` without creating a Worker
  lane or queue entry.
- `master-direct-bltz-x105-branch-frontier-v1`: direct Master product operation;
  generated proof represents non-linking/non-likely BLTZ through the existing
  full-width signed and ordinary-delay-slot owners, commits the x105 zero-store
  slot, and reaches `Cop0Mtc0` without creating a Worker lane or queue entry.
- `master-direct-mtc0-boot-trio-x105-ri-frontier-v1`: direct Master product
  operation; generated proof represents only MTC0 Cause/Count/Compare plus
  their exact cadence and reaches the RI_SELECT `Lw` direct-target miss without
  creating a Worker lane or queue entry.
- `master-direct-ri-select-x105-ri-config-frontier-v1`: direct Master product
  operation; generated proof represents one cold-entry RI_SELECT state/read,
  commits the cold branch and five high-SP-IMEM stack stores, and reaches the
  RI_CONFIG `Sw` target miss without creating a Worker lane or queue entry.
- `master-direct-ri-config-x105-current-load-frontier-v1`: direct Master product
  operation; generated proof represents one RI_CONFIG field-state/write,
  commits the bounded CPU wait loop, and reaches the RI_CURRENT_LOAD `Sw`
  target miss without creating a Worker lane or queue entry.
- `LIVE_REPO_FACT`: the accepted BLTZ report named the wrong branch while the
  preserved worktree was and remains registered to
  `master/direct-bltz-x105-branch-frontier-v1`. This is report-only
  `MASTER_BRANCH_LABEL_OR_TOPOLOGY_DRIFT`; prior topology and accepted product
  history were not modified.
- Active supervisor operations: none. Active Worker operations and lanes: none.
- `pif-ipl2-handoff-state-mapping-v1`: retired as an unaccepted historical
  donor operation. Candidate `c24ab78c`, context-propagation merge `96840e99`,
  its preserved worktree/branch, and its stale artifact remain unaccepted. Its
  known r31/ra profile-qualification defect remains a donor-audit warning, not
  an active repair assignment.
- Integration queue: empty. No Worker candidate awaits review or integration.
- `LIVE_REPO_FACT`: fleet tools remain available as dormant diagnostics and
  historical infrastructure; they are not the active delivery architecture.
- Repository-purity cleanup is complete for its accepted non-product scope.
- Product Acceleration Wave 1 selected one combined frontier because
  storage/routing, aligned `Lw`, bootstrap knownness, Machine step application,
  and boot-probe continuation shared direct source ownership. Its accepted
  partial result is now product truth; no compatibility fact was earned.

## Blockers and known unknowns

- `LIVE_REPO_FACT`: the current Rust product remains deliberately incomplete and headless.
- `UNKNOWN`: performance, broad hardware compatibility, BOOT-3 and later boot
  behavior, game behavior after handoff, and host-runtime behavior remain
  unmeasured or unavailable.
- `LIVE_REPO_FACT`: fn64 has an explicit PIF-firmware input, structural
  validation, immutable Machine ownership, and reset/bootstrap persistence. It
  still has no authentic firmware classification or firmware execution.
  Explicit profile selection now permits a source-backed copy effect, but no
  private PIF was used, so the authentic `Lw` result and BOOT-2 checkpoint are
  unchanged.
- `LIVE_REPO_FACT`: the profiled copy is only the represented IPL1 copy effect.
  The NTSC-only cold x105 path now adds the bounded inherited CPU facts consumed
  before first overwrite; it does not represent PIF RAM as a device, PI/SI
  state, or IPL2 execution.
- `LIVE_REPO_FACT`: the next generated pressure is aligned `Sw r0,8(r8)` to
  RI_CURRENT_LOAD at represented CPU address `0xA4700008` (physical
  `0x04700008`), and it rejects as a direct target miss. RI_CONFIG has no read
  route or hardware-process model; RI_CURRENT_LOAD and every other RI register
  action remain absent. NMI, all other REGIMM identities, every other COP0
  instruction or MTC0 destination, RDRAM/SP-DMEM/device stores, and every store
  identity other than SP-IMEM or exact RI_CONFIG `Sw` remain absent; no generic
  CP0, branch/store, bus, MMIO, or generalized memory-map route is implied.
- `UNKNOWN`: source-qualified PAL/MPAL retained-link values for product use,
  unexamined PIF revisions, NMI and DD handoffs, other IPL3 families, and any
  later pre-cartridge-entry state. Current evidence still does not prove that
  minimal IPL2 execution is required.
- `USER_DECISION`: retired C++ behavior does not define a product backlog. Any future Rust capability requires its own bounded product decision and proof.
- `LIVE_REPO_FACT`: ignored user-local assets remain outside repository truth and routine evidence.
