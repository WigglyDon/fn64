# Source anchors

No external code or byte sequence is reproduced here.

Current fn64 product anchors at commit `8ba3456`:

- Machine input lifecycle: `rust/crates/fn64-core/src/machine.rs:2481-2624`.
- Bootstrap plan and atomic assignment:
  `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs:426-708`.
- GPR/COP0/control-flow source ledgers: the same file at `125-170` and
  `635-695`.
- CPU registers/control flow/COP0: `cpu/registers.rs`, `cpu/scalars.rs`, and
  `cpu/cop0.rs`.
- PIF copy layout: `rust/crates/fn64-core/src/pif_firmware.rs`.
- SP IMEM knownness/provenance: `rust/crates/fn64-core/src/sp_imem.rs`.
- Host-only selectors and report plumbing:
  `rust/crates/fn64-inspection/src/boot_probe.rs:58-325` and `505-552`.

Pinned reconstruction `928f59089c18a95cbffa59938a18fa6032c5d78c`:

- `src/pifrom.s:55-64`: IPL1 Status and Config.
- `src/pifrom.s:96-124`: IPL2 copy, stack setup, transfer.
- `src/pifrom.s:143-176`: PIF boot-word derivation of s3-s7.
- `src/pifrom.s:244-252`: initial checksum call.
- `src/pifrom.s:289-568`: balanced checksum frame and final link.
- `src/pifrom.s:589-665`: no intervening link write and final jr t3.
- `src/pifrom.s:668-680`: PAL/MPAL code is four bytes longer before padding.
- `src/ipl3.s:70-123`: x105 prelude and common cold first-use window.
- `src/ipl3.s:274`: first later jal that replaces ra.

Independent anchors:

- Mupen64Plus commit `9eb6a7cbefe663c0a7c527afc705f5dea5197d7c`,
  `src/device/pif/bootrom_hle.c:48-150`.
- CEN64 commit `e0641c8452a3ae8edcd2bf4e46794bb4eaafc076`,
  `si/cic.c:14-27` and `si/controller.c:42-77`.
- NEC VR4300 User's Manual revision 2.2, sections 2.2.1, 3.2.3.2,
  7.1.8-7.1.16, 7.6, and 7.7; JAL/JALR links use PC plus eight.
- Nintendo 64 Programming Manual document NU6-06-0030-001G, Chapter 3,
  page 44; SP IMEM and SP DMEM are separate 4 KiB memories.
