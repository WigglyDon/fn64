# BEQL target calculation

The existing conditional-branch helper remains the single target authority:

`target = P + 4 + (sign_extend(immediate16) << 2)`

It uses wrapping CPU-address arithmetic. Positive and negative focused proofs
cover sign extension. For generated word `0x53400018` at `P=0xA400099C`, the
target is `0xA4000A00`. Not-taken annul commits directly to `P+8 = 0xA40009A4`
with `next_pc = P+12 = 0xA40009A8`.
