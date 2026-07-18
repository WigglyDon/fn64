# Opaque word provenance

Each opaque state records only:

- aligned local SP-IMEM offset;
- committing instruction PC;
- source GPR;
- exact `MachineBootstrapGprSource` lineage;
- effective address;
- CPU address;
- physical address.

No word, byte, partial value, checksum relation, seed, or host provenance is
stored or exposed.
