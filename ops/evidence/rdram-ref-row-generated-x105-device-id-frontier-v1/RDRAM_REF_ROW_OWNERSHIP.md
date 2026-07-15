# RDRAM REF_ROW ownership

The existing concrete `Rdram` remains the sole owner of RDRAM bytes and the
broadcast-delay fact. It now also owns one optional typed broadcast REF_ROW
fact. `Machine` exposes one read-only observation; no parallel byte store,
register array, device registry, or mutable host seam was added.
