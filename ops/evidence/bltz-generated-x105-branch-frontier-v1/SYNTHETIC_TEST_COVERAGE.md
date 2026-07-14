# Synthetic test coverage

Core proof covers:

- BLTZ decode/identity selection and exact rs/immediate fields;
- negative, zero, positive, r0, minimum, and maximum signed values;
- full-width discriminators `0x0000000080000000` and
  `0xFFFFFFFF00000000`;
- zero, positive, negative, PC+4-wrapping, and target-wrapping arithmetic;
- taken and untaken one-slot cadence, no link, no annul, and source
  preservation;
- unknown bootstrap source rollback;
- all other recognized REGIMM identities remaining unrepresented;
- BLTZ-in-delay-slot rejection before mutation;
- taken-slot AdES and untaken-slot AdEL with exact owner EPC/BD, BadVAddr, and
  zero faulting-slot Count;
- generated 15-commit composition through the exact SP-IMEM local `0x00C`
  zero store to the `Cop0Mtc0` frontier.

The direct step probe exposes deterministic BLTZ markers for those condition,
target, slot, exception, rejection, and composition cases and ends with
`result: ok`.
