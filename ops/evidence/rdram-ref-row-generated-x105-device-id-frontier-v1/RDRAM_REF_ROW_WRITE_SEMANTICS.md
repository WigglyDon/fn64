# RDRAM REF_ROW write semantics

One exact aligned target accepts a known source whose old low 32 bits are zero.
High GPR bits are ignored by `Sw`. Planning captures old operands and lineage;
application replaces one typed state, commits control flow once, and advances
Count once. It writes no RDRAM bytes or GPR. No prior RDRAM_DELAY, RI, or MI
state is hidden authorization once the bounded MI transfer is absent.
