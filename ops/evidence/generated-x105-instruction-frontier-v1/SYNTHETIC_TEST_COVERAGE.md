# Synthetic test coverage

Core coverage includes:

- direct KSEG0/KSEG1 SP-DMEM word-target boundaries;
- exact cartridge-offset provenance across the staged span;
- rejection of unclassified concrete backing below offset `0x040`;
- generated cold-x105 composition through SP-IMEM `Lw`, SP-DMEM `Lw`, and
  `SpecialXor` to the `Sw` frontier;
- complete snapshot preservation on unknown SP-DMEM rejection;
- unaligned SP-DMEM-shaped `Lw` in a delay slot, including AdEL, EPC, BD,
  BadVAddr, Count, vectoring, and cleared delay context;
- existing load-word, Machine-step, control-flow, bootstrap, SP-IMEM, and
  cold-x105 regressions.

The existing `fn64_step_probe` adds stable public-path markers for committed
SP-DMEM load, unknown rejection, delay-slot AdEL, and the generated next
frontier. It uses only generated inputs and ends with `result: ok`.
