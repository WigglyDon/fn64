# RI_CURRENT_LOAD source decision

Decision: `RI_CURRENT_LOAD_UPDATE_EVENT_REPRESENTABLE`.

Direct public-source facts identify RI_CURRENT_LOAD at physical
`0x04700008`, mark it write-only, and state that any write updates current
control. The pinned bounded source writes RI_CONFIG `0x40`, completes its CPU
wait loop, stores r0 to RI_CURRENT_LOAD, constructs `0x14`, and next stores to
RI_SELECT.

The exact event address, write-only relation, any-word trigger, and source
order are direct evidence. Representing the event as a snapshot of the
Machine-owned RI_CONFIG input and enable fields is a bounded fn64 inference:
it records which represented configuration the update request consumed without
inventing an output.

The event records:

- RI_CONFIG current-control input snapshot;
- RI_CONFIG current-control enable snapshot;
- store instruction PC;
- source GPR index and Machine-owned source lineage;
- low transfer word as CPU-store evidence only.

RI_CONFIG must already be available. No analog current value, measurement,
calibration-complete flag, elapsed cycles, RDRAM-ready fact, RI_SELECT effect,
or timing model is represented.
