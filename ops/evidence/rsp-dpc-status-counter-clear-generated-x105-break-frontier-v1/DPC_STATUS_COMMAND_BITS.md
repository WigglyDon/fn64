# DPC status command bits

The sole accepted raw command is `0x00000240`.

| Bit | Meaning | Public command |
| --- | --- | --- |
| `0x0040` | clear TMEM-load counter | set |
| `0x0080` | clear pipe-busy counter | clear |
| `0x0100` | clear command-busy counter | clear |
| `0x0200` | clear clock counter | set |

All XBUS, FREEZE, FLUSH, and reserved bits are clear. Every other raw command
word rejects. The command is not stored as DPC status readback.
