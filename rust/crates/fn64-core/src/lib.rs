#![forbid(unsafe_code)]

pub mod cartridge;
pub mod cpu;
pub mod machine;
mod mi;
mod pif_firmware;
pub mod rdram;
mod ri;
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
    MachineLoadWordRejectionReason, MachineLoadWordTarget, MachineMtc0Destination,
    MachineMtc0Rejection, MachineMtc0RejectionReason, MachineOrdinaryControlFlowRejection,
    MachineOrdinaryControlFlowRejectionReason, MachinePifIpl2HandoffBootMedium,
    MachinePifIpl2HandoffInputKind, MachinePifIpl2HandoffInputs, MachinePifIpl2HandoffResetKind,
    MachinePifIpl3Family, MachinePifVersionBit, MachineRepresentedStepError,
    MachineRepresentedStepOutcome, MachineSpDmemCpuInstructionFetchError,
    MachineSpDmemInstructionProvenance, MachineSpDmemLoadWordProvenance,
    MachineSpImemOpaqueWordState, MachineSpImemStoreWordProvenance, MachineStepCadencePlan,
    MachineStepCadenceSource, MachineStepControlFlowAction, MachineStepCountAction,
    MachineStepCpuLocalInvocationRejection, MachineStepNoEffectExecutedInstruction,
    MachineStepNoEffectExecutedInstructionCategory, MachineStepStoppedInstruction,
    MachineStepStoppedInstructionCategory, MachineStepUnsupportedInstruction,
    MachineStepUnsupportedInstructionCategory, MachineStoreWordRejection,
    MachineStoreWordRejectionReason, MachineStoreWordTarget, MachineStoreWordUnsupportedTarget,
    MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC, MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET, MACHINE_PIF_IPL1_STATUS,
    MACHINE_PIF_IPL2_HANDOFF_NTSC_LINK_INSTRUCTION_ADDRESS, MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_SP_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_T3_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_X105_SEED,
};
pub use mi::{
    MachineMiInitModeSource, MachineMiInitModeState, MachineMiInitTransferState,
    MachineMiRdramRegisterModeSource, MachineMiRdramRegisterModeState, MachineMiVersionState,
    MI_CLEAR_RDRAM_REGISTER_MODE_WORD, MI_INIT_MODE_PHYSICAL_ADDRESS,
    MI_INIT_MODE_X105_INIT_LENGTH, MI_INIT_MODE_X105_REPEATED_BYTE_COUNT,
    MI_INIT_MODE_X105_WRITE_WORD, MI_SET_RDRAM_REGISTER_MODE_WORD, MI_VERSION_PHYSICAL_ADDRESS,
    MI_VERSION_STANDARD_RETAIL_NUS_WORD,
};
pub use pif_firmware::{
    MachinePifFirmwareState, PifFirmwareClassification, PifFirmwareValidationError,
    PifIpl2CopyLayout, PifIpl2Profile, PIF_BOOT_ROM_SIZE_BYTES,
    PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
};
pub use rdram::{
    MachineRdramBroadcastDelaySource, MachineRdramBroadcastDelayState,
    MachineRdramBroadcastDeviceIdAperture, MachineRdramBroadcastDeviceIdRequestState,
    MachineRdramBroadcastDeviceIdSource, MachineRdramBroadcastRefreshRowAperture,
    MachineRdramBroadcastRefreshRowSource, MachineRdramBroadcastRefreshRowState,
    MachineRdramCalibrationStatus, MachineRdramCpuRegisterWriteSource,
    MachineRdramFirstResponderDeviceIdAperture, MachineRdramFirstResponderDeviceIdRequestState,
    MachineRdramFirstResponderDeviceIdSource, MachineRdramInitialModeAperture,
    MachineRdramInitialModeRequestState, MachineRdramInitialModeSource, MachineRdramModeState,
    MachineRdramModuleState, MachineRdramProfile, MachineRdramProfileError,
    MachineRdramRegisterWordState, Rdram, RdramAccessError, RDRAM_BROADCAST_DELAY_PHYSICAL_ADDRESS,
    RDRAM_BROADCAST_DEVICE_ID_PHYSICAL_ADDRESS, RDRAM_BROADCAST_REFRESH_ROW_PHYSICAL_ADDRESS,
    RDRAM_DELAY_X105_CPU_TRANSFER_WORD, RDRAM_DELAY_X105_LOGICAL_CONFIGURATION,
    RDRAM_DEVICE_ID_X105_CPU_TRANSFER_WORD, RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE,
    RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS,
    RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_REQUESTED_INITIAL_DEVICE_ID,
    RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_WRITE_WORD, RDRAM_INITIAL_MODE_PHYSICAL_ADDRESS,
    RDRAM_INITIAL_MODE_X105_FIRST_MANUAL_WRITE_WORD, RDRAM_MODE_AUTO_SKIP, RDRAM_MODE_CC_ENABLE,
    RDRAM_MODE_CC_MULT, RDRAM_MODE_DEVICE_ENABLE, RDRAM_MODULE_SIZE_BYTES,
    RDRAM_REF_ROW_X105_WRITE_WORD, RDRAM_SIZE_BYTES, RDRAM_STANDARD_RETAIL_4_MIB_PROFILE_NAME,
    RDRAM_STANDARD_RETAIL_8_MIB_PROFILE_NAME, RDRAM_STANDARD_RETAIL_DEVICE_TYPE_WORD,
    RDRAM_STANDARD_RETAIL_MANUFACTURER_WORD, RDRAM_STANDARD_RETAIL_PROFILE_NAME,
    RDRAM_STANDARD_RETAIL_RAS_INTERVAL_WORD,
};
pub use ri::{
    ri_refresh_x105_word, MachineRiConfigSource, MachineRiConfigState, MachineRiCurrentLoadSource,
    MachineRiCurrentLoadState, MachineRiModeSource, MachineRiModeState, MachineRiRefreshSource,
    MachineRiRefreshState, MachineRiSelectSource, MachineRiSelectState,
    RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK, RI_CONFIG_CURRENT_CONTROL_INPUT_MASK,
    RI_CONFIG_DEFINED_FIELDS_MASK, RI_CONFIG_PHYSICAL_ADDRESS, RI_CURRENT_LOAD_PHYSICAL_ADDRESS,
    RI_MODE_DEFINED_FIELDS_MASK, RI_MODE_OPERATING_MODE_MASK, RI_MODE_PHYSICAL_ADDRESS,
    RI_MODE_STOP_RECEIVE_ACTIVE_MASK, RI_MODE_STOP_TRANSMIT_ACTIVE_MASK,
    RI_REFRESH_PHYSICAL_ADDRESS, RI_REFRESH_X105_BASE_WORD, RI_SELECT_PHYSICAL_ADDRESS,
    RI_SELECT_X105_ENABLE_TX_RX_WORD,
};
pub use sp_dmem::{SpDmem, SpDmemOffset, SpDmemReadError, SP_DMEM_SIZE_BYTES};
