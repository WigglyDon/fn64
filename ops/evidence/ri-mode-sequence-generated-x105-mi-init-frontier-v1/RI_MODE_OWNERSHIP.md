# RI_MODE ownership

The existing private per-`Machine` `Ri` owner gains one optional RI_MODE fact.
It stores operating-mode bits, stop-transmit-active, stop-receive-active, and
exact CPU-store lineage. The source records instruction PC, source GPR, and
the prior Machine-owned GPR source.

Construction, general reset, and complete cold-x105 bootstrap leave RI_MODE
unavailable. The first generated store creates it; the second replaces both
fields and provenance. Independent Machines remain independent.

There is no CPU read route, mutable public surface, raw RI register array,
host setter, global state, timing state, electrical engine, MMIO trait, bus,
map, or device registry.
