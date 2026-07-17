# Address and target policy

- Physical MI_VERSION: `0x04300004`
- Direct KSEG0 alias: `0x84300004`
- Direct KSEG1 alias: `0xA4300004`

The exact classifier adds one target. MI_INIT_MODE `0x04300000`, MI_INTR
`0x04300008`, MI_INTR_MASK `0x0430000C`, neighboring MI addresses,
non-direct segments, and MI_VERSION stores remain closed. No generic MI bank,
bus, MMIO layer, or generalized physical map was added.
