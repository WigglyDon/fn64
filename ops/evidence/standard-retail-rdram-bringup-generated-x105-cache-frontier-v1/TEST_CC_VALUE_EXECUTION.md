# TestCCValue Execution

Each generated `TestCCValue` call writes the selected manual mode, performs ten
ordinary iterations, writes `0xFFFFFFFF` twice at offset zero and once at
offset four, loads offset four, and counts eight response bits. Stores remain
ordinary byte truth; only the qualifying read result is shaped.

There are 320 calls from the accepted starting state through loop1 completion:
64 for each present module and 256 for the absent-module probe. The accepted
starting state is already inside the first call, so the post-start trace sees
319 additional TestCCValue JALs/entries. Every guest loop, branch, delay slot,
load, and store executes through public `Machine::step`.
