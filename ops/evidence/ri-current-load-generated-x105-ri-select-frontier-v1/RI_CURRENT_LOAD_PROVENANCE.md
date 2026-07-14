# RI_CURRENT_LOAD provenance

The represented source is `CpuStoreWord` with:

- the `Sw` instruction PC;
- the source GPR index;
- the source GPR's existing Machine-owned lineage;
- the low 32-bit transfer word as store evidence only.

For the generated frontier these are PC `0xA40000DC`, r0,
`ArchitecturalZero`, and word zero. The event also snapshots the stored
RI_CONFIG input zero and enable true. This proves known GPR -> exact CPU store
-> stored configuration snapshot -> Machine event without recording host or
private-input identity.
