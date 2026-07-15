# RDRAM delay ownership

The existing `Rdram` owner remains the sole owner of RDRAM bytes and gains one
optional typed broadcast-delay fact. No byte store is duplicated or moved.
The fact is per Machine and contains only the four logical fields, packed
logical configuration, exact store provenance, consumed MI lineage, and global
aperture classification. Module count and per-module state remain unavailable.
