# RSP Scalar Provenance

Sp::rsp remains the sole scalar-register owner.

An Available `Lui` result records:

- local instruction PC;
- exact SpImem instruction-byte provenance;
- immediate.

An Available `Addi` result records:

- local instruction PC;
- exact SpImem instruction-byte provenance;
- source register index;
- old source value and old source provenance;
- signed immediate.

The result word exists once, in the scalar register state. No parallel register
file, symbolic scalar value, unavailable arithmetic result, or mutable
duplicate result payload was added.
