# Address and target policy

- Physical target: `0x03F80008`
- KSEG0 alias: `0x83F80008`
- KSEG1 alias: `0xA3F80008`
- Next physical frontier: `0x03F80014`
- Next KSEG1 frontier: `0xA3F80014`

Existing direct-address normalization and word alignment apply. The classifier
adds one exact target, not an RDRAM range, register array, MMIO map, or bus.

