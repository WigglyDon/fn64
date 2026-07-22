# x105 checksum execution

The checksum begins from KSEG0 `0x80001000` and processes exactly
`0x00100000` bytes as 262,144 word iterations through public
`Machine::step`. Auxiliary x105 words are read uncached through KSEG1 from
physical `0x200..0x2FF` with the generated wrap rule.

The measured path performs 65,536 D-cache line fills and 196,608 D-cache
hits. The guest final words are `0xFAD40ECC` and `0x1F137F19`, exactly matching
the immutable cartridge header. The failure-loop PC `0x80000248` is never
entered.

The complete post-frontier finalization commits 7,225,461 instructions. Count
advances from 252,351 to 7,477,812 and the total cold-composition committed
step count advances from 252,367 to 7,477,828. The loop is neither compressed
nor host-simulated.

The only newly represented CPU identity is general `RegimmBgezal`. It tests a
known signed 64-bit source, links r31 only when nonnegative, uses the existing
PC+4-relative branch target and single delay-slot owner, and rejects unknown
source lineage atomically. The generated `bal checksum_OK` expansion at PC
`0x80000240` uses this identity.
