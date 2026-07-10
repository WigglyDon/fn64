# fn64 Machine Core

This tracked workspace is fn64's sole current product implementation. The
former C++ machine, host, proof, and CMake lane is retired to Git history; its
unported behavior is intentionally absent and no parity claim is made. Rust is
implementation material rather than fn64's product identity.

## Identity

fn64's public identity is a small headless machine core. Public names should
describe owned N64 machine truth: `Machine`, `Cartridge`, `Cpu`, `Rdram`,
`RomSourceLayout`, `byte_order`, `metadata`, `registers`, `cop0`, `reset`,
`step`, `proof`, and `inspection`.

Hosts are thin host shells when they are earned. SDL, window, renderer,
graphics, frontend/backend, platform, and engine wording must not become
machine-core identity. The `rust/` directory, Cargo files, and cargo logs are
workflow/tooling context only.

Do not describe fn64 as "a Rust emulator"; that phrasing is explicitly rejected
because language choice must not leak into machine semantics.

## Layout Law

Modules in this implementation are named after owned machine truth. Current
owners are:

- `cartridge`
- `cartridge::byte_order`
- `cartridge::metadata`
- `machine`
- `machine::rdram_reservation`
- `cpu`
- `cpu::address`
- `cpu::cop0`
- `cpu::instruction`
- `cpu::registers`
- `cpu::scalars`
- `rdram`
- `sp_dmem`

The `fn64-inspection` crate is outside `fn64-core` because it owns only
no-window construction/reset and represented-step probe process plumbing,
deterministic text output, assertion policy, and exit status. Machine, CPU,
RDRAM, Cartridge, reset, and represented-step truth remain owned by
`fn64-core`.

Broad bucket modules such as `util`, `common`, `misc`, `helpers`, `engine`,
`platform`, `graphics_api`, or `emulator` are intentionally absent. Add a
module only when a specific fn64 truth owner has been earned.

## Current Scope

Earned Rust behavior is limited to:

- cartridge byte-order normalization, metadata parsing, entry inspection, and
  range-checked byte reads
- Machine construction ownership of Cartridge, Cpu, and Rdram
- Machine-owned CPU/RDRAM reservation construction/default state and private
  staging/setup/invalidation state
- RDRAM construction size, zero-fill ownership, raw byte/u16_be/u32_be/u64_be
  storage reads, and raw byte/u16_be/u32_be/u64_be storage writes by storage
  offset
- direct CPU-address classification for KSEG0/KSEG1 RDRAM aliases into raw
  RDRAM offsets
- direct CPU-addressed RDRAM value reads and writes for u8/u16_be/u32_be/u64_be
  through the sealed direct-classification and raw RDRAM access seams
- pure CPU data-address alignment checks and address-error exception-class
  selection for byte/halfword/word/doubleword read/write candidates
- narrow CPU-owned data address-error exception entry from sealed
  `CpuDataAddressError`, mutating only BadVAddr, the local exception code,
  branch-delay flag, Status.EXL, EPC, PC, and next PC according to the current
  local C++ entry rule
- Machine-owned direct RDRAM CPU data read/write value access that preflights
  alignment, enters the narrow address-error path on multi-byte alignment
  faults or aligned direct target rejection, and otherwise uses the sealed
  direct RDRAM value access seams
- Machine-owned reset of the currently represented non-boot power-on state:
  CPU scalar/GPR/COP0 state, RDRAM bytes, CPU/RDRAM reservation state, and the
  local powered-on flag, while preserving the owned Cartridge
- raw CPU instruction-word field decoding from an already-formed `u32` into
  opcode, register, shift amount, function, immediate, and jump-target fields
- pure CPU instruction identity classification from already-decoded raw fields
  into the source-clear C++ identity family, including primary opcode,
  SPECIAL/funct, REGIMM/rt, COP0 subidentity, and unknown identity boundaries
- Machine-owned CPU instruction-fetch target classification for 4-byte-aligned
  CPU addresses, naming direct RDRAM, SP DMEM, unavailable PIF reset fetch,
  non-direct unsupported fetches, and direct-target misses without reading memory
- represented Machine-owned SP DMEM storage, zero-filled construction/reset, and
  read-only SP DMEM instruction-word fetch from a classified SP DMEM offset
- Machine-owned direct KSEG0/KSEG1 RDRAM CPU instruction-word fetch that checks
  4-byte instruction alignment, reads one big-endian RDRAM word through sealed
  direct RDRAM value access, and returns `CpuInstructionWord`
