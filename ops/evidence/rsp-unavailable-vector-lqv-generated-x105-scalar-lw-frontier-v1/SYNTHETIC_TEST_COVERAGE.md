# Synthetic Test Coverage

Focused product proofs cover:

- DMEM construction, bootstrap covered/uncovered knowledge, CPU stores, DMA,
  range observation, failed bootstrap, and independent Machines;
- 32 vector slots, reset unavailability, no unavailable bytes, available byte
  order, accumulator/flag boundary, replacement, lifecycle, and independence;
- LQV decode, low-12 base, signed scaled offsets, wrapping, alignment,
  element-zero boundary, concrete/unavailable/mixed sources, provenance,
  atomic application, cadence, and rejection;
- unchanged MFC0 and selected-processor no-fallback behavior;
- public generated unavailable-v12 commit, one CPU interleave, and scalar-LW
  atomic frontier.

Tests use independently constructed public synthetic words and byte patterns.
They use no private cartridge or user microcode.
