# LQV Unavailable And Mixed Source Policy

When one or more of the sixteen source bytes is unavailable, the complete
aligned LQV still has an exact cause and complete-overwrite shape. Planning
therefore creates one whole-register unavailable result.

The unavailable result records all sixteen address-and-source knowledge
descriptors. It contains no vector byte array, no values copied from
unavailable backing, and no partial payload from available subset bytes.
All-unavailable and mixed ranges use the same smallest general rule.

A later complete concrete LQV may replace this state. Any future consumer or
store requiring unavailable bits must reject before mutation.
