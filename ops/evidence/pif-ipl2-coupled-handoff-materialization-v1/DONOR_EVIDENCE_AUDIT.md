# Donor evidence audit

The preserved donor is commit `c24ab78c9a4b93fe79b660f3428d06a6a570c4dd`
under the historical branch whose current tip is
`96840e996208d35baabbfd6ffe921f01272699c9`. It was inspected read-only and was
not merged, cherry-picked, edited, or promoted.

Accepted only as leads, then independently revalidated:

- t3 is the retained SP-DMEM entry pointer;
- sp is the restored IPL1 stack value after balanced IPL2 frames;
- s3-s7 come from separate boot-word, profile, reset, medium, and x105 facts;
- Status is established by IPL1;
- the final transfer leaves the IPL3 entry outside a delay slot.

Rejected donor claim:

- one universal `ra = 0xFFFFFFFFA4001550` for NTSC, PAL, and MPAL. The donor
  directly corroborated only the common NTSC-like value and inferred the other
  profiles from copy length. The pinned profile source inserts one extra
  instruction before the link for PAL and MPAL, so equality is not valid.

The donor artifact is stale and unaccepted. No donor file became canonical by
copying; this directory records a fresh source-qualified reconstruction.
