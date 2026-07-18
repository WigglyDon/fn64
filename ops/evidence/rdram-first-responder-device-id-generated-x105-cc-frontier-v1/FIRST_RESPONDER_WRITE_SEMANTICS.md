# First-responder write semantics

One exact aligned `Sw` target exists at physical `0x03F08004`, with direct
aliases `0x83F08004` and `0xA3F08004`. Acceptance requires known source
lineage, low word zero, and no pending bounded MI transfer.

Success creates or replaces one request fact, writes no GPR or memory, commits
PC/next-PC once, and advances Count once. It does not require global DEVICE_ID,
RDRAM_DELAY, REF_ROW, RI, or mutable MI state as hidden authorization.
