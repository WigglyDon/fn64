# Final interrupt-control sequence

Generated execution commits this exact sequence:

| PC | Word | Effect |
|---:|---:|---|
| `0x800001A0` | `0xAC2B0010` | SP_STATUS `0x00AAAAAE`; halt and clear SP pending/signals |
| `0x800001AC` | `0xAC28000C` | MI_INTR_MASK `0x00000555`; clear all six masks |
| `0x800001B4` | `0xAC200018` | SI_STATUS any-write clear; SI pending false |
| `0x800001BC` | `0xAC20000C` | AI_STATUS clear; AI pending false |
| `0x800001C8` | `0xAC290000` | MI_INIT_MODE `0x00000800`; DP pending false |
| `0x800001D4` | `0xAC290010` | PI_STATUS `0x00000002`; PI pending false |

MI remains the sole interrupt-truth owner. Each clear retains exact CPU-store
provenance. The final pending and mask states are false for SP, SI, AI, VI,
PI, and DP. No CPU interrupt delivery, SI/AI DMA, VI behavior, DP execution,
or PI reset is claimed.
