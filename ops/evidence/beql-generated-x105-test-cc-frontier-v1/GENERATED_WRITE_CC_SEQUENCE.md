# Generated WriteCC sequence

WriteCC enters at `0xA4000B44`, makes a `0x28`-byte frame, masks nominal input
zero to zero, and XORs with `0x3F`, producing `r4=0x3F`. Manual mode (`r5=2`)
branches around the automatic-enable OR while executing `Lui r15,0x4600` in
the slot. The six generated mask/shift/OR groups scatter the bits into
`0x00C0C0C0`; after `Or` at `0xA4000BAC`, `r15=0x46C0C0C0`.

At `0xA4000BB4`, `Bne r5,r27,0xA4000BC4` is taken. Its ordinary delay-slot
instruction at `0xA4000BB8` is the first device frontier and is attempted once.
