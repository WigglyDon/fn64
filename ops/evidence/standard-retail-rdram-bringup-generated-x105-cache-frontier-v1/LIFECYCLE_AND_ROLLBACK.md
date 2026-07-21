# Lifecycle and Rollback

Construction and reset retain the immutable capacity-selected profile and
recreate untested module records with no mutable requests, mappings,
calibration, modes, RAS words, MI register mode, or RI refresh.

Successful repeated cold-x105 bootstrap performs the same replacement.
Failed bootstrap preserves the complete prior Machine, including completed
module configuration and provenance. Clone/equality includes profile, module,
MI, RI, SP, CPU, and byte truth; independently mutated Machines do not share
state.

