# Post-DPC CPU interleave

The authoritative generated cold-x105 composition executes one real
CPU-selected instruction:

- PC `0x80000184`;
- word `0x151fffe3`;
- identity `Bne`;
- r8 `0x94`, produced by Addiu at `0x80000168`;
- r31 `0x00100000`, produced by Lui at `0x800000e0`;
- branch taken;
- post PC/next `0x80000188/0x80000114`;
- delay-context owner `0x80000184`;
- Count/commits `253436/253452`;
- next processor token RSP.

All four DPC counter states survive unchanged. No hidden RSP execution occurs
during this call.
