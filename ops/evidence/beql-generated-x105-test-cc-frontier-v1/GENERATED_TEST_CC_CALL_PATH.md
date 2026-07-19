# Generated TestCCValue call path

The generated successor `Jal` at `0xA40009A4` targets `0xA4000A10`, writes
link `0xFFFFFFFFA40009AC`, and schedules `Or r4,r12,r0` at `0xA40009A8` as its
single delay slot. The slot copies nominal CC input zero and reaches
TestCCValue with Count `32211`, committed steps `32227`.

TestCCValue executes:

- `A4000A10 27BDFFD8` `Addiu sp,sp,-0x28`;
- `A4000A14 AFBF001C` `Sw ra,0x1C(sp)`;
- `A4000A18 00001025` `Or v0,r0,r0`;
- `A4000A1C 0D0002D1` `Jal 0xA4000B44`, link `FFFFFFFFA4000A24`;
- `A4000A20 24050002` `Addiu a1,r0,2` in the delay slot (`CC_MANUAL`).

All calls and slots are public `Machine::step` commits; no host substitution
or skipped call is used.
