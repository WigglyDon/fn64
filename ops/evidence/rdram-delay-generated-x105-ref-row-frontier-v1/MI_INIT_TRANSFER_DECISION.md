# MI initialization transfer decision

Decision: `EXACT_X105_MI_INIT_TO_RDRAM_DELAY_PAIR_ONLY`.

The accepted command word `0x0000010F` provides initialization length 15 and
the bounded source identifies the next written value as a 16-byte transfer.
fn64 therefore arms exactly one transfer for the exact generated RDRAM_DELAY
consumer. It is not a generic burst, transaction, queue, bus, or timing model.
No other represented successful store may bypass the pending effect.

