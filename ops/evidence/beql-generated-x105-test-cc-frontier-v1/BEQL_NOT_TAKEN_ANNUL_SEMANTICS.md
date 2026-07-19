# BEQL not-taken annul semantics

Unequal available operands commit only BEQL. Application sets `pc=P+8`,
`next_pc=P+12`, creates no delay context, and advances Count once. The word at
`P+4` is not fetched as an execution dependency, decoded as executed truth,
applied, committed, or counted.

The generated `Or r2,r0,r0` at `0xA40009A0` therefore has zero executions and
zero effects. `r2` retains backing storage zero with source classification
`UnknownPifProduced` until the later executed `Or` in TestCCValue.
