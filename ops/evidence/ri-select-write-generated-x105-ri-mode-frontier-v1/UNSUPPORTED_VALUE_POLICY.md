# Unsupported-value policy

Decision: `RI_SELECT_EXACT_X105_VALUE_ONLY`.

The old source GPR's low 32 bits must equal `0x00000014`. Low words zero,
`0x04`, `0x10`, `0x15`, and any other value reject before mutation. High 32 GPR
bits are outside the `Sw` transfer word, so a 64-bit source whose low word is
exactly `0x14` succeeds.

This is fn64's explicit unsupported boundary. It does not claim hardware traps
or rejects other values, and it does not imply general receive/transmit fields,
reserved-bit semantics, physical-revision coverage, or arbitrary programming.
