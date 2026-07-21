# Generated CPU Identities

Newly represented identities required by the bounded bring-up are:

- `MULTU`: unsigned multiplication of the low 32-bit operands with
  sign-extended 32-bit HI/LO halves;
- `BNEL`: complete known 64-bit inequality, taken slot or not-taken annul;
- `BLEZL`: signed complete-GPR less-than-or-equal-zero test with likely annul;
- `BGEZL`: signed complete-GPR greater-than-or-equal-zero REGIMM likely test;
- `LBU`: zero-extended known-byte load on the represented SP-IMEM byte route;
- `SB`: known low-byte store on the represented SP-IMEM byte route.

All are general with respect to PC and registers inside their represented
memory/control-flow surfaces. Existing source-knownness, zero-register,
read-before-write, delay-slot, exception, and Count ownership remain. No CACHE,
new TLB, FPU, RSP, or unrelated instruction was implemented.

The return-frame `Lw` operations over four opaque SP-IMEM words transport the
same unavailable lineage into r2-r5. Their deterministic backing zero remains
non-truth; any later genuine consumer still rejects unavailable lineage.

