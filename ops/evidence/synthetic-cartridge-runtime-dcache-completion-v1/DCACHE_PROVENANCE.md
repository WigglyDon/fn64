# D-cache provenance

Each valid line retains:

- physical tag;
- sixteen cache bytes;
- clean or dirty state;
- fill provenance;
- latest CPU-store provenance when dirty.

Each applied RDRAM writeback records:

- evicting instruction PC;
- victim physical line address;
- direct-mapped line index;
- all sixteen written bytes;
- latest dirty-store cause.

The runtime finishes with shared line index `0` holding physical tag
`0x00102000`, B bytes beginning `0x55667788`, and `ValidClean` state. No dirty
line remains, and no unexpected line was mutated.
