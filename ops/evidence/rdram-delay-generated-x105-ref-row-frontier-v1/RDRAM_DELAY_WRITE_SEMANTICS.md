# RDRAM delay write semantics

Only aligned direct aliases `0x83F80008` and `0xA3F80008` classify as the exact
target. Success requires known source lineage, low word `0x18082838`, and the
exact pending 15/16 transfer derived from command `0x0000010F`. High source
bits are ignored by existing Sw semantics. One immutable plan creates the
broadcast fact, consumes the transfer, makes MI readback unavailable, commits
PC/next_pc once, and advances Count once without changing any GPR or RDRAM byte.
