# Boot handoff globals

Generated stores create ordinary big-endian RDRAM byte truth at physical
addresses `0x300..0x31B`:

| Address | Name | Value |
|---:|---|---:|
| `0x00000300` | osTvType | `0x00000001` |
| `0x00000304` | osRomType | `0x00000000` |
| `0x00000308` | osRomBase | `0xB0000000` |
| `0x0000030C` | osResetType | `0x00000000` |
| `0x00000310` | osCicType | `0x000017D9` (6105) |
| `0x00000314` | osVersion | `0x00000000` |
| `0x00000318` | osMemSize | `0x00400000` |

`Rdram` remains the sole owner of these bytes. They are guest-produced handoff
data and do not become host policy, cartridge identification, or compatibility
metadata.
