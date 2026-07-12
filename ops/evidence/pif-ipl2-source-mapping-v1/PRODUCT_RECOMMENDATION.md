# Product recommendation

## Classification

`VARIANT_SPECIFIC_MAPPING_REQUIRES_EXPLICIT_MACHINE_PROFILE`

Three exact pinned mappings are established, but NTSC differs from PAL and
MPAL by one copied word. Current structural validation accepts a common
1,984-byte shape and proves no region or revision. The mapping therefore cannot
be selected honestly without explicit Machine configuration.

## Smallest next product lane

Recommended later lane:
`pif-ipl2-profiled-copy-materialization-v1`.

Exact truth owned by that lane: when an explicit supported PIF IPL profile and
structurally accepted user-supplied raw PIF bytes are present, `Machine`
materializes the complete profile-selected IPL1 copy effect into SP IMEM,
preserves source provenance across reset, and otherwise fails closed.

Allowed ownership:

- host: an explicit local path and explicit profile-selection syntax only;
- `Machine`: profile validity, selected source range, destination mutation,
  provenance, reset behavior, and unavailable-state failure;
- proof/tests: synthetic-pattern mapping and no-private-input validation only.

Required inputs:

- already accepted user-supplied raw PIF bytes;
- an explicit supported profile: pinned NTSC, PAL, or MPAL mapping;
- no hidden default, content inference, hash lookup, filename inference, or
  compatibility database.

Validation boundary:

- exact source/destination arithmetic for each profile;
- full-range copy, including both `[0x000, 0x020)` and `[0x000, 0x02C)`;
- byte-for-byte provenance using generated synthetic patterns only;
- reset persistence and replacement behavior;
- fail-closed behavior when firmware or profile is missing or unsupported;
- unchanged `Machine::step` execution entrance and `rust/verify-forward`.

Strict non-goals:

- no firmware execution or claim that IPL1/IPL2 ran;
- no PIF RAM, SI, PI, checksum, or security-handshake emulation;
- no synthesis of `t3`, `ra`, or saved registers without separate evidence;
- no automatic firmware acquisition or identification;
- no commercial ROM testing;
- no BOOT-3, host-runtime, or game-compatibility claim.

The copy-only lane may proceed from this mapping evidence if it stays within
those limits. Further evidence is required before any lane claims a complete
IPL2 handoff, materializes dynamic register/device end state, or advances
beyond the next unknown fact. Minimal firmware execution is not earned: the
sources show dynamic effects, but do not prove execution is the only honest way
to represent their end state.

This recommendation does not earn BOOT-3 or compatibility because it supplies
only one pre-IPL3 Machine state effect and deliberately leaves the rest
unavailable.
