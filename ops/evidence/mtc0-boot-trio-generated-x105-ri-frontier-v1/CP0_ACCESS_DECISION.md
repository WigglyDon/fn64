# CP0 access decision

Decision: `MTC0_ACCESS_COLD_X105_ONLY`.

The accepted coupled handoff owns one exact `NTSC_PINNED`, x105, cold,
cartridge state with Status `0x34000000`; its KSU field is kernel. The VR4300
manual states that CP0 is usable in kernel mode independent of CU0. fn64 does
not otherwise own complete privilege, CU0, or coprocessor-unusable behavior,
so the bounded MTC0 plan requires the existing coupled-cold-x105 bootstrap
state and its source-clear IPL1 Status lineage. Every other context rejects
before mutation.

This is not a general kernel/privilege implementation. User, supervisor,
unqualified bootstrap, other profile, and non-bootstrap CP0 access remain
unsupported.
