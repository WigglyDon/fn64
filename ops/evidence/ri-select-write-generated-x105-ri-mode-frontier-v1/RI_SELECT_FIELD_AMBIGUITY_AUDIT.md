# RI_SELECT field ambiguity audit

The pinned SGI/Nintendo RCP header describes RI_SELECT as:

> RI select (R/W): [2:0] receive select, [2:0] transmit select

That duplicated range does not state whether receive and transmit selection
share bits, occupy different undocumented ranges, or vary by physical revision.
The official online SDK-header copy repeats the same wording. The pinned x105
source constructs `0x10 | 4`, writes `0x00000014`, and comments that the write
enables TX/RX select, but value and purpose alone do not recover a general
field layout.

A targeted primary-source check also inspected Nintendo's N64 system patent.
It describes the RDRAM transmit/receive clock relationship but does not name
RI_SELECT or define these register bits. No non-ambiguous primary field source
was found, so emulator implementations and expected behavior were not promoted
to product authority.

Selected decision: `RI_SELECT_EXACT_X105_VALUE_ONLY`.

Only exact word `0x00000014` is supported. Every other value, every general
receive/transmit programming claim, unspecified physical revisions, TX/RX
electrical behavior, NMI retention, and downstream RDRAM effects remain
unsupported. fn64's unsupported-value rejection is an honest modeling boundary,
not a hardware-trap claim.
