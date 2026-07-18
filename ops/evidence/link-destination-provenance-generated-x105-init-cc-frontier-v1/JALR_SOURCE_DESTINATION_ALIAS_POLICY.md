# JALR source/destination alias policy

JALR consumes old `rs`; it does not consume old `rd` merely because `rd`
is the link destination. Current fn64 ordering captures old `rs` during
planning, before applying the link. That preserves the repository's existing
`rs == rd` behavior. `rd == 0` discards the link through architectural-zero
handling.

Unknown old `rs` still rejects atomically. JR source requirements are
unchanged.

