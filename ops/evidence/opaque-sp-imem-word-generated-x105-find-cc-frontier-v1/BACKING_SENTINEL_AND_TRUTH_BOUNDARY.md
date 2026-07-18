# Backing sentinel and truth boundary

An opaque commit sets exactly four private backing bytes to zero. This is a
deterministic storage sentinel used to erase ghost differences from prior
backing bytes.

The sentinel is not transferred CPU data, byte truth, word truth, inspection
truth, instruction data, or an input to arithmetic, branching, addressing, or
device behavior. Every truth-bearing read consults knowledge before bytes.
