# Public x105 post-Break CPU frontier

After Break, read-only inspection identifies:

- selected processor: CPU;
- CPU PC: `0x80000188`;
- CPU next PC: `0x80000114`;
- active delay owner: `0x80000184`;
- current authoritative instruction-cache word: `0x02cfb024`;
- identity: `SpecialAnd`;
- represented by the current CPU decoder: yes.

No post-Break `Machine::step` is called. The CPU instruction is identified but
not committed. A stale RDRAM backing word is not substituted for the current
instruction-cache fetch truth.

