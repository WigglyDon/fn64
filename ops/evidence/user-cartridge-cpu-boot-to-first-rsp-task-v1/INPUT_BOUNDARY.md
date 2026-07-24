# Input boundary

The local proof accepts one explicit host path. Host code owns argument parsing,
file reading, read errors, and transfer of one owned byte vector. It reports
only the selected basename, source length, detected source layout, and
normalized length.

`load_cartridge` detects the byte order and constructs the Machine-owned
`Cartridge`. Product behavior does not receive or retain the path. The Machine
does not inspect a filename, title, cartridge ID, region, checksum, CRC, or
digest to select behavior.

The user cartridge begins execution only after fn64's public deterministic x105
bootstrap, RDRAM bring-up, cache initialization, IPL3 relocation, PI DMA,
checksum, and final handoff complete through `Machine::step`. The public
bootstrap is not proprietary PIF execution and does not promote the authentic
checkpoint.

The committed optional probe is no-window, requires an explicit path, has a
positive step ceiling, and is excluded from ordinary CI inputs. Standard tests
continue to use public synthetic bytes only.
