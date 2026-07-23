# Lifecycle and rollback

Primary caches remain per Machine and participate in clone, equality, complete
snapshots, rollback, repeated bootstrap, and independent-Machine proofs.

Cold software initialization invalidates all 512 D-cache lines. Runtime fills,
dirty bytes, and provenance belong only to that Machine. Repeated complete
bootstrap returns to the same cold relationship through generated cache
initialization and never mutates Cartridge bytes. Failed bootstrap and rejected
steps preserve the complete prior Machine.

Rdram writeback records and backing bytes are also per Machine. No static,
global, host-owned, or inspection-owned cache state exists.
