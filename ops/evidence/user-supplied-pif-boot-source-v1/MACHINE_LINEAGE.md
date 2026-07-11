# Machine lineage

## Accepted input

explicit CLI `--pif-rom` path
→ host `std::fs::read` of that one path
→ owned `Vec<u8>` transfer
→ `Machine::install_pif_firmware`
→ complete structural validation
→ private immutable `PifFirmware`
→ read-only `MachinePifFirmwareState::Accepted` observation.

## Bootstrap behavior

accepted or absent firmware state
→ `Machine::stage_cartridge_bootstrap`
→ state classification copied into the bootstrap observation
→ fresh SP IMEM zero backing with all bytes `Unknown`
→ existing `Machine::step`
→ existing BOOT-2 `Lw` rejection at unknown SP IMEM offset zero.

## Rejection

owned malformed or unsupported bytes
→ complete local validation failure
→ no optional-owner replacement
→ no cartridge, CPU, memory, provenance, cadence, Count, checkpoint, or power
mutation.

`UNKNOWN`: the future lineage from accepted firmware bytes to retained IPL2 SP
IMEM provenance.
