# Interrupt delivery

`Mi` remains sole owner of SP/SI/AI/VI/PI/DP pending and mask facts. `Cpu.Cop0`
owns Status, Cause, EPC, BD, EXL, and interrupt exception control.

After a committed device or mask transition, Machine synchronizes the MI RCP
pending relationship into COP0. Interrupt recognition happens only at an
instruction boundary. A taken interrupt:

- consumes no guest instruction;
- advances Count zero times;
- records EPC as the current PC, or the existing branch owner while in a delay
  slot;
- sets Cause exception code zero and exact BD;
- sets EXL;
- clears the delay context;
- enters the established general exception vector.

The local run first entered this path at boundary PC `0x80001938`, then returned
through guest-executed ERET. No host scheduler or wall clock drove the event.

VI owns the only new deterministic device cadence: one half-line per 1,500
committed Machine steps, with the configured vertical-sync field length. A VI
pending source is asserted only when a programmed vertical-interrupt line is
reached. Writing VI_CURRENT clears the MI-owned VI pending source.
