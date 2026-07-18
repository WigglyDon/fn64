# Link-destination provenance and generated InitCC frontier

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This evidence bounds one CPU correction: the prior value and lineage of a
link destination are not execution inputs. JAL consumes no GPR source. JALR
continues to consume its old `rs`, captured before any link write. The
accepted x105 state remains synthetic public composition through
`Machine::step`; it is not authentic firmware or cartridge execution.

The exact generated JAL at `0xA40001A0` can therefore replace retained PIF
IPL2 r31 truth with PC+8 and named instruction-result provenance. Generated
execution then reaches `Beql r26,r0,0xA4000A00` at `0xA400099C`, which is
the first unsupported pressure. RDRAM_MODE is not reached and remains closed.
