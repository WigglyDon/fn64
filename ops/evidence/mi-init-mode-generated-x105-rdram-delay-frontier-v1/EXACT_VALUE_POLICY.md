# Exact-value policy

Accepted low word: `0x0000010F`.

Representative rejected words include `0x0000000F`, `0x0000008F`,
`0x0000010E`, `0x0000030F`, `0x0000090F`, `0x0000210F`, and
`0x8000010F`. Unknown source lineage also rejects.

All rejection occurs before mutation. The boundary means only that fn64 has
not earned those commands; it makes no hardware rejection or trap claim.
