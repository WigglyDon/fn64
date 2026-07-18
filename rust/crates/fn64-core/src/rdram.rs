use core::fmt;

use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;
use crate::mi::MachineMiInitTransferState;

pub const RDRAM_SIZE_BYTES: usize = 4 * 1024 * 1024;
pub const RDRAM_BROADCAST_DEVICE_ID_PHYSICAL_ADDRESS: u32 = 0x03f8_0004;
pub const RDRAM_BROADCAST_DELAY_PHYSICAL_ADDRESS: u32 = 0x03f8_0008;
pub const RDRAM_BROADCAST_REFRESH_ROW_PHYSICAL_ADDRESS: u32 = 0x03f8_0014;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS: u32 = 0x03f0_8004;
pub const RDRAM_DEVICE_ID_X105_CPU_TRANSFER_WORD: u32 = 0x8000_0000;
pub const RDRAM_DEVICE_ID_X105_REQUESTED_PHYSICAL_BASE: u32 = 0x0200_0000;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_WRITE_WORD: u32 = 0x0000_0000;
pub const RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_REQUESTED_INITIAL_DEVICE_ID: u32 = 0x0000_0000;
pub const RDRAM_DELAY_X105_CPU_TRANSFER_WORD: u32 = 0x1808_2838;
pub const RDRAM_DELAY_X105_LOGICAL_CONFIGURATION: u32 = 0x2838_1808;
pub const RDRAM_REF_ROW_X105_WRITE_WORD: u32 = 0x0000_0000;

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
    aperture: MachineRdramFirstResponderDeviceIdAperture,
    source: MachineRdramFirstResponderDeviceIdSource,
}

impl MachineRdramFirstResponderDeviceIdRequestState {
    pub(crate) const fn from_exact_x105_zero_cpu_store(
        source: MachineRdramFirstResponderDeviceIdSource,
    ) -> Self {
        debug_assert!(
            source.physical_address() == RDRAM_FIRST_RESPONDER_DEVICE_ID_PHYSICAL_ADDRESS
        );
        Self {
            raw_cpu_word: RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_WRITE_WORD,
            requested_initial_device_id:
                RDRAM_FIRST_RESPONDER_DEVICE_ID_X105_REQUESTED_INITIAL_DEVICE_ID,
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

    pub const fn aperture(self) -> MachineRdramFirstResponderDeviceIdAperture {
        self.aperture
    }

    pub const fn source(self) -> MachineRdramFirstResponderDeviceIdSource {
        self.source
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

pub struct Rdram {
    bytes: Vec<u8>,
    broadcast_device_id_request: Option<MachineRdramBroadcastDeviceIdRequestState>,
    broadcast_delay: Option<MachineRdramBroadcastDelayState>,
    broadcast_refresh_row: Option<MachineRdramBroadcastRefreshRowState>,
    first_responder_device_id_request: Option<MachineRdramFirstResponderDeviceIdRequestState>,
}

impl Rdram {
    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
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
        Self {
            bytes: vec![0; RDRAM_SIZE_BYTES],
            broadcast_device_id_request: None,
            broadcast_delay: None,
            broadcast_refresh_row: None,
            first_responder_device_id_request: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rdram_has_cpp_construction_size() {
        let rdram = Rdram::default();

        assert_eq!(rdram.size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(rdram.size_bytes(), 4 * 1024 * 1024);
        assert_eq!(rdram.broadcast_device_id_request_state(), None);
        assert_eq!(rdram.broadcast_delay_state(), None);
        assert_eq!(rdram.broadcast_refresh_row_state(), None);
        assert_eq!(rdram.first_responder_device_id_request_state(), None);
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
