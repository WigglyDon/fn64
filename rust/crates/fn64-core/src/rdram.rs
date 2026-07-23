use core::fmt;

use crate::cpu::address::CpuAddress;
use crate::cpu::{
    MachinePrimaryDataCacheStoreProvenance, MachinePrimaryDataCacheWritebackPlan,
    PRIMARY_DATA_CACHE_LINE_SIZE_BYTES,
};
use crate::machine::MachineBootstrapGprSource;
use crate::mi::MachineMiInitTransferState;

pub const RDRAM_SIZE_BYTES: usize = 4 * 1024 * 1024;
pub const RDRAM_BROADCAST_DEVICE_ID_PHYSICAL_ADDRESS: u32 = 0x03f8_0004;
pub const RDRAM_BROADCAST_DELAY_PHYSICAL_ADDRESS: u32 = 0x03f8_0008;
pub const RDRAM_BROADCAST_REFRESH_ROW_PHYSICAL_ADDRESS: u32 = 0x03f8_0014;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS: u32 = 0x03f0_8004;
pub const RDRAM_INITIAL_MODE_PHYSICAL_ADDRESS: u32 = 0x03f0_000c;
pub const RDRAM_DEVICE_ID_X105_CPU_TRANSFER_WORD: u32 = 0x8000_0000;
pub const RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE: u32 = 0x0200_0000;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_WRITE_WORD: u32 = 0x0000_0000;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_REQUESTED_INITIAL_DEVICE_ID: u32 = 0x0000_0000;
pub const RDRAM_DELAY_X105_CPU_TRANSFER_WORD: u32 = 0x1808_2838;
pub const RDRAM_DELAY_X105_LOGICAL_CONFIGURATION: u32 = 0x2838_1808;
pub const RDRAM_REF_ROW_X105_WRITE_WORD: u32 = 0x0000_0000;
pub const RDRAM_INITIAL_MODE_X105_FIRST_MANUAL_WRITE_WORD: u32 = 0x46c0_c0c0;
pub const RDRAM_MODE_DEVICE_ENABLE: u32 = 0x0200_0000;
pub const RDRAM_MODE_AUTO_SKIP: u32 = 0x0400_0000;
pub const RDRAM_MODE_CC_MULT: u32 = 0x4000_0000;
pub const RDRAM_MODE_CC_ENABLE: u32 = 0x8000_0000;
pub const RDRAM_MODULE_SIZE_BYTES: u32 = 0x0020_0000;
pub const RDRAM_STANDARD_RETAIL_DEVICE_TYPE_WORD: u32 = 0xb019_0000;
pub const RDRAM_STANDARD_RETAIL_MANUFACTURER_WORD: u32 = 0x0000_0500;
pub const RDRAM_STANDARD_RETAIL_RAS_INTERVAL_WORD: u32 = 0x101c_0a04;
pub const RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS: u32 = 0x03f0_0000;
pub const RDRAM_RCP2_REGISTER_SPACING: u32 = 0x0000_0400;
pub const RDRAM_DEVICE_TYPE_REGISTER_OFFSET: u32 = 0x00;
pub const RDRAM_DEVICE_ID_REGISTER_OFFSET: u32 = 0x04;
pub const RDRAM_MODE_REGISTER_OFFSET: u32 = 0x0c;
pub const RDRAM_RAS_INTERVAL_REGISTER_OFFSET: u32 = 0x18;
pub const RDRAM_DEVICE_MANUFACTURER_REGISTER_OFFSET: u32 = 0x24;
pub const RDRAM_GLOBAL_MODE_X105_WRITE_WORD: u32 = 0xc400_0000;
pub const RDRAM_GLOBAL_MODE_PHYSICAL_ADDRESS: u32 = 0x03f8_000c;
pub const RDRAM_STANDARD_RETAIL_4_MIB_PROFILE_NAME: &str =
    "fixed-standard-retail-4mib-two-module-digital-cc-v1";
pub const RDRAM_STANDARD_RETAIL_8_MIB_PROFILE_NAME: &str =
    "fixed-standard-retail-8mib-four-module-digital-cc-v1";
