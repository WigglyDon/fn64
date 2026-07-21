# Represented Machine Capability Ledger

Canonical role: the single detailed owner for current fn64 represented-machine
capability and explicit absence.

This document describes the tracked Rust product at the current repository
revision. It is not a transition timeline, a comparison against retired source,
or a compatibility claim. Current source and tests are the executable truth;
this ledger must change with any accepted capability change.

The filename remains as a stable discovery path inherited from earlier project
history. Its legacy name grants no comparison authority. Durable transition and
retirement history belongs in
[PROJECT_HISTORY.md](../docs/context/PROJECT_HISTORY.md), accepted choices in
[DECISION_LOG.md](../docs/context/DECISION_LOG.md), and retired source in Git
history.

## Authority and claim boundary

- The Cargo workspace under `rust/` is the sole current product
  implementation.
- A `Machine` instance is the unit of represented emulated truth.
- `fn64-core` owns represented state and mutation.
- `fn64-inspection` owns deterministic setup, assertions, formatting, and
  process exit only.
- `Machine::step` is the sole public represented execution entrance.
- The product is incomplete and headless. `BOOT-2` is the highest earned
  cartridge checkpoint; it is not bootstrap handoff, game boot, compatibility,
  timing accuracy, or host-runtime support.

## Represented owners

| Owner | Represented truth | Explicit boundary |
| --- | --- | --- |
| `Cartridge` | normalized owned bytes, source layout, parsed header metadata, entry/IPL3-span inspection, range-checked byte reads | no filesystem path, broad CPU mapping, CIC policy, or direct game-entry execution |
| `PifFirmware` | private immutable owned bytes for one structurally accepted raw-Boot-ROM-shaped input and the source bytes for explicit profiled copy materialization | no path, authenticity/revision claim, profile selection, firmware execution, or compatibility policy |
| `PifIpl2Profile` | one explicit Machine-owned `NtscPinned`, `PalPinned`, or `MpalPinned` copy layout | no CLI spelling, default, autodetection, firmware-hash policy, or compatibility claim |
| `Cpu` | 32 GPRs, HI/LO, `pc` / `next_pc`, one narrow delay-slot context, and the represented COP0 subset | no host cadence, full ISA, interrupt controller, or TLB/MMU |
| `Rdram` | 4 MiB zero-filled storage; immutable capacity-derived two-module standard-retail profile; checked raw access; concrete module inventory, register/mapping/provenance state; deterministic digital calibration response; prior global/broadcast and DEVICE_ID facts | no cartridge/host profile selection, arbitrary module topology, analog/current accuracy claim, timing/readiness engine, general register array, generic bus, or MMIO framework |
| `SpDmem` | 4 KiB zero-filled storage, checked reads, and private Machine-owned range staging for the normalized bootstrap span | no public write surface, DMA, RSP, or COP2 execution |
| `SpImem` | 4 KiB private backing storage, per-byte provenance/knownness, coherent cause-known value-unavailable aligned words, checked known big-endian reads, concrete/opaque CPU-store provenance, narrow opaque-word load transport preserving unavailable lineage, and atomic profiled-copy replacement | no public mutable access, opaque value exposure as known truth, SP instruction-fetch route, profile policy, SP register/status/DMA, or RSP execution |
| `Ri` | optional RI_MODE, RI_SELECT, RI_CONFIG, RI_CURRENT_LOAD, and exact RI_REFRESH raw/provenance state with source-clear derived fields | no RI_MODE/RI_CONFIG/RI_CURRENT_LOAD read, general RI_SELECT fields, refresh timing/electrical effect, NMI lifecycle, register bank, MMIO framework, or bus |
| `Mi` | immutable MI_VERSION `0x02020102`, optional exact-x105 initialization state, one bounded pending transfer, and exact generated RDRAM-register access-mode command state with CPU provenance | no alternate identity/configuration, unrelated MI register behavior, command bank, general next-write replication, timing, MMIO framework, or bus |
| `Machine` | Cartridge, optional accepted PifFirmware and PifIpl2Profile, explicit handoff selectors, Cpu, Rdram, SpDmem, SpImem, Ri, Mi, bootstrap provenance/GPR-knownness/COP0/control-flow state, private RDRAM reservation state, powered/reset state, represented fetch/data composition, and public step composition | no hidden global machine, platform clock, file path, renderer, audio, input, or event loop |
| `fn64-inspection` | construction/reset, represented-step, and bounded cartridge-bootstrap no-window probes over public core APIs; exact CLI spellings for explicit firmware, profile, family, reset, medium, and PIF-version inputs | no machine truth, selector meaning, general runtime loop, graphics, or compatibility authority |

## Cartridge representation

The cartridge owner accepts in-memory bytes only. It represents:

- source-layout detection for big-endian, byte-swapped 16-bit, and
  little-endian 32-bit ROM byte layouts;
- normalization into canonical big-endian byte order;
- explicit rejection of undersized, mis-sized, or unsupported input;
- parsed header magic, clock rate, entry point, release address, CRC values,
  and bounded ASCII name fields;
- range-checked byte reads;
- inspection of the header entry word and availability of the candidate IPL3
  span.

`Machine::stage_cartridge_bootstrap` consumes the already normalized,
Machine-owned cartridge span `[0x40, 0x1000)`, preflights it, stages it into the
same SP DMEM offsets, and records cartridge provenance. This is a narrow
bootstrap path, not a general cartridge mapping, PI DMA, CIC model, direct game
entry, or complete boot.

## Explicit PIF firmware input boundary

The existing no-window `fn64_boot_probe` accepts one optional literal
`--pif-rom` path and a separate optional `--pif-profile` spelling of
`ntsc-pinned`, `pal-pinned`, or `mpal-pinned`. Inspection owns parsing, the
profile spellings, opening, read failure, and transfer of an owned byte vector.
It performs no automatic search, default lookup, fallback, download, profile
inference, or variant selection. The core never receives the path or CLI text.

`Machine::install_pif_firmware` validates before replacing the private input
owner. Exactly 1,984 bytes are accepted as structurally shaped raw Boot ROM
input, exactly 2,048 bytes are named as a full PIF address-space image and
rejected as unsupported, and other lengths are malformed. These rules are
structural only: they do not authenticate firmware, establish a revision,
prove executability, or claim compatibility. Content, filenames, cartridge
identity, and digests do not select acceptance or behavior.

The same probe accepts a separate explicit selector set for the bounded
handoff: `--ipl3-family x105`, `--reset-kind cold`, `--boot-medium cartridge`,
and `--pif-version-bit 0|1`. Inspection parses only those tokens. Machine owns
their semantic independence and requires the complete set before coupled
handoff materialization. There is no default, title/product-code/filename/full-
ROM-digest selector, host-region inference, or compatibility database.

