# MI_VERSION provenance boundary

`MachineMiVersionState` contains immutable hardware identity only. It does not
store CPU-load provenance.

The destination GPR receives ordinary CPU lineage:
`KnownInstructionResult { execution_address: 0xA400016C, identity: Lw,
source_gpr_a: r1, source_gpr_b: none }`.

Hardware truth and execution lineage therefore keep separate owners.
