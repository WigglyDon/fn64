# RI_CONFIG field decision

Decision: `RI_CONFIG_DEFINED_FIELDS_REPRESENTABLE`.

The pinned public RI definitions identify physical `0x04700004` as the R/W
RI_CONFIG register, bits 5:0 as current-control input, and bit 6 as
current-control enable. The defined mask is therefore `0x0000007F`:

- `current_control_input = word & 0x3F`;
- `current_control_enable = (word & 0x40) != 0`.

These field locations are direct source facts; the mask arithmetic and the
unsupported-high-bit boundary are fn64 inferences. A transfer with any bit
outside `0x7F` set rejects before mutation. That is an explicit unsupported
boundary, not a claim that hardware traps such a write.

The bounded generated word `0x40` produces input zero and enable true. No raw
32-bit register value, reserved-bit state, calibration result, analog state,
or elapsed process is represented.
