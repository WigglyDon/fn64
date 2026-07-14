# RI_CURRENT_LOAD write semantics

The represented `Sw` route uses the existing old-base/old-source capture,
sign-extended immediate, alignment check, direct-address law, and low-32-bit
transfer. Only physical `0x04700008` is classified as RI_CURRENT_LOAD.

Planning requires a known source and an already stored RI_CONFIG. It then
builds a closed event containing the RI_CONFIG input/enable snapshot and CPU
store evidence. Application assigns that event once and invokes the existing
sequential or ordinary-delay-slot cadence once. It writes no GPR or memory and
does not mutate RI_SELECT or RI_CONFIG. Unaligned access uses existing AdES;
missing RI_CONFIG and all unsupported targets reject before mutation.
