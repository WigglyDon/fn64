# SP-IMEM write provenance

Each committed `Sw` byte is known and carries one CPU-store source containing
the instruction PC, source GPR index, and the captured Machine bootstrap GPR
source classification. This distinguishes CPU execution from user-PIF copy
materialization and test-only generated staging without introducing a taint
graph.

The selected four bytes replace their previous provenance. Neighboring bytes
retain both value and provenance. Reset returns SP IMEM to concrete zero with
Unknown provenance. Bootstrap restaging reconstructs the selected PIF copy and
therefore replaces overwritten bytes in that range with user-PIF provenance.
