# Scalar LW Byte Order

DMEM address `a + 0` supplies result bits 31:24, `a + 1` supplies 23:16,
`a + 2` supplies 15:8, and `a + 3` supplies 7:0.

The public available bytes at `0x040..0x044` are:

| Address | Value |
| --- | --- |
| `0x040` | `0x03` |
| `0x041` | `0xA0` |
| `0x042` | `0x48` |
| `0x043` | `0x20` |

Their exact big-endian result is `0x03A04820`.
