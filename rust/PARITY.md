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
- The product is incomplete and headless. Nothing here claims cartridge boot,
  game compatibility, timing accuracy, or host-runtime support.

## Represented owners

| Owner | Represented truth | Explicit boundary |
| --- | --- | --- |
| `Cartridge` | normalized owned bytes, source layout, parsed header metadata, entry/IPL3-span inspection, range-checked byte reads | no filesystem path, CPU mapping, boot policy, or execution |
| `Cpu` | 32 GPRs, HI/LO, `pc` / `next_pc`, and the represented COP0 subset | no host cadence, full ISA, interrupt controller, or TLB/MMU |
| `Rdram` | 4 MiB zero-filled storage and checked raw fixed-width reads | no general bus, device routing, or CPU instruction semantics |
| `SpDmem` | 4 KiB zero-filled storage and checked read-only access used by represented instruction fetch | no SP IMEM, public write surface, DMA, RSP, or COP2 execution |
| `Machine` | Cartridge, Cpu, Rdram, SpDmem, private RDRAM reservation state, powered/reset state, represented fetch/data composition, and public step composition | no hidden global machine, platform clock, file path, renderer, audio, input, or event loop |
| `fn64-inspection` | two no-window proof programs over public core APIs | no machine truth, ROM-path host, runtime loop, or compatibility authority |

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

These are byte and metadata facts. They do not map cartridge bytes into the CPU
address space, select an execution entry, model PIF/CIC behavior, or establish
boot.

## Machine lifecycle and state

Construction and `Machine::reset` establish the represented non-boot state:

- `pc = 0xbfc00000` and `next_pc = 0xbfc00004`;
- zeroed GPR, HI/LO, represented COP0, RDRAM, and SP DMEM state;
- cleared private CPU/RDRAM reservation state;
- `powered_on = true`;
- the Machine-owned Cartridge is preserved across reset.

`Machine::stage_cpu_pc` is a narrow deterministic staging surface. It sets the
selected `pc` and establishes `next_pc = pc.wrapping_add(4)` without fetching
or executing.

There is no power-off transition, boot reset, PIF byte source, broad mutable
CPU accessor, savestate format, or serialization contract.

## Storage and CPU-address access

RDRAM supports checked raw `u8`, `u16_be`, `u32_be`, and `u64_be` reads.
Machine-owned raw writes support the same widths, validate the complete span
before mutation, write multi-byte values in big-endian order, and invalidate
only overlapping private reservation state.

Direct CPU-address classification represents KSEG0 and KSEG1 aliases into the
4 MiB RDRAM span. Direct fixed-width value APIs compose classification with raw
RDRAM access. Raw storage offsets do not impose CPU alignment; CPU-data access
APIs separately enforce byte/halfword/word/doubleword alignment and select
represented AdEL or AdES entry for alignment faults and aligned direct-target
rejection.

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
- explicit-address and current-PC composition;
- a named unavailable reset-vector PIF target;
- named non-direct, direct-target-miss, and lower-source rejections;
- selected fetch AdEL planning and narrow exception entry.

Fetch forms one big-endian instruction word. Fetch APIs do not themselves
decode, identify, execute, advance cadence, or enter exceptions; public
`Machine::step` owns the represented composition.

## Public represented step

The represented sequence is:

`control-flow snapshot -> sequential next-PC staging -> one fetch -> one decode -> one identity -> one represented action -> one application`

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

## Explicitly absent execution and hardware

Identity classification may name instructions that the public step does not
execute. Current explicit absences include:

- branch, branch-likely, jump, link, and complete delay-slot execution;
- CPU load/store instructions and unaligned merge operations;
- multiply, divide, trap, COP0 instruction, ERET, and LL/SC execution;
- interrupt delivery, complete COP0 behavior, TLB, and MMU;
- cartridge execution mapping, PIF/BIOS execution, CIC behavior, and an
  authentic boot path;
- SP IMEM, RSP/COP2 execution, and SP register/control behavior;
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
calls only public `Machine::step` for execution. Its eight cases cover:

- CPU-local committed success;
- arithmetic-overflow exception entry;
- SYNC committed no-effect;
- SYSCALL stopped;
- BREAK stopped;
- unsupported rollback;
- selected instruction-fetch AdEL;
- source-clear fetch rejection.

Both probes use plain text and end with `result: ok` on success. They prove only
their named represented facts and do not launch a ROM, window, audio device, or
runtime host.

## Required gate

From repository root:

```sh
./rust/verify-forward
```

The gate owns the required order: formatting, warnings-denied clippy, complete
Rust tests, the machine probe, and the step probe. It ends with
`forward gate: ok` on success.

A green gate proves the bounded current Rust source at the tested commit. It
does not prove boot, compatibility, performance, host runtime, or any absent
capability listed above.

## Update rule

Change this ledger only when represented source capability or explicit absence
changes. Project phase belongs in
[CURRENT_STATE.md](../docs/context/CURRENT_STATE.md), stable boundary law in the
[subsystem pages](../docs/INDEX.md), decisions in
[DECISION_LOG.md](../docs/context/DECISION_LOG.md), history in
[PROJECT_HISTORY.md](../docs/context/PROJECT_HISTORY.md), and measured results
in identified evidence artifacts.
