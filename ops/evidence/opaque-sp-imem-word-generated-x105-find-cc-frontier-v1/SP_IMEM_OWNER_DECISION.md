# SP-IMEM owner decision

The existing `SpImem` in `rust/crates/fn64-core/src/sp_imem.rs` remains the
sole owner of backing storage, concrete knowledge, opaque aligned-word state,
CPU-store provenance, bootstrap replacement, equality, and lifecycle.

Four compact byte-knowledge markers name the same aligned word. The same
`SpImem` owns one immutable causal record for that word. No Machine-level map,
second byte store, or generalized memory-knowledge owner exists.
