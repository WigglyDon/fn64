# Break interrupt-on-break input

Interrupt-on-break is an existing `Sp`-owned input to Break. Break preserves
that bit exactly.

The public run-start status command cleared interrupt-on-break. No later
bounded public CPU or RSP instruction sets it before Break, and the reproduced
pre-Break state is false. The public Break therefore does not assert
`MI_INTR_SP`.

