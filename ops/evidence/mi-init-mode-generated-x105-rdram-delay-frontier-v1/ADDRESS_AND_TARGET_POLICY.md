# Address and target policy

The store classifier adds only exact physical address `0x04300000`. Current
direct-address law admits KSEG0 `0x84300000` and KSEG1 `0xA4300000` aliases.

Nearby MI addresses remain direct target misses. Existing exact RI and SP
targets remain distinct. The RDRAM global aperture `0x03F80008` is not direct
RDRAM byte storage and remains a target miss. No MI range, RDRAM-register
range, bus, or generic memory map is classified.
