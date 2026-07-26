# Accumulator And Flag Boundary

`MachineRspAccumulatorAndFlagsState` remains explicitly unavailable.
Accumulator, VCC, VCO, and VCE have no concrete storage or behavior.

Construction, reset, MFC0, run-start, SP-PC writes, and represented LQV leave
that unavailable state unchanged. No vector arithmetic, comparison, select,
multiply, add, logical, or divide identity is represented.
