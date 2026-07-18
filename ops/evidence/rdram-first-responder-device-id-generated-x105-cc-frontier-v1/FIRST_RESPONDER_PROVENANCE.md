# First-responder provenance

Generated provenance is exact:

- store PC: `0xA4000198`;
- source GPR: r14/t6;
- source value and low word: zero;
- source lineage: `KnownInstructionResult`, generated `Addu r14,r0,r0` at
  `0xA4000138`;
- base GPR: r17=`0xFFFFFFFFA3F08000`, generated `Ori` lineage from
  `0xA4000194`;
- effective address: `0xFFFFFFFFA3F08004`;
- CPU address: `0xA3F08004`;
- physical address: `0x03F08004`.

The state stores source provenance, not a module response.
