# Analysis window

Start: cold x105 IPL3 entry at CPU address `0xA4000040`.

The x105 prelude first consumes sp, t3, retained SP IMEM, and ra. The common
cold path then overwrites Cause, Count, and Compare, reads the cold selector,
creates its stack frame, and stores inherited s3-s7 before repurposing them.

End: the store of inherited s7 in the common cold entry at pinned
`src/ipl3.s:123`. By this point each bounded inherited fact is consumed before
first overwrite, overwritten before first read, proved unused, or explicitly
unknown. Later RDRAM discovery, MMIO, checksum, and game handoff are outside
this product.

Address-qualified x105 prelude:

- `0xA4000040`: first read of sp; writes t1.
- `0xA4000044`: first retained-SP-IMEM load; writes t0.
- `0xA4000048`: first read of t3; writes t2 from SP DMEM.
- `0xA400004C`: transforms t2.
- `0xA4000050`: first retained-SP-IMEM store.
- `0xA4000054`: read/write t3.
- `0xA4000058`: masks t0.
- `0xA400005C`: loop branch.
- `0xA4000060`: branch delay instruction.
- `0xA4000064..0xA4000070`: two tail loads and stores.
- `0xA4000074`: first read of ra by a signed branch relation.
- `0xA4000078`: branch-delay tail store.

The stale pinned inline comment for t3 plus `0x44` says `0xA4000088`; the
source relation and t3 value yield `0xA4000084`. The relation controls.
