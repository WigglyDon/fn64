# Synthetic test coverage

- `RUNTIME_FACT` SP IMEM tests cover exact `0x1000` capacity, construction,
  concrete-zero/unknown separation, byte bounds, first and last aligned words,
  unaligned and out-of-range reads, all-four-byte knownness, named test
  provenance, and N64 big-endian interpretation.
- `RUNTIME_FACT` Routing tests cover both direct aliases of SP IMEM offset zero,
  the last aligned word, direct RDRAM, unaligned classification, end miss,
  unrelated SP-register miss, and non-direct rejection.
- `RUNTIME_FACT` Lw tests cover positive and negative 32-to-64 sign extension,
  immediate sign extension, wrapping 64-bit arithmetic, direct RDRAM, SP IMEM,
  pre-write base/destination aliasing, GPR zero, destination lineage, successful
  cadence, and exactly one Count advance.
- `RUNTIME_FACT` Failure tests cover unknown base, unknown SP IMEM, target miss,
  unaligned data-AdEL, exact BadVAddr, exception cadence, and blocked exception
  entry. Complete snapshots prove no partial mutation where rejection occurs.
- `RUNTIME_FACT` Bootstrap tests prove generated knownness is Machine-owned and
  test-only, normal bootstrap keeps SP IMEM unknown, reset clears storage
  provenance, authentic-frontier-shaped `SpecialAdd` remains valid, and a
  generated known SP IMEM word permits a complete `Lw` commit.
- `RUNTIME_FACT` Boot-probe library and CLI tests prove deterministic reporting
  of target, effective address, first unknown offset, and rejection-before-
  mutation without embedding private ROM content.
