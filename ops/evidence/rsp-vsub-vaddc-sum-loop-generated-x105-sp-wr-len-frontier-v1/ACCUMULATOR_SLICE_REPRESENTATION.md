# Accumulator Slice Representation

The accumulator has eight lanes. Every lane owns three independent 16-bit
states: high, middle, and low. Each slice is either Available with a value and
source or Unavailable with an exact cause.

Construction, reset, and complete bootstrap make all 24 slices Unavailable
from `ConstructionOrReset`. `Vsub` and `Vaddc` replace only the eight low
slices. High and middle slices are preserved byte-for-byte and cause-for-cause.
This shape avoids fabricating unavailable high or middle bits while accepting
the source-defined partial low-slice writes.
