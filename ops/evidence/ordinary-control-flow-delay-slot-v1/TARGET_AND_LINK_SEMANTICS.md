# Target and link semantics

- Conditional target: `B.wrapping_add(4).wrapping_add((sign_extend(imm16)
  wrapping_mul 4) as u32)`.
- Untaken fall-through: `B.wrapping_add(8)`; the slot still commits first.
- `J/JAL`: `(B.wrapping_add(4) & 0xf000_0000) | ((index26 &
  0x03ff_ffff) << 2)`.
- `JR/JALR`: the plan captures the old encoded `rs` value, then selects its
  low represented 32 address bits.
- Link address: `B.wrapping_add(8)`, sign-extended into the represented 64-bit
  GPR value.
- `JAL` destination: r31. `JALR` destination: encoded `rd`.
- `rd=0` is discarded by the existing zero-register owner.
- For `JALR rs=rd`, target capture and link capture both finish before the
  application writes the link; the old source chooses `next_pc`.

Bootstrap guard: unknown bootstrap source operands reject before mutation.
Nonzero bootstrap link writes also reject because this lane may not modify the
separate bootstrap provenance owner. This is explicit conservative behavior,
not a false known-value claim.
