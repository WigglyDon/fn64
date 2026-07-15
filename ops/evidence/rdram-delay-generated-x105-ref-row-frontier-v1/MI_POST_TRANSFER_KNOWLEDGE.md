# MI post-transfer knowledge

Decision: `POST_TRANSFER_MI_READBACK_UNAVAILABLE_UNLESS_PRIMARY_SOURCE_PROVES_EXACT_STATE`.

The pinned sources establish the bounded transfer effect but do not establish
exact MI_INIT_MODE readback fields after consumption. Success therefore clears
the represented current MI_INIT_MODE state instead of retaining stale
length-15/mode-true truth or inventing length zero/mode false. The consumed MI
lineage remains embedded in the RDRAM-delay provenance.

