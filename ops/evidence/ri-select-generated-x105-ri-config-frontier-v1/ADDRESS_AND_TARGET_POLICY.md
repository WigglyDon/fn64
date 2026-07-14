# Address and target policy

Only physical word `0x0470000C` is represented. Existing direct-address law
therefore accepts KSEG0 `0x8470000C` and KSEG1 `0xA470000C`; the generated
frontier uses KSEG1. There is no mirroring.

RI_MODE `+0x00`, RI_CONFIG `+0x04`, RI_CURRENT_LOAD `+0x08`, RI_REFRESH
`+0x10`, RI_LATENCY `+0x14`, RI error words, addresses outside the RI block,
and every other direct target continue to miss explicitly. The exact word is
not routed through RDRAM storage or a generic MMIO layer.
