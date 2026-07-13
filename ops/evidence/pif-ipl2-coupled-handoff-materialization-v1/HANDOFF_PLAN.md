# Coupled handoff plan

The Machine-owned bootstrap creation point first validates all independent
inputs, cartridge source bounds, PIF copy bounds, the supported NTSC profile,
and every coupled value. It then constructs replacement SP DMEM, replacement
SP IMEM, replacement CPU, GPR-source ledger, and bootstrap inspection state.

The supported plan stages only:

- r0 architectural zero;
- t3 `0xFFFFFFFFA4000040`;
- sp `0xFFFFFFFFA4001FF0`;
- ra `0xFFFFFFFFA4001550`;
- s3 cartridge value zero;
- s4 NTSC value one;
- s5 cold value zero;
- s6 x105 seed `0x91`;
- s7 explicit PIF version bit;
- Status `0x34000000`;
- PC `0xA4000040`, next PC `0xA4000044`, and no active delay slot.

Every other GPR remains source-Unknown even though replacement backing storage
is zero. Count, Compare, EPC, BadVAddr, Config, PRId, timer state, and device
state receive no handoff provenance.
