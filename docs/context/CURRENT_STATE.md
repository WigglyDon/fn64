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
  and applicator for direct KSEG0/KSEG1 aliases of SP IMEM, exact RI_MODE,
  exact RI_CONFIG, exact RI_CURRENT_LOAD, or exact RI_SELECT. SP IMEM stores old `rt` low 32 bits as four known
  big-endian bytes with instruction-PC, source-GPR, and source-lineage
  provenance. The four RI targets use destination-specific Machine state and
  write no memory. Unknown operands and every other target reject before
  mutation. Unaligned stores enter AdES code 5 through the existing COP0 owner,
  including exact BadVAddr and delay-slot EPC/BD; success advances Count once
  and faults advance it zero times.
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
  BOOT-3, authentic handoff, user-provided or commercial cartridge execution,
  compatibility, graphics, window, and audio are not claimed. Public generated
  runtime-v2 execution is synthetic proof only.
- `LIVE_REPO_FACT`: ordinary `BEQ`, `BNE`, non-linking/non-likely `BLTZ`,
  `J`, `JAL`, `JR`, and `JALR`
  execute through `Machine::step` with one CPU-owned delay-slot context.
  Taken and untaken branches both execute one slot; link, alias, Count,
  branch-in-delay-slot rejection, and delay-slot EPC/BD behavior are explicit.
  Prior r31 value or lineage is not a JAL input. JALR still consumes only its
  captured old `rs`, including when source and link destination alias; every
  genuine branch, jump-register, load, and store source-knownness gate remains.
  BLTZ reuses the established full-GPR signed comparison used by SLT/SLTI;
  every other REGIMM identity remains unrepresented. This is bounded ordinary
  control flow, not complete MIPS control flow.
- `LIVE_REPO_FACT`: `Cop0Mtc0` executes for Cause software-pending bits,
  Count, Compare, and the exact CPU-owned primary-cache TagLo/TagHi
  destinations while the source-backed cold-x105 kernel state is active.
  It transfers the known source GPR's low word, preserves the source, and
  rejects malformed encodings, other destinations, unavailable sources, or
  other access contexts before mutation. Cause writes only IP1/IP0 and does
  not clear timer pending; Count writes before normal committed cadence;
  Compare clears timer pending before that cadence, whose post-increment
  equality check may relatch it. TagLo and TagHi each retain a raw low word and
  source provenance; the generated cold sequence writes zero from
  ArchitecturalZero. Interrupt delivery and general COP0 access remain absent.
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
  RI_MODE, RI_SELECT, RI_CONFIG, RI_CURRENT_LOAD event, and RI_REFRESH facts. Construction and
  general reset leave all unavailable; the complete supported NTSC
  cold-cartridge x105 bootstrap atomically creates RI_SELECT zero with
  `ColdX105Entry` provenance and leaves the other three facts unavailable. Exact
  aligned `Lw` reads only RI_SELECT at physical `0x0470000C`. Exact aligned
  `Sw` writes RI_CONFIG fields at physical `0x04700004` or creates an
  RI_CURRENT_LOAD event at `0x04700008`. The event requires and snapshots the
  stored RI_CONFIG input/enable fields, plus CPU-store lineage and transfer
  word as evidence. It creates no output, calibration, timing, or RDRAM-ready
  fact. Exact RI_SELECT `Sw` at `0x0470000C` accepts only x105 word `0x14`,
  replacing the stored value and `ColdX105Entry` source with CPU-store
  provenance; all other words reject as an unsupported product boundary. The
  existing `Lw` reads the updated stored value. RI_MODE at physical
  `0x04700000` stores operating-mode bits 1:0, stop-transmit-active bit 2, and
  stop-receive-active bit 3 with CPU-store provenance. Any bit above bit 3
  rejects before mutation as fn64's unsupported boundary; no hardware-trap
  claim follows. RI_CONFIG, RI_CURRENT_LOAD, and RI_MODE have no read route.
  Exact generated RI_REFRESH word `0x001E3634` writes and reads at physical
  `0x04700010`; its named fields derive from the one raw word, with no timing
  effect or refresh engine.
  General RI_SELECT programming, other RI actions,
  NMI, a register bank, MMIO framework, and bus remain absent.
