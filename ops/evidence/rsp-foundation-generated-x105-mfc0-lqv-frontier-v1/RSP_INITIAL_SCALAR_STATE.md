# RSP Initial Scalar State

The nested RSP owner contains exactly 32 scalar-register availability entries.

- r0 is `Available { value: 0, source: ArchitecturalZero }`.
- r1 through r31 are `Unavailable { source: ConstructionOrReset }`.
- reads of r0 produce zero;
- writes to r0 are discarded;
- unavailable backing bits cannot be read as values;
- MFC0 does not consume the prior destination state, so it may overwrite an
  unavailable destination atomically.

Run-start preserves all scalar state. Construction, successful complete
bootstrap replacement, and reset restore the policy. Independent Machines own
independent scalar arrays.

