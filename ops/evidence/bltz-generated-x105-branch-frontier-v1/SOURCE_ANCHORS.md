# Source anchors

## Current fn64 owners

- `rust/crates/fn64-core/src/cpu/instruction.rs`: REGIMM decode and identity;
  `signed_cpu_value_less_than`; existing `SLT` and `SLTI` consumers.
- `rust/crates/fn64-core/src/machine.rs`: public `Machine::step`, ordinary
  control-flow planning/application, branch target helper, bootstrap GPR source
  rejection, delay-slot rejection, and generated-x105 composition.
- `rust/crates/fn64-core/src/cpu/scalars.rs`: control-flow snapshot, branch
  commit, and delay-slot context.
- `rust/crates/fn64-core/src/cpu/cop0.rs`: existing delay-slot AdEL/AdES
  exception lineage.

## External technical evidence

- NEC VR4300 User's Manual, revision 2.2, July 1995, accessed 2026-07-13:
  BLTZ branches when the complete source register is less than zero; ordinary
  conditional branches have one delay slot. The manual also identifies the
  processor as a 64-bit architecture.
  <https://www.bitsavers.org/components/nec/mips/1995_NEC_VR4300_MIPS_RISC_Microprocessor_Users_Manual.pdf>
- decompals/N64-IPL commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `src/ipl3.s:70-105`,
  accessed 2026-07-13: bounded x105 identity and operand relationships show
  non-linking, non-likely BLTZ on ra, an aligned zero-word store in its delay
  slot, and the common target label beginning with MTC0 to Cause.
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L70-L105>

Copied external code, instruction words, assembly, disassembly, firmware, or
cartridge bytes: no. Claims above are paraphrased facts and arithmetic only.
