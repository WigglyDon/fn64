# Product invariant audit

The product diff was searched for title, filename, path, digest, known-PC, and
known-function policy.

Result:

- no product identifier contains the selected title or basename;
- no core branch consumes a filename, title, ID, region, checksum, CRC, digest,
  or host path;
- no PC or instruction-sequence whitelist was added;
- inspection never writes guest GPRs, PC, memory, cache, interrupt, DMA, or SP
  state;
- all execution uses public `Machine::step`;
- all device state is private per Machine;
- all multi-owner transfers preflight before mutation;
- no host scheduler, platform clock, window, audio, renderer, or RSP execution
  exists.

The selected basename appears only in redacted local evidence and probe output,
where the packet explicitly permits it.
