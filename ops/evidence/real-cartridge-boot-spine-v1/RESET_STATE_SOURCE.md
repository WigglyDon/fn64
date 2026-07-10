# Reset-state source

The narrowly represented reset fact is the general PIF ROM stack-pointer
effect documented by the public n64docs boot-process reference:

<https://n64.readthedocs.io/>

The candidate stages only GPR 29 as `0xFFFFFFFFA4001FF0`. This fact is selected
unconditionally by the named Machine-owned bootstrap creation point. It is not
selected by title, complete ROM digest, region, CIC family, or any other game
identity. No PIF ROM or BIOS bytes are read, copied, bundled, or executed.

GPR zero remains the architectural known zero. Although the reference describes
additional PIF effects, this repair does not stage a wholesale post-PIF register
set: every other PIF-produced GPR stays explicitly `UnknownPifProduced` until a
future observed trace earns another source-backed fact.