Accepted bytes remain private and immutable, survive reset and repeated
cartridge-bootstrap staging, and are exposed only as absent/accepted
classification plus size. Profile selection is an independent Machine input;
firmware-first and profile-first installation converge, while either input by
itself remains non-materializing. Rejected replacement preserves every
represented Machine fact. Installation executes no firmware and does not
advance PC, Count, checkpoint, or boot state.

## Machine lifecycle and state

Construction and `Machine::reset` establish the represented non-boot state:

- `pc = 0xbfc00000` and `next_pc = 0xbfc00004`;
- zeroed GPR, HI/LO, represented COP0, RDRAM, SP DMEM, and SP IMEM backing;
- every constructed or reset SP IMEM byte has `Unknown` provenance despite its
  concrete zero value;
- cleared private CPU/RDRAM reservation state;
- `powered_on = true`;
- the Machine-owned Cartridge is preserved across reset.
- any accepted immutable PIF firmware input is preserved across reset; absence
  remains explicit when no input was installed.
- any selected PIF IPL2 copy profile is preserved across reset; absence remains
  explicit and no default profile is introduced.
- RI_SELECT, RI_CONFIG, and RI_CURRENT_LOAD are unavailable; construction and
  general reset do not invent hardware power-on values or progress.

`Machine::stage_cpu_pc` is a narrow deterministic staging surface. It sets the
selected `pc` and establishes `next_pc = pc.wrapping_add(4)` without fetching
or executing.

`Machine::stage_cartridge_bootstrap` replaces represented CPU, RDRAM, SP DMEM,
SP IMEM, and reservation state only after complete source/destination
preflight. When both accepted firmware and an explicit profile are installed,
the replacement SP IMEM atomically materializes that profile's complete copy
range before assignment; otherwise its bytes remain zero-backed and
`Unknown`. It sets
`pc / next_pc` to `0xA4000040 / 0xA4000044`, represents GPR zero as known
architectural zero, and represents GPR29 as the known restored PIF IPL2 stack
pointer `0xFFFFFFFFA4001FF0`. Every other unstaged PIF-produced GPR remains
explicitly unknown even though its concrete storage is zero. Reset clears the
staged bootstrap bytes and provenance.

When all explicit selectors request the supported `NTSC_PINNED` cold
cartridge x105 path, the same creation point first produces a private complete
plan and then atomically applies the coupled state. Known entry GPRs are
t3=`0xFFFFFFFFA4000040`, sp=`0xFFFFFFFFA4001FF0`,
ra=`0xFFFFFFFFA4001550`, s3=0, s4=1, s5=0, s6=`0x91`, and s7 equal to the
explicit PIF-version bit. Status=`0x34000000`, PC/next-PC are
`0xA4000040 / 0xA4000044`, and delay-slot context is clear. GPR, Status, and
completed-control-transfer sources are inspectable. The same complete plan
atomically creates RI_SELECT=0 with `ColdX105Entry` provenance and leaves
RI_CONFIG and RI_CURRENT_LOAD unavailable, clearing any stale CPU-written state
on repeated staging. This is a bounded entry fact, not a generic RI reset
value. All other
inherited GPRs remain `Unknown`; Count, Compare, EPC, BadVAddr, Cause, and timer state receive
no source-backed handoff claim. PAL/MPAL coupled requests fail before mutation.

There is no power-off transition, boot reset, PIF execution source, broad
mutable CPU accessor, savestate format, or serialization contract.

## Storage and CPU-address access

RDRAM supports checked raw `u8`, `u16_be`, `u32_be`, and `u64_be` reads.
Machine-owned raw writes support the same widths, validate the complete span
before mutation, write multi-byte values in big-endian order, and invalidate
only overlapping private reservation state.

The current 4 MiB backing selects immutable profile
`fixed-standard-retail-4mib-two-module-digital-cc-v1`. The sole `Rdram` owner
contains two present 2 MiB module records with DEVICE_TYPE `0xB0190000`, fixed
NEC manufacturer word `0x0500`, enhanced-speed false, mutable DEVICE_ID/mode/
RAS/provenance facts, and mapping metadata. The same constructor supports the
already-owned 8 MiB capacity as four 2 MiB modules; other capacities reject
profile construction. No cartridge, filename, host option, or digest selects
the profile, and mapping metadata never duplicates or relocates backing bytes.

SP DMEM remains 4 KiB Machine-owned storage. The bootstrap creation point has
one private preflighted range-write seam for normalized cartridge bytes; public
inspection remains read-only.

SP IMEM is exactly 4 KiB at physical `0x04001000..0x04001fff`. The Machine
constructs and resets it as zero backing with independent `Unknown` byte
provenance. Bootstrap restaging starts from the same replacement state, then
copies the complete explicitly selected range when both inputs exist. Each
copied byte is known with `UserSuppliedPifFirmware` provenance naming profile
and source offset; every byte outside the range remains `Unknown`. A word read
requires four known bytes, assembles them in N64 big-endian order, and reports
the first unknown byte otherwise. `SpImem` also owns coherent aligned opaque
word records for committed CPU stores whose cause and destination are exact
but whose transferred bits are unavailable. Four private zero sentinel bytes
are not machine truth. Only test builds can directly stage
generated known words. Production and inspection have no mutable SP IMEM
surface.

Integrated source-qualified evidence identifies the external hardware
causality: IPL1 copies proprietary IPL2
firmware content into SP IMEM, CPU control enters IPL2 there, IPL2 stages
cartridge IPL3 in SP DMEM, and the observed x105 IPL3 entry consumes retained
SP IMEM `[0x000, 0x020)` before initially mutating `[0x000, 0x02c)`. The exact
values remain external user input. External observability is not authority to
embed the content, infer a profile, or claim authentic execution.

Pinned source mapping narrows that external effect. NTSC raw
`[0x0d4, 0x71c)` maps to SP IMEM `[0x000, 0x648)`; pinned PAL and MPAL raw
`[0x0d4, 0x720)` map to `[0x000, 0x64c)`. All cover the consumed
`[0x000, 0x020)` and mutation-frontier `[0x000, 0x02c)` ranges. The
structurally accepted 1,984-byte shape does not identify one profile, and the
mapping is not generalized to unexamined physical PIF revisions.

The product represents those three layouts exactly. NTSC copies raw
`[0x0d4,0x71c)` to SP IMEM `[0x000,0x648)`; PAL and MPAL remain distinct
profile identities and copy raw `[0x0d4,0x720)` to `[0x000,0x64c)`. No
longest-copy fallback exists. Repeated bootstrap reconstructs a fresh image, so
a later NTSC selection leaves the prior PAL/MPAL tail `[0x648,0x64c)`
`Unknown`.

