# SP DMA Record Provenance

The public transfer appends existing-format DMA record zero:

- direction: `RdramToSp`;
- trigger: RSP `Mtc0` source index 2 at local PC `0x024`;
- raw length word: `0x00000000`;
- transfer length: 8 bytes;
- block count: 1;
- DRAM skip: 0;
- initial local address: `0x000`;
- initial local-address source: RSP `Mtc0` source index 0;
- initial physical RDRAM address: `0x00000180`;
- initial DRAM-address source: RSP `Mtc0` source index 1;
- final local address: `0x008`;
- final physical RDRAM address: `0x00000188`;
- transferred byte count: 8.

Each resulting DMEM byte names this record through `SpDma` provenance. The
record owns transfer causality, not a duplicate copy of RDRAM or DMEM.
