# RI_SELECT write semantics

The represented `Sw` route uses existing old-base/old-source capture,
sign-extended immediate, alignment classification, direct-address law, and
low-32-bit transfer. Only physical `0x0470000C` is classified as RI_SELECT.

Planning requires a known source and transfer word exactly `0x00000014`. It
then creates a closed RI_SELECT state with CPU-store lineage. Application
replaces RI_SELECT once and invokes existing sequential or ordinary-delay-slot
cadence once. It writes no memory or GPR and does not mutate RI_CONFIG or
RI_CURRENT_LOAD. Unaligned access uses existing AdES; every other low word and
unsupported target rejects before mutation.

RI_CONFIG and RI_CURRENT_LOAD are not authorization inputs. Their source order
is composition evidence only.
