# Public x105 Run-Start Sequence

Pinned public x105 source establishes:

- status command `0x000000CE` sets halt and single-step;
- SP PC is reset to local zero;
- the public RSP program is already present in IMEM;
- status command `0x000000AD` clears halt and single-step and starts RSP
  execution.

The generated Machine composition reproduces the run-start store at CPU
`0xA4000508`, word `0xAC2A0010`, source r10=`0x000000AD`, in the delay slot
owned by `0xA4000504`. Before commit Count is 252,344 and the CPU committed
count is 252,360. Halt and single-step are true, SP PC is zero, MI SP pending
is false, and RSP count is zero.

After the CPU store commits alone, CPU PC/next-PC are
`0x80000004/0x80000008`, Count is 252,345, CPU committed count is 252,361,
halt and single-step are false, RSP next-PC is `0x004`, turn is RSP, and
run-start is Pending.

