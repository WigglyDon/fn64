# ConvertManualToAuto Execution

Eight successful conversions execute, four per present module. For candidates
zero through seven, `ReadCC` exposes nominal values and the guest multiplies by
800. Candidate six gives 4800, distance 480 from target 5280; candidate seven
gives 5600, distance 320 and crosses the target. The guest returns
`(7 + 7) / 2 = 7`.

Each candidate performs one automatic `WriteCC` and two ordinary `ReadCC`
calls. The complete trace therefore contains 64 conversion-candidate writes
and 128 register-mode reads. No return value is assigned by host or inspection
code.

