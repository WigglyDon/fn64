# Vector Control Owner Decision

`Sp::rsp` is the sole owner of:

- 32 independently Available or Unavailable vector slots;
- eight accumulator lanes;
- independent VCO carry/borrow and not-equal halves;
- independent VCC and VCE truth;
- immutable vector-arithmetic lineage.

No Machine-level flag owner, generic COP2 bank, generic vector ALU, symbolic
expression graph, or second accumulator owner exists. Snapshots and Machine
equality include this nested state through ordinary `Sp` ownership.
