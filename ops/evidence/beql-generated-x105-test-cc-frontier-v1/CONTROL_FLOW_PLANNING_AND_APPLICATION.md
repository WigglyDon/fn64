# Control-flow planning and application

The ordinary-control-flow plan captures instruction and slot PCs, fields,
operand indices, old values, operand lineages, signed immediate, target,
condition, selected path, and active-slot eligibility before mutation.

Application chooses exactly one existing path:

- taken: `commit_ordinary_control_flow`, existing delay owner, one Count;
- not taken: `commit_beql_annul`, no delay owner, one Count.

There is no recursive `Machine::step`, hidden multi-instruction execution,
pipeline abstraction, PC whitelist, x105 special case, or public mutable API.
