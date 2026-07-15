# MI initialization transfer ownership

The existing private per-Machine `Mi` owner holds one optional transfer with:

- source initialization length 15;
- repeated-byte count 16;
- accepted command word `0x0000010F` as provenance;
- source MI instruction PC, GPR, and GPR lineage.

The exact MI store creates register state and transfer atomically. The exact
RDRAM_DELAY store consumes the transfer once. Construction, reset, and complete
bootstrap leave it unavailable; failed bootstrap preserves it.

