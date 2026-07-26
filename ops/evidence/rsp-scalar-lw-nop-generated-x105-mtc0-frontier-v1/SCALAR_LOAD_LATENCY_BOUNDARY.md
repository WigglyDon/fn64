# Scalar Load Latency Boundary

Decision: `SCALAR_LW_FUNCTIONAL_RESULT_VISIBLE_AT_INSTRUCTION_COMMIT`.

The committed destination is the functional value a dependent instruction
would observe after hardware interlocking. The prior destination cannot remain
visible after commit. fn64 represents no load queue, countdown, stall event,
cycle count, or extra instruction commit. CPU Count and VI remain unchanged on
RSP-selected calls.

`RSP_SCALAR_LOAD_STALL_CYCLES`: `NOT_REPRESENTED`.