- `LIVE_REPO_FACT`: one private per-Machine `Mi` owner stores immutable
  standard-retail MI_VERSION identity `0x02020102`, optional current MI
  initialization-mode state, one bounded pending transfer, and optional
  RDRAM-register access mode. The raw version
  word is the single stored truth; IO/RAC/RDP/RSP fields derive as
  `02/01/02/02`. Construction, reset, and complete cold-x105 bootstrap
  preserve the identity while leaving both mutable facts unavailable. Exact aligned
  `Sw` at physical `0x04300000` accepts only low word `0x0000010F`, stores
  initialization length 15 and initialization mode true with CPU-store
  provenance, and arms one 16-byte transfer for the exact generated RDRAM_DELAY
  consumer. Other represented successful stores cannot bypass it. Repeated
  bootstrap clears stale MI state/transfer and preserves identity; failed
  bootstrap preserves all three. Exact aligned `Lw` at physical
  `0x04300004` reads the identity through either direct alias with ordinary
  CPU provenance and cadence. Exact generated commands `0x00002000` and
  `0x00001000` enable and disable RDRAM-register access; generated zero records
  its source-clear no-change command. Module-register reads remain closed while
  the mode is disabled. Other MI reads and MI_VERSION writes remain unavailable.
- `LIVE_REPO_FACT`: the existing per-Machine `Rdram` owner remains the sole
  RDRAM-byte, fixed-profile, module, mapping, register-request, and deterministic
  calibration-response owner. The current fixed 4 MiB backing selects profile
  `fixed-standard-retail-4mib-two-module-digital-cc-v1`: two present 2 MiB
  modules with DEVICE_TYPE `0xB0190000`, fixed NEC manufacturer `0x0500`, and
  enhanced-speed false. Optional global/broadcast delay, raw REF_ROW,
  relocation, first-responder/final DEVICE_ID, RDRAM_MODE, RAS_INTERVAL, and
  calibration facts all remain inside that one owner.
  Exact aligned `Sw` at physical `0x03F80008` requires the
  pending 15/16 MI transfer and low word `0x18082838`, then stores logical
  fields 5/7/3/1 and packed logical configuration `0x28381808` with CPU and
  consumed-MI provenance. It consumes the transfer and makes current
  MI_INIT_MODE readback unavailable because exact post-transfer fields are not
  source-clear. Exact aligned `Sw` at physical `0x03F80014` accepts only low
  word zero with known CPU-store lineage and records the global aperture without
  interpreting fields. Physical `0x03F80004` accepts only `0x80000000` and
  records requested base `0x02000000` as a global relocation request. Exact
  RCP 2.0 first-responder and direct module apertures follow exact `0x400`
  register spacing. Generated DEVICE_ID requests select present unassigned
  modules in profile order, allow one exact absent-module probe, and finish at
  physical bases `0x00000000` and `0x00200000` without copying backing bytes.
  Generated module-register mode reads expose DEVICE_TYPE, manufacturer, and
  current mode only while MI RDRAM-register mode is enabled. Manual calibration
  reads deterministically return `min(n + 1, 8)` one bits in response byte
  23:16 for nominal input `n`; absent modules return zero. This digital profile
  makes guest scores `10,20,30,40,50,60,70,80`, selects manual/automatic input
  7, and ends with per-module mode `0xC6808080` and RAS_INTERVAL `0x101C0A04`.
  Outside active calibration, direct RDRAM reads retain backing-byte truth.
  The model claims no analog accuracy, electrical timing, generic register
  bank, MMIO framework, bus, or cartridge-selected policy.
