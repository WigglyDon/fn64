# RI ownership

`rust/crates/fn64-core/src/ri.rs` owns one private per-Machine `Ri` value. Its
only represented register is optional RI_SELECT state containing a `u32` value
and `MachineRiSelectSource::ColdX105Entry`.

`Machine` owns the `Ri` instance. There is no global state, host setter,
register array, numeric register map, callback, device registry, write API, or
general RI abstraction. Public observation is read-only through
`Machine::ri_select_state`; successful load outcomes expose the same narrow
source through `MachineLoadWordTarget::RiSelect`.
