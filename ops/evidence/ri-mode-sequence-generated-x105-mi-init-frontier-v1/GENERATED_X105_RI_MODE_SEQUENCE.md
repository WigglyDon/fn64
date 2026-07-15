# Generated x105 RI_MODE sequence

`RUNTIME_FACT`: independently encoded generated instructions reproduce the
accepted 32,038-commit prefix at PC/next-PC
`0xA40000E8 / 0xA40000EC`, Count 32,022.

Commit 32,039 stores zero to RI_MODE. Commit 32,040 uses `Addiu` to set s1 to
four. The loop at `0xA40000F0` comprises NOP, `Addi`, BNE, and a NOP delay
slot. Four iterations commit 16 instructions: three taken branches, one
untaken branch, and four slots. It ends at PC/next-PC
`0xA4000100 / 0xA4000104`, Count 32,040, with RI_MODE unchanged.

Commit 32,057 uses `Ori` to construct `0x0E`; commit 32,058 stores it to
RI_MODE and replaces provenance. Commit 32,059 uses `Addiu` to set s1 to 32.
The loop at `0xA400010C` comprises `Addi`, BNE, and `Ori r9,r0,0x10F` in the
delay slot. Thirty-two iterations commit 96 instructions: 31 taken branches,
one untaken branch, and 32 executed `Ori` slots.

The final state is PC/next-PC `0xA4000118 / 0xA400011C`, Count 32,139, total
commits 32,155, s1 zero, and r9 `0x10F`. RI_MODE retains operating mode two
and both stop flags true. This is synthetic public-`Machine::step` CPU
composition only.
