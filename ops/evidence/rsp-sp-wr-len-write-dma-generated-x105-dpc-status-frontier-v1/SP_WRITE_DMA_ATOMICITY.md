# SP write-DMA atomicity

Planning captures the complete source byte set, preflights all destination
blocks, verifies record capacity, and determines both final addresses before
application.

Any planning rejection preserves the entire `Machine`: no byte copy, partial
block, partial record, address evolution, RSP-PC advance, committed-count
increment, or processor-turn rotation occurs. Successful application performs
the whole transfer at the single Mtc0 instruction boundary.

