# Bootstrap lineage

For explicit material the ownership chain is:

`host path -> host read bytes -> PifFirmware explicit input -> explicit
PifIpl2Profile -> Machine bootstrap composition -> SpImem bytes/knownness/
provenance -> CPU preimage read`.

The host owns only the first two facts. `PifFirmware` owns accepted bytes and
classification. `PifIpl2Profile` owns the copy layout. Machine owns atomic
composition. `SpImem` owns bytes, knownness, opacity, and per-byte provenance.
The copied provenance names explicit user-supplied firmware, profile, and source
offset without storing the host path or cartridge identity.

Malformed, unsupported, absent, or incomplete source inputs reject before any
partial retained-IMEM materialization.