pub const RDRAM_STANDARD_RETAIL_PROFILE_NAME: &str = RDRAM_STANDARD_RETAIL_4_MIB_PROFILE_NAME;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramProfileError {
    UnsupportedCapacity { capacity_bytes: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramProfile {
    capacity_bytes: usize,
    module_count: u8,
}

impl MachineRdramProfile {
    fn from_capacity_bytes(capacity_bytes: usize) -> Result<Self, MachineRdramProfileError> {
        let module_count = match capacity_bytes {
            0x0040_0000 => 2,
            0x0080_0000 => 4,
            _ => {
                return Err(MachineRdramProfileError::UnsupportedCapacity { capacity_bytes });
            }
        };
        Ok(Self {
            capacity_bytes,
            module_count,
        })
    }

    pub const fn name(self) -> &'static str {
        match self.capacity_bytes {
            0x0040_0000 => RDRAM_STANDARD_RETAIL_4_MIB_PROFILE_NAME,
            0x0080_0000 => RDRAM_STANDARD_RETAIL_8_MIB_PROFILE_NAME,
            _ => unreachable!(),
        }
    }

    pub const fn capacity_bytes(self) -> usize {
        self.capacity_bytes
    }

    pub const fn module_count(self) -> u8 {
        self.module_count
    }

    pub const fn module_size_bytes(self) -> u32 {
        RDRAM_MODULE_SIZE_BYTES
    }

    pub const fn device_type_word(self) -> u32 {
        RDRAM_STANDARD_RETAIL_DEVICE_TYPE_WORD
    }

    pub const fn manufacturer_word(self) -> u32 {
        RDRAM_STANDARD_RETAIL_MANUFACTURER_WORD
    }

    pub const fn enhanced_speed(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramCpuRegisterWriteSource {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineRdramCpuRegisterWriteSource {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    ) -> Self {
        Self {
            instruction_pc,
            source_gpr,
            source_lineage,
            effective_address,
            cpu_address,
            physical_address,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn source_gpr(self) -> u8 {
        self.source_gpr
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        self.source_lineage
    }

    pub const fn effective_address(self) -> u64 {
        self.effective_address
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramModeState {
    raw_word: u32,
    source: MachineRdramCpuRegisterWriteSource,
}

impl MachineRdramModeState {
    pub(crate) const fn new(raw_word: u32, source: MachineRdramCpuRegisterWriteSource) -> Self {
        Self { raw_word, source }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn source(self) -> MachineRdramCpuRegisterWriteSource {
        self.source
    }

    pub const fn device_enable(self) -> bool {
        self.raw_word & RDRAM_MODE_DEVICE_ENABLE != 0
    }

    pub const fn auto_skip(self) -> bool {
        self.raw_word & RDRAM_MODE_AUTO_SKIP != 0
    }

    pub const fn current_control_multiplier(self) -> bool {
        self.raw_word & RDRAM_MODE_CC_MULT != 0
    }

    pub const fn current_control_enable(self) -> bool {
        self.raw_word & RDRAM_MODE_CC_ENABLE != 0
    }

    pub const fn encoded_current_control_code(self) -> u8 {
        decode_mode_current_control_code(self.raw_word)
    }

    pub const fn nominal_current_control_input(self) -> u8 {
        self.encoded_current_control_code() ^ 0x3f
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramRegisterWordState {
    raw_word: u32,
    source: MachineRdramCpuRegisterWriteSource,
}

impl MachineRdramRegisterWordState {
    pub(crate) const fn new(raw_word: u32, source: MachineRdramCpuRegisterWriteSource) -> Self {
        Self { raw_word, source }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn source(self) -> MachineRdramCpuRegisterWriteSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramCalibrationStatus {
    Untested,
    ManualSearching { nominal_input: u8, score: u8 },
    Calibrated { automatic_input: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRdramModuleState {
    index: u8,
    module_size_bytes: u32,
    device_type_word: u32,
    manufacturer_word: u32,
    temporary_device_id_request: Option<MachineRdramFirstResponderDeviceIdRequestState>,
    final_device_id_request: Option<MachineRdramFirstResponderDeviceIdRequestState>,
    mapped_physical_base: u32,
    mode: Option<MachineRdramModeState>,
    ras_interval: Option<MachineRdramRegisterWordState>,
    calibration_status: MachineRdramCalibrationStatus,
}

impl MachineRdramModuleState {
    fn new(index: u8) -> Self {
        Self {
            index,
            module_size_bytes: RDRAM_MODULE_SIZE_BYTES,
            device_type_word: RDRAM_STANDARD_RETAIL_DEVICE_TYPE_WORD,
            manufacturer_word: RDRAM_STANDARD_RETAIL_MANUFACTURER_WORD,
            temporary_device_id_request: None,
            final_device_id_request: None,
            mapped_physical_base: RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE,
            mode: None,
            ras_interval: None,
            calibration_status: MachineRdramCalibrationStatus::Untested,
        }
    }

    pub const fn index(&self) -> u8 {
        self.index
    }

    pub const fn module_size_bytes(&self) -> u32 {
        self.module_size_bytes
    }

    pub const fn device_type_word(&self) -> u32 {
        self.device_type_word
    }

    pub const fn manufacturer_word(&self) -> u32 {
        self.manufacturer_word
    }

    pub const fn temporary_device_id_word(&self) -> Option<u32> {
        match self.temporary_device_id_request {
            Some(state) => Some(state.raw_cpu_word()),
            None => None,
        }
    }

    pub const fn final_device_id_word(&self) -> Option<u32> {
        match self.final_device_id_request {
            Some(state) => Some(state.raw_cpu_word()),
            None => None,
        }
    }

    pub const fn temporary_device_id_request_state(
        &self,
    ) -> Option<MachineRdramFirstResponderDeviceIdRequestState> {
        self.temporary_device_id_request
    }

    pub const fn final_device_id_request_state(
        &self,
    ) -> Option<MachineRdramFirstResponderDeviceIdRequestState> {
        self.final_device_id_request
    }

    pub const fn mapped_physical_base(&self) -> u32 {
        self.mapped_physical_base
    }

    pub const fn mode_state(&self) -> Option<MachineRdramModeState> {
        self.mode
    }

    pub const fn ras_interval_state(&self) -> Option<MachineRdramRegisterWordState> {
        self.ras_interval
    }

    pub const fn calibration_status(&self) -> MachineRdramCalibrationStatus {
        self.calibration_status
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MachineRdramCalibrationTarget {
    Present { module_index: u8, probe_base: u32 },
    Absent { probe_base: u32 },
}

const fn decode_mode_current_control_code(raw_word: u32) -> u8 {
    (((raw_word >> 6) & 1) as u8)
        | ((((raw_word >> 14) & 1) as u8) << 1)
        | ((((raw_word >> 22) & 1) as u8) << 2)
        | ((((raw_word >> 7) & 1) as u8) << 3)
        | ((((raw_word >> 15) & 1) as u8) << 4)
        | ((((raw_word >> 23) & 1) as u8) << 5)
}

const fn encode_mode_current_control_code(code: u8) -> u32 {
    (((code as u32) & 0x01) << 6)
        | (((code as u32) & 0x02) << 13)
        | (((code as u32) & 0x04) << 20)
        | (((code as u32) & 0x08) << 4)
        | (((code as u32) & 0x10) << 11)
        | (((code as u32) & 0x20) << 18)
}

pub const fn rdram_mode_generated_family_word(raw_word: u32) -> bool {
    let field_mask = encode_mode_current_control_code(0x3f);
    let base = raw_word & !field_mask;
    base == (RDRAM_MODE_DEVICE_ENABLE | RDRAM_MODE_AUTO_SKIP | RDRAM_MODE_CC_MULT)
        || base
            == (RDRAM_MODE_DEVICE_ENABLE
                | RDRAM_MODE_AUTO_SKIP
                | RDRAM_MODE_CC_MULT
                | RDRAM_MODE_CC_ENABLE)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramBroadcastDelaySource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
        cpu_transfer_word: u32,
        consumed_mi_transfer: MachineMiInitTransferState,
    },
}

impl MachineRdramBroadcastDelaySource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }

    pub const fn effective_address(self) -> u64 {
        match self {
            Self::CpuStoreWord {
                effective_address, ..
            } => effective_address,
        }
    }

    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn physical_address(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                physical_address, ..
            } => physical_address,
        }
    }

    pub const fn cpu_transfer_word(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                cpu_transfer_word, ..
            } => cpu_transfer_word,
        }
    }

    pub const fn consumed_mi_transfer(self) -> MachineMiInitTransferState {
        match self {
            Self::CpuStoreWord {
                consumed_mi_transfer,
                ..
            } => consumed_mi_transfer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramBroadcastDelayState {
    ack_window_delay: u8,
    read_delay: u8,
    ack_delay: u8,
    write_delay: u8,
    logical_configuration: u32,
    source: MachineRdramBroadcastDelaySource,
}

impl MachineRdramBroadcastDelayState {
    pub(crate) const fn from_exact_x105_cpu_store(
        source: MachineRdramBroadcastDelaySource,
    ) -> Self {
        debug_assert!(source.physical_address() == RDRAM_BROADCAST_DELAY_PHYSICAL_ADDRESS);
        debug_assert!(source.cpu_transfer_word() == RDRAM_DELAY_X105_CPU_TRANSFER_WORD);
        debug_assert!(source.consumed_mi_transfer().source_init_length() == 15);
        debug_assert!(source.consumed_mi_transfer().repeated_byte_count() == 16);
        Self {
            ack_window_delay: 5,
            read_delay: 7,
            ack_delay: 3,
            write_delay: 1,
            logical_configuration: RDRAM_DELAY_X105_LOGICAL_CONFIGURATION,
            source,
        }
    }

    pub const fn ack_window_delay(self) -> u8 {
        self.ack_window_delay
    }

    pub const fn read_delay(self) -> u8 {
        self.read_delay
    }

    pub const fn ack_delay(self) -> u8 {
        self.ack_delay
    }

    pub const fn write_delay(self) -> u8 {
        self.write_delay
    }

    pub const fn logical_configuration(self) -> u32 {
        self.logical_configuration
    }

    pub const fn source(self) -> MachineRdramBroadcastDelaySource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramBroadcastRefreshRowAperture {
    GlobalBroadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramBroadcastRefreshRowSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    },
}

impl MachineRdramBroadcastRefreshRowSource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }

    pub const fn effective_address(self) -> u64 {
        match self {
            Self::CpuStoreWord {
                effective_address, ..
            } => effective_address,
        }
    }

    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn physical_address(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                physical_address, ..
            } => physical_address,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramBroadcastRefreshRowState {
    raw_word: u32,
    aperture: MachineRdramBroadcastRefreshRowAperture,
    source: MachineRdramBroadcastRefreshRowSource,
}

impl MachineRdramBroadcastRefreshRowState {
    pub(crate) const fn from_exact_x105_zero_cpu_store(
        source: MachineRdramBroadcastRefreshRowSource,
    ) -> Self {
        debug_assert!(source.physical_address() == RDRAM_BROADCAST_REFRESH_ROW_PHYSICAL_ADDRESS);
        Self {
            raw_word: RDRAM_REF_ROW_X105_WRITE_WORD,
            aperture: MachineRdramBroadcastRefreshRowAperture::GlobalBroadcast,
            source,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn aperture(self) -> MachineRdramBroadcastRefreshRowAperture {
        self.aperture
    }

    pub const fn source(self) -> MachineRdramBroadcastRefreshRowSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramBroadcastDeviceIdAperture {
    GlobalBroadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramBroadcastDeviceIdSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    },
}

impl MachineRdramBroadcastDeviceIdSource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }

    pub const fn effective_address(self) -> u64 {
        match self {
            Self::CpuStoreWord {
                effective_address, ..
            } => effective_address,
        }
    }

    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn physical_address(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                physical_address, ..
            } => physical_address,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramBroadcastDeviceIdRequestState {
    raw_cpu_word: u32,
    requested_physical_base: u32,
    aperture: MachineRdramBroadcastDeviceIdAperture,
    source: MachineRdramBroadcastDeviceIdSource,
}

impl MachineRdramBroadcastDeviceIdRequestState {
    pub(crate) const fn from_exact_x105_cpu_store(
        source: MachineRdramBroadcastDeviceIdSource,
    ) -> Self {
        debug_assert!(source.physical_address() == RDRAM_BROADCAST_DEVICE_ID_PHYSICAL_ADDRESS);
        Self {
            raw_cpu_word: RDRAM_DEVICE_ID_X105_CPU_TRANSFER_WORD,
            requested_physical_base: RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE,
            aperture: MachineRdramBroadcastDeviceIdAperture::GlobalBroadcast,
            source,
        }
    }

    pub const fn raw_cpu_word(self) -> u32 {
        self.raw_cpu_word
    }

    pub const fn requested_physical_base(self) -> u32 {
        self.requested_physical_base
    }

    pub const fn aperture(self) -> MachineRdramBroadcastDeviceIdAperture {
        self.aperture
    }

    pub const fn source(self) -> MachineRdramBroadcastDeviceIdSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramFirstResponderDeviceIdAperture {
    Rcp2FirstResponder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramFirstResponderDeviceIdSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    },
}

impl MachineRdramFirstResponderDeviceIdSource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }

    pub const fn effective_address(self) -> u64 {
        match self {
            Self::CpuStoreWord {
                effective_address, ..
            } => effective_address,
        }
    }

    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn physical_address(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                physical_address, ..
            } => physical_address,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramFirstResponderDeviceIdRequestState {
    raw_cpu_word: u32,
    requested_initial_device_id: u32,
    requested_physical_base: u32,
    aperture: MachineRdramFirstResponderDeviceIdAperture,
    source: MachineRdramFirstResponderDeviceIdSource,
}

impl MachineRdramFirstResponderDeviceIdRequestState {
    pub(crate) const fn from_x105_cpu_store(
        raw_cpu_word: u32,
        source: MachineRdramFirstResponderDeviceIdSource,
    ) -> Self {
        debug_assert!(
            source.physical_address() == RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS
        );
        Self {
            raw_cpu_word,
            requested_initial_device_id: raw_cpu_word,
            requested_physical_base: raw_cpu_word >> 6,
            aperture: MachineRdramFirstResponderDeviceIdAperture::Rcp2FirstResponder,
            source,
        }
    }

    pub const fn raw_cpu_word(self) -> u32 {
        self.raw_cpu_word
    }

    pub const fn requested_initial_device_id(self) -> u32 {
        self.requested_initial_device_id
    }

    pub const fn requested_physical_base(self) -> u32 {
        self.requested_physical_base
    }

    pub const fn aperture(self) -> MachineRdramFirstResponderDeviceIdAperture {
        self.aperture
    }

    pub const fn source(self) -> MachineRdramFirstResponderDeviceIdSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramInitialModeAperture {
    InitialNonGlobal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRdramInitialModeSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    },
}

impl MachineRdramInitialModeSource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }

    pub const fn effective_address(self) -> u64 {
        match self {
            Self::CpuStoreWord {
                effective_address, ..
            } => effective_address,
        }
    }

    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn physical_address(self) -> u32 {
        match self {
            Self::CpuStoreWord {
                physical_address, ..
            } => physical_address,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramInitialModeRequestState {
    raw_word: u32,
    aperture: MachineRdramInitialModeAperture,
    source: MachineRdramInitialModeSource,
}

impl MachineRdramInitialModeRequestState {
    pub(crate) const fn from_exact_x105_first_manual_cpu_store(
        source: MachineRdramInitialModeSource,
    ) -> Self {
        debug_assert!(source.physical_address() == RDRAM_INITIAL_MODE_PHYSICAL_ADDRESS);
        Self {
            raw_word: RDRAM_INITIAL_MODE_X105_FIRST_MANUAL_WRITE_WORD,
            aperture: MachineRdramInitialModeAperture::InitialNonGlobal,
            source,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn aperture(self) -> MachineRdramInitialModeAperture {
        self.aperture
    }

    pub const fn source(self) -> MachineRdramInitialModeSource {
        self.source
    }

    pub const fn device_enable(self) -> bool {
        self.raw_word & RDRAM_MODE_DEVICE_ENABLE != 0
    }

    pub const fn auto_skip(self) -> bool {
        self.raw_word & RDRAM_MODE_AUTO_SKIP != 0
    }

    pub const fn current_control_multiplier(self) -> bool {
        self.raw_word & RDRAM_MODE_CC_MULT != 0
    }

    pub const fn current_control_enable(self) -> bool {
        self.raw_word & RDRAM_MODE_CC_ENABLE != 0
    }

    pub const fn encoded_current_control_code(self) -> u8 {
        (((self.raw_word >> 6) & 1) as u8)
            | ((((self.raw_word >> 14) & 1) as u8) << 1)
            | ((((self.raw_word >> 22) & 1) as u8) << 2)
            | ((((self.raw_word >> 7) & 1) as u8) << 3)
            | ((((self.raw_word >> 15) & 1) as u8) << 4)
            | ((((self.raw_word >> 23) & 1) as u8) << 5)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RdramAccessError {
    offset: usize,
    width: usize,
}

impl RdramAccessError {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for RdramAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RDRAM access out of range: address={} width={}",
            self.offset, self.width
        )
    }
}

impl std::error::Error for RdramAccessError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRdramPrimaryDataCacheWritebackState {
    evicting_instruction_pc: CpuAddress,
    physical_line_address: u32,
    cache_line_index: u16,
    data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
    latest_dirty_store: MachinePrimaryDataCacheStoreProvenance,
}

impl MachineRdramPrimaryDataCacheWritebackState {
    pub(crate) const fn from_plan(
        evicting_instruction_pc: CpuAddress,
        plan: MachinePrimaryDataCacheWritebackPlan,
    ) -> Self {
        Self {
            evicting_instruction_pc,
            physical_line_address: plan.physical_line_address(),
            cache_line_index: plan.line_index() as u16,
            data: plan.data(),
            latest_dirty_store: plan.latest_store(),
        }
    }

    pub const fn evicting_instruction_pc(self) -> CpuAddress {
        self.evicting_instruction_pc
    }

    pub const fn physical_line_address(self) -> u32 {
        self.physical_line_address
    }

    pub const fn cache_line_index(self) -> u16 {
        self.cache_line_index
    }

    pub const fn data(self) -> [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES] {
        self.data
    }

    pub const fn latest_dirty_store(self) -> MachinePrimaryDataCacheStoreProvenance {
        self.latest_dirty_store
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rdram {
    bytes: Vec<u8>,
    profile: MachineRdramProfile,
    modules: Vec<MachineRdramModuleState>,
    broadcast_device_id_request: Option<MachineRdramBroadcastDeviceIdRequestState>,
    broadcast_delay: Option<MachineRdramBroadcastDelayState>,
    broadcast_refresh_row: Option<MachineRdramBroadcastRefreshRowState>,
    first_responder_device_id_request: Option<MachineRdramFirstResponderDeviceIdRequestState>,
    initial_mode_request: Option<MachineRdramInitialModeRequestState>,
    global_mode_request: Option<MachineRdramModeState>,
    finalization_started: bool,
    active_calibration: Option<MachineRdramCalibrationTarget>,
    primary_data_cache_writebacks: Vec<MachineRdramPrimaryDataCacheWritebackState>,
}

impl Rdram {
    fn with_size_bytes(size_bytes: usize) -> Result<Self, MachineRdramProfileError> {
        let profile = MachineRdramProfile::from_capacity_bytes(size_bytes)?;
        let modules = (0..profile.module_count())
            .map(MachineRdramModuleState::new)
            .collect();
        Ok(Self {
            bytes: vec![0; size_bytes],
            profile,
            modules,
            broadcast_device_id_request: None,
            broadcast_delay: None,
            broadcast_refresh_row: None,
            first_responder_device_id_request: None,
            initial_mode_request: None,
            global_mode_request: None,
            finalization_started: false,
            active_calibration: None,
            primary_data_cache_writebacks: Vec::new(),
        })
    }

    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) const fn profile_state(&self) -> MachineRdramProfile {
        self.profile
    }

    pub(crate) fn module_states(&self) -> &[MachineRdramModuleState] {
        &self.modules
    }

    pub fn primary_data_cache_writebacks(&self) -> &[MachineRdramPrimaryDataCacheWritebackState] {
        &self.primary_data_cache_writebacks
    }

    pub(crate) fn apply_primary_data_cache_writeback(
        &mut self,
        evicting_instruction_pc: CpuAddress,
        plan: MachinePrimaryDataCacheWritebackPlan,
    ) {
        let state =
            MachineRdramPrimaryDataCacheWritebackState::from_plan(evicting_instruction_pc, plan);
        let offset = state.physical_line_address() as usize;
        self.bytes[offset..offset + PRIMARY_DATA_CACHE_LINE_SIZE_BYTES]
            .copy_from_slice(&state.data());
        self.primary_data_cache_writebacks.push(state);
    }

    pub(crate) const fn global_mode_request_state(&self) -> Option<MachineRdramModeState> {
        self.global_mode_request
    }

    pub(crate) fn initialization_complete(&self) -> bool {
        self.modules.iter().enumerate().all(|(index, module)| {
            module.final_device_id_request.is_some()
                && module.mapped_physical_base == index as u32 * RDRAM_MODULE_SIZE_BYTES
                && module.ras_interval.is_some()
                && module
                    .mode
                    .is_some_and(MachineRdramModeState::current_control_enable)
        })
    }

    pub(crate) fn supports_device_id_mapping_word(&self, raw_word: u32) -> bool {
        let requested_base = raw_word >> 6;
        raw_word & 0x3f == 0
            && requested_base.is_multiple_of(RDRAM_MODULE_SIZE_BYTES)
            && requested_base <= self.profile.capacity_bytes() as u32
    }

    pub(crate) const fn broadcast_device_id_request_state(
        &self,
    ) -> Option<MachineRdramBroadcastDeviceIdRequestState> {
        self.broadcast_device_id_request
    }

    pub(crate) fn apply_broadcast_device_id_store(
        &mut self,
        state: MachineRdramBroadcastDeviceIdRequestState,
    ) {
        self.broadcast_device_id_request = Some(state);
        for module in &mut self.modules {
            module.mapped_physical_base = state.requested_physical_base();
        }
        self.active_calibration = None;
    }

    pub(crate) const fn broadcast_delay_state(&self) -> Option<MachineRdramBroadcastDelayState> {
        self.broadcast_delay
    }

    pub(crate) fn apply_broadcast_delay_store(&mut self, state: MachineRdramBroadcastDelayState) {
        self.broadcast_delay = Some(state);
    }

    pub(crate) const fn broadcast_refresh_row_state(
        &self,
    ) -> Option<MachineRdramBroadcastRefreshRowState> {
        self.broadcast_refresh_row
    }

    pub(crate) fn apply_broadcast_refresh_row_store(
        &mut self,
        state: MachineRdramBroadcastRefreshRowState,
    ) {
        self.broadcast_refresh_row = Some(state);
    }

    pub(crate) const fn first_responder_device_id_request_state(
        &self,
    ) -> Option<MachineRdramFirstResponderDeviceIdRequestState> {
        self.first_responder_device_id_request
    }

    pub(crate) fn apply_first_responder_device_id_store(
        &mut self,
        state: MachineRdramFirstResponderDeviceIdRequestState,
    ) {
        self.first_responder_device_id_request = Some(state);
        let selected = self.modules.iter_mut().find(|module| {
            module.mapped_physical_base == RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE
        });
        if let Some(module) = selected {
            module.mapped_physical_base = state.requested_physical_base();
            if self.finalization_started {
                module.final_device_id_request = Some(state);
            } else {
                module.temporary_device_id_request = Some(state);
            }
        }
    }

    pub(crate) const fn initial_mode_request_state(
        &self,
    ) -> Option<MachineRdramInitialModeRequestState> {
        self.initial_mode_request
    }

    pub(crate) fn apply_initial_mode_store(&mut self, state: MachineRdramInitialModeRequestState) {
        self.initial_mode_request = Some(state);
        let source = state.source();
        self.apply_module_mode_store(
            source.physical_address(),
            MachineRdramModeState::new(
                state.raw_word(),
                MachineRdramCpuRegisterWriteSource::new(
                    source.instruction_pc(),
                    source.source_gpr(),
                    source.source_lineage(),
                    source.effective_address(),
                    source.cpu_address(),
                    source.physical_address(),
                ),
            ),
        );
    }

    pub(crate) fn apply_global_mode_store(&mut self, state: MachineRdramModeState) {
        debug_assert!(state.raw_word() == RDRAM_GLOBAL_MODE_X105_WRITE_WORD);
        self.global_mode_request = Some(state);
        self.finalization_started = true;
        self.active_calibration = None;
        for module in &mut self.modules {
            module.mode = Some(state);
        }
    }

    pub(crate) fn apply_module_mode_store(
        &mut self,
        physical_address: u32,
        state: MachineRdramModeState,
    ) {
        let Some((mapped_base, register_offset)) = module_register_aperture(physical_address)
        else {
            return;
        };
        debug_assert!(register_offset == RDRAM_MODE_REGISTER_OFFSET);
        let module_index = self
            .modules
            .iter()
            .position(|module| module.mapped_physical_base == mapped_base);
        let nominal_input = state.nominal_current_control_input();
        self.active_calibration = if state.current_control_enable() {
            None
        } else {
            Some(match module_index {
                Some(index) => MachineRdramCalibrationTarget::Present {
                    module_index: index as u8,
                    probe_base: mapped_base,
                },
                None => MachineRdramCalibrationTarget::Absent {
                    probe_base: mapped_base,
                },
            })
        };
        if let Some(index) = module_index {
            let module = &mut self.modules[index];
            module.mode = Some(state);
            module.calibration_status = if state.current_control_enable() {
                MachineRdramCalibrationStatus::Calibrated {
                    automatic_input: nominal_input,
                }
            } else {
                MachineRdramCalibrationStatus::ManualSearching {
                    nominal_input,
                    score: manual_calibration_score(nominal_input),
                }
            };
        }
    }

    pub(crate) fn apply_module_ras_interval_store(
        &mut self,
        physical_address: u32,
        state: MachineRdramRegisterWordState,
    ) -> bool {
        let Some((mapped_base, register_offset)) = module_register_aperture(physical_address)
        else {
            return false;
        };
        if register_offset != RDRAM_RAS_INTERVAL_REGISTER_OFFSET {
            return false;
        }
        let Some(module) = self
            .modules
            .iter_mut()
            .find(|module| module.mapped_physical_base == mapped_base)
        else {
            return false;
        };
        module.ras_interval = Some(state);
        true
    }

    pub(crate) fn read_module_register_word(&self, physical_address: u32) -> Option<u32> {
        let (mapped_base, register_offset) = module_register_aperture(physical_address)?;
        let module = self
            .modules
            .iter()
            .find(|module| module.mapped_physical_base == mapped_base)?;
        match register_offset {
            RDRAM_DEVICE_TYPE_REGISTER_OFFSET => Some(module.device_type_word),
            RDRAM_MODE_REGISTER_OFFSET => module.mode.map(|mode| {
                if mode.current_control_enable() {
                    let field_mask = encode_mode_current_control_code(0x3f);
                    (mode.raw_word() & !field_mask)
                        | encode_mode_current_control_code(mode.nominal_current_control_input())
                } else {
                    mode.raw_word()
                }
            }),
            RDRAM_RAS_INTERVAL_REGISTER_OFFSET => module
                .ras_interval
                .map(MachineRdramRegisterWordState::raw_word),
            RDRAM_DEVICE_MANUFACTURER_REGISTER_OFFSET => Some(module.manufacturer_word),
            _ => None,
        }
    }

    pub(crate) fn is_module_register_address(physical_address: u32) -> bool {
        module_register_aperture(physical_address).is_some_and(|(_, register_offset)| {
            matches!(
                register_offset,
                RDRAM_DEVICE_TYPE_REGISTER_OFFSET
                    | RDRAM_MODE_REGISTER_OFFSET
                    | RDRAM_RAS_INTERVAL_REGISTER_OFFSET
                    | RDRAM_DEVICE_MANUFACTURER_REGISTER_OFFSET
            )
        })
    }

    pub(crate) fn module_register_offset(physical_address: u32) -> Option<u32> {
        module_register_aperture(physical_address).map(|(_, offset)| offset)
    }

    pub(crate) fn has_module_at_register_address(&self, physical_address: u32) -> bool {
        let Some((mapped_base, _)) = module_register_aperture(physical_address) else {
            return false;
        };
        self.modules
            .iter()
            .any(|module| module.mapped_physical_base == mapped_base)
    }

    pub(crate) fn absent_calibration_store_is_no_effect(
        &self,
        physical_address: u32,
        width: usize,
    ) -> bool {
        matches!(
            self.active_calibration,
            Some(MachineRdramCalibrationTarget::Absent { probe_base })
                if physical_address >= probe_base
                    && physical_address.saturating_add(width as u32) <= probe_base + 8
        )
    }

    pub(crate) fn calibration_read_word(
        &self,
        physical_address: u32,
        backing_word: Option<u32>,
    ) -> Option<u32> {
        let target = self.active_calibration?;
        match target {
            MachineRdramCalibrationTarget::Absent { probe_base }
                if physical_address >= probe_base && physical_address < probe_base + 8 =>
            {
                Some(0)
            }
            MachineRdramCalibrationTarget::Present {
                module_index,
                probe_base,
            } if physical_address >= probe_base && physical_address < probe_base + 8 => {
                let module = self.modules.get(module_index as usize)?;
                let mode = module.mode?;
                if mode.current_control_enable() {
                    return backing_word;
                }
                let set_bits = u32::from((mode.nominal_current_control_input() + 1).min(8));
                let response_byte = ((1_u16 << set_bits) - 1) as u32;
                Some((backing_word.unwrap_or(0) & !0x00ff_0000) | (response_byte << 16))
            }
            _ => backing_word,
        }
    }

    pub fn read_u8(&self, offset: usize) -> Result<u8, RdramAccessError> {
        self.bytes
            .get(offset)
            .copied()
            .ok_or(RdramAccessError { offset, width: 1 })
    }

    pub fn read_u16_be(&self, offset: usize) -> Result<u16, RdramAccessError> {
        self.require_u16_be_offset(offset)?;

        Ok(((self.bytes[offset] as u16) << 8) | self.bytes[offset + 1] as u16)
    }

    pub fn read_u32_be(&self, offset: usize) -> Result<u32, RdramAccessError> {
        self.require_u32_be_offset(offset)?;

        Ok(((self.bytes[offset] as u32) << 24)
            | ((self.bytes[offset + 1] as u32) << 16)
            | ((self.bytes[offset + 2] as u32) << 8)
            | self.bytes[offset + 3] as u32)
    }

    pub fn read_u64_be(&self, offset: usize) -> Result<u64, RdramAccessError> {
        self.require_u64_be_offset(offset)?;

        Ok(((self.bytes[offset] as u64) << 56)
            | ((self.bytes[offset + 1] as u64) << 48)
            | ((self.bytes[offset + 2] as u64) << 40)
            | ((self.bytes[offset + 3] as u64) << 32)
            | ((self.bytes[offset + 4] as u64) << 24)
            | ((self.bytes[offset + 5] as u64) << 16)
            | ((self.bytes[offset + 6] as u64) << 8)
            | self.bytes[offset + 7] as u64)
    }

    pub(crate) fn require_u8_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        self.bytes
            .get(offset)
            .map(|_| ())
            .ok_or(RdramAccessError { offset, width: 1 })
    }

    pub(crate) fn require_u16_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 2 {
            return Err(RdramAccessError { offset, width: 2 });
        }

        Ok(())
    }

    pub(crate) fn require_u32_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 4 {
            return Err(RdramAccessError { offset, width: 4 });
        }

        Ok(())
    }

    pub(crate) fn require_u64_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 8 {
            return Err(RdramAccessError { offset, width: 8 });
        }

        Ok(())
    }

    pub(crate) fn write_u8_at_checked_offset(&mut self, offset: usize, value: u8) {
        self.bytes[offset] = value;
    }

    pub(crate) fn write_u16_be_at_checked_offset(&mut self, offset: usize, value: u16) {
        self.bytes[offset] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 1] = (value & 0xff) as u8;
    }

    pub(crate) fn write_u32_be_at_checked_offset(&mut self, offset: usize, value: u32) {
        self.bytes[offset] = ((value >> 24) & 0xff) as u8;
        self.bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 3] = (value & 0xff) as u8;
    }

    pub(crate) fn write_u64_be_at_checked_offset(&mut self, offset: usize, value: u64) {
        self.bytes[offset] = ((value >> 56) & 0xff) as u8;
        self.bytes[offset + 1] = ((value >> 48) & 0xff) as u8;
        self.bytes[offset + 2] = ((value >> 40) & 0xff) as u8;
        self.bytes[offset + 3] = ((value >> 32) & 0xff) as u8;
        self.bytes[offset + 4] = ((value >> 24) & 0xff) as u8;
        self.bytes[offset + 5] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset + 6] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 7] = (value & 0xff) as u8;
    }
}

impl Default for Rdram {
    fn default() -> Self {
        Self::with_size_bytes(RDRAM_SIZE_BYTES)
            .expect("the current Machine RDRAM capacity selects one fixed profile")
    }
}

const fn manual_calibration_score(nominal_input: u8) -> u8 {
    let incremented = nominal_input.saturating_add(1);
    if incremented < 8 {
        incremented * 10
    } else {
        80
    }
}

const fn module_register_aperture(physical_address: u32) -> Option<(u32, u32)> {
    if physical_address < RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS
        || physical_address >= RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS
    {
        return None;
    }
    let relative = physical_address - RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS;
    let aperture_index = relative / RDRAM_RCP2_REGISTER_SPACING;
    let register_offset = relative % RDRAM_RCP2_REGISTER_SPACING;
    Some((aperture_index << 20, register_offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cpu_register_source(physical_address: u32) -> MachineRdramCpuRegisterWriteSource {
        MachineRdramCpuRegisterWriteSource::new(
            CpuAddress::new(0xa400_0bb8),
            15,
            MachineBootstrapGprSource::ArchitecturalZero,
            u64::from(physical_address | 0xa000_0000),
            CpuAddress::new(physical_address | 0xa000_0000),
            physical_address,
        )
    }

    fn manual_mode_word(nominal_input: u8) -> u32 {
        RDRAM_MODE_DEVICE_ENABLE
            | RDRAM_MODE_AUTO_SKIP
            | RDRAM_MODE_CC_MULT
            | encode_mode_current_control_code(nominal_input ^ 0x3f)
    }

    #[test]
    fn default_rdram_has_cpp_construction_size() {
        let rdram = Rdram::default();

        assert_eq!(rdram.size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(rdram.size_bytes(), 4 * 1024 * 1024);
        assert_eq!(rdram.broadcast_device_id_request_state(), None);
        assert_eq!(rdram.broadcast_delay_state(), None);
        assert_eq!(rdram.broadcast_refresh_row_state(), None);
        assert_eq!(rdram.first_responder_device_id_request_state(), None);
        assert_eq!(rdram.initial_mode_request_state(), None);
    }

    #[test]
    fn fixed_profiles_follow_only_the_owned_capacity_and_reject_other_sizes() {
        let four_mib = Rdram::with_size_bytes(0x0040_0000).unwrap();
        assert_eq!(four_mib.profile_state().capacity_bytes(), 0x0040_0000);
        assert_eq!(four_mib.profile_state().module_count(), 2);
        assert_eq!(four_mib.profile_state().module_size_bytes(), 0x0020_0000);
        assert_eq!(
            four_mib.profile_state().name(),
            RDRAM_STANDARD_RETAIL_4_MIB_PROFILE_NAME
        );
        assert_eq!(four_mib.module_states().len(), 2);

        let eight_mib = Rdram::with_size_bytes(0x0080_0000).unwrap();
        assert_eq!(eight_mib.profile_state().capacity_bytes(), 0x0080_0000);
        assert_eq!(eight_mib.profile_state().module_count(), 4);
        assert_eq!(
            eight_mib.profile_state().name(),
            RDRAM_STANDARD_RETAIL_8_MIB_PROFILE_NAME
        );
        assert_eq!(eight_mib.module_states().len(), 4);

        for module in four_mib
            .module_states()
            .iter()
            .chain(eight_mib.module_states())
        {
            assert_eq!(module.module_size_bytes(), RDRAM_MODULE_SIZE_BYTES);
            assert_eq!(
                module.device_type_word(),
                RDRAM_STANDARD_RETAIL_DEVICE_TYPE_WORD
            );
            assert_eq!(
                module.manufacturer_word(),
                RDRAM_STANDARD_RETAIL_MANUFACTURER_WORD
            );
        }
        assert_eq!(
            Rdram::with_size_bytes(0x0020_0000),
            Err(MachineRdramProfileError::UnsupportedCapacity {
                capacity_bytes: 0x0020_0000,
            })
        );
        assert_eq!(
            Rdram::with_size_bytes(0x0060_0000),
            Err(MachineRdramProfileError::UnsupportedCapacity {
                capacity_bytes: 0x0060_0000,
            })
        );
    }

    #[test]
    fn digital_current_control_response_is_monotonic_local_and_non_mutating() {
        let mut rdram = Rdram::default();
        rdram.modules[0].mapped_physical_base = 0;
        rdram.modules[1].mapped_physical_base = RDRAM_MODULE_SIZE_BYTES;
        rdram.write_u32_be_at_checked_offset(0, 0xa5a5_5a5a);

        let mut scores = Vec::new();
        for nominal_input in 0..=7 {
            let raw_word = manual_mode_word(nominal_input);
            assert!(rdram_mode_generated_family_word(raw_word));
            rdram.apply_module_mode_store(
                RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS + RDRAM_MODE_REGISTER_OFFSET,
                MachineRdramModeState::new(
                    raw_word,
                    test_cpu_register_source(
                        RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS + RDRAM_MODE_REGISTER_OFFSET,
                    ),
                ),
            );
            let response = rdram.calibration_read_word(0, Some(0xa5a5_5a5a)).unwrap();
            let score = (response >> 16) as u8;
            scores.push(score.count_ones() as u8 * 10);
            assert_eq!(rdram.read_u32_be(0), Ok(0xa5a5_5a5a));
            assert_eq!(
                rdram.modules[0].calibration_status(),
                MachineRdramCalibrationStatus::ManualSearching {
                    nominal_input,
                    score: (nominal_input + 1) * 10,
                }
            );
        }
        assert_eq!(scores, [10, 20, 30, 40, 50, 60, 70, 80]);

        let automatic_word = RDRAM_MODE_DEVICE_ENABLE
            | RDRAM_MODE_AUTO_SKIP
            | RDRAM_MODE_CC_MULT
            | RDRAM_MODE_CC_ENABLE
            | encode_mode_current_control_code(7 ^ 0x3f);
        rdram.apply_module_mode_store(
            RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS + RDRAM_MODE_REGISTER_OFFSET,
            MachineRdramModeState::new(
                automatic_word,
                test_cpu_register_source(
                    RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS + RDRAM_MODE_REGISTER_OFFSET,
                ),
            ),
        );
        assert_eq!(rdram.calibration_read_word(0, Some(0xa5a5_5a5a)), None);
        assert_eq!(rdram.read_u32_be(0), Ok(0xa5a5_5a5a));
        assert_eq!(
            rdram.modules[0].calibration_status(),
            MachineRdramCalibrationStatus::Calibrated { automatic_input: 7 }
        );
    }

    #[test]
    fn absent_module_calibration_returns_zero_without_creating_a_module() {
        let mut rdram = Rdram::default();
        let absent_base = rdram.profile_state().capacity_bytes() as u32;
        let module_count = rdram.module_states().len();
        rdram.apply_module_mode_store(
            RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS
                + absent_base / RDRAM_MODULE_SIZE_BYTES * (2 * RDRAM_RCP2_REGISTER_SPACING)
                + RDRAM_MODE_REGISTER_OFFSET,
            MachineRdramModeState::new(
                manual_mode_word(0),
                test_cpu_register_source(
                    RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS
                        + absent_base / RDRAM_MODULE_SIZE_BYTES * (2 * RDRAM_RCP2_REGISTER_SPACING)
                        + RDRAM_MODE_REGISTER_OFFSET,
                ),
            ),
        );

        assert_eq!(rdram.calibration_read_word(absent_base, None), Some(0));
        assert!(rdram.absent_calibration_store_is_no_effect(absent_base, 4));
        assert_eq!(rdram.module_states().len(), module_count);
        assert!(!rdram.has_module_at_register_address(
            RDRAM_REGISTER_BASE_PHYSICAL_ADDRESS
                + absent_base / RDRAM_MODULE_SIZE_BYTES * (2 * RDRAM_RCP2_REGISTER_SPACING)
        ));
    }

    #[test]
    fn generated_device_id_words_include_one_absent_probe_and_nothing_beyond_it() {
        let rdram = Rdram::default();
        assert!(rdram.supports_device_id_mapping_word(0x0000_0000));
        assert!(rdram.supports_device_id_mapping_word(0x0800_0000));
        assert!(rdram.supports_device_id_mapping_word(0x1000_0000));
        assert!(!rdram.supports_device_id_mapping_word(0x1800_0000));
        assert!(!rdram.supports_device_id_mapping_word(0x0000_0001));
    }

    #[test]
    fn default_rdram_storage_is_zero_filled() {
        let rdram = Rdram::default();

        assert!(rdram.bytes.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn byte_read_returns_default_storage_bytes_by_offset_without_mutation() {
        let rdram = Rdram::default();
        let last_offset = RDRAM_SIZE_BYTES - 1;

        assert_eq!(rdram.read_u8(0), Ok(0));
        assert_eq!(rdram.read_u8(last_offset), Ok(0));
        assert_eq!(rdram.size_bytes(), RDRAM_SIZE_BYTES);
        assert!(rdram.bytes.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn byte_read_out_of_range_is_explicit_rust_api_safety() {
        let rdram = Rdram::default();
        let error = rdram.read_u8(RDRAM_SIZE_BYTES).unwrap_err();
        let past_error = rdram.read_u8(RDRAM_SIZE_BYTES + 1).unwrap_err();

        assert_eq!(error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(error.width(), 1);
        assert_eq!(
            error.to_string(),
            "RDRAM access out of range: address=4194304 width=1"
        );
        assert_eq!(past_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_error.width(), 1);
        assert_eq!(
            past_error.to_string(),
            "RDRAM access out of range: address=4194305 width=1"
        );
    }

    #[test]
    fn read_width_out_of_range_errors_carry_raw_storage_offset_and_width() {
        let rdram = Rdram::default();

        let u16_error = rdram.read_u16_be(RDRAM_SIZE_BYTES - 1).unwrap_err();
        let u32_error = rdram.read_u32_be(RDRAM_SIZE_BYTES - 3).unwrap_err();
        let u64_error = rdram.read_u64_be(RDRAM_SIZE_BYTES - 7).unwrap_err();

        assert_eq!(u16_error.offset(), RDRAM_SIZE_BYTES - 1);
        assert_eq!(u16_error.width(), 2);
        assert_eq!(
            u16_error.to_string(),
            "RDRAM access out of range: address=4194303 width=2"
        );
        assert_eq!(u32_error.offset(), RDRAM_SIZE_BYTES - 3);
        assert_eq!(u32_error.width(), 4);
        assert_eq!(
            u32_error.to_string(),
            "RDRAM access out of range: address=4194301 width=4"
        );
        assert_eq!(u64_error.offset(), RDRAM_SIZE_BYTES - 7);
        assert_eq!(u64_error.width(), 8);
        assert_eq!(
            u64_error.to_string(),
            "RDRAM access out of range: address=4194297 width=8"
        );
    }
}