- `LIVE_REPO_FACT`: generated-only public-step composition retains the
  accepted 32,185-instruction prefix and now continues through complete RDRAM
  initialization. The accepted 32,038-step prefix
  ends after the 8,000-iteration CPU wait loop. Commit 32,036 stores r0 to
  RI_CURRENT_LOAD, snapshotting RI_CONFIG input zero and enable true; commit
  32,037 executes `Ori r9,r0,0x14`. Final PC/next-PC are
  `0xA40000E4 / 0xA40000E8`, Count is `32021`, and r9 is `0x14`. Commit 32,038
  stores that exact word to RI_SELECT and installs CPU-store provenance.
  Commit 32,039 stores RI_MODE zero. An Addiu and four iterations of
  NOP/Addi/Bne/NOP commit 17 more instructions with three taken and one
  untaken branch. Commit 32,057 constructs `0x0E`; commit 32,058 replaces
  RI_MODE with operating mode 2 and both stop-active flags. An Addiu and 32
  iterations of Addi/Bne/Ori commit 97 more instructions; the ORI is the BNE
  delay slot on every iteration and leaves r9=`0x10F`. Final PC/next-PC are
  `0xA4000118 / 0xA400011C`, Count is `32139`, and s1 is zero. Commit 32,156
  stores exact word `0x10F` to MI_INIT_MODE and creates length 15 / init-mode
  true with CPU-store provenance. `Lui` and `Ori` then construct
  r9=`0x18082838`. Commit 32,159 stores it through global RDRAM_DELAY at CPU
  `0xA3F80008` / physical `0x03F80008`, creates the 5/7/3/1 broadcast fact,
  consumes the MI transfer, and leaves current MI readback unavailable. Commit
  32,160 stores raw zero through global RDRAM_REF_ROW at CPU `0xA3F80014` /
  physical `0x03F80014`, preserving the delay fact. Commit 32,161 executes
  `Lui r9,0x8000`, producing `0xFFFFFFFF80000000` with instruction lineage.
  Commit 32,162 stores its low word through global RDRAM_DEVICE_ID at CPU
  `0xA3F80004` / physical `0x03F80004`, recording requested physical base
  `0x02000000` with exact CPU provenance while preserving RDRAM bytes, routing,
  delay state, and REF_ROW state. Fourteen generated CPU-local setup commits
  leave PC/next-PC `0xA400016C / 0xA4000170`, Count `32160`, and total commits
  `32176`. Commit 32,177 reads MI_VERSION word `0x02020102` at CPU
  `0xA4300004` / physical `0x04300004` into r16 with ordinary load
  provenance. Generated `Lui`/`Ori` build `0x01010101`; the guest BNE is
  taken and its Nop delay slot executes once. RCP 2.0 setup selects spacing
  `0x400` and first-responder base `0xFFFFFFFFA3F08000`. PC/next-PC become
  `0xA4000198 / 0xA400019C`, Count is `32167`, and total commits are
  `32183`. Commit 32,184 stores low word zero through first-responder
  non-global RDRAM_DEVICE_ID CPU `0xA3F08004` / physical `0x03F08004`,
  recording a bounded assignment request with r14's generated `Addu` lineage
  from `0xA4000138`. Commit 32,185 executes `Addiu r21,r15,0x000C`, producing
  initial RDRAM_MODE address `0xFFFFFFFFA3F0000C`. PC/next-PC become
  `0xA40001A0 / 0xA40001A4`, Count is `32169`, and total commits are `32185`.
  Commit 32,186 executes generated `Jal 0xA400087C` at `0xA40001A0`, replacing
  retained r31 `0xFFFFFFFFA4001550` with link `0xFFFFFFFFA40001A8` and exact
  instruction-result lineage. PC/next-PC become
  `0xA40001A4 / 0xA400087C`, Count is `32170`. Commit 32,187 executes the Nop
  delay slot once and enters InitCCValue. Five further commits through
  `0xA400088C` subtract `0xA0` from sp, save spacing `0x00000400` and
  first-responder base `0xA3F08000` to SP-IMEM offsets `0xF30/0xF34`, and
  zero r17/r16. At 32,192 commits, PC/next-PC are
  `0xA4000890 / 0xA4000894` and Count is `32176`. Four generated stores of
  inherited r2-r5 then commit cause-known, value-unavailable aligned SP-IMEM
  words at offsets `0xEF0`, `0xEF4`, `0xEF8`, and `0xEFC`; their private zero
  backing sentinels are not transferred truth. Twenty following known-source
  stores commit normally through `0xA40008EC` without changing those opaque
  words. At 32,216 commits, PC/next-PC are
  `0xA40008F0 / 0xA40008F4` and Count is `32200`. Commit 32,217 executes
  generated `Jal 0xA4000984` (FindCC), writes link
  `0xFFFFFFFFA40008F8`, and schedules its Nop slot; commit 32,218 executes the
  slot once. Six FindCC entry/setup commits produce nominal CC input zero and
  known `r26=1`. Commit 32,225 executes word `0x53400018`,
  `Beql r26,r0,0xA4000A00`, as not taken and architecturally annuls
  `Or r2,r0,r0` at `0xA40009A0`: the slot is not executed, committed, or
  counted, creates no delay context, and leaves r2 `UnknownPifProduced`.
  Generated JAL/delay-slot execution then enters TestCCValue with a0 zero,
  creates its frame, and enters WriteCC with a1=`CC_MANUAL`. Actual generated
  arithmetic transforms zero through `0x3F`, combines base flags
  `0x46000000` with scattered bits `0x00C0C0C0`, and produces
  `r15=0x46C0C0C0`. At 32,259 commits, PC/next-PC are
  `0xA4000BB8 / 0xA4000BC4` and Count is `32243`. Commit 32,260 executes
  delay-slot word `0xAEAF0000`, `Sw r15,0(r21)`, to CPU `0xA3F0000C` /
  physical `0x03F0000C`; it records the exact initial non-global request,
  reaches `0xA4000BC4 / 0xA4000BC8`, advances Count to `32244`, and clears the
  existing BNE context. Six further supported commits restore WriteCC ra/sp,
  return through JR/Nop to TestCCValue, zero s8, and construct r26 all ones.
  At 32,266 commits, PC/next-PC are `0xA4000A2C / 0xA4000A30` and Count is
  `32250`. From there 214,734 further public `Machine::step` calls commit the
  three initial RDRAM test stores, all guest TestCCValue/FindCC/
  ConvertManualToAuto/ReadCC/WriteCC loops and calls, discovery of both present
  modules plus one absent probe, module register reads/writes, final mapping,
  RI_REFRESH write/read, detected-size store, and frame teardown. Total commits
  are `247000`; Count is `246984`; PC/next-PC are
  `0xA4000400 / 0xA4000404`. Guest-written memory size is `0x00400000`, matching
  the Machine backing. The current unexecuted word is `0x4080E000`,
  `Cop0Mtc0 zero,$28` (C0_TAGLO), the first cache-initialization-specific
  operation. From there, 5,367 further public steps write zero TagLo/TagHi,
  invalidate all 512 direct-mapped 32-byte I-cache lines and all 512
  direct-mapped 16-byte D-cache lines, apply exact SP status words
  `0x000000CE` and `0x000000AD` plus SP_PC zero, and copy 820 public bytes from
  SP DMEM `[0x554,0x888)` to physical RDRAM `[0x4,0x338)`. The generated JR
  enters KSEG0 at `0x80000004`; a real CPU-owned I-cache miss fills physical
  line zero from the relocated RDRAM bytes, and the following cached
  instructions consume a synthetic Machine-owned cartridge-header word. At
  total commit 252,367, Count `252351`, PC/next-PC
  `0x8000001C / 0x80000020`, `Sw r9,0(r1)` word `0xAC290000` begins the PI
  path. From there 7,225,461 public steps program PI_DRAM_ADDR `0x00001000`,
  PI_CART_ADDR `0x10001000`, and PI_WR_LEN `0x000FFFFF`; atomically copy one
  MiB of public synthetic cartridge bytes to RDRAM; set and later clear the
  MI-owned PI pending source; fill 65,536 D-cache lines and take 196,608 hits
  while the guest produces checksum words `0xFAD40ECC / 0x1F137F19`; clear the
  generated MI/SP/SI/AI/DP/PI interrupt state; write the seven boot globals;
  and execute all 2,048 known-word SP-memory teardown stores. Final JR word
  `0x01200008` at `0x8000027C` and its Nop slot reach PC/next-PC
  `0x80001000 / 0x80001004`, Count `7477812`, total commit 7,477,828, with no
  active delay context. The independently encoded entry word `0x24020042`
  remains unexecuted. Every instruction and represented state is generated
  from public synthetic input. This proves deterministic PI/D-cache/final-IPL3
  composition, not PI timing, RSP execution, authentic IPL2 or cartridge
  execution, BOOT-3, or compatibility; BOOT-2 remains unchanged.
