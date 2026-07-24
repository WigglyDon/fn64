# User cartridge CPU boot to first RSP task

This evidence records one no-window run of a user-owned cartridge through the
public source-derived x105 bootstrap and public `Machine::step`. The selected
input was `oot.z64`, a 33,554,432-byte big-endian `.z64` image. The host supplied
only an explicit path and owned-byte read; `Cartridge` normalized and owned the
bytes after construction.

The first cartridge instruction committed once at `0x80000400`. CPU-side
execution then committed 13,988,271 cartridge-runtime instructions and reached
the first genuine RSP start request. The `Sw` at `0x800D5A98` committed
SP_STATUS command `0x00000125`, changing halt from true to false after two
represented RDRAM-to-SP DMAs. The run stopped immediately after that CPU
instruction. No RSP instruction executed.

Classification:
`USER_PROVIDED_CARTRIDGE_MACHINE_STEP_COMPOSITION`.

Milestone:
`USER-CARTRIDGE-CPU-BOOT-TO-FIRST-RSP-TASK`.

The authentic checkpoint remains `BOOT-2`. Reaching one task submission is not
a graphics, audio, input, game-boot, or compatibility claim.

No ROM byte, hash, string, header dump, disassembly block, microcode byte, or
private absolute path is present in this directory.
