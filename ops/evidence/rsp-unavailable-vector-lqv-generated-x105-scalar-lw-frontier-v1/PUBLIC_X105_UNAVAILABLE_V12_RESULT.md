# Public x105 Unavailable-v12 Result

Before LQV:

- CPU `PC/next_pc = 0x8000000C/0x80000010`;
- CPU Count/commit = `252347/252363`;
- SP `PC/next_pc = 0x008/0x00C`;
- RSP count `2`, turn RSP, consumed run-start, semaphore set;
- scalar `r8` and `r11` available zero from their exact MFC0 sources;
- all vector slots unavailable.

`Machine::step` commits public `0xC80C2000` once. Afterward:

- SP `PC/next_pc = 0x00C/0x010`;
- RSP count `3`, turn CPU;
- `v12` is unavailable with exact LQV/IMEM/base/DMEM-knowledge cause and no
  byte array;
- all other vectors remain construction/reset unavailable;
- CPU Count/commit remain `252347/252363`;
- DMEM, scalars, semaphore, run-start, MI, and VI are unchanged.
