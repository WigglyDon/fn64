# First-responder ownership

The existing per-Machine `Rdram` remains sole owner of RDRAM bytes, global
delay configuration, global raw REF_ROW, global DEVICE_ID relocation request,
and the new optional `MachineRdramFirstResponderDeviceIdRequestState`.

The state contains only raw word zero, requested initial ID zero, exact RCP 2.0
first-responder aperture classification, and CPU-store provenance. No parallel
RDRAM owner or global mutable state exists.
