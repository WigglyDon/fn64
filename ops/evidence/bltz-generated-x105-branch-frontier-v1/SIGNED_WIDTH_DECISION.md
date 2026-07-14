# Signed-width decision

Decision: `BLTZ_SIGNED_WIDTH_REUSES_CURRENT_FULL_GPR_POLICY`.

The current product has one stable signed GPR comparison owner:
`cpu/instruction.rs::signed_cpu_value_less_than`. Both represented `SLT`
and `SLTI` use it, and it compares the complete `u64` register values as
signed `i64` values. BLTZ reuses that exact crate-private owner against zero.
No Status bit or separate 32/64-bit execution-mode selector currently changes
signed comparison width.

Required discriminators are not ambiguous:

- `0x0000000080000000` is positive under the current full-width rule and
  must leave BLTZ untaken, although an accidental low-word comparison would
  treat it as negative.
- `0xFFFFFFFF00000000` is negative under the current full-width rule and
  must take BLTZ, although an accidental low-word comparison would treat it as
  zero.
- `0x8000000000000000` is taken and `0x7FFFFFFFFFFFFFFF` is untaken.

Limitation: this is the current represented full-GPR product policy, not a
claim that fn64 models every architectural execution mode. Adding a privilege
or width-mode framework is outside this bounded identity.
