# Synthetic test coverage

Focused independent fixtures cover exact decode, nonzero-code and adjacent
SPECIAL rejection, no operands/exceptions, ordinary successor PC state,
halt/broke effects, status preservation, all four interrupt-on-break/pending
combinations, idempotent already-pending behavior, CPU-only interrupt
recognition, delay-slot and single-step rejection, full architectural
preservation, reset/bootstrap/failed-bootstrap lifecycle, and Machine
independence.

The public generated fixture separately proves the authentic pre-Break state,
one exact commit, DPC/DMA/memory preservation, halted successor state, and the
unexecuted CPU and RSP frontiers.

