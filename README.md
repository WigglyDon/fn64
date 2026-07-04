# fn64

A Nintendo 64 emulator project built for clarity, minimalism, and direct control.

Fedora is the first workbench, not the emulator's identity.

Current goal:

- build cleanly on Fedora
- open one minimal window
- load ROM bytes from a direct path
- grow only under real pressure

## Dependencies

```bash
sudo dnf install gcc-c++ cmake ninja-build SDL3-devel
```

## Run modes

fn64_selftest

Runs the internal CPU/RDRAM bootstrap demos and exits. This is the proof path, not the emulator runtime.

fn64_inspect path/to/game.z64

Loads a cartridge, prints the normalized ROM metadata and initial machine state, then exits without opening an SDL window.

fn64_step_probe

Runs a synthetic no-window Machine step probe. It loads no ROM path, uses generated synthetic bytes only, and does not imply boot, cartridge execution, or game compatibility.

fn64 path/to/game.z64

Loads a cartridge, prints the normalized ROM metadata, and opens the session window.

This does not yet execute cartridge code. Cartridge execution mapping has not been earned yet.

## Machine session state

Normal ROM launch reports the cartridge metadata and the initial machine state.

The current machine state is intentionally plain:
- cartridge bytes are loaded and owned
- RDRAM exists
- CPU pc / next_pc exist
- the reset model is an explicit blank RDRAM power-on state, not N64 reset/PIF boot
- CPU instruction fetch currently uses only KSEG0/KSEG1-style direct RDRAM aliases
- CPU data load/store currently reaches direct RDRAM, local SP DMEM/IMEM byte memories, minimal local SP DMA MMIO, minimal local PI MMIO, minimal local SI/PIF DMA MMIO, plus minimal local MI pending/mask MMIO
- the CPU can observe a tiny local COP0 Status/Count/Compare/Cause/EPC/BadVAddr subset, deliver/return from minimal local software, MI-derived, or Count/Compare timer interrupt entries, and enter the same local vector for signed arithmetic overflow, unaligned fetch/data/control-transfer address-error exceptions, eligible CPU data addresses with no supported local target, or eligible aligned direct-alias instruction-fetch target misses
- cartridge execution mapping is not wired yet

This keeps ROM loading honest without pretending the cartridge is executing.

## CPU RDRAM address aliases

The CPU now reaches RDRAM through a tiny machine-local translation rule:
- 0x80000000 based RDRAM aliases map to local RDRAM
- 0xa0000000 based RDRAM aliases map to local RDRAM

Raw physical RDRAM offsets are staging/inspection addresses, not CPU addresses.

This is not a bus, general memory map, TLB translation, or cartridge ROM mapping.
The current non-RDRAM data targets are local SP DMEM/IMEM byte memories, a tiny local SP DMA MMIO subset, a tiny local PI MMIO subset, a tiny local SI MMIO subset, and local MI pending/mask MMIO. Cartridge-like CPU data addresses remain unmapped; eligible CPU data reads/writes with no supported local target now report through the narrow local COP0 address-error seam. PIF RAM and PIF ROM are not CPU fetch/data mapped. This is not RSP execution, COP2, timing, general COP0 exception delivery, boot, or compatibility.

## Blank reset state

A newly constructed machine now deliberately resets into a blank local state:
- powered_on is true
- RDRAM is zeroed
- CPU pc is 0x00000000
- CPU next_pc is 0x00000004
- CPU hi/lo are zero
- CPU general registers are zero

This is not real N64 reset behavior.
It is a named temporary starting point until boot/PIF/reset behavior is earned.

## Cartridge staging seam

The machine now has an explicit method that can copy normalized cartridge bytes into local RDRAM.

This is not cartridge execution mapping.
It is not CPU cartridge ROM mapping.
It is not a bus.
It is not N64 boot.

The `fn64_selftest` proof path proves the seam by loading a tiny generated ROM, staging two cartridge instructions into RDRAM, setting the CPU PC to the staged KSEG0 address, and stepping ORI then BREAK.

Normal ROM launch does not stage or execute cartridge bytes automatically.

## Minimal PI MMIO subset

The current CPU data path recognizes a tiny local PI register window for aligned 32-bit loads and stores. Writing the local cartridge-to-RDRAM length register immediately copies from the supported local PI cart ROM address window into physical RDRAM. PI cart address 0x10000000 maps to normalized Cartridge offset 0.

Successful PI DMA latches a local MI PI pending bit. PI itself does not deliver CPU interrupts; delivery, when enabled, goes through the narrow local COP0 seam. This is not PI timing, DMA scheduling, boot, cartridge CPU mapping, or game compatibility.

## Minimal SI/PIF DMA MMIO subset

The current CPU data path recognizes a tiny local SI register window for aligned 32-bit loads and stores. Machine owns a local 64-byte PIF RAM buffer that is reachable only through this SI DMA seam, not through CPU fetch or data mapping.

Writing the local SI PIF-to-RDRAM trigger immediately copies exactly 64 bytes from local PIF RAM to physical RDRAM. Writing the local SI RDRAM-to-PIF trigger immediately copies exactly 64 bytes from physical RDRAM to local PIF RAM. The only supported local PIF RAM DMA address is 0x1fc007c0. Each transfer is preflighted before mutation; failed SI DMA leaves RDRAM and PIF RAM unchanged and does not latch MI pending.

