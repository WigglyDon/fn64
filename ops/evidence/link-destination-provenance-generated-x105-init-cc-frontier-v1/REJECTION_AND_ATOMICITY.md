# Rejection and atomicity

Preserved rejection boundaries:

- JAL/J/JR/JALR/BEQ/BNE/BLTZ in an active delay slot;
- unknown old JALR or JR source;
- unknown BEQ/BNE/BLTZ source;
- unknown load/store sources;
- all exception and store/load atomicity.

No rejection may partially write a link. The exact `Sw r2,0(sp)` frontier at
`0xA4000890` rejects because its transfer source has `UnknownPifProduced`
lineage. It preserves PC, next_pc, Count, all GPR values and lineage, the
completed JAL link, represented SP memory, and every device fact.
