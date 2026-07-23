# Final RDRAM state

Guest-observable final backing truth:

- physical `0x00100000`: `0x11AA3344`;
- physical `0x00102000`: `0x55667788`;
- physical `0x003FF000..0x003FF01F`: exact success mailbox.

Both test words reach backing only through genuine D-cache dirty replacement.
The last B access leaves index `0`, tag `0x00102000`, B bytes, and valid-clean
state. No partial writeback exists and no unrelated dirty line remains.

The PI-completed payload, boot globals, SP teardown truth, and final cleared MI
interrupt/mask state from the accepted handoff remain intact.
