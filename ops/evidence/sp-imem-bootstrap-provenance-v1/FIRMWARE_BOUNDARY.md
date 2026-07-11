# Firmware boundary

- `USER_DECISION` fn64 must not ship, commit, package, reconstruct, download,
  or disguise PIF ROM, IPL1, or IPL2 executable content.
- `INFERENCE` The authentic `Lw` result is an IPL2 instruction word retained in
  SP IMEM. The next x105 operations use full word values, not merely a general
  boolean or range property.
- `INFERENCE` No source-clear semantic constant can replace those bytes: the
  prelude combines each firmware word with cartridge IPL3 data and writes the
  exact result back.
- `WORKER_CLAIM` A one-word constant, eight-word prefix, generated table,
  encoded string, checksum-derived reconstruction, or imported emulator state
  would all reproduce firmware content under another name.
- `WORKER_CLAIM` Bootcode-family classification may identify evidence and a
  future explicit firmware compatibility check; it cannot select embedded
  machine bytes.
- `UNKNOWN` Whether every physical PIF revision shares the same complete IPL2
  span and prefix is not established by the available lawful evidence.

Audit result:

- Verbatim external code entered Git: no
- Firmware bytes entered Git: no
- Private ROM entered Git: no
- Private or proprietary content entered artifact: no
- Copied assembly entered Git or artifact: no
- Firmware-derived lookup table entered Git or artifact: no
