# Cold RI_SELECT read and generated x105 RI_CONFIG frontier

The generated cold-x105 composition reaches an aligned `Lw` at
`0xA470000C`, physical `0x0470000C`. Public register definitions identify that
single word as the R/W RI_SELECT register. The bounded source path reads zero,
takes the cold fall-through, saves the five retained handoff registers at the
high end of SP IMEM, and next stops before mutation on `Sw` to RI_CONFIG at
physical `0x04700004`.

The zero is represented as Machine-owned state created by the complete coupled
cold-x105 bootstrap. Construction and general reset leave RI_SELECT
unavailable because the inspected sources do not state a generic hardware
power-on register value. No RI write, NMI behavior, other RI register, MMIO
framework, bus, or generalized map is represented.

All proof bytes and instruction fields are generated. This is synthetic
composition proof; the authentic checkpoint remains BOOT-2.
