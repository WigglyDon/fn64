# Public x105 Mtc0, Xori, And DMA Sequence

Evidence classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

| Local PC | Word | Result | RSP count | Next processor |
| --- | --- | --- | ---: | --- |
| `0x018` | `40800000` | `Mtc0 r0,SP_MEM_ADDR`; DMEM `0x000` | 7 | CPU |
| `0x01C` | `38030180` | `Xori r3,r0,0x0180`; `r3=0x180` | 8 | CPU |
| `0x020` | `40830800` | `Mtc0 r3,SP_DRAM_ADDR`; RDRAM `0x180` | 9 | CPU |
| `0x024` | `40801000` | `Mtc0 r0,SP_RD_LEN`; atomic eight-byte read DMA | 10 | CPU |
| `0x028` | `3C050020` | `Lui r5,0x0020`; explicit rejection | 10 | RSP |

The required CPU-selected commits between these RSP instructions are,
respectively, `Sw` at `0x8000001C`, `Lui` at `0x80000020`, `Lw` at
`0x80000024`, and `Andi` at `0x80000028`. CPU Count/committed count progress
from `252351/252367` to `252355/252371`; no RSP-selected commit changes them.