- Machine-owned explicit-address CPU instruction-word fetch over already
  represented direct RDRAM and SP DMEM targets, returning named errors for
  unaligned, non-direct, direct-target-miss, and unavailable PIF reset fetch
  cases without PC mutation, exception conversion, decode, identify, execute, or
  step
- Machine-owned current-PC CPU instruction-word fetch wrapper that reads the
  represented CPU PC, delegates to explicit-address instruction fetch, and
  returns the same value/error shape without PC mutation, exception conversion,
  decode, identify, execute, or step
- Machine-owned pure instruction-fetch fault to address-error selection for the
  source-clear C++ step-convertible fetch faults, preserving the faulting
  `CpuAddress` as future BadVAddr input and selecting AdEL/code 4 without
  mutation, fetch, decode, identify, execute, or step
- Machine-owned narrow instruction-fetch address-error entry for the selected
  fetch faults, mutating BadVAddr/Cause/EPC/Status.EXL/PC/next PC only through
  the source-clear ordinary local AdEL entry path, with no Count change, fetch,
  decode, identify, execute, or step
- Machine-owned pure step fetch-fault action classification for already-returned
  fetch errors, naming source-clear `EnterAddressError` versus `Rethrow`
  behavior without fetching, entering exceptions, advancing PC/Count, executing,
  or stepping
- Machine-owned pure unsupported-step outcome readiness for already-decoded
  unknown identities plus source-clear known-unimplemented COP0, coprocessor,
  coprocessor-memory, CACHE, and invalid COP0 register forms, preserving raw
  fields and identity without execute, rollback, PC/Count cadence, exceptions,
  or step
- Machine-owned pure stopped-step outcome readiness for already-decoded
  `SYSCALL` and `BREAK` identities, preserving raw fields and identity without
  execute, cadence commit, Count mutation, syscall/break exception behavior,
  host stop/runtime policy, or step
- Machine-owned pure no-effect executed-step outcome readiness for the
  already-decoded `SYNC` identity, preserving raw fields and identity without
  execute, cadence commit, Count mutation, side effects, or step
- crate-private CPU-owned SPECIAL shift GPR writeback execution for the
  source-clear `SLL`, `SRL`, `SRA`, `SLLV`, `SRLV`, `SRAV`, `DSLL`, `DSRL`,
  `DSRA`, `DSLL32`, `DSRL32`, `DSRA32`, `DSLLV`, `DSRLV`, and `DSRAV`
  identities, reading source GPRs before destination writeback, sign-extending
  32-bit word results, preserving full-width 64-bit shift results, preserving
  zero-register write behavior, and avoiding fetch, decode, identify, cadence
  commit, Count advancement, branch behavior, memory access, exceptions, or step
- crate-private CPU-owned SPECIAL bitwise logical GPR writeback execution for
  the source-clear `AND`, `OR`, `XOR`, and `NOR` identities, reading `rs` and
  `rt` before destination writeback, preserving full-width 64-bit logical
  results and zero-register write behavior, and avoiding immediate logical
  semantics, fetch, decode, identify, cadence commit, Count advancement, branch
  behavior, memory access, exceptions, or step
- crate-private CPU-owned SPECIAL HI/LO transfer execution for the source-clear
  `MFHI`, `MTHI`, `MFLO`, and `MTLO` identities, moving full-width values
  between HI/LO and GPR state through sealed scalar and GPR semantics while
  avoiding multiply/divide behavior, fetch, decode, identify, cadence commit,
  Count advancement, branch behavior, memory access, exceptions, or step
- crate-private CPU-owned SPECIAL non-trapping integer GPR writeback execution
  for the source-clear `ADDU`, `SUBU`, `DADDU`, `DSUBU`, `SLT`, and `SLTU`
  identities, reading `rs` and `rt` before destination writeback, preserving
  C++ word wrapping plus sign extension for `ADDU`/`SUBU`, full-width wrapping
  for `DADDU`/`DSUBU`, signed/unsigned full-width comparison for `SLT`/`SLTU`,
  and zero-register write behavior while avoiding `ADD`/`SUB`/`DADD`/`DSUB`,
  overflow exceptions, fetch, decode, identify, cadence commit, Count
  advancement, branch behavior, memory access, exceptions, or step
- crate-private CPU-owned SPECIAL trapping integer readiness/writeback for the
  source-clear `ADD`, `SUB`, `DADD`, and `DSUB` identities, returning an
  overflow outcome before destination writeback when signed overflow occurs,
  writing `rd` only on non-overflow, preserving C++ 32-bit sign-extended and
  64-bit full-width result behavior, and avoiding fetch, decode, identify,
  cadence commit, Count advancement, branch behavior, memory access, generic
  exceptions, or step
