# RSP Lui Next Frontier

After the fourth intervening CPU-selected commit, the RSP token selects local
PC `0x028`, next PC `0x02C`, word `0x3C050020`.

The word is identified as `Lui r5,0x0020`, but scalar Lui execution is not
represented. `Machine::step` returns `ScalarLuiUnsupported` without changing
`r5`, the programmed SP registers, the DMA record, DMEM, `r3`, `r4`, `v12`,
PCs, counts, VI, run-start lineage, or processor turn. No CPU fallback occurs.
