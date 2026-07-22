# PI DMA range and bytes

The exact transfer spans are:

- cartridge bus `0x10001000..0x10100FFF`;
- cartridge byte offsets `0x00001000..0x00100FFF`;
- physical RDRAM `0x00001000..0x00100FFF`;
- byte count `0x00100000`.

The generated proof compares every one of the 1,048,576 destination bytes
against the Machine-owned cartridge source after the DMA. The first word is
`0x24020042`; it is the independently encoded synthetic entry sentinel and is
present identically at cartridge offset and RDRAM physical address `0x1000`.

The transfer mutates the existing `Rdram` backing directly under an immutable
prevalidated plan. It creates no shadow byte owner. Source and destination
out-of-range cases, missing programmed registers, an unsupported domain, and
an unsupported length reject atomically.
