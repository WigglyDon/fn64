# Scalar LW R0 Policy

Destination `r0` does not bypass planning. The base, address, alignment, full
four-byte knowledge range, and big-endian value are all validated.

On a valid commit only the destination write is discarded. PC, last
instruction, RSP committed count, and processor-turn cadence still advance.
`r0` remains Available zero from `ArchitecturalZero`.
