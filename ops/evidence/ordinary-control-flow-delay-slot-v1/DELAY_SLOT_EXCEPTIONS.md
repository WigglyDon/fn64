# Delay-slot exceptions

COP0 obtains `(EPC, BD)` from explicit CPU delay context. With active context,
EPC is its owning branch/jump PC and BD is true. With no context, existing
sequential entry uses the current PC and BD false. Successful entry clears
context only after every entry guard is known.

| Exception | Generated setup | Proved result |
| --- | --- | --- |
| arithmetic overflow | `J` at `0x80000000`; slot `ADDI` overflows | EPC `0x80000000`, BD true, code 12, BadVAddr unchanged, destination unchanged, Count remains 1, vector entered |
| instruction-fetch AdEL | test-only slot PC `0x80000006`, owner `0x80000000`, selected target `0x80000020` | EPC owner, BD true, code 4, BadVAddr slot PC, Count remains staged 1, vector entered |
| data-AdEL | untaken `BEQ` at `0x80000000`; slot unaligned `Lw` at `0x80000004` | EPC owner, BD true, code 4, BadVAddr `0x80000101`, destination unchanged, Count remains 1, vector entered |

In every case the selected target/fall-through is never committed as `pc`, the
faulting slot advances Count zero, and context does not leak into vector
execution. Exception code and BadVAddr remain owned by their existing paths.
