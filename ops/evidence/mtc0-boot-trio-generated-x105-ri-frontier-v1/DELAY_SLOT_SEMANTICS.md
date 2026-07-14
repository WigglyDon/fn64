# Delay-slot semantics

MTC0 is not control flow. In an ordinary outer delay slot its destination
mutation occurs once, the existing outer target or fall-through commits, Count
cadence occurs once after the destination mutation, and delay context clears.

Generated tests execute Cause, Count, and Compare in taken BEQ delay slots.
Count still writes zero before finishing at one. Cause and Compare preserve the
ordinary two-commit branch-plus-slot cadence. MTC0 creates no link, target,
annul, or exception of its own in the bounded access scope.
