# Cause IP7 contradiction audit

The VR4300 Cause-register definition says every Cause bit except IP1 and IP0
is read-only. Its timer section separately says a timer request may be cleared
by clearing Cause.IP7 or by changing Compare. The latter text does not identify
MTC0 Cause as a software-writable IP7 mechanism, and no stronger inspected
VR4300 source resolves that wording into a broader MTC0 mask.

Current fn64 owns timer pending as a distinct COP0 boolean. The bounded product
therefore follows the formal Cause write definition: MTC0 Cause transfers only
source bits 9:8 into software-pending IP1:IP0 and preserves timer pending. MTC0
Compare remains the sole newly represented timer-clear action. No raw Cause
register or interrupt delivery is introduced.

Limitation: this pass does not claim a complete physical implementation of
every possible VR4300 IP7-clearing mechanism.
