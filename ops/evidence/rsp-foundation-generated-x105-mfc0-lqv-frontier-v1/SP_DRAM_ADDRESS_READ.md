# SP DRAM Address Read

`Sp` remains the singular owner of the current SP DRAM address.

- construction/reset value: `0x00000000`;
- programming mask: `0x00FFFFF8`;
- represented CPU writes and completed atomic DMA evolution retain their
  existing owner and provenance.

`Mfc0 SP_DRAM_ADDR` returns that currently owned 24-bit-aligned value. It has
no source-register side effect. The old scalar destination is not consumed;
an r0 destination discards only the scalar write. No second DRAM-address
register or general RSP COP0 bank was introduced.