- `RUNTIME_FACT`: the separate public
  `synthetic-x105-cartridge-runtime-v2` fixture preserves that exact v1 proof
  and adds one 92-word self-checking program. A complete cold public
  composition executes 7,477,116 instructions and reaches Count `7477100`.
  The cartridge program itself commits 77 instructions: entry word
  `0x24020042` executes once; seven guest comparisons all select success;
  failure code executes zero times; and two J/Nop iterations complete at
  PC/next-PC `0x80001124 / 0x80001128`. CPU-owned KSEG0 D-cache `Sw`, `Sb`,
  `Lw`, and `Lbu` produce load hits/misses `4/2`, store hits/misses `2/1`,
  and three atomic dirty writebacks at physical lines
  `0x00100000`, `0x00102000`, and `0x00100000`. Final backing words are
  `0x11AA3344` and `0x55667788`; eight uncached KSEG1 stores write the exact
  success mailbox at physical `0x003FF000`; shared D-cache index zero ends
  valid clean with tag `0x00102000`. Runtime-v2 header and guest checksum words
  match at `0x4077ADEF / 0x096B847A`. This earns synthetic milestone
  `SYNTHETIC-CARTRIDGE-RUNTIME-COMPLETE`, not BOOT-3, authentic cartridge
  execution, or compatibility.
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
- `master-direct-ri-current-load-x105-ri-select-frontier-v1`: direct Master
  product operation; generated proof represents one RI_CURRENT_LOAD update
  event consuming stored RI_CONFIG, commits the following `Ori`, and reaches
  the RI_SELECT `Sw` target miss without creating a Worker lane or queue entry.
