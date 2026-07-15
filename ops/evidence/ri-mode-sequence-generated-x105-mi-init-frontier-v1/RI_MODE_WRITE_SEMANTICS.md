# RI_MODE write semantics

The aligned-`Sw` route captures the old base and old source, applies the
existing sign-extended address rule, checks word alignment, classifies exactly
physical `0x04700000`, requires known source lineage, and uses the old source's
low 32 bits.

With all undefined high bits zero, the transfer becomes the three represented
fields plus `CpuStoreWord` provenance. Application assigns RI_MODE once and
uses existing sequential or ordinary-delay-slot cadence once. It writes no
GPR or memory and preserves RI_SELECT, RI_CONFIG, and RI_CURRENT_LOAD.

Unaligned access uses existing AdES. Unknown operands, unsupported addresses,
and undefined high bits reject before RI mutation or normal cadence.
