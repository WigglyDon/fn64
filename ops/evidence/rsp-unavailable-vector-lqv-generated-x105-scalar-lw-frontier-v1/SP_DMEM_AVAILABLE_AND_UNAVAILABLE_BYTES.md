# SP DMEM Available And Unavailable Bytes

`observe_byte` and the bounded `observe_range` operation return truth-bearing
knowledge. An available observation carries the byte and its exact source. An
unavailable observation carries only the unavailable cause.

Construction/reset backing is private deterministic storage, but its bytes are
`Unavailable(ConstructionOrReset)`. Complete cartridge bootstrap marks
`0x000..0x040` as `Unavailable(BootstrapUncovered)` and marks
`0x040..0x1000` available with exact cartridge offsets. A represented CPU
store or SP DMA makes exactly its destination bytes available with its
existing provenance.

Failed bootstrap preserves backing and knowledge. Repeated bootstrap restores
the same covered/uncovered split. Machine instances remain independent.
