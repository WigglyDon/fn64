# Exact Scalar RSP MFC0 Identity

The represented scalar identity is RSP `Mfc0`.

Decode requires:

- major opcode `0x10`;
- MFC0 transfer selector zero;
- source-defined reserved/function bits zero;
- a scalar destination `rt`;
- control source index 7 (`SP_SEMAPHORE`) or 1 (`SP_DRAM_ADDR`).

Another source index, MTC0, another scalar identity, and all other COP0/COP2
transfers reject before mutation. The old destination is rollback state only,
not an instruction input. Destination r0 discards the scalar write while
preserving source side effects.
