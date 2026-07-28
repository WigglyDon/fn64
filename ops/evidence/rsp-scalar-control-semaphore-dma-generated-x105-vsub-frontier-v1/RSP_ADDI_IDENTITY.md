# Exact RSP Addi

`Addi` is recognized only at major opcode `0x08`. It requires an Available old
`rs`, sign-extends the 16-bit immediate, and performs 32-bit wrapping addition.
RSP overflow does not raise an exception.

The source value and provenance are captured before any destination change, so
`rs == rt` is read-before-write. The old destination is otherwise not an
input. Destination `r0` discards only the write. An unavailable source rejects
before mutation.

Public `0x20A5FFFF` at `0x038` decrements `r5` once in every semaphore-branch
delay slot.
