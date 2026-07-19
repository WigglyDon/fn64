# BEQL identity decision

`BEQL_EXACT_CPU_IDENTITY_ONLY` is represented. Opcode `0x14` already decoded
as `CpuInstructionIdentity::Beql`; the product change routes that one identity
through ordinary control-flow planning and application. BNEL, BLEZL, BGTZL,
REGIMM likely branches, likely-link branches, and coprocessor likely branches
remain unrepresented.

No general branch-likely family or new Machine state owner was added.
