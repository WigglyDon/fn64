# RI_MODE provenance

Each successful CPU store records:

- the `Sw` instruction PC;
- source GPR index;
- the source GPR's existing Machine-owned lineage.

For the first generated write these are PC `0xA40000E8`, r0, and
`ArchitecturalZero`. For the second they are PC `0xA4000104`, r9, and the
`KnownInstructionResult` created by the independently encoded `Ori` at
`0xA4000100`. The second source replaces the first.

No host path, cartridge identity, expected sequence position, frontier flag,
or private-input identity is recorded.
