# Mailbox format

Physical base: `0x003FF000`. Guest CPU alias: `0xA03FF000` (KSEG1).

| Offset | Success word | Meaning |
| --- | --- | --- |
| `0x00` | `0x464E3634` | `FN64` magic |
| `0x04` | `0x00000002` | fixture version |
| `0x08` | `0x600D0001` | success |
| `0x0C` | `0x00000000` | no failing stage |
| `0x10` | `0x11AA3344` | final A |
| `0x14` | `0x55667788` | final B |
| `0x18` | `0x00000042` | entry sentinel |
| `0x1C` | `0x0000007F` | all seven checks passed |

These are ordinary RDRAM bytes written by eight guest KSEG1 `Sw` instructions.
There is no Machine mailbox device or host completion state.
