# Rejection and atomicity

Preserved rejection boundaries:

- JAL/J/JR/JALR/BEQ/BNE/BLTZ in an active delay slot;
- unknown old JALR or JR source;
- unknown BEQ/BNE/BLTZ source;
- unknown load/store sources;
- all exception and store/load atomicity.

No rejection may partially write a link. Unsupported `Beql` at
`0xA400099C` restores the staged sequential PC and preserves complete
Machine state with no Count advance.
