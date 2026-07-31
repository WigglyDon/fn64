# RSP Break halt, broke, and CPU frontier

This evidence records one bounded product seam: exact zero-code RSP `Break`,
the existing `Sp`-owned halt/broke transition, conditional assertion of the
existing `Mi`-owned SP-pending bit, and the read-only next CPU frontier.

The public composition uses only the deterministic generated cold-x105 fixture
and public `Machine::step`. It commits Break once, leaves the RSP halted before
either following NOP executes, and does not execute the identified CPU
frontier.

