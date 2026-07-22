# Final JR and delay slot

The generated code reloads cartridge header entry word `0x80001000` into r9.
At PC `0x8000027C`, word `0x01200008` executes `Jr r9` through the existing
single delay-context owner. The r9 value is
`0xFFFFFFFF80001000` with lineage from the generated header `Lw`.

The delay slot at PC `0x80000280` is word `0x00000000`, identity Nop
(`SpecialSll`), and commits exactly once. It clears the delay context and
selects PC `0x80001000`, next_pc `0x80001004`.

Final Count is 7,477,812 and total committed steps are 7,477,828. The first
instruction at the target is deliberately not fetched or executed by the
composition.
