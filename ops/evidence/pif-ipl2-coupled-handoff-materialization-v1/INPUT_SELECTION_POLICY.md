# Input selection policy

The supported path requires separate explicit facts:

- accepted 1,984-byte raw PIF input;
- `NTSC_PINNED` PIF copy profile;
- IPL3 family `X105`;
- reset kind `Cold`;
- boot medium `Cartridge`;
- PIF version bit `Zero` or `One`.

The host may parse literal spellings and transfer typed values. Machine owns
their meaning and rejects missing, partial, or unsupported combinations before
mutation. Cartridge bootstrap already identifies the boot medium; the explicit
typed value prevents that fact from being silently inferred by a future caller.

There is no default, autodetection, title branch, product-code branch, filename
branch, full-ROM-digest branch, firmware-digest branch, host-region branch, or
expected-trace selector. PAL_PINNED and MPAL_PINNED remain valid copy profiles
but are unsupported coupled-handoff profiles in this product.
