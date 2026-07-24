# Legal and private-input audit

The only private input read was the user-owned local cartridge explicitly
authorized by Don.

The pass did not:

- read or search for another ROM;
- download, infer, reconstruct, or patch cartridge data;
- read a private PIF image;
- copy the input into a worktree, cache payload, clean checkout, commit,
  evidence directory, patch, or artifact;
- calculate or record a ROM hash;
- record header strings, title, ID, region, bulk words, disassembly, or
  microcode bytes;
- execute an RSP instruction.

All committed tests use public deterministic synthetic fixtures. Artifact
sealing includes a filename/content audit for ROM-like payloads and private
absolute paths.
