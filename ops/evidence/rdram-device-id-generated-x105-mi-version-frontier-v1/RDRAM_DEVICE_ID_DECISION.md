# RDRAM DEVICE_ID decision

Decision: `RDRAM_DEVICE_ID_EXACT_X105_BROADCAST_WORD_ONLY`.

The accepted surface is one aligned global/broadcast store whose old source low word is exactly `0x80000000`. Every other low word rejects before mutation. High GPR bits are ignored by existing `Sw` semantics.

The bounded source establishes the transformation `0x02000000 << 6` and requested base `0x02000000`. No general DEVICE_ID decoder is represented.
