# JAL link-destination policy

JAL may overwrite any prior represented r31 value or lineage. Architectural
zero, instruction-result, retained PIF IPL2, and representable unknown
destination states do not change its link or target.

The generated delay slot observes the newly written link. The target does not
execute during the JAL step.
