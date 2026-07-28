# RSP MTC0 Identity

Decode requires COP0 opcode `0x10`, `rs=0x04`, and zero reserved/function bits.
`rt` is the scalar source. Represented `rd` values are exactly 0
(`SP_MEM_ADDR`), 1 (`SP_DRAM_ADDR`), and 2 (`SP_RD_LEN`). Other indices and
malformed words reject before mutation. Existing Mfc0 behavior is unchanged.
