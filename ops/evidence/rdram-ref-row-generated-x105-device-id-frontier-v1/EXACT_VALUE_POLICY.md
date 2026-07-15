# Exact value policy

Accepted low word: `0x00000000`.

Every nonzero low word rejects before mutation as fn64's bounded unsupported
surface. This makes no hardware-trap claim. Known 64-bit sources with nonzero
high halves and a zero low word remain valid under ordinary `Sw` semantics.
