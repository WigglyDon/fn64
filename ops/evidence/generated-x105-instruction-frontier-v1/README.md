# Generated x105 instruction frontier

This evidence follows the bounded, generated NTSC-pinned cold-x105 shape from
`0xA4000040` only far enough to identify the first unavailable execution fact.
It records instruction identities and data-flow relationships, not copied IPL3
code or cartridge bytes.

The selected result is:

`DIFFERENT_BOUNDED_FAMILY_SELECTED — SP-DMEM-ROUTED ALIGNED LW`

The first two generated steps can use the represented arithmetic and SP-IMEM
load path. The next load identity is already decoded and executed by
`Machine::step`, but its required SP-DMEM data target is not represented.
Aligned `Sw` is therefore a later frontier and is deliberately not selected in
this pass.

All proof inputs are generated. This evidence does not establish authentic
IPL3 execution, BOOT-3, or game compatibility.