- crate-private CPU-owned immediate trapping integer readiness/writeback for the
  source-clear `ADDI` and `DADDI` identities, interpreting the signed immediate
  only inside this instruction family, returning an overflow outcome before
  destination writeback when signed overflow occurs, writing `rt` only on
  non-overflow, preserving C++ `ADDI` sign-extended word and `DADDI` full-width
  result behavior, and keeping non-trapping immediates, generic immediate
  semantics, fetch, decode, identify, cadence commit, Count advancement, branch
  behavior, memory access, generic exceptions, or step separate
- crate-private CPU-owned immediate non-trapping integer execution/writeback for
  the source-clear `ADDIU` and `DADDIU` identities, interpreting the signed
  immediate only inside this instruction family, preserving C++ `ADDIU`
  sign-extended wrapping word result behavior and `DADDIU` full-width wrapping
  result behavior, writing `rt` through sealed GPR semantics, and avoiding
  `ADDI`/`DADDI` changes, immediate comparison/logical/upper-immediate
  behavior, overflow exceptions, generic immediate semantics, fetch, decode,
  identify, cadence commit, Count advancement, branch behavior, memory access,
  generic execute, or step
- crate-private CPU-owned immediate comparison execution/writeback for the
  source-clear `SLTI` and `SLTIU` identities, interpreting the immediate only
  inside this instruction family, preserving C++ sign-extended immediate
  behavior, signed full-width comparison for `SLTI`, unsigned full-width
  comparison for `SLTIU`, 1/0 `rt` writeback through sealed GPR semantics, and
  avoiding `ADDI`/`DADDI` changes, `ADDIU`/`DADDIU` changes,
  `ANDI`/`ORI`/`XORI`, `LUI`, generic immediate semantics, fetch, decode,
  identify, cadence commit, Count advancement, branch behavior, memory access,
  generic execute, or step
- crate-private CPU-owned immediate bitwise logical execution/writeback for
  the source-clear `ANDI`, `ORI`, and `XORI` identities, interpreting the raw
  immediate as a zero-extended CPU value only inside this instruction family,
  preserving C++ full-width GPR source/result behavior, writing `rt` through
  sealed GPR semantics, and avoiding `LUI`, `SLTI`/`SLTIU` changes,
  `ADDI`/`DADDI` changes, `ADDIU`/`DADDIU` changes, generic immediate
  semantics, generic zero-extension semantics, fetch, decode, identify,
  cadence commit, Count advancement, branch behavior, memory access, generic
  execute, or step
- crate-private CPU-owned upper-immediate execution/writeback for the
  source-clear `LUI` identity, interpreting the raw immediate only inside this
  instruction family as `(immediate_u16 << 16)` followed by C++-matching
  sign-extension from the 32-bit word result, writing `rt` through sealed GPR
  semantics, ignoring `rs` as C++ does, and avoiding `ANDI`/`ORI`/`XORI`
  changes, `SLTI`/`SLTIU` changes, `ADDI`/`DADDI` changes,
  `ADDIU`/`DADDIU` changes, generic immediate semantics, generic
  upper-immediate semantics, fetch, decode, identify, cadence commit, Count
  advancement, branch behavior, memory access, generic execute, or step
- crate-private CPU-owned pure executed-helper selection readiness for
  already identified source-clear CPU-local executed identities, naming only
  the already sealed no-effect SYNC, SPECIAL shift, SPECIAL bitwise logical,
  SPECIAL HI/LO transfer, SPECIAL non-trapping integer, SPECIAL trapping
  integer, immediate trapping integer, immediate non-trapping integer,
  immediate comparison, immediate bitwise logical, and LUI helper families
  without executing, mutating state, fetching, decoding, identifying, cadence
  commit, Count advancement, branch behavior, memory access, generic execute,
  or step
- crate-private CPU-owned executed-helper invocation for already decoded fields
  plus a source-clear selected helper family, calling exactly one already sealed
  CPU-local helper and returning either executed or arithmetic-overflow outcome
  without fetching, decoding, identifying, calling Machine, committing cadence,
  advancing Count, entering arithmetic-overflow exception entry, touching memory
  or reservation state, generic execute, or step
- crate-private Machine-owned pure CPU-local invocation outcome to future step
  action planning, mapping successful local execution to the already sealed
  committed-step cadence plan and arithmetic overflow to future
  arithmetic-overflow exception-entry planning while keeping invocation errors
  as Rust-side rejections, with no execution helper calls, cadence mutation,
  Count advancement, exception entry, fetch/decode/identify, generic step
  result, or step
