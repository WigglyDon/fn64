# Generated InitCC call path

After the outer JAL and slot, public Machine execution reaches:

- InitCCValue prologue and 24 exact SP-IMEM stack saves from
  `0xA400087C` through `0xA40008EC`;
- `0xA40008F0`, word `0x0D000261`, `Jal 0xA4000984`;
- `0xA40008F4`, word `0x00000000`, its Nop delay slot;
- FindCC entry and initialization from `0xA4000984` through
  `0xA4000998`;
- `0xA400099C`, word `0x53400018`, unsupported `Beql`.

At the frontier r12/t4 is zero, r26/k0 is one from the generated Slti, r0 is
architectural zero, and the branch target is `0xA4000A00`.

