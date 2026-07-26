# NOP Cadence And Preservation

Each NOP is fetched from `SpImem`, commits through a separate public
`Machine::step`, advances current/next local PC once, increments only the RSP
committed count once, records exact fetch provenance, and selects CPU next.

The words at local `0x010` and `0x014` are two distinct commits. One ordinary
CPU-selected call separates each successful RSP commit. They are neither
batched nor skipped.
