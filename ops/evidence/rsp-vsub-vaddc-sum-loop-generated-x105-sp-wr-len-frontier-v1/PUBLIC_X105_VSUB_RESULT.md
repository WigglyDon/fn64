# Public X105 Vsub Result

Pre-state:

- CPU PC/next: `0x800000E8/0x800000EC`
- CPU Count/committed: `252401/252417`
- RSP PC/next/count/turn: `0x060/0x064/56/RSP`
- v13 and both VCO halves: Unavailable / `ConstructionOrReset`
- all accumulator slices, VCC, and VCE: Unavailable / `ConstructionOrReset`

One public `Machine::step` commits `0x4A0D6B51` once.

Post-state:

- RSP PC/next/count/turn: `0x064/0x068/57/CPU`
- v13: cause-known Unavailable due to the unavailable borrow byte
- accumulator-low: cause-known Unavailable due to Vsub
- accumulator high/middle: preserved `ConstructionOrReset` Unavailable
- both VCO halves: Available zero
- VCC/VCE: preserved Unavailable
- CPU state, Count, committed count, and VI: unchanged
