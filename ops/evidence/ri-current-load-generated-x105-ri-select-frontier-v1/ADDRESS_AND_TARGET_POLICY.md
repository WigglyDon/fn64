# Address and target policy

RI_CURRENT_LOAD is exactly physical `0x04700008`, reached through the existing
direct KSEG0 alias `0x84700008` or KSEG1 alias `0xA4700008`. The generated path
uses KSEG1.

The store classifier adds only this exact target beside the accepted
RI_CONFIG and SP-IMEM targets. RI_MODE `+0x00`, RI_SELECT `+0x0c`, RI_REFRESH,
RI_LATENCY, and all other RI-block writes remain explicit target misses.
There is no RI_CURRENT_LOAD CPU read route, mirroring, RDRAM routing,
range-wide RI classification, MMIO framework, bus, or generalized map.
