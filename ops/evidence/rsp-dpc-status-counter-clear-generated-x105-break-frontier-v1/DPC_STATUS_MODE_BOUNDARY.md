# DPC status mode boundary

No DPC status readback word exists. No owner exists for XBUS, FREEZE, FLUSH,
GCLK, busy, ready, START, END, or CURRENT truth. `0x240` is decoded only as an
exact write command and is never treated as stored status.
