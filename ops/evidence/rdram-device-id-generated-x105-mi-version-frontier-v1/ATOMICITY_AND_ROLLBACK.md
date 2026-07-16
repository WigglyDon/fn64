# Atomicity and rollback

Non-exception rejection preserves PC/next PC/Count/delay context, all GPR values/lineage, COP0, RDRAM bytes, SP memories/provenance, reservation, RI, MI, prior RDRAM facts, and host state.

Covered causes include wrong word, unknown source, pending MI transfer, non-global/nearby target, absent read route, and rejection after success. Unaligned ordinary/delay-slot stores use existing AdES with exact BadVAddr/EPC/BD, no request mutation, and no normal Count cadence.
