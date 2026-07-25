# SP Control And PC Ownership

`Sp::pc` remains the sole current RSP PC. The nested execution state owns only
`next_pc`, which is the aligned sequential local successor.

Local PC rules:

- 12-bit IMEM address space;
- four-byte alignment;
- sequential arithmetic wraps within 4 KiB;
- no CPU segment or physical-address interpretation.

A represented CPU SP-PC write atomically updates `Sp::pc`, synchronizes
`rsp.next_pc`, and clears stale RSP delay context. It does not reset scalar or
unit availability, reset the RSP committed count, create a run-start, or
execute RSP code.

