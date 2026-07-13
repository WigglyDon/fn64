# Selected bounded family

Result: `DIFFERENT_BOUNDED_FAMILY_SELECTED — SP-DMEM-ROUTED ALIGNED LW`.

The pinned identity order is arithmetic register copy, retained SP-IMEM load,
cartridge-staged SP-DMEM load, logical transform, then aligned store. Current
Rust represents the first, second, and fourth identities. It decodes and
identifies the third as `Lw`, but the word-data classifier routes only direct
RDRAM and SP IMEM. The first unavailable execution fact is therefore the
narrow SP-DMEM target for the already bounded aligned-`Lw` family.

Selected scope:

- direct KSEG0/KSEG1 translation to the existing 4 KiB SP-DMEM owner;
- naturally aligned four-byte reads only;
- only cartridge-bootstrap-staged bytes are known production data;
- explicit cartridge-offset provenance in the public load target;
- existing AdEL, delay-slot, rollback, cadence, and GPR-lineage owners;
- generated composition through the following logical transform.

Not selected:

- `Sw`, because it occurs after the missing SP-DMEM load;
- SP-DMEM writes;
- RDRAM or SP-IMEM behavior changes;
- a bus, generalized map, device route, or new exception framework.