Direct CPU-address classification represents KSEG0 and KSEG1 aliases into the
4 MiB RDRAM span. Direct fixed-width value APIs compose classification with raw
RDRAM access. Raw storage offsets do not impose CPU alignment; CPU-data access
APIs separately enforce byte/halfword/word/doubleword alignment and select
represented AdEL or AdES entry for alignment faults and aligned direct-target
rejection.

The aligned `Lw` data route accepts direct KSEG0/KSEG1 RDRAM, the narrow SP
IMEM physical range, direct KSEG0/KSEG1 aliases of the existing 4 KiB SP DMEM
owner, exactly RI_SELECT at physical `0x0470000C`, immutable MI_VERSION at
physical `0x04300004`, exact RI_REFRESH at `0x04700010`, and generated
RCP-2 module DEVICE_TYPE, DEVICE_MANUF, and RDRAM_MODE reads while MI
RDRAM-register mode is enabled. SP-DMEM words are
readable only when the current cartridge bootstrap span classifies all four
offsets as staged production bytes; other concrete backing remains
unclassified. The RI_SELECT route reads the stored Machine-owned word without
side effects and rejects while that optional state is unavailable. Neighboring
RI registers remain direct target misses. This does not introduce mirroring,
general MMIO policy, a bus, or a generalized memory map.

Every Machine's existing `Mi` owner contains one immutable standard-retail
MI_VERSION raw word `0x02020102`. IO/RAC/RDP/RSP bytes derive as
`02/01/02/02`; they are not separately stored. Exact MI_VERSION `Lw`
returns `0x0000000002020102` with ordinary destination provenance and no
device mutation. Other MI reads, MI_VERSION writes, alternate identities, and
configuration surfaces remain absent.

The aligned `Sw` data route accepts direct KSEG0/KSEG1 aliases of direct RDRAM
and SP IMEM,
RI_MODE at physical `0x04700000`, RI_CONFIG at `0x04700004`, RI_CURRENT_LOAD at
`0x04700008`, RI_SELECT at `0x0470000C`, RI_REFRESH at `0x04700010`, exact
generated MI_INIT_MODE commands at `0x04300000`,
exact global RDRAM_DELAY at `0x03F80008`, and exact global RDRAM_REF_ROW at
`0x03F80014`, exact global RDRAM_DEVICE_ID at `0x03F80004`, and exact RCP 2.0
first-responder/direct-module RDRAM_DEVICE_ID, module/global RDRAM_MODE, and
module RAS_INTERVAL families exercised by generated x105 bring-up. RI_MODE stores operating-mode bits
1:0 and stop-active bits 2/3; RI_CONFIG stores only defined input bits 5:0, enable
bit 6, and CPU-store lineage; RI_CURRENT_LOAD snapshots stored configuration;
RI_SELECT accepts only exact x105 word `0x14` and replaces its source with
CPU-store lineage. RI_MODE bits above bit 3 and other destination-specific
unsupported inputs reject before mutation. MI_INIT_MODE accepts only low word
`0x0000010F`, stores initialization length 15 plus initialization mode true,
and arms one 16-byte transfer with CPU-store lineage. Global RDRAM_DELAY accepts
only `0x18082838` with that transfer and stores logical fields 5/7/3/1 plus
packed configuration `0x28381808` without changing RDRAM bytes. Other MI/RDRAM
words reject before mutation. Global RDRAM_REF_ROW accepts only low word zero,
stores the raw word/global-aperture/CPU provenance fact, and preserves the delay
configuration without changing RDRAM bytes. Global RDRAM_DEVICE_ID accepts
only `0x80000000` and records requested base `0x02000000`, global aperture,
and CPU-store provenance without relocating bytes or changing routing. Exact
RCP 2.0 first-responder DEVICE_ID accepts only low word zero with known source
lineage and no pending MI transfer, then records requested initial device ID
zero, exact aperture, addresses, and CPU-store provenance. It creates no
responder/module/completion fact at the earlier boundary. The completed path
accepts only source-defined generated mode words, one global mode word
`0xC4000000`, per-module RAS word `0x101C0A04`, profile-valid DEVICE_ID
mappings, and profile-derived RI_REFRESH `0x001E3634`; each records CPU
provenance. MI words `0x2000` and `0x1000` toggle RDRAM-register mode, while
generated zero records no invented transition. RCP 1.0, SP DMEM, every other
device/MMIO address, non-direct, and target-miss addresses reject without
routing. No generic store abstraction or broader address map is introduced.

The represented address-error entry owns BadVAddr, Cause code, branch-delay
flag, EPC, Status.EXL, and exception-vector `pc` / `next_pc` mutation. It does
not execute a load/store instruction, write a destination GPR, route a device,
or advance Count.

## Instruction production

`CpuInstructionWord` and `CpuInstructionFields` represent pure raw fixed-width
decode. `CpuInstructionIdentity` classifies primary, SPECIAL, REGIMM, COP0,
coarse coprocessor/cache/memory, and unknown identities. Recognition is not
execution support.

Instruction fetch represents:

- 4-byte alignment before target access;
- direct KSEG0/KSEG1 RDRAM word fetch;
- read-only SP DMEM word fetch;
- source classification that distinguishes cartridge-bootstrap SP DMEM bytes
  from unclassified Machine storage;
- explicit-address and current-PC composition;
- a named unavailable reset-vector PIF target;
- named non-direct, direct-target-miss, and lower-source rejections;
- selected fetch AdEL planning and narrow exception entry.

Fetch forms one big-endian instruction word. Fetch APIs do not themselves
decode, identify, execute, advance cadence, or enter exceptions; public
`Machine::step` owns the represented composition.
`Machine::inspect_current_cpu_instruction` exposes the current address, decoded
fields, identity, and source provenance without mutable CPU, COP0, memory, or
control-flow access.

## Public represented step

The represented sequence is:

`control-flow snapshot -> one fetch -> one decode -> one identity -> ordinary-control-flow planning or sequential next-PC staging -> one represented action -> one application`

At most one selected CPU-local helper or one Machine-owned load/store action
runs. Application owns exactly one of committed cadence, stopped cadence,
unsupported rollback, arithmetic-overflow entry, fetch/data-address-error
entry, or explicit rejection.

### CPU-local committed instruction families

The following identities execute through public `Machine::step`:

