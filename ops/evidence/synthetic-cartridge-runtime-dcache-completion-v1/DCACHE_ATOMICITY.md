# D-cache atomicity

Planning captures old operands, target line, fill bytes, victim state, and
writeback range before mutation.

Rejected operations produce:

- no partial victim writeback;
- no partial fill;
- no partial store;
- no cache-line replacement;
- no RDRAM mutation;
- no PC, next-PC, Count, device, or provenance mutation.

Focused proofs cover unknown store sources, unavailable bases, unavailable
dirty truth, unaligned `Sw` AdES precedence, KSEG1 bypass, and exact
replacement plans. Existing complete-Machine rollback tests remain green.
