# Synthetic test coverage

Focused tests cover identity construction and derivation; reset/bootstrap
lifecycle; independent Machines; KSEG0/KSEG1 reads; sign extension;
base/destination aliasing; r0; ordinary and delay-slot cadence; mutable MI,
pending transfer, RI, RDRAM, memory, and host preservation; unknown and closed
targets; MI_VERSION write closure; ordinary and delay-slot AdEL; generated
comparison, taken Bne, one delay slot, RCP 2.0 setup, and atomic first-responder
rejection.

The no-window step probe exposes stable cases for fixed identity, committed
MI_VERSION read, RCP 2.0 branch, and first-responder frontier.
