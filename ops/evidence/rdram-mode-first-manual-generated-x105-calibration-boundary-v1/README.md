# Exact First Manual RDRAM Mode Request Evidence

This bounded evidence records one synthetic public `Machine::step` composition.
The generated delay-slot `Sw` at `0xA4000BB8` records raw request word
`0x46C0C0C0` at the initial non-global RDRAM_MODE aperture. Execution then
returns from WriteCC and stops before the first RDRAM response-test access at
`0xA4000A2C`.

The request proves CPU intent and provenance only. It proves no RDRAM response,
readback, physical current-control effect, calibration result, module presence,
BOOT-3, or compatibility.
