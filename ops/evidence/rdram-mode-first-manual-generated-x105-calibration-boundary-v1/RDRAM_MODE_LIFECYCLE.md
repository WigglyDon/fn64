# RDRAM Mode Lifecycle

Construction, reset, and a complete cold-x105 bootstrap have no request.
Successful exact stores create or replace one request. Repeated complete
bootstrap clears it. Failed bootstrap preserves it atomically. Clones,
snapshots, equality, and independent Machines retain per-instance truth.
