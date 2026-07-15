# MI_INIT_MODE provenance

The immutable store plan captures the old source GPR and its known lineage
before mutation. Successful generated lineage is:

`Ori at 0xA4000114 -> r9 low word 0x0000010F -> Sw at 0xA4000118 -> MI state`.

The stored source records instruction PC `0xA4000118`, source GPR 9, and the
`KnownInstructionResult` lineage from that generated `Ori`. Repeated writes
replace both state and provenance; independent Machines never share either.
