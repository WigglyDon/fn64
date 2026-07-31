# RSP DPC counter clear and Break frontier

This evidence records one bounded product seam: a private per-`Machine` DPC
counter owner, exact RSP `Mtc0 r3,DPC_STATUS` command `0x00000240`, one real
CPU interleave, and atomic rejection of RSP `Break` at local PC `0x09c`.

The public composition uses only the repository's deterministic generated
cold-x105 fixture and public `Machine::step`. It preserves the accepted three
SP DMA records and stops before SP completion, MI interrupt-on-break, DPC
status modes, counter cadence, or RDP execution.
