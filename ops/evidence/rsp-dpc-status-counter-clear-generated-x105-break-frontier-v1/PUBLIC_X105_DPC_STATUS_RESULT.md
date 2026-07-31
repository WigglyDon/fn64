# Public x105 DPC status result

Pre-command:

- CPU PC/next `0x80000184/0x80000188`;
- CPU Count/commits `253435/253451`;
- RSP PC/next `0x098/0x09c`;
- RSP commits 1090; token RSP;
- r3 Available `0x00000240` from Xori at `0x094`;
- all four DPC counters value-unavailable;
- three prior SP DMA records.

One `Machine::step` commits `Mtc0 r3,DPC_STATUS`.

Immediate post-command:

- CPU state and cadence unchanged;
- RSP PC/next `0x09c/0x0a0`;
- RSP commits 1091; token CPU;
- clock and TMEM-load counters Available zero;
- command and pipe counters unchanged Unavailable;
- all scalar, vector, accumulator, VCO/VCC/VCE, SP, DMA, memory, and MI truth
  preserved;
- Break not executed.
