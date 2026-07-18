# Known overwrite policy

An aligned known CPU `Sw` to an opaque word writes the exact big-endian bytes,
removes the opaque state, installs the existing concrete CPU-store provenance,
and restores the existing known-word read surface. A known write to another
word leaves the opaque word unchanged. Partial stores are not represented.
