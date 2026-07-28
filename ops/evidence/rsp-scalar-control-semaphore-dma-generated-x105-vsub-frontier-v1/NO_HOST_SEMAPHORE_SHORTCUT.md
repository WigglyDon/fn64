# No Host Semaphore Shortcut

The public gate does not:

- mutate the semaphore from inspection or test code;
- force a scalar value;
- alter the processor token;
- jump to local PC `0x03C`;
- batch a branch with its slot;
- synthesize a host completion event.

The release is caused by represented guest CPU `Sw` at `0x800000B0`.
Subsequent RSP acquisition is the pre-existing Sp-owned read-and-set operation.
The bounded trace and complete Machine snapshots prove ordinary CPU/RSP
composition.
