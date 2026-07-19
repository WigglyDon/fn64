# PC, next_pc, and Count cadence

Taken BEQL:

- branch commit: `pc=P+4`, `next_pc=target`, Count `+1`;
- successful slot: `pc=target`, `next_pc=target+4`, Count `+1`.

Not-taken BEQL:

- branch commit: `pc=P+8`, `next_pc=P+12`, Count `+1`;
- annulled slot: no step and no Count.

Generated transition: `A400099C/A40009A0`, Count `32208`, committed `32224`
becomes `A40009A4/A40009A8`, Count `32209`, committed `32225`.
