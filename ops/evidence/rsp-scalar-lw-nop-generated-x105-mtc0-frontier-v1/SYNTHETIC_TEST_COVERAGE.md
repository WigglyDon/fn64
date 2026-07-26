# Synthetic Test Coverage

Independent public synthetic tests cover decode, base availability, signed
offsets, low-12 wrapping, alignment, all-Available DMEM knowledge, unavailable
and malformed knowledge rejection, big-endian order, destination overwrite,
r0 discard-after-read, exact provenance, lifecycle, independence, NOP
preservation/cadence, closed SLL/load/store identities, and complete rejection
atomicity.

The exact public generated composition separately proves Lw, three CPU
rotations, two RSP NOP commits, and the Mtc0 stop. Final test counts are sealed
in the pass artifact after validation.
