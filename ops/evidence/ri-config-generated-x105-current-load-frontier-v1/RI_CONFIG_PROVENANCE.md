# RI_CONFIG provenance

The represented source is `CpuStoreWord` with:

- the `Sw` instruction PC;
- the source GPR index;
- the source GPR's existing Machine-owned lineage.

For the generated frontier these are PC `0xA40000C4`, r9/t1, and the
`KnownInstructionResult` created by the generated `Ori` at `0xA40000C0`.
This proves known GPR -> exact CPU store -> RI_CONFIG fields without recording
a host path, private input, cartridge identity, expected branch, or frontier.
