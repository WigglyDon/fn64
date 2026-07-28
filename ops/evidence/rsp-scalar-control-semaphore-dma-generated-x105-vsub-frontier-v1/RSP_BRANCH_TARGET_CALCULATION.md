# RSP Branch Target Calculation

For `Bltz` and `Bne`, planning captures the old source values and provenance,
the signed 16-bit immediate, the delay-slot address `P + 4`, and the decision.

The target is:

`(delay_slot_address + (sign_extend(immediate) << 2)) & 0x0FFC`

The low twelve aligned local-PC bits are retained. The public targets are
`0x09C`, `0x02C`, and `0x054`.

No CPU 64-bit address rule, generic branch framework, or architectural
exception was introduced.
