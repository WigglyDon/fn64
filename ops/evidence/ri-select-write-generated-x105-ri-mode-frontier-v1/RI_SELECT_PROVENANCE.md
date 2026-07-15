# RI_SELECT provenance

The represented CPU-store source records:

- the `Sw` instruction PC;
- source GPR index;
- the source GPR's existing Machine-owned lineage.

The stored value is the exact supported low transfer word. For the generated
composition these are PC `0xA40000E4`, r9, lineage from the independently
encoded `Ori` at `0xA40000E0`, and value `0x00000014`.

The lineage is known GPR -> exact CPU store -> Machine-owned RI_SELECT. It
records no host path, game/cartridge identity, expected trace, branch result,
or private-input identity.
