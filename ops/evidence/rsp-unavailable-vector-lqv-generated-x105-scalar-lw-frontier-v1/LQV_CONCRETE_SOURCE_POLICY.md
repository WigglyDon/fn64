# LQV Concrete Source Policy

When all sixteen `SpDmem` observations are available, planning captures each
exact value and source. Application replaces one destination slot with an
available sixteen-byte vector and exact `MachineRspLqvSource` provenance.

The source values map sequentially from DMEM to vector byte elements. No old
destination byte or availability is an instruction input. The load does not
mutate DMEM or any other vector slot.
