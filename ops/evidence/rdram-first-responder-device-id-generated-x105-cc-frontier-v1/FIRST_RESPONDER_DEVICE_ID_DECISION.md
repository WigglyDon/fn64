# First-responder DEVICE_ID decision

Decision: `RDRAM_FIRST_RESPONDER_DEVICE_ID_EXACT_X105_ZERO_WRITE_ONLY`.

The accepted CPU transfer word is exactly `0x00000000`. The bounded
source-level meaning is a request to assign InitialDeviceID zero to a first
responder. High 32 source-GPR bits remain irrelevant under existing `Sw`
semantics. Every nonzero low word rejects before mutation.

This is not a general DEVICE_ID field decoder or a ranged register route.
