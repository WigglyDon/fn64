# RSP MFC0 Provenance

Every written scalar MFC0 result records:

- local RSP instruction PC;
- exact control source (`SP_SEMAPHORE` or `SP_DRAM_ADDR`);
- source value and prior source/provenance;
- the four existing `SpImem` byte-provenance records.

The instruction source is classified without copying bytes as public synthetic
cold-x105 bootstrap, CPU store word/byte, SP DMA record, user-supplied PIF
firmware, mixed known, or test-only generated staging.

Last-instruction truth records local PC, MFC0 identity, destination, control
source, and the same IMEM provenance. No ROM path, digest, title, PC whitelist,
or microcode signature enters provenance.