- shifts: `SLL`, `SRL`, `SRA`, `SLLV`, `SRLV`, `SRAV`, `DSLL`, `DSRL`,
  `DSRA`, `DSLL32`, `DSRL32`, `DSRA32`, `DSLLV`, `DSRLV`, `DSRAV`;
- register logical: `AND`, `OR`, `XOR`, `NOR`;
- HI/LO transfer: `MFHI`, `MTHI`, `MFLO`, `MTLO`;
- multiply: `MULTU` with unsigned low-32-bit operands and sign-extended HI/LO
  word halves;
- non-trapping integer: `ADDU`, `SUBU`, `DADDU`, `DSUBU`, `SLT`, `SLTU`;
- trapping integer: `ADD`, `SUB`, `DADD`, `DSUB`;
- immediate integer: `ADDI`, `DADDI`, `ADDIU`, `DADDIU`, `SLTI`, `SLTIU`;
- immediate logical and upper-immediate: `ANDI`, `ORI`, `XORI`, `LUI`.

Successful CPU-local execution commits the captured control-flow cadence and
advances represented COP0 Count once. Trapping arithmetic overflow writes no
destination GPR, restores speculative control flow, enters represented Cause
code 12 state, and does not advance Count or BadVAddr.

### Ordinary control flow and one delay slot

`BEQ`, `BNE`, `BLTZ`, exact `BEQL`, `BNEL`, `BLEZL`, `BGEZL`, `J`, `JAL`,
`JR`, and `JALR` execute through one
Machine-owned planning/application family. Conditional targets use wrapping instruction
PC+4 plus the sign-extended shifted displacement. J/JAL region bits come from
wrapping PC+4. JAL writes r31; JALR writes encoded `rd`; links are PC+8 under
the represented 32-to-64-bit sign-extension rule. JALR captures the old source
before an aliased link write, and link writes to r0 are discarded. Old r31 is
not a JAL input; old JALR `rd` is not an input except where it is also the
captured old `rs`. Bootstrap lineage remains truthful until the named CPU
instruction overwrites its destination.

Ordinary branches and jumps, plus taken likely branches, commit to exactly one explicit
CPU-owned delay-slot context. `next_pc` owns the selected target or
fall-through. The branch/jump and successful slot each advance Count once.
Successful slot completion clears context. Each represented not-taken likely
branch compares its architecturally defined complete known GPR values and
architecturally nullifies PC+4: it commits
directly to P+8/P+12, creates no delay context, and the slot is not executed,
committed, counted, mutated, or allowed to raise an exception. Reset and direct
synthetic PC staging clear stale context; rollback restores it.

A branch or jump encountered in a delay slot is explicitly unsupported and
restores all represented state without link, target, PC, Count, or COP0
mutation. Arithmetic-overflow, instruction-fetch AdEL, and data-AdEL entry from
a represented delay slot set Cause.BD, use the owning branch/jump PC for EPC,
advance the faulting slot Count by zero, do not commit the selected target, and
clear context on successful exception entry.

### Machine-owned aligned `Lw`

`Lw` executes through one Machine-owned plan/application rule. Planning reads
the old base, sign-extends the 16-bit immediate, performs wrapping represented
address arithmetic, checks alignment, classifies the target, and obtains the
complete source word before mutation. Direct RDRAM, known SP IMEM,
cartridge-bootstrap-staged SP DMEM, and the exact stored RI_SELECT word share
this semantic rule. SP-DMEM target classification records the exact source
cartridge offset; unclassified Machine backing rejects rather than becoming
known by virtue of concrete storage. RI_SELECT target classification records
its current cold-entry or CPU-store source and is side-effect free; every neighboring RI address
remains unsupported. A successful word is assembled big-endian, sign-extended
from 32 to 64 bits, written with GPR-zero and base/destination alias rules,
assigned `KnownInstructionResult` lineage when bootstrap state is active, and
commits
`pc` / `next_pc` plus Count exactly once.

Unaligned access delegates to the existing data-AdEL owner and exact BadVAddr
policy without destination write or normal cadence. Unknown bootstrap base,
unknown SP IMEM byte, unclassified SP DMEM, target miss, unsupported address
form, blocked exception entry, and lower read failure leave all represented
state unchanged. An unaligned SP-DMEM-shaped load uses the same AdEL owner,
including delay-slot EPC/BD and zero faulting-instruction Count.
An aligned Lw from an opaque SP-IMEM word now commits a narrow unavailable-value
transport: destination backing is canonical zero, while the exact original
unavailable source lineage is restored and remains mandatory for every later
consumer. The zero is never reclassified or exposed as known value truth. This
is sufficient for generated frame teardown without symbolic arithmetic;
unaligned access retains AdEL precedence and other unknown-source consumers
remain closed.

### Machine-owned aligned `Sw`

`Sw` executes through a separate immutable Machine-owned plan and applicator.
Planning captures the old base, applies the sign-extended immediate with the
same wrapping represented-address rule as `Lw`, checks word alignment before
source-value consumption, and accepts only direct KSEG0/KSEG1 aliases of direct
RDRAM or SP IMEM, or the exact represented RI, MI, global RDRAM, and generated
RCP-2 module-register targets. All
supported paths capture old `rt` backing
and source classification. A known SP-IMEM source stores four big-endian bytes
and replaces only those bytes' provenance with the instruction PC, source GPR,
and source lineage. An explicitly unavailable source may commit only after the
destination is classified as an aligned SP-IMEM word: it canonicalizes four
private bytes to zero and records aligned offset, instruction PC, source GPR
and lineage, and exact effective/CPU/physical addresses without any value bit.
A second opaque store replaces the word record; a known aligned overwrite
removes it and restores concrete readable truth. RI_CONFIG
accepts only words with zero undefined high bits, stores current-control input
bits 5:0 and enable bit 6 with the same CPU-store lineage, and changes no
memory. RI_CURRENT_LOAD requires stored RI_CONFIG and creates an event that
snapshots its input/enable fields with transfer-word/CPU-store evidence; it
creates no hardware output. RI_SELECT accepts only low word `0x14`, replaces
the prior value/source with exact CPU-store provenance, and does not consult
RI_CONFIG or RI_CURRENT_LOAD as authorization. RI_MODE stores its three
defined field facts, uses no prior RI state as authorization, and creates no
physical RI effect or timer. MI_INIT_MODE accepts only `0x0000010F`, stores
length 15 and initialization mode true, and arms one bounded pending transfer
with exact CPU-store provenance. Other represented commits cannot bypass it.
Global RDRAM_DELAY requires that transfer and low word `0x18082838`, stores the
logical broadcast fact with consumed lineage, consumes the transfer, and makes
current MI readback unavailable. Global RDRAM_REF_ROW accepts only low word
zero once no MI transfer is pending and stores a raw global-aperture fact with
CPU-store lineage; it neither requires nor mutates the prior delay fact and
interprets no refresh field. Global RDRAM_DEVICE_ID accepts only low word
`0x80000000` once no MI transfer is pending and stores requested physical base
`0x02000000` plus global-aperture/CPU provenance; it requires neither prior
RDRAM fact and changes no byte or address route. Initial RDRAM_MODE accepts
only `0x46C0C0C0`, stores one raw request plus CPU provenance, derives the
five named fields, and changes no byte or route. The generated bring-up family
additionally owns module/global modes, profile-valid DEVICE_ID mapping, module
RAS_INTERVAL `0x101C0A04`, and RI_REFRESH `0x001E3634`, with exact CPU
provenance and no generic register array. Direct RDRAM stores retain ordinary
big-endian byte truth; absent-module calibration stores are explicit no-effect
device responses and do not fabricate a module. All paths then commit
`pc` / `next_pc` and Count
once. `rs == rt` uses the old shared value and r0 transfers a known zero word.

