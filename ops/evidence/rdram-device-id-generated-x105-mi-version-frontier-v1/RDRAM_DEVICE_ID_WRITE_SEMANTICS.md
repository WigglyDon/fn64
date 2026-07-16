# RDRAM DEVICE_ID write semantics

The immutable planner captures old base/source values and lineage, uses existing effective/direct-address classification, then validates the old low word.

Success creates or replaces one request, commits PC/next PC once, advances Count once, writes no GPR, and preserves all other state. Generated cadence moves from PC/next PC/Count `A4000130/A4000134/32145` to `A4000134/A4000138/32146`; total commits become `32162`. No fallible work remains after mutation begins.
