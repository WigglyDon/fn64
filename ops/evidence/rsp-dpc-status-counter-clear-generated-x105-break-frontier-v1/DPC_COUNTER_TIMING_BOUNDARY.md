# DPC counter timing boundary

Represented:

- four independent counter knowledge states;
- one exact two-counter clear command;
- exact immutable clear provenance;
- ordinary one-instruction RSP cadence.

Not represented:

- RDP or DPC clock progression;
- command, pipe, or TMEM counter progression;
- command-buffer execution;
- busy duration;
- rasterization;
- cycles, wall clock, host timers, or service events.

Known zeros remain zero until a later earned architectural effect changes them.
This is model truth, not a hardware timing claim.
