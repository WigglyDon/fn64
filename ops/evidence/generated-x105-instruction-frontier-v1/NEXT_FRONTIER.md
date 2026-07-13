# Next frontier

After four generated committed steps, the next exact frontier is aligned `Sw`
at synthetic PC `0xA4000050` targeting the already represented SP-IMEM range.
Decode and identity exist, but `Machine::step` has no store planner,
applicator, SP-IMEM instruction-store provenance, or AdES-producing execution
path for `Sw`.

This pass deliberately stops because its earlier missing capability was the
SP-DMEM target of aligned `Lw`, and only one missing bounded capability was
authorized. A future `Sw` pass must independently prove read-before-write,
source knownness, big-endian mutation, provenance, AdES/BadVAddr/EPC/BD,
Count/cadence, delay-slot behavior, and rejection atomicity. It must not add a
generic bus or generalized memory map.
