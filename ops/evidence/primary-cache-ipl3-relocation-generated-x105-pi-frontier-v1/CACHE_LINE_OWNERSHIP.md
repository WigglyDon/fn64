# Cache-line ownership

MachinePrimaryCaches is held by the CPU and is the only primary-cache truth
owner. It owns both fixed-size line arrays, Index Store Tag provenance, valid
I-cache bytes, physical tags, and fill provenance.

RDRAM remains the backing-memory owner. Cache fills copy a 32-byte line into
the CPU cache; they do not transfer ownership of backing bytes. Inspection
observes read-only cache truth and cannot create a fill or line state.

Cache arrays and CP0 tag facts participate in complete Machine comparison,
rejection rollback, failed bootstrap preservation, repeated-bootstrap reset,
and independent-Machine tests. No static or global mutable cache state exists.
