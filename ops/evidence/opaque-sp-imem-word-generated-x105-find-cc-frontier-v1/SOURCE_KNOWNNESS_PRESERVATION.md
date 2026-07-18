# Source-knownness preservation

Arithmetic, logical instructions, branches, jump-register targets, load/store
addresses, device command words, and every other genuine value consumer retain
their existing knownness requirements. No backing zero is relabelled as known.
