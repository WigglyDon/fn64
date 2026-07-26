# Low-DMEM Backing Truth Boundary

At the public x105 LQV boundary, backing offsets `0x000..0x00F` are
numerically zero. No primary source, represented CPU store, RSP store, SP DMA,
or cartridge-bootstrap copy establishes those values. Their knowledge is
therefore `Unavailable(BootstrapUncovered)`.

The zero backing is not Machine value truth and is never copied into `v12`.
It remains internal storage needed for deterministic ownership and rollback.
Truth-bearing reads must consult `SpDmem` knowledge.

By contrast, `0x040..0x1000` is available cartridge-bootstrap truth at this
boundary. In particular, `0x040..0x044` is `03 A0 48 20`.
