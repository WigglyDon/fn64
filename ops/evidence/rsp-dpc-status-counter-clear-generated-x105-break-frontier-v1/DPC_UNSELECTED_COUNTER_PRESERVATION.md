# Unselected counter preservation

The public command does not select pipe-busy or command-busy. Their complete
Unavailable states and `ConstructionOrResetUndefined` provenance survive:

- DPC_STATUS planning;
- DPC_STATUS application;
- the following CPU instruction;
- rejected Break.

Synthetic nonzero Available states additionally prove that unselected values
and provenance are preserved, rather than reconstructed.
