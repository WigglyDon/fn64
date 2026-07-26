# Scalar LW DMEM Knowledge Policy

One successful Lw requires four consecutive `SpDmem` observations whose
offsets are coherent and whose byte knowledge is Available. One unavailable
byte rejects the complete operation; inconsistent knowledge rejects before
mutation.

Unavailable DMEM never produces an unavailable scalar value. Backing storage
is not consulted as value truth when knowledge is unavailable.
