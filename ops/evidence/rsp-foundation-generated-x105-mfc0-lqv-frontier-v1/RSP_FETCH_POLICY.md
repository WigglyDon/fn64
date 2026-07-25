# RSP Fetch Policy

When RSP is selected, the Machine:

1. requires halt false;
2. rejects represented single-step before fetch;
3. captures singular `Sp::pc`;
4. requires a word-aligned local PC in 4 KiB IMEM;
5. reads one big-endian known word through `SpImem`;
6. retains all four byte-provenance records;
7. rejects unknown, opaque, inconsistent, or unavailable word truth before
   decode;
8. decodes only exact bounded identities.

No IMEM copy, RSP I-cache, backing-zero inference, CPU fallback, or raw private
word diagnostic exists.
