# RDRAM boundary

All 24 destination blocks preflight inside the represented RDRAM owner before
any byte is written. The 192 selected pre-transfer destination bytes are zero
in the public fixture and have FNV-1a-64 digest `ab0c262759a1d225`.

No destination ranges overlap. Unrelated RDRAM bytes remain outside the
mutation plan.

