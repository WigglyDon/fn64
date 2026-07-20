# RDRAM Mode Write Semantics

Planning requires alignment, exact target, known source lineage, exact low
word, and no pending bounded MI transfer. Application replaces the one request
fact, preserves RDRAM bytes/routing and earlier facts, and uses the existing
store cadence. Source high bits are ignored exactly as for existing `Sw`.