Unaligned `Sw` enters AdES code 5 through the existing COP0 owner with exact
BadVAddr and sequential or delay-slot EPC/BD lineage. It performs no store or
normal cadence and advances Count zero times. Unknown base, unavailable source
to any non-SP-IMEM target, non-direct address, target miss, undefined RI_CONFIG
bits, unavailable RI_CONFIG for an
RI_CURRENT_LOAD event, unsupported RI_SELECT words, undefined RI_MODE high
bits, wrong/missing delay transfer, nonzero REF_ROW word, SP DMEM, blocked exception entry, and bounds failure preserve all
represented state. Other MI/RDRAM/device stores, other store identities, a generic store path, bus,
and generalized map remain absent.

### Machine-owned SP-IMEM byte access

`LBU` and `SB` are represented generally over the existing direct SP-IMEM
owner only. They use ordinary effective-address knownness, read-before-write,
zero-register, provenance, delay-slot, rejection, and Count owners. `LBU`
zero-extends a known byte; `SB` transfers the old source low byte. They do not
add SP-DMEM, RDRAM, device, generic byte-routing, or partial opaque-word
semantics.

### Machine-owned non-likely `BLTZ`

`RegimmBltz` extends the existing ordinary-control-flow planner and applicator
without generalizing REGIMM. Planning reads old known `rs`, reuses the exact
full-GPR signed comparison already owned by SLT/SLTI, and selects the common
wrapping branch target `PC + 4 + (sign_extend(offset16) << 2)` or wrapping
untaken successor `PC + 8`. Both paths schedule exactly one ordinary delay
slot. BLTZ writes no destination, creates no link, performs no annul, preserves
the source value/lineage, and advances Count once.

Unknown bootstrap source rejects before application. BLTZ inside an active
delay slot joins the existing control-flow rejection matrix before source
consumption. A represented slot fault retains the branch Count, advances the
slot Count zero times, uses the BLTZ PC for EPC with BD set, commits neither
target nor fall-through, and reuses the existing AdEL/AdES owner. BGEZ, likely
and link variants, and REGIMM traps remain unrepresented.

### Machine-owned bounded `MTC0` boot trio

`Cop0Mtc0` selects a closed Machine plan only for Cause register 13, Count
register 9, and Compare register 11 while the source-backed NTSC cold-x105
kernel handoff is active. Planning validates zero reserved low bits, the exact
destination, access scope, and a known old `rt`, then transfers only its low
32 bits. Unsupported destinations, malformed encodings, unavailable sources,
and other contexts reject before COP0, control-flow, or Count mutation.

Cause writes only software-pending IP1/IP0 (`0x00000300`) and makes that
two-bit state known; it preserves exception code, BD, timer pending, and all
other read-only state. Count installs the transfer word before normal cadence,
which then advances Count once and performs the existing Compare equality
check. Compare installs the transfer word and clears timer pending before
normal cadence, whose post-increment equality may relatch it. Successful MTC0
in an ordinary delay slot uses the existing slot cadence and creates no branch
or GPR write. Interrupt delivery, unrelated RI behavior, other MTC0
destinations, MFC0,
DMTC0, privilege completeness, and a generic CP0 register bank remain absent.

### Minimal RI and RI_REFRESH state

One private per-Machine `Ri` owner stores optional RI_MODE, RI_SELECT,
RI_CONFIG, RI_CURRENT_LOAD event, and RI_REFRESH state. The complete NTSC cold-cartridge x105 bootstrap
plan creates value zero with
`ColdX105Entry` provenance atomically with the coupled handoff. Construction,
general reset, ordinary bootstrap, incomplete selectors, and unsupported
profiles leave the state unavailable; repeated complete staging recreates the
same cold RI_SELECT fact and clears stale RI_MODE/RI_CONFIG/event/CPU-store state, and
independent Machines remain independent. Exact RI_SELECT `Sw` replaces that
zero with `0x14` and `CpuStoreWord` provenance; all other low words are an
explicit unsupported boundary, not a claimed hardware trap. The stored RI_SELECT word is
separate from the reset-kind selector and is never recomputed at read time.

The aligned-`Lw` planner recognizes exactly physical `0x0470000C` through the
existing direct KSEG0/KSEG1 aliases. It reads the stored word without side
effects, applies existing word sign extension, destination lineage, and
committed cadence, and rejects atomically if the state is unavailable.
The aligned-`Sw` planner recognizes exactly RI_MODE physical `0x04700000`,
RI_CONFIG physical `0x04700004`, RI_CURRENT_LOAD physical `0x04700008`, and
RI_SELECT physical `0x0470000C`
through the same aliases. RI_CONFIG
stores only current-control input bits 5:0, enable bit 6, and exact CPU-store
provenance; undefined high bits reject before mutation. RI_CURRENT_LOAD
requires stored RI_CONFIG and records an update event containing its field
snapshot plus transfer-word/CPU lineage. RI_SELECT `Sw` accepts only word
`0x14` and preserves both sibling facts; the existing `Lw` reads that updated
stored word and source without side effects. RI_MODE stores operating-mode
bits 1:0, stop-transmit-active bit 2, stop-receive-active bit 3, and exact
CPU-store provenance. Bits above bit 3 reject before mutation. The later store
replaces the earlier fields and source. RI_MODE/RI_CONFIG/RI_CURRENT_LOAD have no
read route. Exact physical `0x04700010` stores and reads profile-derived raw
RI_REFRESH word `0x001E3634`, from which module mask 3, optimize/enable, bank,
dirty-delay, and clean-delay facts derive. RI_LATENCY, general RI_SELECT
fields/values, and all other RI actions remain unsupported. No refresh timing,
current-control electrical process, NMI behavior,
generic register bank, MMIO framework, bus, or generalized map is represented.

