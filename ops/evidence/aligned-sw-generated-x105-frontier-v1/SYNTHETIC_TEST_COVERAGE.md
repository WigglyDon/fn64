# Synthetic test coverage

Passing generated coverage includes decode fields; zero/positive/negative and
wrapping address arithmetic; both direct aliases and SP-IMEM endpoints; low
word and big-endian bytes; `r0`; `rs == rt`; exact four-byte/provenance
replacement; later `Lw`; success cadence; sequential and delay-slot AdES;
unknown operands; unsupported targets; blocked exception entry; reset,
bootstrap restaging, and independent Machines; and the generated x105
composition frontier.

The existing `fn64_step_probe` was extended rather than creating another
executable. It proves committed SP-IMEM `Sw`, bytes/provenance, `Lw` round
trip, zero and alias cases, sequential/delay-slot AdES, unknown-source and
unsupported-target rejection, and the post-store `RegimmBltz` frontier.