- `master-direct-ri-select-write-x105-ri-mode-frontier-v1`: direct Master
  product operation; generated proof represents exact x105 RI_SELECT CPU
  write/read-after-write state and reaches the RI_MODE `Sw` target miss without
  creating a Worker lane or queue entry.
- `master-direct-ri-mode-sequence-x105-mi-init-frontier-v1`: direct Master
  product operation; generated proof represents RI_MODE defined fields and
  CPU-store provenance, commits both bounded writes and both CPU waits, and
  reaches the MI_INIT_MODE `Sw` target miss without creating a Worker lane or
  queue entry.
- `master-direct-mi-init-mode-x105-rdram-delay-frontier-v1`: direct Master
  product operation; generated proof represents the exact x105 MI_INIT_MODE
  word and minimal Machine-owned result state, commits the following `Lui` and
  `Ori`, and reaches the global RDRAM_DELAY `Sw` target miss without creating a
  Worker lane or queue entry.
- `master-direct-rdram-delay-x105-ref-row-frontier-v1`: direct Master product
  operation; the accepted MI write arms one bounded transfer, exact global
  RDRAM_DELAY consumes it into a Machine-owned configuration fact, and
  generated proof reaches the global RDRAM_REF_ROW `Sw` target miss without
  creating a Worker lane or queue entry.
- `master-direct-rdram-ref-row-x105-device-id-frontier-v1`: direct Master
  product operation; exact global RDRAM_REF_ROW records raw zero and CPU-store
  provenance, the following LUI constructs `0x80000000`, and generated proof
  reaches the global RDRAM_DEVICE_ID `Sw` target miss without creating a Worker
  lane or queue entry.
- `master-direct-rdram-device-id-x105-mi-version-frontier-v1`: direct Master
  product operation; exact global RDRAM_DEVICE_ID records word `0x80000000`,
  requested base `0x02000000`, and CPU-store provenance without relocation;
  generated CPU-local setup reaches the MI_VERSION `Lw` target miss without a
  Worker lane or queue entry.
- `master-direct-mi-version-fixed-identity-x105-first-responder-frontier-v1`:
  direct Master product operation; immutable per-Machine MI_VERSION
  `0x02020102` and one exact aligned read let the guest comparison select the
  RCP 2.0 branch, and generated proof reaches the first-responder non-global
  RDRAM_DEVICE_ID `Sw` target miss without a Worker lane or queue entry.
- `master-direct-rdram-first-responder-device-id-x105-cc-frontier-v1`:
  direct Master product operation; exact RCP 2.0 first-responder
  RDRAM_DEVICE_ID zero records a bounded assignment request, generated `Addiu`
  constructs the initial RDRAM_MODE address, and proof stops at the following
  JAL link-lineage rejection before entering current-control code, without a
  Worker lane or queue entry.
- `master-direct-link-destination-provenance-x105-init-cc-frontier-v1`:
  direct Master product operation; prior link-destination state is removed as
  a false JAL/JALR input, retained bootstrap r31 remains truthful until the
  generated JAL replaces it, and public stepping enters InitCCValue before
  stopping atomically at the first unknown store source, without a Worker lane
  or queue entry.
