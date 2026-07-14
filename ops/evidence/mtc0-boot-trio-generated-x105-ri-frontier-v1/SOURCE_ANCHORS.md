# Source anchors

## Current fn64 owners

- `cpu/instruction.rs`: COP0/MTC0 decode and exact `rt`, `rd`, and low-bit
  fields.
- `cpu/cop0.rs`: Count, Compare, timer pending, Status, Cause subfields, and
  committed-step Count equality latch.
- `machine.rs`: public `Machine::step`, immutable action production,
  application, cadence, rollback, and generated composition.
- `machine/cartridge_bootstrap.rs`: coupled cold-x105 state, Status lineage,
  and GPR knownness/provenance.

## Primary external evidence

- NEC VR4300 User's Manual, MTC0 detail (chapter 16, printed pages 473-475):
  word transfer from GPR `rt` to CP0 `rd`, low eleven encoding bits zero, and
  CP0-unusable scope outside permitted modes.
- Same manual, Count/Compare/Cause definitions (chapter 6, printed pages
  163-172): 32-bit Count/Compare, Compare-write timer clear, Cause IP1:IP0
  software write mask, and kernel CP0 usability.
- Same manual, timer discussion (chapter 14, printed page 353): the IP7 wording
  audited separately in `CAUSE_IP7_CONTRADICTION_AUDIT.md`.
- decompals/N64-IPL commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `src/ipl3.s:106-114` and
  `include/PR/rcp.h:786-798`: Cause/Count/Compare ordering, RI base address,
  and RI_SELECT offset.

Copied code, instruction stream, assembly, disassembly, firmware, or cartridge
bytes: no. Durable facts are paraphrases, field arithmetic, and narrow source
anchors only.
