# Public X105 Sum Loop

Starting at local PC `0x064`, public `Machine::step` composition performs 256
iterations without forcing PC, registers, processor turn, memory, or delay
state.

Each iteration commits one aligned full-register Lqv, one scalar Addi, one
Bgez, and one Vaddc delay slot. Every RSP commit is separated by one real
CPU-selected commit. Bgez is taken for the first 255 values and not taken after
r3 becomes `0xFFFFFFF0`.

Exact result:

- selected attempts: 2048
- CPU/RSP commits: 1024/1024
- Lqv/Addi/Bgez/Vaddc: 256/256/256/256
- Bgez taken/not taken: 255/1
- final PC/next: `0x074/0x078`
- final RSP count: 1081
- final CPU Count/committed: `253425/253441`
- final delay context: unavailable
