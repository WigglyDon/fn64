# Break PC commit decision

Selected bounded policy:

`BREAK_COMMITS_ORDINARY_SUCCESSOR_STATE_BEFORE_HALT`

The primary Break description specifies halt, broke, and conditional interrupt
effects but is silent about a post-Break PC register value. fn64 therefore
applies its existing successful RSP instruction-boundary law: current PC takes
the prior `rsp.next_pc`, and `rsp.next_pc` advances to the following aligned
word. The same atomic commit then leaves the RSP halted.

Public result:

- pre: `0x09c/0x0a0`;
- post: `0x0a0/0x0a4`;
- the word at `0x0a0` is not executed.

This is a functional state decision, not a cycle or pipeline claim.

