# RDRAM REF_ROW lifecycle

Construction, general reset, and complete cold-x105 bootstrap leave REF_ROW
unavailable. An exact store creates or replaces it. Repeated complete bootstrap
clears it with the other MI/RDRAM configuration facts. Failed bootstrap
preserves it atomically. Separate Machines own independent state and lineage.
