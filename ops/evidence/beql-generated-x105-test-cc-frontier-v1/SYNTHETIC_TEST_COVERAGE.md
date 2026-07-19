# Synthetic test coverage

Core proofs cover opcode identity, full 64-bit equality, high-half
discrimination, positive and negative targets, source aliasing, architectural
zero, unavailable sources, active-delay rejection, taken slot cadence,
slot exception EPC/BD, not-taken annul of GPR/memory/device/exception and
unsupported words, other likely identities remaining closed, and the complete
generated path through the RDRAM_MODE miss.

The no-window step probe adds stable taken, annul, unknown-source,
active-delay, generated-BEQL, TestCCValue, WriteCC, and RDRAM_MODE-frontier
cases.
