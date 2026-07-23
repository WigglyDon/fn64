# Cache event summary

During the 77 committed runtime-program instructions:

- I-cache fills: `10`;
- I-cache hits: `67`;
- D-cache load hits: `4`;
- D-cache load misses: `2`;
- D-cache store hits: `2`;
- D-cache store misses: `1`;
- dirty writebacks: `3`;
- clean replacements: `0`;
- KSEG1 bypass stores: `8`.

PI DMA remains non-snooping. I-cache ownership and bytes are independent of
D-cache updates. No dirty line remains at the boundary.
