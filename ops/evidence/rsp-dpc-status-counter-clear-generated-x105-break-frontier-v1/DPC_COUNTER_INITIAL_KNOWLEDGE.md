# DPC counter initial knowledge

Construction, general reset, and complete cartridge bootstrap create all four
counters as value-unavailable with
`ConstructionOrResetUndefined` provenance.

The primary documentation calls the power-up values undefined. fn64 does not
convert undefined into zero. Only an exact architectural clear establishes
known zero.
