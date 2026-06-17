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

fn64 --self-test

Runs the internal CPU/RDRAM bootstrap demos and exits. This is the proof path.

fn64 --inspect-rom path/to/game.z64

Loads a cartridge, prints the normalized ROM metadata and initial machine state, then exits without opening an SDL window.

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
- CPU fetch currently reaches only RDRAM physical/KSEG0/KSEG1 aliases
- cartridge execution mapping is not wired yet

This keeps ROM loading honest without pretending the cartridge is executing.

## CPU RDRAM address aliases

The CPU now reaches RDRAM through a tiny machine-local translation rule:
- low physical RDRAM addresses map directly
- 0x80000000 based RDRAM aliases map to local RDRAM
- 0xa0000000 based RDRAM aliases map to local RDRAM

This is not a bus and not a general memory map.
It is the smallest earned address truth needed before cartridge or boot execution work.

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
It is not a bus.
It is not N64 boot.

The self-test path proves the seam by loading a tiny generated ROM, staging two cartridge instructions into RDRAM, setting the CPU PC to the staged KSEG0 address, and stepping ORI then BREAK.

Normal ROM launch does not stage or execute cartridge bytes automatically.

## No-window ROM inspection

fn64 --inspect-rom path/to/game.z64

This loads the cartridge, prints cartridge metadata and initial machine state, then exits.

It does not:
- open SDL
- run bootstrap demos
- stage cartridge bytes
- step CPU instructions
- execute cartridge code

This is the default safe observation path for ROM/session truth.
