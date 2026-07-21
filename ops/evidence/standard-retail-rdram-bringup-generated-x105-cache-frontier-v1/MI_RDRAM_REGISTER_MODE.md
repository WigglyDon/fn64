# MI RDRAM-Register Mode

The existing per-Machine `Mi` owner now records one mutable enabled flag, the
last exact command word, and CPU-store provenance.

- `0x00002000` sets register mode.
- `0x00001000` clears register mode.
- generated zero preserves the current enabled state and has no invented
  command effect.

Module-register loads require enabled mode. Post-start command counts are two
set/clear pairs for DEVICE_TYPE, two for manufacturer/type, 69 zero writes from
automatic `WriteCC`, and 128 set/clear pairs from `ReadCC`. Unrelated MI
behavior remains unchanged.

