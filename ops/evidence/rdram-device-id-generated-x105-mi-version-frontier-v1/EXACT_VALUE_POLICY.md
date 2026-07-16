# Exact value policy

Accepted old source low word: `0x80000000` only. Atomic rejection covers `0`, `0x02000000`, `0x40000000`, `0x7FFFFFFF`, `0x80000001`, `0xC0000000`, and `0xFFFFFFFF`.

Unknown lineage rejects. Full-width `0xFFFFFFFF80000000` succeeds because `Sw` consumes its low word. Rejection is fn64's bounded unsupported surface, not a hardware-trap claim.
