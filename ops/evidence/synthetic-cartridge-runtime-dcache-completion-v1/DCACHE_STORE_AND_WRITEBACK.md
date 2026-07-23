# D-cache store and writeback

CPU remains the sole primary D-cache owner. Rdram remains the sole backing-byte
owner.

KSEG0 `Sw` and `Sb` are write-allocate operations. A hit updates selected
big-endian cache bytes and marks the line valid dirty. A miss preflights the
target 16-byte fill line and any dirty victim writeback, writes the complete
victim to Rdram atomically, fills the requested line, applies the store, and
marks it dirty.

KSEG0 `Lw` and `Lbu` read matching clean or dirty lines. A miss performs the
same atomic dirty-victim handling before a clean fill. KSEG1 loads and stores
continue to bypass D-cache.

Runtime-v2 measured events:

- load hits / misses: `4 / 2`;
- store hits / misses: `2 / 1`;
- dirty writebacks: `3`;
- clean replacements: `0`;
- KSEG1 mailbox bypass stores: `8`.

Exact writeback physical line ranges:

1. `0x00100000..0x0010000F`;
2. `0x00102000..0x0010200F`;
3. `0x00100000..0x0010000F`.
