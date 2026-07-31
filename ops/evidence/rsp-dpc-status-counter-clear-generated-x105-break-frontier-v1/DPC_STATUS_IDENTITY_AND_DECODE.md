# DPC status identity and decode

Public instruction:

- local PC: `0x098`;
- word: `0x40835800`;
- identity: RSP `Mtc0`;
- scalar source: r3;
- control index: 11;
- destination identity: `DpcStatus`.

The source scalar must be Available. Existing Mtc0 destinations retain their
prior behavior. Other control indices remain unsupported.
