# IPL1 copy causality

## Result

`EXTERNAL_TECHNICAL_EVIDENCE`: in the three pinned regional reconstructions,
the CPU executes IPL1 from the PIF Boot ROM mapping, waits until RSP DMA is not
busy, then performs an aligned word-for-word copy into SP IMEM and transfers CPU
control to the SP IMEM start. The copied bytes become Machine-owned SP IMEM
state; the host owns no copy policy.

The direct observation in `SRC-DEBUG-PIF12` independently gives a source start
of CPU address `0xBFC000D4`, source end exclusive `0xBFC0071C`, and destination
start `0xA4001000` for its examined NTSC-like setup. Subtracting the Boot ROM
CPU base `0xBFC00000` yields raw offsets `0x0D4` and `0x71C`.

## Exact mappings

| Pinned variant | Raw source | Source arithmetic | SP IMEM local destination | Destination arithmetic |
| --- | --- | --- | --- | --- |
| NTSC | `[0x0D4, 0x71C)` | `0x71C - 0x0D4 = 0x648 = 1,608` bytes | `[0x000, 0x648)` | `0x648 - 0x000 = 0x648 = 1,608` bytes |
| PAL | `[0x0D4, 0x720)` | `0x720 - 0x0D4 = 0x64C = 1,612` bytes | `[0x000, 0x64C)` | `0x64C - 0x000 = 0x64C = 1,612` bytes |
| MPAL | `[0x0D4, 0x720)` | `0x720 - 0x0D4 = 0x64C = 1,612` bytes | `[0x000, 0x64C)` | `0x64C - 0x000 = 0x64C = 1,612` bytes |

The local destination ranges correspond to physical SP IMEM beginning at
`0x04001000` and its KSEG1 CPU alias beginning at `0xA4001000`.

Bounds audit:

- NTSC source end `0x71C` and PAL/MPAL source end `0x720` are each at or below
  the accepted raw-shape end `0x7C0`.
- Destination ends `0x648` and `0x64C` are each at or below the represented SP
  IMEM capacity `0x1000`.
- NTSC leaves `0xA4` bytes after its IPL2 end; PAL and MPAL leave `0xA0` bytes.
  These suffix sizes and the common `0x7C0` total are direct pinned-source
  structure, not measurements of private input.

## Event and stage

Cause: CPU execution of the IPL1 copy loop.

Source: aligned words beginning at the pinned `ipl2_rom` label and ending
before `ipl2_rom_end` in `SRC-RE-DECOMPALS-928F`.

Owner: the emulated CPU performs the historical event; its resulting SP IMEM
bytes are Machine-owned state in fn64.

Destination: SP IMEM local offset zero, using the physical/KSEG1 aliases above.

Stage: IPL1, after RSP-DMA-idle observation and before the CPU enters IPL2.

Effect kind: hardware evidence describes dynamic IPL1 execution. A future fn64
implementation may represent its end effect as source-backed static
materialization, but that would not be firmware execution.

Transformation: the pinned loop uses an aligned 32-bit load followed by an
aligned 32-bit store and advances both addresses by four. `INFERENCE`, supported
by those paired operations and equal ranges: there is no decompression,
relocation, checksum transform, or byte permutation in the copy effect; the
loaded word bit pattern is preserved at the destination.

Control transfer: after setting the stack near the end of SP IMEM, IPL1 jumps
to KSEG1 address `0xA4001000`, the IPL2 start.

## Lifecycle through IPL3

`EXTERNAL_TECHNICAL_EVIDENCE`: the pinned IPL2 performs stack traffic near the
top of SP IMEM, reads PIF RAM and cartridge inputs, programs devices, stages
IPL3 in SP DMEM, verifies it, and transfers to `0xA4000040`. Its source contains
no write into the copied low SP IMEM range.

`INFERENCE`, supported by that pinned source review and the distinct stack
range: for these three pinned builds, the copied low range survives unchanged
from IPL1 completion until IPL3 begins. The x105 entry then reads and mutates
that retained range. This is not a universal lifecycle claim for an
unexamined physical PIF revision.

## Coverage

- Current consumed range: destination `[0x000, 0x020)` maps to raw source
  `[0x0D4, 0x0F4)`. Arithmetic: `0x020 - 0x000 = 0x20 = 32` bytes and
  `0x0F4 - 0x0D4 = 0x20 = 32` bytes. It is fully inside all three pinned copies.
- Current mutation input/output range: destination `[0x000, 0x02C)` maps to raw
  source `[0x0D4, 0x100)`. Arithmetic: `0x02C - 0x000 = 0x2C = 44` bytes and
  `0x100 - 0x0D4 = 0x2C = 44` bytes. It is fully inside all three pinned copies.

## Disagreements and limits

`SRC-DEBUG-PIF12` prints endpoints whose difference is `0x648` bytes but also
describes a smaller instruction/byte count. Those statements cannot both be
true. This lane retains the directly observed endpoints, rejects the prose
count, and relies on the pinned matching reconstruction for the regional
structure. The page is unversioned, so it is corroboration rather than the
sole numeric authority.

`UNKNOWN`: whether every physical PIF revision uses one of these three mappings.
No content, filename, hash, or shape rule in current fn64 distinguishes the
pinned variants.
