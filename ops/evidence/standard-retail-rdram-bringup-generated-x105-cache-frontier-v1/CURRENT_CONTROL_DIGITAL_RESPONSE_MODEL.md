# Current-Control Digital Response Model

For a present module in a manual RDRAM_MODE request with nominal input `n`, a
direct calibration read preserves all backing bits except bits 23:16 and
returns `min(n + 1, 8)` low-order one bits in that byte. Thus the ten-read,
eight-bit guest loop observes scores:

`10, 20, 30, 40, 50, 60, 70, 80` for nominal inputs `0..7`.

The response is deterministic, monotonic, derived from the active mapped
module and its current mode, and does not mutate backing bytes. An absent
module returns zero during its active probe. Outside active manual calibration,
direct reads return ordinary backing truth.

Automatic register-mode readback exposes the candidate nominal code required
by the pinned `ReadCC`/`ConvertManualToAuto` algorithm. This is a deliberate
digital profile. It makes no claim about current, voltage, timing, process
variation, or analog transfer accuracy.
