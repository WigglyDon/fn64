# RDRAM Mode Owner Decision

The existing per-Machine `Rdram` in `rust/crates/fn64-core/src/rdram.rs`
remains the sole owner of RDRAM bytes and request/configuration facts. Its one
optional initial-mode request is not duplicated in `Machine`, inspection, or a
generic register map.
