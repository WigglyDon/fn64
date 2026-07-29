# SP write-DMA source-memory policy

SP memory-address bit 12 selects IMEM; a clear bit selects DMEM. The low twelve
bits select the local byte offset. Source bytes advance contiguously by block
length, independent of DRAM skip.

The public state is transfer word `0x0000B120`, selecting IMEM offset `0x120`.
The 24 eight-byte blocks consume aggregate local range `0x120..0x1E0`.
Unsupported source wrapping rejects before mutation.
