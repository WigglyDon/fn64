# BLTZ semantics

`RegimmBltz rs, offset` reads the old known `rs` value and tests it with
the existing full-width signed GPR policy. A negative value selects the common
conditional target; zero and positive values select the untaken successor.

Target arithmetic reuses the existing conditional-branch helper:
`PC + 4 + (sign_extend(offset16) << 2)`, with represented wrapping arithmetic.
The untaken successor is wrapping `PC + 8`. Both decisions execute one
ordinary delay slot. BLTZ writes no GPR, creates no link, and performs no annul.
The source GPR value and provenance remain unchanged.

Only subcode zero is selected. BGEZ, link forms, likely forms, and REGIMM traps
remain recognized identities without represented execution.
