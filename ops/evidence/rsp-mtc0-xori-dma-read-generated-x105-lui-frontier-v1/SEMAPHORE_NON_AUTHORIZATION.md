# Semaphore Non-Authorization

The public SP semaphore remains set with its earlier provenance throughout the
four new RSP commits and the atomic DMA. Neither Mtc0 planning nor shared DMA
preflight consults it.

The semaphore is therefore preserved SP truth, not hidden task
authorization, DMA authorization, or a transfer-completion signal.
