# Address and target policy

RI_SELECT is exactly physical `0x0470000C`, reached through existing direct
KSEG0 alias `0x8470000C` or KSEG1 alias `0xA470000C`. The generated path uses
KSEG1.

The store classifier adds only this exact target beside accepted SP-IMEM,
RI_CONFIG, and RI_CURRENT_LOAD targets. RI_MODE `+0x00`, RI_REFRESH,
RI_LATENCY, error state, and all other RI-block writes remain target misses.
There is no mirroring, RDRAM route, range-wide classification, numeric register
array, MMIO framework, bus, or generalized map.
