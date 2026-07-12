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
    MachineArithmeticOverflowExceptionEntryRejection, MachineBootstrapCpuStateKind,
    MachineBootstrapCpuStateUnavailable, MachineBootstrapGprSource, MachineCartridgeBootstrapError,
    MachineCartridgeBootstrapState, MachineCpuInstructionFetchError,
    MachineCpuInstructionFetchTarget, MachineCpuInstructionFetchTargetError,
    MachineCpuInstructionInspection, MachineCpuInstructionSource,
    MachineDirectRdramCpuDataAccessError, MachineDirectRdramCpuInstructionFetchError,
    MachineInstructionFetchAddressErrorPlan, MachineInstructionFetchAddressErrorPlanError,
    MachineInstructionFetchAddressErrorSource, MachineLoadWordRejection,
    MachineLoadWordRejectionReason, MachineLoadWordTarget, MachineOrdinaryControlFlowRejection,
    MachineOrdinaryControlFlowRejectionReason, MachineRepresentedStepError,
    MachineRepresentedStepOutcome, MachineSpDmemCpuInstructionFetchError,
    MachineSpDmemInstructionProvenance, MachineStepCadencePlan, MachineStepCadenceSource,
    MachineStepControlFlowAction, MachineStepCountAction, MachineStepCpuLocalInvocationRejection,
    MachineStepNoEffectExecutedInstruction, MachineStepNoEffectExecutedInstructionCategory,
    MachineStepStoppedInstruction, MachineStepStoppedInstructionCategory,
    MachineStepUnsupportedInstruction, MachineStepUnsupportedInstructionCategory,
    MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC, MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET, MACHINE_GENERAL_PIF_RESET_GPR29_VALUE,
    MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
};
pub use pif_firmware::{
    MachinePifFirmwareState, PifFirmwareClassification, PifFirmwareValidationError,
    PIF_BOOT_ROM_SIZE_BYTES, PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
};
pub use rdram::{Rdram, RdramAccessError, RDRAM_SIZE_BYTES};
pub use sp_dmem::{SpDmem, SpDmemOffset, SpDmemReadError, SP_DMEM_SIZE_BYTES};
