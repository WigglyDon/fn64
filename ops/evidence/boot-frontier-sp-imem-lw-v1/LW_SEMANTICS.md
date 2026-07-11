# Complete aligned Lw semantics

- `LIVE_REPO_FACT` Existing instruction decode and identity owners remain
  `cpu::instruction::decode_cpu_instruction_word` and
  `cpu::instruction::identify_cpu_instruction`.
- `LIVE_REPO_FACT` `Machine::produce_load_word_step_action` owns source-GPR
  preflight, pre-write base capture, sign-extended 16-bit immediate addition,
  low-32-bit CPU address selection, alignment, target classification, and the
  complete fallible read.
- `LIVE_REPO_FACT` Effective-address arithmetic is wrapping 64-bit addition.
  The aligned word is interpreted big-endian and sign-extended from 32 to 64
  bits, consistent with the VR4300 `LW` definition:
  <https://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf>.
- `LIVE_REPO_FACT` The represented targets are direct RDRAM and SP IMEM. The
  classifier reuses current direct-segment translation; it is not a bus or a
  generalized memory map.
- `LIVE_REPO_FACT` Physical SP IMEM is `0x04001000..=0x04001FFF`; aligned word
  starts are `0x04001000..=0x04001FFC`; local offset is physical address minus
  `0x04001000`. Thus CPU address `0xA4001000` maps to offset `0x000`.
- `LIVE_REPO_FACT` Unaligned `Lw` enters the existing data-AdEL owner, records
  the effective low-32-bit address in BadVAddr, writes no destination, applies
  no normal cadence, and does not advance Count.
- `LIVE_REPO_FACT` `Machine::apply_load_word_step_action` writes the
  destination only after every fallible source fact is known, records
  `KnownInstructionResult` lineage, commits `pc/next_pc` once, and advances
  Count once. GPR zero remains immutable and base/destination aliasing uses the
  pre-write base.
- `RUNTIME_FACT` Complete-state snapshots prove unknown base, unknown SP IMEM,
  target miss, wrapping-address rejection, and blocked exception entry leave
  GPR values and sources, HI, LO, COP0, RDRAM, SP DMEM, SP IMEM, control flow,
  Count, exception facts, reservation state, and power state unchanged.
- `LIVE_REPO_FACT` Other loads, stores, non-direct segments, and unrelated MMIO
  remain explicit unsupported or rejected behavior.
