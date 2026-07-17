# MI_VERSION lifecycle

- Construction creates `0x02020102`.
- General reset preserves it.
- Complete, repeated, and failed cold-x105 bootstrap preserve it.
- Mutable MI_INIT_MODE and pending-transfer state retain their existing
  independent lifecycle.
- Each Machine contains its own typed identity; all current Machines initialize
  identically without mutable global state.
