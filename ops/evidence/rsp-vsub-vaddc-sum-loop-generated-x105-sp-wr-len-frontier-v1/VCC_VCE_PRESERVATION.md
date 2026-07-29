# VCC And VCE Preservation

VCC is independent Available-or-Unavailable 16-bit truth. VCE is independent
Available-or-Unavailable eight-bit truth. Both begin Unavailable from
`ConstructionOrReset`.

Neither exact `Vsub` nor exact `Vaddc` reads or writes VCC or VCE. Planning
captures their complete old states; successful application preserves them
exactly. Rejection preserves the complete Machine.
