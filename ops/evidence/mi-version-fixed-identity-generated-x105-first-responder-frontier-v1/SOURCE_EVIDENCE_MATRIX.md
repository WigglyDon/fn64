# Source evidence matrix

| Evidence | Kind | Establishes | Does not establish |
|---|---|---|---|
| pinned RCP header | register-layout documentation | MI_VERSION address and IO/RAC/RDP/RSP byte positions | a retail readback word |
| pinned x105 source | source control flow | comparison with `0x01010101`; unequal selects RCP 2.0 path | every valid hardware identity |
| pinned revision-test source | direct-hardware test method | a volatile 32-bit read at `0xA4300004` | alternate console identities |
| pinned revision-test record | direct-hardware observation | standard-retail `0x02020102` | configurable or mutable identity |
| generated public fixture | synthetic CPU composition | exact encoded instructions and cadence | authentic firmware execution |

The direct observation authorizes the fixed current identity. Emulator
convention is not used as authority.
