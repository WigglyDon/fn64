# Fixed Standard-Retail Profile

Profile name: `fixed-standard-retail-4mib-two-module-digital-cc-v1`.

Immutable facts:

- capacity `0x00400000`;
- two modules;
- module size `0x00200000`;
- device type `0xB0190000`;
- manufacturer `0x00000500` (fixed NEC choice);
- enhanced-speed flag false;
- RCP 2.0 register spacing `0x400`;
- deterministic digital current-control response described in
  `CURRENT_CONTROL_DIGITAL_RESPONSE_MODEL.md`.

Selection depends only on Machine-owned byte capacity. It does not consult a
cartridge, title, filename, region, digest, environment variable, or host
callback.

