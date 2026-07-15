# MI_INIT_MODE write semantics

The exact physical target is `0x04300000`; supported direct aliases are
`0x84300000` and `0xA4300000`. `Sw` consumes the old source GPR's low 32 bits,
so arbitrary high 32 bits do not affect acceptance.

Only `0x0000010F` succeeds. The immutable plan creates length 15,
initialization mode true, and CPU-store provenance. Application mutates MI
once, commits staged PC/next_pc once, advances Count once, writes no GPR, and
uses the existing ordinary delay-slot cadence.

Success preserves all RI state, RDRAM bytes, SP memories and provenance,
reservations, cartridge data, COP0 except normal Count cadence, and host state.
