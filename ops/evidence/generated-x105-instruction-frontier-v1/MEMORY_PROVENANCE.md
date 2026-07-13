# Memory provenance

The SP-DMEM bytes used by the selected load already belong to the cartridge
bootstrap copy. This pass does not create a second byte owner.

`MachineSpDmemLoadWordProvenance::CartridgeBootstrap` records the exact source
cartridge offset on the public `MachineLoadWordTarget`. The mapping is
identity-offset for the current copy: local SP-DMEM offset `0x084` came from
generated cartridge offset `0x084`.

Offsets below `0x040` and any SP-DMEM state without a current bootstrap span
are classified as `UnclassifiedMachineStorage`; concrete backing bytes are not
promoted to known production truth. A rejected load reports the first unknown
offset and does not create destination lineage.

On success, the existing instruction-result GPR source records the execution
address, `Lw` identity, and source-base register. SP-IMEM user-PIF provenance
and cartridge-to-SP-DMEM provenance remain distinct and inspectable.