Successful SI DMA latches a local MI SI pending bit. SI itself does not deliver CPU interrupts; delivery, when enabled, goes through the already-earned MI/COP0 interrupt seam. This is not PIF boot ROM, CIC/security behavior, controller protocol, SI timing/status fidelity, busy delay, boot, cartridge CPU mapping, a public bus, or game compatibility.

## Local SP DMEM/IMEM data memory

CPU data load/store can access Machine-owned 4 KiB SP DMEM and 4 KiB SP IMEM byte memories through direct aliases. Instruction fetch still remains RDRAM-only.

The current CPU data path also recognizes a tiny local SP register window for aligned 32-bit loads and stores. Writing the local SP read/write length registers immediately performs a deterministic local length/count/skip block copy between physical RDRAM and local SP memory.

This is not full SP register behavior, SP status/timing/interrupt fidelity, RSP execution, COP2, renderer/audio, or game compatibility.

## Minimal MI MMIO subset

The current CPU data path recognizes a tiny local MI register window for aligned 32-bit loads and stores. It exposes local SP/SI/PI pending bits and local SP/SI/PI mask bits. Successful PI DMA latches the PI pending bit, successful SI DMA latches the SI pending bit, and successful SP read/write DMA latches the SP pending bit. CPU writes clear supported pending bits with write-one-to-clear and assign supported mask bits directly.

MI pending/mask state is observable local machine state only. MI does not fetch exception vectors or change pc/next_pc by itself; the narrow COP0 seam below is the only local interrupt-entry path currently earned.

## Minimal COP0 interrupt/exception subset

The current CPU instruction path supports only a tiny local COP0 MFC0/MTC0 seam for Status, Count, Compare, Cause, EPC, and BadVAddr, plus a narrow ERET return from the local interrupt-entry state. Status stores supported local IE, EXL, and interrupt-mask bits. Count advances by one after committed executed CPU instructions, not host time or real N64 cycles. Compare match latches a local timer pending bit as Cause.IP7, and MTC0 Compare clears that timer pending bit. MTC0 Cause directly assigns supported local software pending bits IP0/IP1. MFC0 Cause composes those IP0/IP1 bits with a local IP2 bit when supported MI pending bits are also enabled in the local MI mask, the local timer IP7 bit, the earned local Cause ExcCode for signed arithmetic overflow or address-error exceptions, and a local Cause.BD bit for already-earned synchronous exceptions in delay-slot-shaped cadence. EPC stores the interrupted or faulting PC, stores the branch/control instruction PC for that narrow local delay-slot exception case, BadVAddr stores the faulting address for earned unaligned fetch/data/control-transfer address-error paths and eligible CPU data target misses, and local handler code can write EPC before ERET.

A local interrupt can enter 0x80000180 only when COP0 Status IE/EXL and the matching IM0/IM1/IM2/IM7 bit allow a pending IP0/IP1/IP2/IP7 line, and only from ordinary pc/next_pc cadence at a fetchable direct-RDRAM interrupted PC. Signed arithmetic overflow, unaligned instruction fetch/data read/data write, unaligned JR/JALR targets, CPU data reads/writes with no supported local target, and eligible aligned KSEG0/KSEG1 direct-alias instruction-fetch target misses can enter the same local vector from ordinary pc/next_pc cadence with EXL clear; the same already-earned synchronous data/control exception sources can also enter from a narrow delay-slot-shaped cadence, setting Cause.BD and storing EPC as the branch/control instruction PC. CPU data target misses use AdEL for reads and AdES for writes, with BadVAddr set to the rejected effective CPU data address. Eligible direct-alias fetch target misses use AdEL with EPC and BadVAddr set to the rejected fetch PC; blank initial PC, raw/non-direct fetch rejection, and unaligned fetch nonordinary cases remain local fault behavior. Instruction fetch remains RDRAM-only, and cartridge-like or device fetch aliases remain unmapped/non-executable. The faulting instruction does not commit, Count does not advance, Status EXL is set, BadVAddr and Cause ExcCode remain source-specific, and handler code can write EPC before ERET to retry, return to the delay slot, or skip the branch plus delay slot. ERET is supported only with EXL set and ordinary pc/next_pc cadence; it validates EPC alignment, returns to EPC, and clears EXL without clearing software pending, MI pending, MI mask, timer pending, BadVAddr, Cause.BD, or Cause ExcCode. ERET does not preflight target fetchability; the next fetch owns that rejection. This is not general COP0 exception delivery, general address-error delivery, general control-flow exception delivery, full delay-slot exception fidelity, or exception-return fidelity: unsupported MMIO, raw/non-direct fetch exception delivery, interrupt delivery from delay slots, SYSCALL/BREAK/trap exception delivery, TLB operations, real Count/Compare timing fidelity, reset/boot vectors, cartridge CPU mapping, device execution, and compatibility behavior remain unimplemented.

## No-window ROM inspection

The `fn64_inspect` build target provides a tiny SDL-free inspection executable for ROM/core facts.

fn64_inspect path/to/game.z64

This loads the cartridge, prints cartridge metadata and initial machine state, then exits.

It does not:
- open SDL
- run bootstrap demos
- stage cartridge bytes
- step CPU instructions
- execute cartridge code

This is the default safe observation path for ROM/session truth.
