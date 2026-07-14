# Cause writable mask

MTC0 Cause uses mask `0x00000300`: source bit 9 writes IP1 and source bit 8
writes IP0. All other source bits are ignored. Exception code, BD, Status, EPC,
BadVAddr, Count, Compare, and the separately owned timer-pending latch remain
unchanged except for the ordinary post-commit Count cadence.

Generated tests cover zero, IP0, IP1, both bits, and unrelated source bits.
They also preserve a staged exception code and BD fact and a latched timer fact.
No interrupt delivery is represented.
