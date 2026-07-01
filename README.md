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
- CPU data load/store currently reaches direct RDRAM, local SP DMEM/IMEM byte memories, minimal local SP DMA MMIO, plus a minimal local PI MMIO subset
- cartridge execution mapping is not wired yet

This keeps ROM loading honest without pretending the cartridge is executing.

## CPU RDRAM address aliases

The CPU now reaches RDRAM through a tiny machine-local translation rule:
- 0x80000000 based RDRAM aliases map to local RDRAM
- 0xa0000000 based RDRAM aliases map to local RDRAM

Raw physical RDRAM offsets are staging/inspection addresses, not CPU addresses.

This is not a bus, general memory map, TLB translation, or cartridge ROM mapping.
The current non-RDRAM data targets are local SP DMEM/IMEM byte memories, a tiny local SP DMA MMIO subset, and a tiny local PI MMIO subset. This is not RSP execution, COP2, timing, interrupts, boot, or compatibility.

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

This is not PI timing, DMA scheduling, MI interrupts, boot, cartridge CPU mapping, or game compatibility.

## Local SP DMEM/IMEM data memory

CPU data load/store can access Machine-owned 4 KiB SP DMEM and 4 KiB SP IMEM byte memories through direct aliases. Instruction fetch still remains RDRAM-only.

The current CPU data path also recognizes a tiny local SP register window for aligned 32-bit loads and stores. Writing the local SP read/write length registers immediately copies exact bytes between physical RDRAM and local SP memory.

This is not full SP register behavior, SP status/timing/interrupt fidelity, RSP execution, COP2, renderer/audio, or game compatibility.

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
