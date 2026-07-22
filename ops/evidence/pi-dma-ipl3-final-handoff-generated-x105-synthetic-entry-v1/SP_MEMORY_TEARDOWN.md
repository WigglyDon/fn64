# SP memory teardown

The generated loop begins from KSEG1 SP address `0xA4000000`, stops at
`0xA4002000`, and commits 2,048 ordinary `Sw` instructions at PC
`0x80000270`. Each store transfers the generated fill word `0xA4002000` from
r9 and records exact SP-local address and CPU provenance.

All 1,024 DMEM words and all 1,024 IMEM words contain the fill word after the
loop. Every earlier opaque IMEM word and relocated-source word is replaced by
genuine known full-word stores; no opaque word remains. The arrays are not
host-cleared or bulk-mutated by proof code.

CPU execution remains in RDRAM/I-cache during teardown, so replacing SP IMEM
does not affect the active instruction stream.
