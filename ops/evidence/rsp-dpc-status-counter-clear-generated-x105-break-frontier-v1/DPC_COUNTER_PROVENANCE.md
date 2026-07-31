# DPC counter provenance

Each cleared counter retains immutable `RspMtc0StatusClear` causality:

- local instruction PC `0x098`;
- exact four-byte SP IMEM fetch provenance;
- source register r3;
- old source value `0x00000240`;
- old r3 Xori lineage;
- control index 11;
- raw command `0x00000240`;
- that counter's clear bit;
- that counter's identity.

The counter state owns the value. Provenance does not duplicate a mutable
counter or status word.
