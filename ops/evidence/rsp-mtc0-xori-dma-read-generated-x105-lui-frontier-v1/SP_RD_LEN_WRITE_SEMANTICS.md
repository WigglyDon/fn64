# SP RD LEN Write Semantics

Public r0 raw word zero decodes as eight bytes, one block, zero skip. The shared
plan requires programmed addresses, in-range RDRAM, destination preflight, and
record capacity before mutation. One commit records source index 2, copies all
eight bytes, appends one record, evolves addresses, advances SP PC once,
increments only RSP count once, and selects CPU.
