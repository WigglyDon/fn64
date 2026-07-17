# MI_VERSION ownership

The existing per-Machine `Mi` in `fn64-core/src/mi.rs` owns one non-optional
`MachineMiVersionState`. It stores the raw word once. `Machine` exposes only
a read-only observation.

The CPU Lw executor owns destination writeback and its instruction lineage. It
does not own hardware identity. No global, host, profile, filename, cartridge,
or proof fixture owns the word.
