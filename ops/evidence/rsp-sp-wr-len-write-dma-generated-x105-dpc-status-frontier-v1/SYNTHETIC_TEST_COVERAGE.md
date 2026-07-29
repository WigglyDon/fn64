# Synthetic test coverage

The focused test matrix covers exact Mtc0 decode, shared field decoding,
DMEM/IMEM selection, all 24 public blocks, non-symmetric byte ordering,
complete source-knownness, record and register evolution, rejection atomicity,
ordinary CPU/RSP cadence, post-DMA Xori, and the DPC_STATUS rejection.

All test source patterns are independently generated in repository test code.