### Immutable MI_VERSION and minimal initialization-mode state

One private per-Machine `Mi` owner stores immutable standard-retail
MI_VERSION word `0x02020102`, one optional initialization-mode state, one
bounded pending transfer, and optional RDRAM-register access-mode state. Its derived version bytes are IO/RAC/RDP/RSP
`02/01/02/02`. Construction, general reset, and complete cold-x105 bootstrap
preserve identity while leaving both mutable facts unavailable. Exact direct
aliases of physical `0x04300004` read the immutable word with ordinary Lw
semantics. Exact direct aliases of physical `0x04300000` accept only x105
word `0x0000010F`, store initialization length 15 and initialization mode true,
and retain instruction PC, source GPR, and old source lineage. The write-command
bit is not stored as a readback bit. The write arms length 15 / 16 repeated
bytes for exactly one source-proven RDRAM_DELAY consumer. Repeated bootstrap
clears stale state/transfer; failed bootstrap and every rejection preserve
them; Machines remain independent. Exact generated commands `0x2000` and
`0x1000` enable and disable module-register access; zero records its
source-clear no-change command. Reads of module registers remain closed while
disabled. No unrelated MI command or readback is implied.

The existing `Rdram` byte owner also owns the immutable capacity-derived fixed
profile, module collection, mapping, calibration response, optional global
delay/raw REF_ROW/global DEVICE_ID request, and generated RCP-2 module facts.
Physical `0x03F80008` accepts only `0x18082838` with the exact transfer,
then stores fields 5/7/3/1 and logical packed value `0x28381808` with CPU and
consumed-MI provenance. It changes no bytes, consumes the transfer, and makes
current MI state unavailable because post-transfer readback is not source-clear.
Physical `0x03F80014` stores raw zero without interpreting REF_ROW; physical
`0x03F80004` stores requested base `0x02000000`. RCP-2 module apertures use
exact spacing `0x400`; present modules expose DEVICE_TYPE `0xB0190000`, fixed
manufacturer `0x0500`, mode readback, DEVICE_ID relocation, and RAS_INTERVAL
`0x101C0A04`. Manual response score is
`10 * min(nominal_input + 1, 8)`; the response byte monotonically exposes
`min(nominal_input + 1, 8)` one bits per read. Absent apertures return zero
under the active probe and create no module. Guest execution selects automatic
input 7 and final mode `0xC6808080` for both current modules, mapping them at
physical `0x00000000` and `0x00200000`. Outside active calibration, direct
reads return backing bytes. No EBUS/DP interrupt, analog/timing model, general
register array, arbitrary topology, generic MMIO, or bus is represented.

### Other represented outcomes

- `SYNC` commits as an explicit no-effect instruction.
- `SYSCALL` and `BREAK` commit and return a stopped outcome; no syscall/break
  exception or host stop policy is implied.
- source-classified unknown and selected known-unimplemented identities return
  unsupported and restore `pc` / `next_pc` without Count advancement.
- selected instruction-fetch faults enter represented AdEL state without
  normal commit or Count advancement.
- non-converting fetch faults, invocation rejection, unrepresented identities,
  and entry-guard failure return explicit errors without a false success claim.

The public result types describe only these represented categories. They are
not an all-future step result or a complete N64 execution contract.

### Bootstrap knownness and earned BOOT-2

While Machine-owned cartridge-bootstrap state is active, each represented
CPU-local instruction checks every architecturally consumed GPR before helper
invocation. An unknown PIF-produced source rejects before GPR, HI/LO, COP0,
memory, committed `pc` / `next_pc`, or Count mutation, and restores staged
control flow. Successful nonzero GPR writes receive explicit
`KnownInstructionResult` lineage; writes to GPR zero remain discarded and its
architectural-zero source remains known.

The accepted private no-window proof commits one cartridge-derived
`SpecialAdd` at `0xA4000040` through public `Machine::step`. Known r29 and r0
produce r9=`0xFFFFFFFFA4001FF0`; r9 changes from unknown to a known instruction
result, `pc / next_pc` become `0xA4000044 / 0xA4000048`, and Count advances
from 0 to 1. This earns **BOOT-2 — ROM-derived instruction committed with
complete represented state lineage**.

