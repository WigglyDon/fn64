# IPL3 relocation range

The generated loop copies the public block17s through the word immediately
before pifipl3e.

| Fact | Exact value |
| --- | --- |
| source CPU range | 0xA4000554..0xA4000887 |
| source physical range | 0x04000554..0x04000887 |
| source SP-DMEM local range | 0x0554..0x0887 |
| destination CPU range | 0xA0000004..0xA0000337 |
| destination physical range | 0x00000004..0x00000337 |
| byte count | 820 |
| word count | 205 |
| first word | 0x3C0BB000 |
| last word | 0xAFB10044 |

Every source word is classified CartridgeBootstrap with its exact cartridge
offset. Every destination store uses existing direct RDRAM ownership. Final
RDRAM bytes equal the source range byte for byte; there is no relocation shadow
buffer.
