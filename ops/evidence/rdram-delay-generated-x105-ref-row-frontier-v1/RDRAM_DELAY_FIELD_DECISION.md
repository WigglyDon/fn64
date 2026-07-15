# RDRAM delay field decision

Decision: `RDRAM_DELAY_BROADCAST_CONFIGURATION_FACT_ONLY`.

The exact logical configuration is:

- Ack-window delay: 5
- Read delay: 7
- Ack delay: 3
- Write delay: 1
- Packed logical configuration: `0x28381808`

The CPU transfer word `0x18082838` is rotated provenance, not logical readback.
