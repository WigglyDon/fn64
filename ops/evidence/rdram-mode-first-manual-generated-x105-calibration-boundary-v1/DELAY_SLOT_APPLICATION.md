# Delay-Slot Application

The taken BNE at `0xA4000BB4` owns the existing delay context. Before the store,
PC/next-PC are `0xA4000BB8/0xA4000BC4`, Count is 32243, and commits are 32259.
The successful store advances once to `0xA4000BC4/0xA4000BC8`, Count 32244,
commits 32260, and clears the existing context. No second owner or recursive
step exists.
