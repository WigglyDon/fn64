# Manufacturer and RAS Decision

The current profile deliberately chooses NEC manufacturer word `0x00000500`
with enhanced-speed false. This is one fixed Machine identity, not a statement
about every retail console or Expansion Pak.

The pinned manufacturer branch consequently selects
`RASINTERVAL(16, 28, 10, 4)`, raw word `0x101C0A04`. Generated execution writes
that exact word to both present modules. No timing or electrical effect is
represented.
