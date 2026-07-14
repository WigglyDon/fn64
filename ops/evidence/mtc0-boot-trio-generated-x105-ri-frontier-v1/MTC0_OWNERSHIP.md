# MTC0 ownership

`Machine::produce_mtc0_step_action` creates one closed immutable plan after all
fallible checks. Its destination enum has only Cause software pending, Count,
and Compare. `Machine::apply_mtc0_step_action` performs the destination-specific
COP0 mutation and then reuses the existing staged control-flow and Count
cadence owners.

`cpu/cop0.rs` remains the sole owner of COP0 storage and destination side
effects. `Machine::step` remains the sole public execution entrance. No numeric
CP0 bank, raw mutation API, generic writer, second executor, or privilege
framework was added.
