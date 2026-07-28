# Public x105 DMEM Result

Immediately before DMA, DMEM `[0x000,0x010)` contains no available value
truth. Its private backing bytes happen to be zero, but each byte is
`Unavailable(BootstrapUncovered)`.

The public source-known RDRAM range and resulting DMEM range are:

| Offset | RDRAM value | DMEM value after | DMEM source |
| ---: | ---: | ---: | --- |
| `0x000` | `25` | `25` | `SpDma`, record 0 |
| `0x001` | `29` | `29` | `SpDma`, record 0 |
| `0x002` | `00` | `00` | `SpDma`, record 0 |
| `0x003` | `04` | `04` | `SpDma`, record 0 |
| `0x004` | `15` | `15` | `SpDma`, record 0 |
| `0x005` | `1F` | `1F` | `SpDma`, record 0 |
| `0x006` | `FF` | `FF` | `SpDma`, record 0 |
| `0x007` | `E3` | `E3` | `SpDma`, record 0 |

DMEM `[0x008,0x010)` is untouched and remains value-unavailable. `v12`
remains its pre-DMA whole-register unavailable Lqv result; later memory
mutation does not retroactively change a register.
