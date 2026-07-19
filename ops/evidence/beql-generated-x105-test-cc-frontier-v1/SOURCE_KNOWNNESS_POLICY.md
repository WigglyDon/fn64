# Source-knownness policy

BEQL uses the existing bootstrap/instruction-result GPR lineage owner. Both
operands must be available. Unknown `rs`, unknown `rt`, two unknown sources,
and same-register unknown operands reject before control-flow mutation or Count
cadence.

BEQ, BNE, JR, JALR, loads, stores, arithmetic, branch addresses, and device
command knownness retain their prior rules. No unknown execution or symbolic
comparison was added.
