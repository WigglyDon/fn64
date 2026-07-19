# BEQL source operands

Both old GPR values and their lineages are genuine execution inputs. Equality
compares all 64 bits only after both sources are available. Architectural zero
is available. An unavailable source, including an unavailable same-register
pair, rejects atomically; backing bits are not compared.

Generated operands at `0xA400099C`:

- `r26 = 0x0000000000000001`, produced by `Slti` at `0xA4000998` from `r12`;
- `r0 = 0x0000000000000000`, `ArchitecturalZero`;
- result: unequal, so the likely branch is not taken.
