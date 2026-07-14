# RI_SELECT cold-entry lifecycle

- Machine construction: unavailable.
- General `Machine::reset`: unavailable and any prior bounded state cleared.
- Complete NTSC_PINNED/X105/cold/cartridge bootstrap: known zero from
  `ColdX105Entry`, applied with the already complete replacement state.
- Repeated complete cold bootstrap: known zero recreated.
- Unsupported or incomplete selector state: bootstrap rejects before any RI
  mutation.
- Ordinary represented-reset-subset bootstrap: unavailable.
- Independent Machines: independent optional RI_SELECT state.

No represented path creates a nonzero value. No RI write or NMI lifecycle is
present.
