# Public x105 MFC0 Sequence

The public generated composition commits:

1. `0x40083800` at local PC `0x000`:
   `Mfc0 r8,SP_SEMAPHORE`. Old semaphore clear returns r8=0, sets the
   semaphore, advances SP PC/next-PC to `0x004/0x008`, increments RSP count to
   1, consumes run-start, and selects CPU. CPU Count remains 252,345.
2. CPU word `0x3C0BB000`, `Lui`, at `0x80000004` commits once. CPU
   PC/next-PC become `0x80000008/0x8000000C`, Count becomes 252,346, CPU
   committed count becomes 252,362, and turn becomes RSP.
3. `0x400B0800` at local PC `0x004`:
   `Mfc0 r11,SP_DRAM_ADDR`. Singular cold source zero returns r11=0 without
   source side effect, advances SP PC/next-PC to `0x008/0x00C`, increments RSP
   count to 2, preserves Consumed run-start, and selects CPU. CPU cadence is
   unchanged.
4. CPU word `0x8D690008`, `Lw`, at `0x80000008` commits once. CPU
   PC/next-PC become `0x8000000C/0x80000010`, Count becomes 252,347, CPU
   committed count becomes 252,363, and turn becomes RSP.

RSP-selected calls do not advance CPU Count, CPU committed count, or VI.
