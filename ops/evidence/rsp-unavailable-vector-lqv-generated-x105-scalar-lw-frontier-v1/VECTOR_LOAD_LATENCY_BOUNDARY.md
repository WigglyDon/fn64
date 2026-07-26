# Vector Load Latency Boundary

Decision:

`LQV_FUNCTIONAL_RESULT_VISIBLE_AT_INSTRUCTION_COMMIT`

`RSP_VECTOR_LOAD_STALL_CYCLES: NOT_REPRESENTED`

The committed available or unavailable destination is the functional result a
dependent instruction would observe after hardware interlocking. Replacing the
whole old state prevents stale destination bytes from appearing after commit.

There is no pending load, countdown, cycle/stall counter, extra Machine step,
or extra RSP committed instruction. CPU Count and VI remain CPU-only.
