# Copyright and private-input boundary

## Explicit audit

- Private PIF searched: no
- Private PIF read: no
- Private PIF hashed: no
- Private ROM read: no
- External source vendored: no
- Copied code entered Git: no
- Firmware bytes entered Git: no
- Firmware words entered Git: no
- Firmware or private content packaged: no

No local storage, emulator cache, likely firmware filename, or private input
path was searched or enumerated. No firmware path was requested. No commercial
cartridge input was read or executed.

Public technical sources were inspected only through revision-pinned web
anchors or public pages. No public source tree was checked out or downloaded
into the repository. No source code, source passage, assembly, disassembly,
binary, reconstructed firmware, hash, byte sequence, or word sequence was
retained. The repository records only arithmetic facts, paraphrased behavior,
source identifiers, and narrow anchors.

The external artifact is built from an explicit staging manifest. It excludes
external source trees, proprietary content, private input, Git objects,
worktree metadata, `rust/target`, credentials, caches, unrelated repository
files, and private chat.

`USER_DECISION`: public observability does not authorize transport of
proprietary content or grant product runtime authority. `LIVE_REPO_FACT`:
fn64's accepted input remains structural and user-supplied; this lane does not
alter that boundary.
