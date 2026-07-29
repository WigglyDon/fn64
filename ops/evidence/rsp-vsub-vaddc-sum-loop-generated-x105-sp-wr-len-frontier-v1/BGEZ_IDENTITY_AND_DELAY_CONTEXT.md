# Bgez Identity And Delay Context

Exact identity:

- REGIMM major opcode `0x01`;
- exact selector `rt=0x01`;
- Available scalar source required;
- taken when source bit 31 is clear.

The target is the low-12 aligned result of delay-slot address plus the signed
16-bit offset shifted left two. Public word `0x0461FFFD` at `0x06C` targets
`0x064`; its delay slot is `0x070`.

The branch commits alone, creates one `Sp::rsp` delay context, and selects CPU.
That context survives the real CPU-selected call. The later `Vaddc` commits as
one separate slot, follows target/fallthrough, and clears the context once.
