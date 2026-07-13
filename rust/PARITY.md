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
| `Rdram` | 4 MiB zero-filled storage and checked raw fixed-width reads | no general bus, device routing, or CPU instruction semantics |
| `SpDmem` | 4 KiB zero-filled storage, checked reads, and private Machine-owned range staging for the normalized bootstrap span | no public write surface, DMA, RSP, or COP2 execution |
| `SpImem` | 4 KiB private backing storage, per-byte provenance/knownness, checked known big-endian word reads, and an atomic profiled-copy constructor | no public mutable access, profile policy, SP register/status/DMA, or RSP execution |
| `Machine` | Cartridge, optional accepted PifFirmware and PifIpl2Profile, explicit handoff selectors, Cpu, Rdram, SpDmem, SpImem, bootstrap provenance/GPR-knownness/COP0/control-flow state, private RDRAM reservation state, powered/reset state, represented fetch/data composition, and public step composition | no hidden global machine, platform clock, file path, renderer, audio, input, or event loop |
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
completed-control-transfer sources are inspectable. All other inherited GPRs
remain `Unknown`; Count, Compare, EPC, BadVAddr, Cause, and timer state receive
no source-backed handoff claim. PAL/MPAL coupled requests fail before mutation.

There is no power-off transition, boot reset, PIF execution source, broad
mutable CPU accessor, savestate format, or serialization contract.

## Storage and CPU-address access

RDRAM supports checked raw `u8`, `u16_be`, `u32_be`, and `u64_be` reads.
Machine-owned raw writes support the same widths, validate the complete span
before mutation, write multi-byte values in big-endian order, and invalidate
only overlapping private reservation state.

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
the first unknown byte otherwise. Only test builds can directly stage
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

The aligned `Lw` data route accepts direct KSEG0/KSEG1 RDRAM and the narrow SP
IMEM physical range only. It does not introduce mirroring, MMIO policy, a bus,
or a generalized memory map.

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

At most one selected CPU-local helper runs. Application owns exactly one of
committed cadence, stopped cadence, unsupported rollback, arithmetic-overflow
entry, fetch-address-error entry, or explicit rejection.

### CPU-local committed instruction families

The following identities execute through public `Machine::step`:

- shifts: `SLL`, `SRL`, `SRA`, `SLLV`, `SRLV`, `SRAV`, `DSLL`, `DSRL`,
  `DSRA`, `DSLL32`, `DSRL32`, `DSRA32`, `DSLLV`, `DSRLV`, `DSRAV`;
- register logical: `AND`, `OR`, `XOR`, `NOR`;
- HI/LO transfer: `MFHI`, `MTHI`, `MFLO`, `MTLO`;
- non-trapping integer: `ADDU`, `SUBU`, `DADDU`, `DSUBU`, `SLT`, `SLTU`;
- trapping integer: `ADD`, `SUB`, `DADD`, `DSUB`;
- immediate integer: `ADDI`, `DADDI`, `ADDIU`, `DADDIU`, `SLTI`, `SLTIU`;
- immediate logical and upper-immediate: `ANDI`, `ORI`, `XORI`, `LUI`.

Successful CPU-local execution commits the captured control-flow cadence and
advances represented COP0 Count once. Trapping arithmetic overflow writes no
destination GPR, restores speculative control flow, enters represented Cause
code 12 state, and does not advance Count or BadVAddr.

### Ordinary control flow and one delay slot

`BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR` execute through one Machine-owned
planning/application family. Conditional targets use wrapping instruction
PC+4 plus the sign-extended shifted displacement. J/JAL region bits come from
wrapping PC+4. JAL writes r31; JALR writes encoded `rd`; links are PC+8 under
the represented 32-to-64-bit sign-extension rule. JALR captures the old source
before an aliased link write, and link writes to r0 are discarded.

Every taken or untaken branch/jump commits to exactly one explicit CPU-owned
delay-slot context. `next_pc` owns the selected target or fall-through. The
branch/jump and successful slot each advance Count once. Successful slot
completion clears context. Reset and direct synthetic PC staging clear stale
context; rollback restores it.

A branch or jump encountered in a delay slot is explicitly unsupported and
restores all represented state without link, target, PC, Count, or COP0
mutation. Arithmetic-overflow, instruction-fetch AdEL, and data-AdEL entry from
a represented delay slot set Cause.BD, use the owning branch/jump PC for EPC,
advance the faulting slot Count by zero, do not commit the selected target, and
clear context on successful exception entry.

### Machine-owned aligned `Lw`

`Lw` executes through one Machine-owned plan/application rule. Planning reads
the old base, sign-extends the 16-bit immediate, performs wrapping represented
address arithmetic, checks alignment, classifies the target, and obtains all
four source bytes before mutation. Direct RDRAM and known SP IMEM share this
semantic rule. A successful word is assembled big-endian, sign-extended from
32 to 64 bits, written with GPR-zero and base/destination alias rules, assigned
`KnownInstructionResult` lineage when bootstrap state is active, and commits
`pc` / `next_pc` plus Count exactly once.

Unaligned access delegates to the existing data-AdEL owner and exact BadVAddr
policy without destination write or normal cadence. Unknown bootstrap base,
unknown SP IMEM byte, target miss, unsupported address form, blocked exception
entry, and lower read failure leave all represented state unchanged.

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
Generated tests separately prove the NTSC cold x105 coupled creation point:
one instruction consumes source-backed t3, and the existing SP-IMEM `Lw`
consumes bytes produced by the selected copy. These tests are synthetic
composition proof and are not an authentic IPL2-to-IPL3 run.
One-word staging would be both incomplete and unauthorized: the observed x105
prelude consumes eight words and mutates through offset `0x02b`.
BOOT-3, authentic bootstrap handoff, and cartridge entry `0x80000400` are not
reached.

## Explicitly absent execution and hardware

Identity classification may name instructions that the public step does not
execute. Current explicit absences include:

- branch-likely annul, REGIMM branches, COP0 branches, and execution of a
  branch or jump inside a delay slot;
- CPU load/store instructions other than aligned `Lw`, plus unaligned merge
  operations;
- multiply, divide, trap, COP0 instruction, ERET, and LL/SC execution;
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
calls only public `Machine::step` for execution. Its fourteen cases cover:

- CPU-local committed success;
- arithmetic-overflow exception entry;
- SYNC committed no-effect;
- SYSCALL stopped;
- BREAK stopped;
- unsupported rollback;
- selected instruction-fetch AdEL;
- source-clear fetch rejection;
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
