# Synthetic test coverage

| Family | Proof |
| --- | --- |
| structural validation | exact 1,984-byte acceptance; malformed short/other length; named unsupported 2 KiB full-address-space image |
| no hash selector | two different generated accepted patterns receive identical classification and probe behavior |
| Machine ownership | byte-exact private ownership and caller-buffer independence |
| absence | construction and no-option probe report `Absent` |
| reset | accepted bytes survive reset while represented mutable machine state resets |
| repeated bootstrap | accepted bytes survive; stale generated SP IMEM provenance is cleared; offset zero remains Unknown |
| no partial mutation | malformed and unsupported replacement preserve a complete Machine snapshot and prior firmware |
| CLI parsing | optional flag, either option order, missing value, invalid budget, usage |
| explicit read failure | one generated nonexistent path fails without fallback |
| no default | generated `pifdata.bin` in current directory is ignored without the flag |
| accepted CLI | generated file is accepted; output reports size/classification but no path or sentinel bytes |
| regressions | existing SP IMEM, `Lw`, bootstrap, `Machine::step`, boot-probe, and complete Rust gate |

`WORKER_CLAIM` Synthetic fixtures prove only boundary and lifecycle behavior,
not authentic firmware compatibility or boot.
