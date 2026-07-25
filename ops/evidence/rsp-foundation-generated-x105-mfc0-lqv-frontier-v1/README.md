# RSP Foundation And Public x105 MFC0 Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This directory records the first represented RSP instruction execution in
fn64. The public generated cold-x105 path performs its ordinary CPU work,
commits the source-defined SP halt-to-run command, alternates CPU and RSP
instruction attempts through the sole public `Machine::step`, commits two
scalar `Mfc0` instructions, and stops on an atomic vector `Lqv` rejection.

The represented RSP instructions are:

- local `0x000`: public word `0x40083800`, `Mfc0 r8,SP_SEMAPHORE`;
- local `0x004`: public word `0x400B0800`, `Mfc0 r11,SP_DRAM_ADDR`;
- local `0x008`: public word `0xC80C2000`, identified `Lqv v12[0],0(r0)` and
  rejected without execution.

The proof uses only independently generated public bytes. It uses no user
cartridge, private PIF, proprietary microcode, title policy, digest policy, or
inspection-side mutation. It earns neither BOOT-3 nor compatibility.

