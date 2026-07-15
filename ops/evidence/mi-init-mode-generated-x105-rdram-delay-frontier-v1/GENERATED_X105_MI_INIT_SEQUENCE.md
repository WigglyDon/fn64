# Generated x105 MI initialization sequence

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

The proof uses a generated 1,984-byte PIF-shaped input, generated cartridge
bytes, `NTSC_PINNED`, `X105`, cold reset, cartridge medium, an explicit PIF
version bit, independently encoded fields, and public `Machine::step` only.

Exact generated words and identities:

| PC | word | identity | result |
| --- | --- | --- | --- |
| `0xA4000118` | `0xAD890000` | `Sw r9,0(r12)` | MI length 15, init mode true |
| `0xA400011C` | `0x3C091808` | `Lui r9,0x1808` | `r9=0x18080000` |
| `0xA4000120` | `0x35292838` | `Ori r9,r9,0x2838` | `r9=0x18082838` |
| `0xA4000124` | `0xAD490008` | `Sw r9,8(r10)` | closed RDRAM-delay frontier |

The accepted pre-MI state has PC `0xA4000118`, next_pc `0xA400011C`, Count
32139, and 32,155 committed steps. The MI store yields PC `0xA400011C`,
next_pc `0xA4000120`, Count 32140, and 32,156 commits. After the two CPU
instructions, the frontier has PC `0xA4000124`, next_pc `0xA4000128`, Count
32142, and 32,158 commits.

This is neither authentic PIF nor authentic cartridge execution.
