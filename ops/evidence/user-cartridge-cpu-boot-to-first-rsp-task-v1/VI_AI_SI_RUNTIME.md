# VI, AI, and SI CPU-side runtime

## VI

One per-Machine `Vi` owns the 14 concrete registers reached by the runtime,
their raw words, CPU-store provenance, current half-line, and committed-step
phase. Programming began at `0x800052E8`. The hostless cadence is deterministic
Machine truth only; there is no renderer, display, or wall-clock claim.

## AI

One per-Machine `Ai` owns raw control, DAC-rate, and bitrate requests plus
provenance. Programming first occurred at PCs `0x800032E8`, `0x800032F4`, and
`0x80003300`. There is no audio DMA, sample production, device clock, or host
audio output.

## SI

One per-Machine `Si` owns 64 bytes of PIF RAM, cold-idle status, and the
explicit hostless input profile `NoControllerConnected`. A PIF-RAM word store
first occurred at `0x80005B84`. There is no controller UI, SI DMA engine,
private PIF execution, or title-specific response.
