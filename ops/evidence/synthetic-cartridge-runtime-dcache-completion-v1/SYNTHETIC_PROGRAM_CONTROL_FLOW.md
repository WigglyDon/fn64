# Synthetic program control flow

The leaf program has seven guest-owned stages. Before each comparison it writes
the stage number to r13. Each BNE targets the failure block at `0x8000112C`;
every generated run comparison is equal, so all seven branches follow the
success direction and their Nop delay slots execute once.

Success writes mailbox words at `0xA03FF000..0xA03FF01F`, jumps to
`0x80001124`, and repeats J/Nop.

Failure writes `0xBAD00000 OR stage`, the failing stage, zero final words, and
the partial pass mask, then loops at `0x80001168`. The authoritative proof
records zero executions at or beyond the failure-block entry.
