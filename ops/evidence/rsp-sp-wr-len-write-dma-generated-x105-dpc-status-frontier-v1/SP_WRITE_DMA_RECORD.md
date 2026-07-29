# SP write-DMA record

Success appends one existing-owner typed record after the two accepted
RDRAM-to-SP records. The new record identifies:

- direction `SpToRdram`
- selected source bank IMEM through initial local address `0x1120`
- raw word `0xFE817000`
- length 8, count 24, skip `0xFE8`
- initial RDRAM address `0x002FB1F0`
- final local address `0x11E0`
- final RDRAM address `0x00313070`
- transferred bytes 192
- trigger `RspMtc0` with the exact scalar-source provenance index

The record owns causality and address evolution, not duplicate memory bytes.
