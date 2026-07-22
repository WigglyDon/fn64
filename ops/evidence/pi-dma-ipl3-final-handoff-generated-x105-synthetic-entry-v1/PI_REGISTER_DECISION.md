# PI register decision

One concrete per-`Machine` `Pi` owner represents only the generated path:

| Register | Physical | KSEG1 | Generated value |
|---|---:|---:|---:|
| PI_DRAM_ADDR | `0x04600000` | `0xA4600000` | `0x00001000` |
| PI_CART_ADDR | `0x04600004` | `0xA4600004` | `0x10001000` |
| PI_WR_LEN | `0x0460000C` | `0xA460000C` | `0x000FFFFF` |
| PI_STATUS | `0x04600010` | `0xA4600010` | read `0`; clear `0x00000002` |

The generated commits occur at PCs `0x8000001C`, `0x80000044`, and
`0x80000054`. PI_STATUS reads at `0x80000024` and `0x8000009C` both return
zero under the atomic completion model. The clear at `0x800001D4` clears the
MI-owned PI pending bit.

PI_RD_LEN remains an explicit unsupported target. Controller reset, domain
timing registers, arbitrary lengths, and cartridge writes are not represented.
