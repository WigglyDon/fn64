# Public x105 Break result

One public `Machine::step` commits Break exactly once:

- RSP PC/next: `0x0a0/0x0a4`;
- RSP committed count: 1092;
- processor turn: CPU;
- halt: true;
- broke: true;
- single-step: false;
- interrupt-on-break: false;
- MI SP pending: false;
- last RSP instruction: exact Break at `0x09c`;
- run-start lineage: preserved.

CPU state, DPC state, every scalar/vector/accumulator/control value, all three
DMA records, SP memories, RDRAM, cartridge, reservations, devices, and host
truth equal their pre-Break state.

