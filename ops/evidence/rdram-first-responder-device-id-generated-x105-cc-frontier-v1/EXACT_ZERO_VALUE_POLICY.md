# Exact zero value policy

Accepted low word: `0x00000000`.

Known 64-bit values with any high half and a zero low half succeed. Required
nonzero rejection coverage includes `0x00000001`, `0x00000400`, `0x02000000`,
`0x40000000`, `0x80000000`, and `0xFFFFFFFF`. Unknown source lineage rejects
before mutation.
