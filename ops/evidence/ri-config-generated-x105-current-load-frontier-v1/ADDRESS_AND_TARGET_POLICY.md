# Address and target policy

RI_CONFIG is exactly physical `0x04700004`, reached through the existing direct
KSEG0 alias `0x84700004` or KSEG1 alias `0xA4700004`. The generated path uses
KSEG1.

The store classifier recognizes only this one RI write target. RI_MODE
`+0x00`, RI_CURRENT_LOAD `+0x08`, RI_SELECT `+0x0c`, RI_REFRESH, RI_LATENCY,
and all other RI-block addresses remain explicit target misses. RI_CONFIG has
no CPU read route. There is no mirroring, RDRAM routing, range-wide RI
classification, MMIO framework, bus, or generalized map.