The next instruction is `Lw` at `0xA4000044`, using known r9 to compute CPU
address `0xA4001000`, which routes to SP IMEM offset zero. In the accepted
private no-firmware observation it is rejected without partial mutation because
the first consumed SP IMEM byte is `Unknown`. With generated 1,984-byte input
and an explicit pinned profile, a synthetic test proves the profiled production
event makes the source word known and lets this represented `Lw` commit. No
private PIF input was used, so that synthetic proof does not advance the
authentic checkpoint.
Generated tests separately prove the NTSC cold x105 coupled creation point and
32,185 public-step commits. The accepted thirty-three-step prefix is followed
by the exact RI_CONFIG `Sw`, a generated wait-counter setup, and exactly 8,000
loop iterations comprising 32,000 commits. The final synthetic state is
PC/next-PC `0xA40000DC / 0xA40000E0`, Count `32019`, and s1 zero; RI_CONFIG
still holds input zero and enable true. Commit 32,036 stores r0 to
RI_CURRENT_LOAD and snapshots that configuration; commit 32,037 constructs
r9=`0x14`; commit 32,038 stores that word to RI_SELECT and replaces its source
with CPU-store provenance. Commit 32,039 stores RI_MODE zero. The generated
four-iteration NOP/Addi/Bne/NOP wait has three taken branches, one untaken
branch, and four delay slots. Commit 32,057 constructs `0x0E`; commit 32,058
replaces RI_MODE with operating mode 2 and both stop-active flags. The
generated 32-iteration Addi/Bne/Ori wait has 31 taken branches, one untaken
branch, and 32 executions of the ORI delay slot, leaving r9=`0x10F`. Final
PC/next-PC are `0xA4000118 / 0xA400011C`, Count is `32139`, and s1 is zero.
Commit 32,156 stores exact word `0x10F` to MI_INIT_MODE at CPU `0xA4300000`
(physical `0x04300000`), creating length 15 and initialization mode true with
CPU-store provenance. The following `Lui`/`Ori` constructs r9=`0x18082838`.
Commit 32,159 stores it to global RDRAM_DELAY CPU `0xA3F80008` (physical
`0x03F80008`), creating the 5/7/3/1 logical fact, consuming the MI transfer,
and making current MI readback unavailable. Commit 32,160 stores raw zero to
global RDRAM_REF_ROW CPU `0xA3F80014` (physical `0x03F80014`) with architectural-
zero provenance while preserving the delay fact. Commit 32,161 executes
`Lui r9,0x8000`, producing `0xFFFFFFFF80000000` with generated-instruction
lineage. Commit 32,162 stores the low word to global RDRAM_DEVICE_ID CPU
`0xA3F80004` (physical `0x03F80004`) and records requested base `0x02000000`
without moving bytes or routes. Fourteen CPU-local setup commits leave
PC/next-PC `0xA400016C / 0xA4000170`, Count `32160`, and 32,176 total commits.
Commit 32,177 reads MI_VERSION `0x02020102` at CPU `0xA4300004` (physical
`0x04300004`). `Lui`/`Ori` construct `0x01010101`; Bne takes the
RCP 2.0 path and its Nop delay slot executes once. `Addiu` selects spacing
`0x400`, and `Ori` builds first-responder base
`0xFFFFFFFFA3F08000`. Commit 32,184 stores low word zero to first-responder
non-global RDRAM_DEVICE_ID physical `0x03F08004`, recording a bounded
assignment request with r14's generated `Addu` lineage. Commit 32,185 executes
`Addiu r21,r15,0x000C`, producing `0xFFFFFFFFA3F0000C`. PC/next-PC are then
`0xA40001A0 / 0xA40001A4`, Count is `32169`. The generated
`Jal 0xA400087C` replaces retained bootstrap r31 with
`0xFFFFFFFFA40001A8`, records JAL lineage, and schedules its Nop slot. The
slot commits once and enters InitCCValue. Five supported entry commits leave
PC/next-PC `0xA4000890 / 0xA4000894`, Count `32176`, and 32,192 total commits.
Four r2-r5 stores commit opaque SP-IMEM words at offsets
`0xEF0/0xEF4/0xEF8/0xEFC`; twenty known-source saves then commit through
`0xA40008EC`. PC/next-PC become `0xA40008F0 / 0xA40008F4`, Count is `32200`,
and total commits are 32,216. The FindCC JAL word `0x0D000261`, target
`0xA4000984`, link `0xFFFFFFFFA40008F8`, and its Nop slot commit. Six setup
instructions reach BEQL word `0x53400018` with r26=1 and r0=0. The branch is
not taken; `Or r2,r0,r0` at `0xA40009A0` is nullified and r2 remains
`UnknownPifProduced`. TestCCValue and WriteCC then commit through public
stepping. Nominal input zero becomes `0x3F`; base flags `0x46000000` and
scattered bits `0x00C0C0C0` produce `0x46C0C0C0`. PC/next-PC become
`0xA4000BB8 / 0xA4000BC4`, Count is `32243`, and total commits are 32,259.
The `Sw` at that PC targets CPU `0xA3F0000C` / physical `0x03F0000C` and
commits one exact request through the existing BNE delay slot. WriteCC restores
ra from known SP IMEM, restores sp, and returns through JR/Nop to
`0xA4000A24`. `Or` zeroes s8 and `Addiu` constructs r26 all ones. PC/next-PC
are `0xA4000A2C / 0xA4000A30`, Count is `32250`, and total commits are
32,266. From this accepted frontier, 214,734 further public
`Machine::step` calls commit the three initial test stores, TestCCValue inner
and ten-read loops, all four FindCC samples, ConvertManualToAuto, ReadCC,
WriteCC, discovery of two present modules and one absent probe, DEVICE_TYPE and
manufacturer reads, RAS writes, global/temporary/final DEVICE_ID requests,
final automatic modes, settling reads, RI_REFRESH write/read, guest size
store, and frame teardown. Final module modes are `0xC6808080`; RAS words are
`0x101C0A04`; mappings cover `0x00000000..0x003FFFFF`; RI_REFRESH is
`0x001E3634`; and guest-written size is `0x00400000`. At total commit 247,000,
Count is `246984` and PC/next-PC are `0xA4000400 / 0xA4000404`. Current word
`0x4080E000`, `Cop0Mtc0 zero,$28` (C0_TAGLO), is inspected but not executed as
the first cache-specific frontier. These tests prove synthetic fixed-profile
CPU/device composition, not an authentic IPL2-to-IPL3 run, analog accuracy,
NMI execution, or game compatibility.
One-word staging would be both incomplete and unauthorized: the observed x105
prelude consumes eight words and mutates through offset `0x02b`.
BOOT-3, authentic bootstrap handoff, and cartridge entry `0x80000400` are not
reached.

## Explicitly absent execution and hardware

Identity classification may name instructions that the public step does not
execute. Current explicit absences include:

- branch-likely identities other than exact BEQL/BNEL/BLEZL/BGEZL, REGIMM
  identities other than BLTZ/BGEZL, COP0 branches, and execution of a branch or
  jump inside a delay slot;
- CPU load/store identities other than the detailed aligned `Lw`/`Sw` and
  SP-IMEM-only `LBU`/`SB` surfaces, including unaligned merge operations,
  unclassified SP-DMEM access, and unearned device/register targets;
- signed multiply, divide, trap, every COP0 instruction except the bounded
  MTC0 boot trio, ERET, and LL/SC execution;
- cache instruction execution and MTC0 destinations beyond the bounded boot
  trio; current execution stops before C0_TAGLO;
- interrupt delivery, complete COP0 behavior, TLB, and MMU;
- completed PIF emulation, proprietary PIF/BIOS execution, general CIC support,
  PI DMA, authentic firmware-executed bootstrap handoff, and cartridge-entry
  execution;
- authentic PIF firmware/revision validation, firmware execution, and
  coupled PAL/MPAL or other-family/NMI/DD handoff-state production;
- authentic private-firmware-backed SP IMEM observations, RSP/COP2 execution, and SP
  register/status/DMA/control behavior;
- a broad bus or memory map, device/MMIO routing, DMA, and N64 scheduling or
  timing;
- renderer, input, window, audio, ROM-path host, and platform event loop;
- performance, broad hardware compatibility, and game compatibility evidence.

Future capability requires its own bounded product decision and source-backed
proof. A recognized enum variant, private helper, historical behavior, or green
test outside public composition is not enough.

## No-window proof surfaces

`fn64_machine_probe` covers deterministic construction and reset inspection.
It does not call `Machine::step`.

`fn64_step_probe` uses generated instruction words and synthetic addresses and
calls only public `Machine::step` for execution. Its 174 cases cover:

