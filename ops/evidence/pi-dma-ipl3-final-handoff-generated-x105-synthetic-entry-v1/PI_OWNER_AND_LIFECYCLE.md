# PI owner and lifecycle

`rust/crates/fn64-core/src/pi.rs` is the single PI state owner. It retains the
three programmed registers with CPU-store provenance, the three status bits,
the last completed cart-to-RDRAM transfer, and the last status-clear command.
It does not own cartridge bytes, RDRAM bytes, CPU caches, or MI interrupt
state.

Cold construction and successful repeated bootstrap establish idle status,
no programmed registers, and no completion record. Reset does the same.
Failed bootstrap preflight preserves the complete prior PI state. Clone,
snapshot equality, rollback tests, and independent `Machine` tests include the
new owner, with no static mutable state.

The one completed record stores trigger PC, all three programming sources and
lineage, cartridge bus and byte addresses, RDRAM destination, byte count,
direction `CartridgeToRdram`, and completion `AtomicFunctional`.
