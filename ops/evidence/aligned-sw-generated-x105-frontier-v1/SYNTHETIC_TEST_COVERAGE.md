# Synthetic test coverage

Planned generated coverage includes decode fields; zero/positive/negative and
wrapping address arithmetic; both direct aliases and SP-IMEM endpoints; low
word and big-endian bytes; `r0`; `rs == rt`; exact four-byte/provenance
replacement; later `Lw`; success cadence; sequential and delay-slot AdES;
unknown operands; unsupported targets; blocked exception entry; reset,
bootstrap restaging, and independent Machines; and the generated x105
composition frontier.

The existing `fn64_step_probe` is extended rather than creating another
executable.
