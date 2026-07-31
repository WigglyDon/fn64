# Public x105 pre-Break state

The public generated cold-x105 composition reaches:

- CPU PC/next: `0x80000188/0x80000114`;
- CPU Count/committed: `253436/253452`;
- active CPU delay owner: `0x80000184`;
- RSP PC/next: `0x09c/0x0a0`;
- RSP committed count: 1091;
- processor turn: RSP;
- halt/broke/single-step/interrupt-on-break: all false;
- MI SP pending: false;
- DMA records: 3;
- DPC clock/TMEM counters: Available zero from the accepted status clear;
- DPC command/pipe counters: preserved Unavailable;
- frontier word: `0x0000000d`.

The pre-implementation baseline rejected this word without mutation or CPU
fallback.