- crate-private CPU/COP0 arithmetic-overflow exception entry for the narrow
  source-clear local overflow path: Cause code 12, EPC from the faulting PC or
  branch-delay PC, branch-delay flag, Status.EXL, and the local exception vector
  are updated without BadVAddr mutation, Count advancement, interrupt
  processing, generic exception dispatch, or step wiring
- crate-private CPU/COP0 Count advancement for future committed step outcomes:
  Count wraps by one and latches timer-pending when the post-increment Count
  equals Compare, without interrupt processing, exception entry, PC cadence, or
  step behavior
- crate-private CPU control-flow snapshot/restore readiness for exactly `pc` and
  `next_pc`, matching the source-clear unsupported-step rollback primitive
  without exposing step, execute, generic rollback, or savestate machinery
- crate-private CPU pre-execute next-PC staging readiness that advances only
  `next_pc` by one sequential instruction with source-clear wrapping arithmetic
  and leaves `pc`, Count, COP0, GPRs, RDRAM, SP DMEM, reservation, and Cartridge
  state unchanged
- crate-private CPU committed-step control-flow commit readiness that sets
  `pc` from a pre-step snapshot's `next_pc` and leaves the already-staged
  `next_pc` unchanged, without Count advancement, branch behavior, execute, or
  step
- Machine-owned pure committed-step cadence planning for source-clear
  step-visible cadence cases, naming control-flow and Count actions without
  mutating state, executing instructions, processing interrupts, implementing
  ERET, or adding a step/result API
- Machine-owned represented `Machine::step` composition over the already sealed
  current-PC classified action producer and classified action applicator,
  covering only currently represented categories: CPU-local committed
  execution, arithmetic-overflow exception entry, SYNC no-effect commit,
  SYSCALL/BREAK stopped commit, unsupported rollback, selected instruction-fetch
  AdEL entry, and source-clear fetch/invocation/unrepresented rejections. It is
  not `Cpu::step`, not generic `execute_cpu_instruction`, not a generic
  all-future step result, and not a full N64 step.
- Machine-owned `Machine::stage_cpu_pc` staging that sets the represented `pc`
  to the selected value and establishes the sequential
  `next_pc = value.wrapping_add(4)` invariant without exposing mutable CPU or
  COP0 state, fetching, or executing. The surface is general machine-state
  staging used by deterministic no-window inspection, not a debug/state
  injection framework.
- a no-window Rust machine probe in `fn64-inspection` that constructs a
  Machine, inspects sealed construction facts, dirties already-sealed
  represented state, calls `Machine::reset`, inspects sealed reset facts, prints
  deterministic plain text, and exits without SDL, a ROM path, CPU step, or
  instruction execution
- a deterministic no-window Rust step probe in `fn64-inspection` that stages
  only generated instruction words or synthetic addresses, calls only public
  `Machine::step` for execution, and covers CPU-local committed success,
  CPU-local arithmetic-overflow exception entry, SYNC committed no-effect,
  SYSCALL stopped, BREAK stopped, unsupported rollback, selected
  instruction-fetch AdEL, and source-clear fetch rejection
- CPU construction/default-state fields
- read-only COP0 construction/default-state inspection
- narrow CPU GPR access/mutation state semantics
- narrow CPU scalar staging for PC, next PC, HI, and LO

Still absent are a complete N64 step, `Cpu::step`,
generic `execute_cpu_instruction`, a generic all-future `MachineStepResult`,
branch/jump/link/delay-slot execution, branch-likely annul, load/store
execution, COP0 instruction execution, ERET, LL/SC, interrupt processing,
TLB/MMU, a bus or memory-map framework, device/MMIO routing, cartridge
execution mapping, PIF/BIOS boot behavior, game-boot or compatibility claims,
and SDL/window/audio runtime. Retired C++ behavior was not migrated and is not
implied by current Rust proof.

## Fixture Policy

Use synthetic byte arrays only. Do not add commercial ROMs, BIOS/PIF blobs,
copyrighted fixtures, or circumvention material.

## Cargo.lock Policy

`Cargo.lock` is tracked in this workspace to make local verification
reproducible. The current workspace has no third-party dependencies, and the
crate is marked `publish = false`.

## Verification

From the repository root, the normal required forward-product gate is:

```sh
./rust/verify-forward
```

That executable is the single owner of the exact required command sequence. It
resolves this workspace from its own path, so it may be invoked from another
current working directory. It runs formatting, clippy with warnings denied, the
complete Rust test suite, `fn64_machine_probe`, and `fn64_step_probe`, then ends
with `forward gate: ok`. All five stages must pass.

The forward gate invokes no CMake or C++ binary. No current C++ build or proof
lane remains.
