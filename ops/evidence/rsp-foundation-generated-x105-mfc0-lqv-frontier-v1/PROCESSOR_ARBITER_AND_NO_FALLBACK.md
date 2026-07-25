# Processor Arbiter And No Fallback

Each Machine owns one private `MachineStepProcessor` turn.

- construction/reset begins `Cpu`;
- halt true canonicalizes selection to CPU and performs no RSP fetch;
- halt false plus `Rsp` selects one RSP attempt;
- halt false plus `Cpu` selects the existing CPU path;
- a CPU halt true-to-false commit selects RSP next;
- any successful CPU commit while RSP remains running selects RSP next;
- a successful RSP commit selects CPU next;
- a selected rejection preserves the token and performs no fallback.

Selection precedes CPU interrupt synchronization, fetch, Count planning, and
VI cadence. The 1:1 successful-commit alternation is deterministic and
host-independent, not a hardware-frequency or cycle-accuracy claim.

