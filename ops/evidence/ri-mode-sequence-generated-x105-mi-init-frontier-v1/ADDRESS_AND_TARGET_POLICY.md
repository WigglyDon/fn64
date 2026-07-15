# Address and target policy

RI_MODE is exactly physical `0x04700000`, reached through existing direct KSEG0
alias `0x84700000` or KSEG1 alias `0xA4700000`. The generated path uses KSEG1.

The store classifier adds only this exact target beside SP IMEM, RI_CONFIG,
RI_CURRENT_LOAD, and RI_SELECT. RI_REFRESH, RI_LATENCY, RI errors, MI_INIT_MODE,
and every other unrepresented direct address remain explicit misses. RI_MODE
has no CPU read route.

There is no mirroring, RDRAM routing, range-wide RI classification, numeric
register array, generic MMIO, bus, generalized map, or device registry.
