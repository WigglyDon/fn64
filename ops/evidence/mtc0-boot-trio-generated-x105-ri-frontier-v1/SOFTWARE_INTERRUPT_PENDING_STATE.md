# Software-interrupt pending state

COP0 owns one masked two-bit value and one knownness bit. Construction, reset,
and cartridge-bootstrap replacement leave the value concretely zero but
semantically unknown. A successful MTC0 Cause is the production point that
makes IP1:IP0 known, including an explicit known-zero write.

The state is per Machine. Generated tests prove independent instances, reset,
and bootstrap restaging. Pending state does not cause interrupt entry in this
product.
