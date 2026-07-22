# State ownership

One fact retains one owner:

- `Pi` owns programmed PI registers, idle status bits, completion record, and
  PI command provenance;
- `Cartridge` owns immutable synthetic cartridge bytes;
- `Rdram` owns all destination, checksum-side-data, and boot-global bytes;
- `Cpu` owns GPR lineage, control flow, COP0, and primary cache arrays;
- `Mi` owns interrupt pending and mask truth;
- `Sp` owns SP status, PC, and semaphore command truth;
- `SpDmem` and `SpImem` own their respective bytes and CPU-store provenance.

The immutable PI transfer plan is temporary preflight state, not a byte owner.
The D-cache contains functional cached copies, not RDRAM ownership. Inspection
only observes public Machine truth and does not mutate guest state.

No generic bus, MMIO registry, generalized memory router, static mutable state,
or host-owned emulation policy was introduced.
