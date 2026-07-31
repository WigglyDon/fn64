# Break processor turn and halt policy

Break sets halt in the same atomic RSP commit and canonicalizes the next
processor token to CPU. While halt remains true, no RSP instruction is fetched
or selected.

The successor PC state is observable, but it does not imply execution of the
successor word. A later existing CPU clear-halt command may resume RSP only
through the existing status-command and run-start rules; this pass performs no
such resume.

