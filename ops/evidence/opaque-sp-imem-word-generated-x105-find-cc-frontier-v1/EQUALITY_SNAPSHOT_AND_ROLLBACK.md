# Equality, snapshot, and rollback

`SpImem` equality includes backing storage, per-byte knowledge, and its owned
opaque-word records. Canonical sentinel bytes make equal opaque causes equal
regardless of prior concrete bytes. Complete Machine snapshots observe both
the sentinel and knowledge; failed bootstrap leaves the prior owner untouched.
