# Second DMA Record And Register Evolution

The new record has index 1 and the existing record format:

- direction: `RdramToSp`;
- raw length: `0x00000FFF`;
- block length: 4096;
- block count: 1;
- skip: 0;
- initial local address: `0x000`;
- initial RDRAM address: `0x00000400`;
- final local address: `0x000`;
- final RDRAM address: `0x00001400`;
- transferred bytes: 4096;
- trigger: `RspMtc0 { source_index: 5 }`.

The prior eight-byte DMA record at index 0 remains unchanged. Existing
owner-local address evolution is applied exactly once. Rejection proofs show
no partial record, byte copy, or register evolution on preflight failure.
