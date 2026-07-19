# No branch-likely framework audit

Only `CpuInstructionIdentity::Beql` was added to the existing ordinary
control-flow action. There is no generic likely-family dispatcher, second
delay owner, recursive step, pseudo-commit, speculative path, pipeline model,
PC/function/register whitelist, or new public mutable surface.

Focused decode/step proofs keep BNEL, BLEZL, BGTZL, BLTZL, BGEZL, BLTZALL,
and BGEZALL unrepresented.