- `master-direct-rdram-mode-first-manual-x105-calibration-boundary-v1`:
  direct Master product operation; exact initial non-global RDRAM_MODE word
  `0x46C0C0C0` records one request-only fact, generated public stepping returns
  from WriteCC, and proof stops before the first RDRAM response-test access
  without a Worker lane or queue entry.
- `master-direct-standard-retail-rdram-bringup-x105-cache-frontier-v1`:
  direct Master coherent-subsystem operation; current 4 MiB backing selects one
  two-module fixed digital profile, generated public stepping completes RDRAM
  calibration/discovery/configuration and detected-size storage, and proof stops
  before cache-specific C0_TAGLO without a Worker lane or queue entry.
- `master-direct-primary-cache-ipl3-relocation-x105-pi-frontier-v1`: direct
  Master coherent-subsystem operation; CPU-owned primary cache arrays and
  TagLo/TagHi execute both generated 512-line invalidation loops, exact
  Machine-owned SP control permits ordinary relocation of the public IPL3
  suffix into RDRAM, and cached relocated KSEG0 execution reaches but does not
  execute the first PI store, without a Worker lane or queue entry.
- `master-direct-pi-dma-ipl3-final-handoff-x105-synthetic-entry-v1`: direct
  Master coherent-subsystem operation; one concrete `Pi` owner, MI-owned
  interrupt truth, functional KSEG0 D-cache reads, a public one-MiB synthetic
  cartridge, exact final device controls, genuine SP teardown stores, and the
  generated JR/delay slot reach but do not execute the synthetic cartridge
  entrypoint, without a Worker lane or queue entry.
- `master-direct-synthetic-cartridge-runtime-dcache-completion-v1`: direct
  Master coherent-runtime operation; CPU-owned valid-dirty D-cache state,
  atomic Rdram writeback, a versioned public runtime-v2 fixture, guest-owned
  self-checks/mailbox, and two stable success-loop iterations complete without
  a Worker lane or queue entry.
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
- `LIVE_REPO_FACT`: generated public stepping now completes the current fixed
  two-module RDRAM bring-up. The sole `Rdram` owner holds the capacity-derived
  profile, module/register/mapping provenance, and deterministic digital
  calibration response; `Mi` holds RDRAM-register mode and `Ri` holds raw
  RI_REFRESH. Both modules select automatic input 7, final mode `0xC6808080`,
  RAS `0x101C0A04`, and final bases zero/2 MiB. Guest size `0x00400000` matches
  backing capacity. Opaque frame restores transport unavailable lineage without
  converting sentinel backing to known truth. CPU-owned TagLo/TagHi and primary
  cache arrays then execute exact 512-line I/D invalidation loops. Exact SP
  register commands bracket an 820-byte SP-DMEM-to-RDRAM relocation; the
  generated KSEG0 jump fills I-cache line zero from those bytes and executes
  through the synthetic cartridge-header load. The exact PI register sequence
  then performs one atomic one-MiB cartridge-to-RDRAM copy, while MI owns its
  completion interrupt. Functional KSEG0 D-cache `Lw` executes the complete
  checksum; exact final control writes and 2,048 ordinary SP stores complete
  IPL3 teardown. The preserved v1 proof stops before its entry word. The
  separate runtime-v2 proof executes that word and its complete guest
  self-check through three genuine dirty D-cache writebacks, an exact uncached
  success mailbox, and two stable success-loop iterations. PI timing, RSP
  execution, authentic or user-provided cartridge execution, analog timing/accuracy,
  host-selected profiles, arbitrary module topology, general device routing,
  broader COP0/CACHE, NMI, a generic bus/MMIO/map, BOOT-3, and compatibility
  remain absent.
- `UNKNOWN`: source-qualified PAL/MPAL retained-link values for product use,
  unexamined PIF revisions, NMI and DD handoffs, other IPL3 families, and any
  later pre-cartridge-entry state. Current evidence still does not prove that
  minimal IPL2 execution is required.
- `USER_DECISION`: retired C++ behavior does not define a product backlog. Any future Rust capability requires its own bounded product decision and proof.
- `LIVE_REPO_FACT`: ignored user-local assets remain outside repository truth and routine evidence.
