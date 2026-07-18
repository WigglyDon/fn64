# Address and target policy

Exact target:

- physical: `0x03F08004`;
- KSEG0: `0x83F08004`;
- KSEG1: `0xA3F08004`.

Distinct closed targets include global DEVICE_ID `0x03F80004`, RCP 1.0 first
responder `0x03F04004`, initial MODE `0x03F0000C`, and neighboring non-global
registers. Existing direct normalization and signed effective-address
arithmetic are reused without a generalized physical map.
