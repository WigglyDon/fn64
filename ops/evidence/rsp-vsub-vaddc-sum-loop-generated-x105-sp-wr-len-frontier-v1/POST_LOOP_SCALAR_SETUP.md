# Post-Loop Scalar Setup

The existing represented sequence commits exactly:

| PC | Identity | Result/effect |
|---:|---|---|
| `0x074` | `Xori r3,r0,0xB120` | r3=`0x0000B120` |
| `0x078` | `Mtc0 r3,SP_MEM_ADDR` | transfer=`0xB120`, local=`0x1120` |
| `0x07C` | `Lui r3,0xB12F` | r3=`0xB12F0000` |
| `0x080` | `Xori r3,r3,0xB1F0` | r3=`0xB12FB1F0` |
| `0x084` | `Mtc0 r3,SP_DRAM_ADDR` | physical=`0x002FB1F0` |
| `0x088` | `Lui r3,0xFE81` | r3=`0xFE810000` |
| `0x08C` | `Xori r3,r3,0x7000` | r3=`0xFE817000` |

Eight real CPU commits separate the seven RSP commits and rotate the final
token. Their PCs/identities are:

`0x80000150 SpecialAddu`, `0x80000160 SpecialXor`, `0x80000164 Lw`,
`0x80000168 Addiu`, `0x8000016C Addiu`, `0x80000170 SpecialXor`,
`0x80000174 SpecialAddu`, `0x80000178 Lui`.

Final CPU PC/next and Count/committed are
`0x8000017C/0x80000180` and `253433/253449`.
