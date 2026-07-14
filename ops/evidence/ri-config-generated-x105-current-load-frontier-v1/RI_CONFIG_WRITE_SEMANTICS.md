# RI_CONFIG write semantics

The represented `Sw` route captures the old base and old source before
mutation, sign-extends the immediate through the existing address rule, and
uses the old source's low 32 bits. Alignment and the exact direct target are
resolved before the destination-specific plan is built.

Only physical `0x04700004` is accepted. A transfer word with no bits outside
`0x0000007f` becomes:

- `current_control_input = word & 0x3f`;
- `current_control_enable = (word & 0x40) != 0`.

Application changes RI_CONFIG once and then uses existing sequential or
ordinary-delay-slot cadence. It writes no GPR or memory. Unaligned access uses
existing AdES; unsupported targets and values reject before mutation.
