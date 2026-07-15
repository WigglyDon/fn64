# Global broadcast boundary

The physical address `0x03F80008` is the source-defined global configuration
aperture and is represented only as a broadcast configuration fact. fn64 does
not infer installed-module count, create per-module copies, repeat a physical
bus transaction, or claim electrical completion. The non-global delay address
and neighboring global registers remain unsupported.

