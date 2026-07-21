# State Ownership

`Rdram` remains the sole owner of bytes, immutable profile, module collection,
module mappings, RDRAM register words, calibration status, and digital response
policy. `Mi` alone owns RDRAM-register-mode state. `Ri` alone owns
`RI_REFRESH`. `Cpu` owns GPR/HI/LO/control-flow/Count. `SpImem` owns concrete
and opaque stack truth.

Inspection reads public immutable observations and drives only public
`Machine::step`; it owns no emulated state. There is no parallel module owner,
general register bank, generic bus, generic MMIO layer, or host calibration
policy.
