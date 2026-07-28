# RSP Delay Slot Across CPU Interleave

Machine::step selects at most one processor instruction. Every public branch
commit selects CPU before its RSP delay slot is selected.

Focused and public-composition tests snapshot the RSP delay context before each
intervening CPU instruction and prove it remains byte-for-byte equal afterward.
CPU and RSP delay contexts remain independent.

A rejected slot preserves the already committed branch, current slot PC,
selected successor, active delay context, processor turn, and complete
post-branch/pre-slot Machine. It does not roll the branch back.

There is no recursive Machine::step, hidden branch-and-slot batch, second RSP
entry point, or dual CPU/RSP commit.
