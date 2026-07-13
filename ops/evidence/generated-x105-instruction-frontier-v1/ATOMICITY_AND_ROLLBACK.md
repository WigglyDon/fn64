# Atomicity and rollback

The load planner establishes, before mutation:

1. decoded `Lw` fields;
2. known old base value and source classification;
3. effective address and natural alignment;
4. direct target and complete word bounds;
5. SP-DMEM cartridge-bootstrap provenance;
6. readable four-byte big-endian value;
7. destination value and GPR lineage;
8. existing cadence plan.

Only the existing applicator then commits the destination, lineage,
`pc`/`next_pc`, Count, and delay-slot completion once. Unknown base, non-direct
address, target miss, unclassified SP-DMEM backing, or read failure returns a
typed rejection before any commit.

Snapshot tests cover CPU GPRs and sources, HI/LO, COP0, PC, next PC, Count,
delay context, RDRAM, SP DMEM, and SP IMEM. The selected capability performs no
memory mutation, and rejection preserves the complete represented snapshot.
