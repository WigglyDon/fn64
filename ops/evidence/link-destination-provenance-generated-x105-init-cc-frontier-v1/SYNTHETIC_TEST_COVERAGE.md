# Synthetic test coverage

Planned focused coverage proves:

- JAL independence from prior r31 state and exact replacement lineage;
- JAL link-before-slot and one-step cadence;
- JALR old-rs capture, destination irrelevance, alias order, and r0 discard;
- source-knownness and active-slot rejection preservation;
- unchanged bootstrap staging and lifecycle;
- exact generated outer JAL and delay-slot composition;
- exact InitCCValue entry and represented SP-IMEM saves;
- atomic unknown-store-source frontier without weakening store knownness.

All inputs are public synthetic instructions and generated PIF-shaped bytes.
