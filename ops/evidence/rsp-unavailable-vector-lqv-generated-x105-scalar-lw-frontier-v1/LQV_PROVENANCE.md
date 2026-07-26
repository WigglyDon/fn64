# LQV Provenance

`MachineRspLqvSource` records:

- local instruction PC and four-byte `SpImem` fetch provenance;
- scalar base index, value, and scalar source;
- byte element and signed encoded offset;
- resolved local DMEM start;
- sixteen `MachineSpDmemByteKnowledgeDescriptor` entries.

Each descriptor records its local address and available/unavailable source
classification. An unavailable descriptor contains no backing value. An
available vector additionally stores the final sixteen bytes; an unavailable
vector never does.
