# Source-knownness preservation

The correction changes no genuine source rule:

- JR and JALR still require old `rs`;
- BEQ and BNE still require both sources;
- BLTZ still requires `rs`;
- loads still require their effective-address base;
- stores still require address and transfer-source lineage.

Only prior link-destination state ceases to be treated as an operand.