- CPU-local committed success;
- arithmetic-overflow exception entry;
- SYNC committed no-effect;
- SYSCALL stopped;
- BREAK stopped;
- unsupported rollback;
- selected instruction-fetch AdEL;
- source-clear fetch rejection;
- cartridge-staged SP-DMEM `Lw` success with exact provenance;
- unclassified SP-DMEM rejection;
- delay-slot SP-DMEM-shaped AdEL;
- SP-IMEM concrete and opaque `Sw` commit, big-endian bytes/provenance, opaque
  cause/coherence/sentinel/overwrite/equality/lifecycle, concrete `Lw` round
  trip, narrow opaque-Lw unavailable-lineage transport, r0 and `rs == rt`, AdEL/AdES, delay-slot
  cadence, and fail-closed operand/device-target rejection;
- BLTZ taken/untaken/full-width signed discrimination, positive/negative
  targets, ordinary slot commit, slot exception, nested-control-flow rejection,
  and unknown-source rejection;
- BEQL taken-slot cadence, not-taken architectural annul, unavailable-source
  rejection, active-delay rejection, generated annul, TestCCValue/WriteCC
  call composition, exact RDRAM_MODE request, WriteCC return, and calibration
  response-test boundary;
- bounded MTC0 Cause masking/knownness, Count and Compare write-before-cadence,
  timer clear/relatch ordering, ordinary-slot success, and atomic rejection;
- exact RI_SELECT lifecycle/read/alias/AdEL/target-miss/rejection behavior,
  cold BNE and NOP-slot behavior, and the high-SP-IMEM stack save;
- exact RI_CONFIG lifecycle/fields/provenance/aliases/reserved-bit/AdES/slot and
  atomic-rejection behavior;
- exact RI_CURRENT_LOAD config dependency/event/provenance/aliases/AdES/slot,
  lifecycle, and atomic-rejection behavior;
- exact RI_SELECT CPU write/value/provenance/read-after-write/aliases/AdES/slot,
  lifecycle, and atomic unsupported-value behavior;
- RI_MODE defined fields/provenance/replacement/aliases/reserved-bit rejection,
  AdES, slot cadence, reset/bootstrap lifecycle, and independent Machines;
- exact MI_INIT_MODE value/state/provenance/aliases/rejections, AdES, slot
  cadence, reset/bootstrap lifecycle, no-read boundary, and independent Machines;
- bounded pending-transfer ownership plus exact RDRAM-delay fields, CPU/MI
  provenance, consumption, and post-transfer unavailable readback;
- exact raw-zero global RDRAM_REF_ROW ownership, CPU provenance, aliases,
  lifecycle, AdES, delay-slot cadence, narrow routing, and atomic rejection;
- exact global RDRAM_DEVICE_ID request ownership, raw word/requested-base CPU
  provenance, aliases, lifecycle, AdES, delay-slot cadence, narrow routing,
  unchanged bytes/routes, and atomic rejection;
- immutable MI_VERSION raw word/derived fields/lifecycle, exact aliases,
  ordinary and delay-slot Lw cadence, destination provenance, AdEL, closed
  neighboring reads, and state preservation;
- exact RCP 2.0 first-responder DEVICE_ID zero-request ownership, provenance,
  aliases, lifecycle, AdES/delay-slot cadence, narrow aperture routing, and
  atomic rejection;
- exact initial non-global RDRAM_MODE raw request/derived fields/provenance,
  aliases, exact-value rejection, AdES/delay-slot cadence, lifecycle, unchanged
  bytes/routes, and closed readback/physical-effect boundary;
- 247,000-step generated x105 composition through the exact 8,000-iteration CPU
  loop, both RI_MODE writes, both bounded CPU waits, the exact MI_INIT_MODE
  write, delay-word construction, RDRAM_DELAY and RDRAM_REF_ROW commits,
  DEVICE_ID-value LUI/store, fourteen CPU-local setup commits, MI_VERSION read,
  guest-selected RCP 2.0 Bne/Nop slot, spacing/base setup, first-responder
  RDRAM_DEVICE_ID commit, initial RDRAM_MODE-address `Addiu`, generated JAL and
  Nop slot, five InitCCValue entry commits, four opaque r2-r5 saves, twenty
  concrete saves, FindCC JAL/Nop, exact BEQL annul, TestCCValue and WriteCC,
  exact `0x46C0C0C0` request commit, complete fixed-profile calibration,
  two-module discovery/final mapping, module reads and RAS writes, RI_REFRESH,
  guest-detected 4 MiB size, frame teardown, and the unexecuted cache-specific
  C0_TAGLO frontier;
- taken and untaken ordinary branches with one slot;
- JAL link behavior;
- JALR source/destination alias behavior;
- delay-slot exception EPC/BD behavior; and
- branch-in-delay-slot rejection.

Both probes use plain text and end with `result: ok` on success. They prove only
their named represented facts and do not launch a ROM, window, audio device, or
runtime host.

`fn64_boot_probe` is a separate bounded inspection instrument. Its host shell
owns one input path and file read, passes owned bytes into the core, and reports
Machine-owned staging, current-instruction provenance, committed effects, and
the first explicit frontier. It also accepts one optional explicit `--pif-rom`
path, passes those owned bytes into structural Machine validation, and reports
only absent/accepted classification and size. A separate explicit
`--pif-profile` selects one parsed Machine profile; inspection duplicates no
copy layout and mutates no SP IMEM. Generated CLI tests prove explicit-profile
materialization and no-search behavior. The four explicit cold-x105 selector
options can request the NTSC-only coupled handoff; generated CLI proof reports
the exact staged state, while PAL/MPAL requests fail closed. Against the
accepted private cartridge input without PIF firmware it reproduces BOOT-2
after two attempted steps and one commit, then reports the represented `Lw`
rejection at unknown SP IMEM offset zero. The input remains untracked and
is identified externally only by digest and size; no ROM bytes are committed or
packaged. This proof does not belong to the default forward gate and does not
claim BOOT-3 or game compatibility.

## Required gate

From repository root:

```sh
./rust/verify-forward
```

The gate owns the required order: formatting, warnings-denied clippy, complete
Rust tests, the machine probe, and the step probe. It ends with
`forward gate: ok` on success.

A green gate proves the bounded current Rust source at the tested commit. It
does not independently prove the private BOOT-2 runtime observation,
compatibility, performance, host runtime, or any absent capability listed
above.

## Update rule

Change this ledger only when represented source capability or explicit absence
changes. Project phase belongs in
[CURRENT_STATE.md](../docs/context/CURRENT_STATE.md), stable boundary law in the
[subsystem pages](../docs/INDEX.md), decisions in
[DECISION_LOG.md](../docs/context/DECISION_LOG.md), history in
[PROJECT_HISTORY.md](../docs/context/PROJECT_HISTORY.md), and measured results
in identified evidence artifacts.
