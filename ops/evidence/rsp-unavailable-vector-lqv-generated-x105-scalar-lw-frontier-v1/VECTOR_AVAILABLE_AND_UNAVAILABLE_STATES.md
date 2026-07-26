# Vector Available And Unavailable States

Each vector slot is exactly one of:

- `Available { bytes: [u8; 16], source }`; or
- `Unavailable { source }`.

Construction/reset makes every slot unavailable with
`ConstructionOrReset`. An unavailable slot contains no byte array and no
fabricated zero. A cause-known unavailable LQV result stores a boxed
`MachineRspLqvSource`, not partial vector bytes.

A later aligned full LQV replaces the whole old state. All-available input
creates `Available`; any unavailable input creates whole-register
`Unavailable`. The old destination is not consumed for this subset.
