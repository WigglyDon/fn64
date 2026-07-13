#![forbid(unsafe_code)]

pub mod cartridge;
pub mod cpu;
pub mod machine;
mod pif_firmware;
pub mod rdram;
pub mod sp_dmem;
mod sp_imem;

pub use cartridge::{
    inspect_cartridge_entry, load_cartridge, normalize_rom_image, rom_source_layout_name,
    Cartridge, CartridgeEntryInspection, CartridgeLoadError, CartridgeReadError,
    NormalizedRomImage, RomMetadata, RomSourceLayout, CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT,
    CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE, CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
    CARTRIDGE_HEADER_ENTRY_WORD_OFFSET,
};
pub use cpu::{
    decode_cpu_instruction_word, identify_cpu_instruction, Cpu, CpuAddressErrorExceptionEntryError,
    CpuAddressErrorKind, CpuDataAccessKind, CpuDataAddressError, CpuDataAlignmentError,
    CpuDataWidth, CpuDelaySlotContext, CpuInstructionFields, CpuInstructionIdentity,
    CpuInstructionWord, CpuRegisterIndexError, CPU_GPR_COUNT, NON_BOOT_RESET_VECTOR_NEXT_PC,
    NON_BOOT_RESET_VECTOR_PC,
};
pub use machine::{
    select_cpu_instruction_fetch_address_error, DirectRdramAccessError, Machine,
    MachineArithmeticOverflowExceptionEntryRejection, MachineBootstrapControlFlowSource,
    MachineBootstrapCop0StatusSource, MachineBootstrapCpuStateKind,
    MachineBootstrapCpuStateUnavailable, MachineBootstrapGprSource, MachineCartridgeBootstrapError,
    MachineCartridgeBootstrapState, MachineCpuInstructionFetchError,
    MachineCpuInstructionFetchTarget, MachineCpuInstructionFetchTargetError,
    MachineCpuInstructionInspection, MachineCpuInstructionSource,
    MachineDirectRdramCpuDataAccessError, MachineDirectRdramCpuInstructionFetchError,
    MachineInstructionFetchAddressErrorPlan, MachineInstructionFetchAddressErrorPlanError,
    MachineInstructionFetchAddressErrorSource, MachineLoadWordRejection,
    MachineLoadWordRejectionReason, MachineLoadWordTarget, MachineOrdinaryControlFlowRejection,
    MachineOrdinaryControlFlowRejectionReason, MachinePifIpl2HandoffBootMedium,
    MachinePifIpl2HandoffInputKind, MachinePifIpl2HandoffInputs, MachinePifIpl2HandoffResetKind,
    MachinePifIpl3Family, MachinePifVersionBit, MachineRepresentedStepError,
    MachineRepresentedStepOutcome, MachineSpDmemCpuInstructionFetchError,
    MachineSpDmemInstructionProvenance, MachineSpDmemLoadWordProvenance,
    MachineSpImemStoreWordProvenance, MachineStepCadencePlan, MachineStepCadenceSource,
    MachineStepControlFlowAction, MachineStepCountAction, MachineStepCpuLocalInvocationRejection,
    MachineStepNoEffectExecutedInstruction, MachineStepNoEffectExecutedInstructionCategory,
    MachineStepStoppedInstruction, MachineStepStoppedInstructionCategory,
    MachineStepUnsupportedInstruction, MachineStepUnsupportedInstructionCategory,
    MachineStoreWordRejection, MachineStoreWordRejectionReason, MachineStoreWordTarget,
    MachineStoreWordUnsupportedTarget, MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC,
    MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC, MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET, MACHINE_PIF_IPL1_STATUS,
    MACHINE_PIF_IPL2_HANDOFF_NTSC_LINK_INSTRUCTION_ADDRESS, MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_SP_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_T3_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_X105_SEED,
};
pub use pif_firmware::{
    MachinePifFirmwareState, PifFirmwareClassification, PifFirmwareValidationError,
    PifIpl2CopyLayout, PifIpl2Profile, PIF_BOOT_ROM_SIZE_BYTES,
    PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
};
pub use rdram::{Rdram, RdramAccessError, RDRAM_SIZE_BYTES};
pub use sp_dmem::{SpDmem, SpDmemOffset, SpDmemReadError, SP_DMEM_SIZE_BYTES};
