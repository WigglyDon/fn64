# Exact RSP Bne

`Bne` is recognized only at major opcode `0x05`. Both scalar operands must be
Available and their complete 32-bit values are compared. Register indices do
not substitute for value truth.

Public branches:

- `0x1460FFFD` at `0x034`: delay slot `0x038`, target `0x02C`;
- `0x1460FFFE` at `0x058`: delay slot `0x05C`, target `0x054`.

The semaphore branch is taken for each failed acquisition and not taken for
the successful acquisition. The DMA-busy branch is not taken after atomic
completion. Both not-taken branches still commit one delay slot.
