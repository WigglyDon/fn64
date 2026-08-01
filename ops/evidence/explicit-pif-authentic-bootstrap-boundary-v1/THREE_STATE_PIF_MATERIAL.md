# Three-state PIF material model

1. Explicit user-provided material is a structurally accepted owned byte vector
   classified by `PifFirmware`. It may participate in authentic composition,
   without authenticating a revision or contents.
2. Public synthetic material is generated proof data selected only through the
   explicit public-synthetic constructor. It is never authentic evidence.
3. Unavailable material is represented by the existing absent firmware state.
   It supplies no bytes, knownness, or provenance.

There is no transition from unavailable to public synthetic in the
user-cartridge route. No unavailable byte is treated as zero.
