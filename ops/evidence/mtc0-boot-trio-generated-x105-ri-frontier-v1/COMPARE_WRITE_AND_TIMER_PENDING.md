# Compare write and timer pending

MTC0 Compare first installs the source low word and clears timer pending. The
normal committed-step cadence then increments Count once; the existing
post-increment equality owner may relatch timer pending.

The discriminator starts with timer pending set and Count one below the new
Compare after the preceding Count cadence. It proves clear-before-increment and
relatch-after-increment. Compare does not change Cause software pending,
exception state, GPRs, or memory.
