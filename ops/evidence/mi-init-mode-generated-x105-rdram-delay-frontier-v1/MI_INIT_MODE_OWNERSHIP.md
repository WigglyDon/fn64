# MI_INIT_MODE ownership

`Machine` owns one private `Mi`. `Mi` owns one optional
`MachineMiInitModeState`. The state contains initialization length,
initialization mode, and one `MachineMiInitModeSource::CpuStoreWord`.

There is no global state, host setter, numeric register bank, generic device
registry, bus, MMIO framework, or CPU read route. The public surface is one
narrow read-only `Machine::mi_init_mode_state` observation.
