# Common MTC0 word transfer

The represented identity is `Cop0Mtc0` with low instruction bits `10:0`
equal to zero. The plan recognizes exactly `rd=13` (Cause software pending),
`rd=9` (Count), and `rd=11` (Compare). Every other destination and malformed
encoding rejects before mutation.

The planner captures the old `rt` value and its Machine-owned GPR source,
requires that source to be known, and transfers only its low 32 bits. The GPR
and source lineage are preserved. Architectural r0 is the normal known-zero
source used by the generated boot trio.

The access boundary is the accepted `CoupledColdX105NtscPinned` bootstrap plus
`PifIpl1ColdBootStatus` lineage. This is the fail-closed
`MTC0_ACCESS_COLD_X105_ONLY` decision, not general CP0 access.
