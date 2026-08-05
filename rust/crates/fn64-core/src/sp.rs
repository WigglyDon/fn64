use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;
use crate::rsp::{
    MachineRspAccumulatorAndFlagsState, MachineRspAddiPlan, MachineRspBranchPlan,
    MachineRspBreakPlan, MachineRspBreakSource, MachineRspControlRegister,
    MachineRspDelaySlotContext, MachineRspExecutionState, MachineRspInstructionIdentity,
    MachineRspInstructionSource, MachineRspLastInstructionState, MachineRspLqvPlan,
    MachineRspLuiPlan, MachineRspMfc0ControlSource, MachineRspMfc0Plan, MachineRspMtc0Plan,
    MachineRspMtc0Source, MachineRspNopPlan, MachineRspOriPlan, MachineRspScalarLwPlan,
    MachineRspScalarRegisterState, MachineRspStepOutcome, MachineRspVectorArithmeticPlan,
    MachineRspVectorRegisterState, MachineRspVectorUnitState, MachineRspXoriPlan,
};

pub const SP_STATUS_PHYSICAL_ADDRESS: u32 = 0x0404_0010;
pub const SP_SEMAPHORE_PHYSICAL_ADDRESS: u32 = 0x0404_001c;
pub const SP_PC_PHYSICAL_ADDRESS: u32 = 0x0408_0000;
pub const SP_MEMORY_ADDRESS_PHYSICAL_ADDRESS: u32 = 0x0404_0000;
pub const SP_DRAM_ADDRESS_PHYSICAL_ADDRESS: u32 = 0x0404_0004;
pub const SP_READ_LENGTH_PHYSICAL_ADDRESS: u32 = 0x0404_0008;
pub const SP_STATUS_X105_HALT_CONFIGURE_WORD: u32 = 0x0000_00ce;
pub const SP_STATUS_X105_START_WORD: u32 = 0x0000_00ad;
pub const SP_STATUS_X105_FINAL_HALT_WORD: u32 = 0x00aa_aaae;
pub const SP_SEMAPHORE_X105_CLEAR_WORD: u32 = 0;
pub const SP_PC_X105_RESET_WORD: u32 = 0;
pub(crate) const SP_STATUS_CLEAR_INTERRUPT_COMMAND: u32 = 1 << 3;
pub(crate) const SP_STATUS_SET_INTERRUPT_COMMAND: u32 = 1 << 4;
const SP_STATUS_DEFINED_COMMAND_MASK: u32 = 0x01ff_ffff;
const SP_DMA_RECORD_CAPACITY: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineSpCpuStoreProvenance {
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
pub enum MachineSpRegisterWriteSource {
    CpuStore(MachineSpCpuStoreProvenance),
    RspMtc0 { source_index: usize },
}

impl MachineSpRegisterWriteSource {
    pub const fn cpu_store(self) -> Option<MachineSpCpuStoreProvenance> {
        match self {
            Self::CpuStore(source) => Some(source),
            Self::RspMtc0 { .. } => None,
        }
    }

    pub const fn rsp_mtc0_source_index(self) -> Option<usize> {
        match self {
            Self::CpuStore(_) => None,
            Self::RspMtc0 { source_index } => Some(source_index),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpStatusState {
    command_word: u32,
    halt: bool,
    broke: bool,
    interrupt_pending: bool,
    single_step: bool,
    interrupt_on_break: bool,
    signals: [bool; 8],
    source: MachineSpRegisterWriteSource,
}

impl MachineSpStatusState {
    pub(crate) fn from_command(
        command_word: u32,
        source: MachineSpCpuStoreProvenance,
        previous: Option<Self>,
    ) -> Option<Self> {
        Self::from_register_command(
            command_word,
            MachineSpRegisterWriteSource::CpuStore(source),
            previous,
        )
    }

    pub(crate) fn from_rsp_mtc0_command(
        command_word: u32,
        source_index: usize,
        previous: Option<Self>,
    ) -> Option<Self> {
        Self::from_register_command(
            command_word,
            MachineSpRegisterWriteSource::RspMtc0 { source_index },
            previous,
        )
    }

    fn from_register_command(
        command_word: u32,
        source: MachineSpRegisterWriteSource,
        previous: Option<Self>,
    ) -> Option<Self> {
        if command_word & !SP_STATUS_DEFINED_COMMAND_MASK != 0
            || command_pair_conflicts(command_word, 0, 1)
            || command_pair_conflicts(command_word, 3, 4)
            || command_pair_conflicts(command_word, 5, 6)
            || command_pair_conflicts(command_word, 7, 8)
            || (0..8)
                .any(|signal| command_pair_conflicts(command_word, 9 + signal * 2, 10 + signal * 2))
        {
            return None;
        }

        let previous = previous.unwrap_or(Self {
            command_word: 0,
            halt: true,
            broke: false,
            interrupt_pending: false,
            single_step: false,
            interrupt_on_break: false,
            signals: [false; 8],
            source,
        });
        let mut signals = previous.signals;
        for (signal, value) in signals.iter_mut().enumerate() {
            *value = apply_command_pair(
                command_word,
                9 + signal as u32 * 2,
                10 + signal as u32 * 2,
                *value,
            );
        }
        Some(Self {
            command_word,
            halt: apply_command_pair(command_word, 0, 1, previous.halt),
            broke: if command_word & (1 << 2) != 0 {
                false
            } else {
                previous.broke
            },
            interrupt_pending: apply_command_pair(command_word, 3, 4, previous.interrupt_pending),
            single_step: apply_command_pair(command_word, 5, 6, previous.single_step),
            interrupt_on_break: apply_command_pair(command_word, 7, 8, previous.interrupt_on_break),
            signals,
            source,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_x105_command(
        command_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        debug_assert!(
            command_word == SP_STATUS_X105_HALT_CONFIGURE_WORD
                || command_word == SP_STATUS_X105_START_WORD
                || command_word == SP_STATUS_X105_FINAL_HALT_WORD
        );
        Self::from_command(command_word, source, None)
            .expect("accepted x105 SP status commands are source-defined")
    }

    pub const fn command_word(self) -> u32 {
        self.command_word
    }

    pub const fn halt(self) -> bool {
        self.halt
    }

    pub const fn broke(self) -> bool {
        self.broke
    }

    pub const fn interrupt_pending(self) -> bool {
        self.interrupt_pending
    }

    pub const fn single_step(self) -> bool {
        self.single_step
    }

    pub const fn interrupt_on_break(self) -> bool {
        self.interrupt_on_break
    }

    pub const fn signals(self) -> [bool; 8] {
        self.signals
    }

    pub const fn source(self) -> MachineSpRegisterWriteSource {
        self.source
    }

    pub const fn read_word(self) -> u32 {
        (self.halt as u32)
            | ((self.broke as u32) << 1)
            | ((self.single_step as u32) << 5)
            | ((self.interrupt_on_break as u32) << 6)
            | ((self.signals[0] as u32) << 7)
            | ((self.signals[1] as u32) << 8)
            | ((self.signals[2] as u32) << 9)
            | ((self.signals[3] as u32) << 10)
            | ((self.signals[4] as u32) << 11)
            | ((self.signals[5] as u32) << 12)
            | ((self.signals[6] as u32) << 13)
            | ((self.signals[7] as u32) << 14)
    }

    fn apply_rsp_break(&mut self) {
        self.halt = true;
        self.broke = true;
    }
}

fn command_pair_conflicts(command_word: u32, clear_bit: u32, set_bit: u32) -> bool {
    command_word & (1 << clear_bit) != 0 && command_word & (1 << set_bit) != 0
}

fn apply_command_pair(command_word: u32, clear_bit: u32, set_bit: u32, previous: bool) -> bool {
    if command_word & (1 << set_bit) != 0 {
        true
    } else if command_word & (1 << clear_bit) != 0 {
        false
    } else {
        previous
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpSemaphoreState {
    set: bool,
    source: MachineSpSemaphoreSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpSemaphorePriorSource {
    SourceDefinedReset,
    CpuStore(MachineSpCpuStoreProvenance),
    RspMfc0ReadAndSet {
        instruction_pc: u16,
        instruction_source: MachineRspInstructionSource,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpSemaphoreSource {
    SourceDefinedReset,
    CpuStore(MachineSpCpuStoreProvenance),
    RspMfc0ReadAndSet {
        instruction_pc: u16,
        instruction_source: MachineRspInstructionSource,
        prior_source: MachineSpSemaphorePriorSource,
        prior_set: bool,
    },
}

impl MachineSpSemaphoreState {
    const fn source_defined_reset() -> Self {
        Self {
            set: false,
            source: MachineSpSemaphoreSource::SourceDefinedReset,
        }
    }

    pub(crate) const fn from_x105_clear(source: MachineSpCpuStoreProvenance) -> Self {
        Self {
            set: false,
            source: MachineSpSemaphoreSource::CpuStore(source),
        }
    }

    pub(crate) const fn from_rsp_mfc0_read_and_set(
        previous: Self,
        instruction_pc: u16,
        instruction_source: MachineRspInstructionSource,
    ) -> Self {
        let prior_source = match previous.source {
            MachineSpSemaphoreSource::SourceDefinedReset => {
                MachineSpSemaphorePriorSource::SourceDefinedReset
            }
            MachineSpSemaphoreSource::CpuStore(source) => {
                MachineSpSemaphorePriorSource::CpuStore(source)
            }
            MachineSpSemaphoreSource::RspMfc0ReadAndSet {
                instruction_pc,
                instruction_source,
                ..
            } => MachineSpSemaphorePriorSource::RspMfc0ReadAndSet {
                instruction_pc,
                instruction_source,
            },
        };
        Self {
            set: true,
            source: MachineSpSemaphoreSource::RspMfc0ReadAndSet {
                instruction_pc,
                instruction_source,
                prior_source,
                prior_set: previous.set,
            },
        }
    }

    pub const fn clear(self) -> bool {
        !self.set
    }

    pub const fn set(self) -> bool {
        self.set
    }

    pub const fn source(self) -> MachineSpSemaphoreSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpPcState {
    raw_low_field: u32,
    source: MachineSpCpuStoreProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspRunStartProvenance {
    source: MachineSpCpuStoreProvenance,
    status_command: u32,
    start_pc: Option<u16>,
}

impl MachineRspRunStartProvenance {
    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }

    pub const fn status_command(self) -> u32 {
        self.status_command
    }

    pub const fn start_pc(self) -> Option<u16> {
        self.start_pc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspRunStartState {
    Pending {
        provenance: MachineRspRunStartProvenance,
    },
    Consumed {
        provenance: MachineRspRunStartProvenance,
        first_rsp_instruction_pc: u16,
        first_rsp_identity: MachineRspInstructionIdentity,
    },
}

impl MachineRspRunStartState {
    pub const fn provenance(self) -> MachineRspRunStartProvenance {
        match self {
            Self::Pending { provenance } | Self::Consumed { provenance, .. } => provenance,
        }
    }

    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Pending { .. })
    }

    pub const fn is_consumed(self) -> bool {
        matches!(self, Self::Consumed { .. })
    }

    pub const fn first_rsp_instruction_pc(self) -> Option<u16> {
        match self {
            Self::Consumed {
                first_rsp_instruction_pc,
                ..
            } => Some(first_rsp_instruction_pc),
            Self::Pending { .. } => None,
        }
    }

    pub const fn first_rsp_identity(self) -> Option<MachineRspInstructionIdentity> {
        match self {
            Self::Consumed {
                first_rsp_identity, ..
            } => Some(first_rsp_identity),
            Self::Pending { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpMemoryAddressState {
    transfer_word: u32,
    local_address: u16,
    source: MachineSpRegisterWriteSource,
}

impl MachineSpMemoryAddressState {
    pub(crate) const fn from_cpu_word(
        transfer_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        Self {
            transfer_word,
            local_address: (transfer_word as u16) & 0x1ff8,
            source: MachineSpRegisterWriteSource::CpuStore(source),
        }
    }

    pub(crate) const fn from_rsp_mtc0_word(transfer_word: u32, source_index: usize) -> Self {
        Self {
            transfer_word,
            local_address: (transfer_word as u16) & 0x1ff8,
            source: MachineSpRegisterWriteSource::RspMtc0 { source_index },
        }
    }

    pub const fn transfer_word(self) -> u32 {
        self.transfer_word
    }

    pub const fn local_address(self) -> u16 {
        self.local_address
    }

    pub const fn source(self) -> MachineSpRegisterWriteSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDramAddressState {
    transfer_word: u32,
    physical_address: u32,
    source: MachineSpDramAddressSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDramAddressSource {
    SourceDefinedReset,
    CpuStore(MachineSpCpuStoreProvenance),
    RspMtc0 {
        source_index: usize,
    },
    DmaAdvance {
        record_index: u8,
        trigger: MachineSpRegisterWriteSource,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmaDirection {
    RdramToSp,
    SpToRdram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmaSpMemory {
    Dmem,
    Imem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmaRecord {
    direction: MachineSpDmaDirection,
    raw_length_word: u32,
    block_length_bytes: u16,
    block_count: u16,
    dram_skip_bytes: u16,
    initial_local_address: u16,
    initial_memory_address_source: MachineSpRegisterWriteSource,
    initial_rdram_address: u32,
    initial_dram_address_source: MachineSpDramAddressSource,
    final_local_address: u16,
    final_rdram_address: u32,
    transferred_byte_count: u32,
    trigger: MachineSpRegisterWriteSource,
}

impl MachineSpDmaRecord {
    const fn new(
        direction: MachineSpDmaDirection,
        raw_length_word: u32,
        memory_address: MachineSpMemoryAddressState,
        dram_address: MachineSpDramAddressState,
        trigger: MachineSpRegisterWriteSource,
    ) -> Self {
        let block_length_bytes = ((raw_length_word & 0x0ff8) + 8) as u16;
        let block_count = (((raw_length_word >> 12) & 0xff) + 1) as u16;
        let dram_skip_bytes = ((raw_length_word >> 20) & 0x0fff) as u16;
        let transferred_byte_count = block_length_bytes as u32 * block_count as u32;
        let initial_local_address = memory_address.local_address();
        let initial_rdram_address = dram_address.physical_address();
        let local_bank = initial_local_address & 0x1000;
        let final_local_address =
            local_bank | ((initial_local_address as u32 + transferred_byte_count) as u16 & 0x0fff);
        let final_rdram_address = initial_rdram_address.wrapping_add(
            (block_length_bytes as u32 + dram_skip_bytes as u32) * block_count as u32,
        );
        Self {
            direction,
            raw_length_word,
            block_length_bytes,
            block_count,
            dram_skip_bytes,
            initial_local_address,
            initial_memory_address_source: memory_address.source(),
            initial_rdram_address,
            initial_dram_address_source: dram_address.source(),
            final_local_address,
            final_rdram_address,
            transferred_byte_count,
            trigger,
        }
    }

    pub(crate) const fn rdram_to_sp(
        raw_length_word: u32,
        memory_address: MachineSpMemoryAddressState,
        dram_address: MachineSpDramAddressState,
        trigger: MachineSpRegisterWriteSource,
    ) -> Self {
        Self::new(
            MachineSpDmaDirection::RdramToSp,
            raw_length_word,
            memory_address,
            dram_address,
            trigger,
        )
    }

    pub(crate) const fn sp_to_rdram(
        raw_length_word: u32,
        memory_address: MachineSpMemoryAddressState,
        dram_address: MachineSpDramAddressState,
        trigger: MachineSpRegisterWriteSource,
    ) -> Self {
        Self::new(
            MachineSpDmaDirection::SpToRdram,
            raw_length_word,
            memory_address,
            dram_address,
            trigger,
        )
    }

    pub const fn direction(self) -> MachineSpDmaDirection {
        self.direction
    }

    pub const fn selected_sp_memory(self) -> MachineSpDmaSpMemory {
        if self.initial_local_address & 0x1000 == 0 {
            MachineSpDmaSpMemory::Dmem
        } else {
            MachineSpDmaSpMemory::Imem
        }
    }

    pub const fn raw_length_word(self) -> u32 {
        self.raw_length_word
    }

    pub const fn block_length_bytes(self) -> u16 {
        self.block_length_bytes
    }

    pub const fn block_count(self) -> u16 {
        self.block_count
    }

    pub const fn dram_skip_bytes(self) -> u16 {
        self.dram_skip_bytes
    }

    pub const fn initial_local_address(self) -> u16 {
        self.initial_local_address
    }

    pub const fn initial_memory_address_source(self) -> MachineSpRegisterWriteSource {
        self.initial_memory_address_source
    }

    pub const fn initial_rdram_address(self) -> u32 {
        self.initial_rdram_address
    }

    pub const fn initial_dram_address_source(self) -> MachineSpDramAddressSource {
        self.initial_dram_address_source
    }

    pub const fn final_local_address(self) -> u16 {
        self.final_local_address
    }

    pub const fn final_rdram_address(self) -> u32 {
        self.final_rdram_address
    }

    pub const fn transferred_byte_count(self) -> u32 {
        self.transferred_byte_count
    }

    pub const fn trigger(self) -> MachineSpRegisterWriteSource {
        self.trigger
    }

    pub(crate) const fn local_address_for_byte(self, byte_index: u32) -> u16 {
        let bank = self.initial_local_address & 0x1000;
        bank | (((self.initial_local_address & 0x0fff) as u32 + byte_index) as u16 & 0x0fff)
    }

    pub(crate) const fn rdram_address_for_byte(self, block_index: u16, byte_in_block: u16) -> u32 {
        self.initial_rdram_address
            + block_index as u32 * (self.block_length_bytes as u32 + self.dram_skip_bytes as u32)
            + byte_in_block as u32
    }
}

impl MachineSpDramAddressState {
    const fn source_defined_reset() -> Self {
        Self {
            transfer_word: 0,
            physical_address: 0,
            source: MachineSpDramAddressSource::SourceDefinedReset,
        }
    }

    pub(crate) const fn from_cpu_word(
        transfer_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        Self {
            transfer_word,
            physical_address: transfer_word & 0x00ff_fff8,
            source: MachineSpDramAddressSource::CpuStore(source),
        }
    }

    pub(crate) const fn from_rsp_mtc0_word(transfer_word: u32, source_index: usize) -> Self {
        Self {
            transfer_word,
            physical_address: transfer_word & 0x00ff_fff8,
            source: MachineSpDramAddressSource::RspMtc0 { source_index },
        }
    }

    pub const fn transfer_word(self) -> u32 {
        self.transfer_word
    }

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn source(self) -> MachineSpDramAddressSource {
        self.source
    }
}

impl MachineSpPcState {
    pub(crate) const fn from_cpu_word(
        transfer_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        Self {
            raw_low_field: transfer_word & 0x0000_0ffc,
            source,
        }
    }

    #[cfg(test)]
    pub(crate) const fn from_x105_zero(source: MachineSpCpuStoreProvenance) -> Self {
        Self::from_cpu_word(SP_PC_X105_RESET_WORD, source)
    }

    pub const fn raw_low_field(self) -> u32 {
        self.raw_low_field
    }

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineSpStatusTransition {
    Unchanged,
    RunStarted,
    Halted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sp {
    status: Option<MachineSpStatusState>,
    pc: Option<MachineSpPcState>,
    semaphore: MachineSpSemaphoreState,
    memory_address: Option<MachineSpMemoryAddressState>,
    dram_address: MachineSpDramAddressState,
    dma_records: [Option<MachineSpDmaRecord>; SP_DMA_RECORD_CAPACITY],
    dma_record_count: u8,
    rsp: MachineRspExecutionState,
    rsp_run_start: Option<MachineRspRunStartState>,
    last_break: Option<MachineRspBreakSource>,
}

impl Sp {
    pub(crate) fn clean_room_ntsc_x105_post_boot() -> Self {
        Self {
            rsp: MachineRspExecutionState::clean_room_ntsc_x105_post_boot(),
            ..Self::default()
        }
    }

    pub(crate) const fn status_state(&self) -> Option<MachineSpStatusState> {
        self.status
    }

    pub(crate) const fn status_word(&self) -> u32 {
        match self.status {
            Some(status) => status.read_word(),
            None => 1,
        }
    }

    pub(crate) const fn pc_state(&self) -> Option<MachineSpPcState> {
        self.pc
    }

    pub(crate) const fn semaphore_state(&self) -> Option<MachineSpSemaphoreState> {
        Some(self.semaphore)
    }

    pub(crate) const fn memory_address_state(&self) -> Option<MachineSpMemoryAddressState> {
        self.memory_address
    }

    pub(crate) const fn dram_address_state(&self) -> Option<MachineSpDramAddressState> {
        Some(self.dram_address)
    }

    pub(crate) const fn last_dma(&self) -> Option<MachineSpDmaRecord> {
        if self.dma_record_count == 0 {
            None
        } else {
            self.dma_records[self.dma_record_count as usize - 1]
        }
    }

    pub(crate) const fn dma_record_count(&self) -> usize {
        self.dma_record_count as usize
    }

    pub(crate) const fn dma_record(&self, index: usize) -> Option<MachineSpDmaRecord> {
        if index < self.dma_record_count as usize {
            self.dma_records[index]
        } else {
            None
        }
    }

    pub(crate) const fn can_record_dma(&self) -> bool {
        (self.dma_record_count as usize) < SP_DMA_RECORD_CAPACITY
    }

    pub(crate) fn rsp_scalar_register(
        &self,
        index: usize,
    ) -> Option<MachineRspScalarRegisterState> {
        self.rsp.scalar_register(index)
    }

    pub(crate) const fn rsp_next_pc(&self) -> Option<u16> {
        self.rsp.next_pc()
    }

    pub(crate) fn rsp_delay_slot_context(&self) -> Option<MachineRspDelaySlotContext> {
        self.rsp.delay_slot_context()
    }

    pub(crate) const fn rsp_committed_instruction_count(&self) -> u64 {
        self.rsp.committed_instruction_count()
    }

    pub(crate) const fn rsp_last_instruction(&self) -> Option<MachineRspLastInstructionState> {
        self.rsp.last_instruction()
    }

    pub(crate) const fn rsp_vector_unit(&self) -> &MachineRspVectorUnitState {
        self.rsp.vector_unit()
    }

    pub(crate) fn rsp_vector_register(
        &self,
        index: usize,
    ) -> Option<&MachineRspVectorRegisterState> {
        self.rsp.vector_register(index)
    }

    pub(crate) fn rsp_accumulator_and_flags(&self) -> MachineRspAccumulatorAndFlagsState {
        self.rsp.accumulator_and_flags()
    }

    pub(crate) const fn rsp_run_start_state(&self) -> Option<MachineRspRunStartState> {
        self.rsp_run_start
    }

    pub(crate) const fn rsp_last_break_source(&self) -> Option<MachineRspBreakSource> {
        self.last_break
    }

    pub(crate) fn rsp_execution(&self) -> &MachineRspExecutionState {
        &self.rsp
    }

    pub(crate) fn rsp_mtc0_source(&self, index: usize) -> Option<&MachineRspMtc0Source> {
        self.rsp.mtc0_source(index)
    }

    pub(crate) fn apply_status_store(
        &mut self,
        state: MachineSpStatusState,
    ) -> MachineSpStatusTransition {
        let previous_halt = self.status.is_none_or(MachineSpStatusState::halt);
        self.status = Some(state);
        if previous_halt && !state.halt() {
            let source = state
                .source()
                .cpu_store()
                .expect("only a CPU status store can make a halted RSP eligible");
            self.rsp_run_start = Some(MachineRspRunStartState::Pending {
                provenance: MachineRspRunStartProvenance {
                    source,
                    status_command: state.command_word(),
                    start_pc: self.pc.map(|pc| pc.raw_low_field() as u16),
                },
            });
            MachineSpStatusTransition::RunStarted
        } else if !previous_halt && state.halt() {
            MachineSpStatusTransition::Halted
        } else {
            MachineSpStatusTransition::Unchanged
        }
    }

    pub(crate) fn apply_pc_store(&mut self, state: MachineSpPcState) {
        self.pc = Some(state);
        self.rsp.synchronize_pc_write(state.raw_low_field() as u16);
    }

    pub(crate) fn apply_semaphore_store(&mut self, state: MachineSpSemaphoreState) {
        self.semaphore = state;
    }

    pub(crate) fn apply_memory_address_store(&mut self, state: MachineSpMemoryAddressState) {
        self.memory_address = Some(state);
    }

    pub(crate) fn apply_dram_address_store(&mut self, state: MachineSpDramAddressState) {
        self.dram_address = state;
    }

    pub(crate) fn apply_dma(&mut self, record: MachineSpDmaRecord) {
        let index = self.dma_record_count as usize;
        debug_assert!(index < SP_DMA_RECORD_CAPACITY);
        self.dma_records[index] = Some(record);
        self.dma_record_count += 1;
        if let Some(mut state) = self.memory_address {
            state.local_address = record.final_local_address();
            self.memory_address = Some(state);
        }
        self.dram_address.transfer_word = record.final_rdram_address() & 0x00ff_fff8;
        self.dram_address.physical_address = record.final_rdram_address() & 0x00ff_fff8;
        self.dram_address.source = MachineSpDramAddressSource::DmaAdvance {
            record_index: index as u8,
            trigger: record.trigger(),
        };
    }

    pub(crate) fn apply_rsp_mfc0(&mut self, plan: MachineRspMfc0Plan) -> MachineRspStepOutcome {
        if let MachineRspMfc0ControlSource::SpSemaphore { .. } = plan.control_source() {
            self.semaphore = MachineSpSemaphoreState::from_rsp_mfc0_read_and_set(
                self.semaphore,
                plan.instruction_pc(),
                plan.instruction_source(),
            );
        }
        let outcome = self.rsp.apply_mfc0(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Mfc0 plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(plan.old_next_pc());
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: plan.instruction_pc(),
                first_rsp_identity: MachineRspInstructionIdentity::Mfc0,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_lqv(&mut self, plan: MachineRspLqvPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_lqv(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Lqv plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Lqv,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_vector_arithmetic(
        &mut self,
        plan: MachineRspVectorArithmeticPlan,
    ) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_vector_arithmetic(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP vector arithmetic plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: outcome.identity(),
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_lw(&mut self, plan: MachineRspScalarLwPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_lw(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP scalar Lw plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Lw,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_nop(&mut self, plan: MachineRspNopPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_nop(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Nop plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Nop,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_break(&mut self, plan: MachineRspBreakPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let source = plan.source();
        let outcome = self.rsp.apply_break(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Break plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        self.status
            .as_mut()
            .expect("selected RSP Break requires represented SP status truth")
            .apply_rsp_break();
        self.last_break = Some(source);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Break,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_mtc0(&mut self, plan: MachineRspMtc0Plan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_mtc0(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Mtc0 plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Mtc0,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_xori(&mut self, plan: MachineRspXoriPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_xori(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Xori plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Xori,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_ori(&mut self, plan: MachineRspOriPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_ori(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Ori plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Ori,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_lui(&mut self, plan: MachineRspLuiPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_lui(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Lui plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Lui,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_addi(&mut self, plan: MachineRspAddiPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let old_next_pc = plan.old_next_pc();
        let outcome = self.rsp.apply_addi(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP Addi plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(old_next_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: MachineRspInstructionIdentity::Addi,
            });
        }
        outcome
    }

    pub(crate) fn apply_rsp_branch(&mut self, plan: MachineRspBranchPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.instruction_pc();
        let delay_slot_pc = plan.delay_slot_pc();
        let outcome = self.rsp.apply_branch(plan);
        let pc = self
            .pc
            .as_mut()
            .expect("RSP branch plan requires one available singular SP PC");
        pc.raw_low_field = u32::from(delay_slot_pc);
        if let Some(MachineRspRunStartState::Pending { provenance }) = self.rsp_run_start {
            self.rsp_run_start = Some(MachineRspRunStartState::Consumed {
                provenance,
                first_rsp_instruction_pc: instruction_pc,
                first_rsp_identity: outcome.identity(),
            });
        }
        outcome
    }

    pub(crate) fn mfc0_control_source(
        &self,
        control_register: MachineRspControlRegister,
    ) -> MachineRspMfc0ControlSource {
        match control_register {
            MachineRspControlRegister::SpSemaphore => MachineRspMfc0ControlSource::SpSemaphore {
                old_set: self.semaphore.set(),
                source: self.semaphore.source(),
            },
            MachineRspControlRegister::SpDramAddress => {
                MachineRspMfc0ControlSource::SpDramAddress {
                    value: self.dram_address.physical_address(),
                    source: self.dram_address.source(),
                }
            }
            MachineRspControlRegister::SpDmaBusy => {
                MachineRspMfc0ControlSource::SpDmaBusy { busy: false }
            }
            MachineRspControlRegister::SpDmaFull => {
                MachineRspMfc0ControlSource::SpDmaFull { full: false }
            }
            MachineRspControlRegister::SpMemoryAddress
            | MachineRspControlRegister::SpReadLength
            | MachineRspControlRegister::SpWriteLength
            | MachineRspControlRegister::SpStatus
            | MachineRspControlRegister::DpcStatus => {
                unreachable!("Mfc0 decoder does not admit write-only packet destinations")
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn clear_rsp_run_start_for_test(&mut self) {
        self.rsp_run_start = None;
    }

    #[cfg(test)]
    pub(crate) fn stage_rsp_delay_for_test(&mut self, owner_pc: u16) {
        self.rsp.stage_delay_for_test(owner_pc);
    }

    #[cfg(test)]
    pub(crate) fn stage_raw_pc_for_test(&mut self, raw_pc: u32) {
        self.pc
            .as_mut()
            .expect("malformed-PC proof requires an existing SP PC state")
            .raw_low_field = raw_pc;
    }

    #[cfg(test)]
    pub(crate) fn stage_broke_for_test(&mut self, broke: bool) {
        self.status
            .as_mut()
            .expect("broke-gate proof requires an existing SP status state")
            .broke = broke;
    }
}

impl Default for Sp {
    fn default() -> Self {
        Self {
            status: None,
            pc: None,
            semaphore: MachineSpSemaphoreState::source_defined_reset(),
            memory_address: None,
            dram_address: MachineSpDramAddressState::source_defined_reset(),
            dma_records: [None; SP_DMA_RECORD_CAPACITY],
            dma_record_count: 0,
            rsp: MachineRspExecutionState::default(),
            rsp_run_start: None,
            last_break: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuInstructionIdentity;

    fn source(pc: u32, physical_address: u32) -> MachineSpCpuStoreProvenance {
        MachineSpCpuStoreProvenance::new(
            CpuAddress::new(pc),
            10,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(pc - 4),
                identity: CpuInstructionIdentity::Addiu,
                source_gpr_a: Some(0),
                source_gpr_b: None,
            },
            u64::from(0xa000_0000 | physical_address),
            CpuAddress::new(0xa000_0000 | physical_address),
            physical_address,
        )
    }

    #[test]
    fn exact_x105_status_commands_derive_only_control_truth() {
        let halted = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_HALT_CONFIGURE_WORD,
            source(0xa400_0490, SP_STATUS_PHYSICAL_ADDRESS),
        );
        assert!(halted.halt());
        assert!(halted.single_step());
        assert!(!halted.broke());
        assert!(!halted.interrupt_pending());
        assert!(!halted.interrupt_on_break());

        let started = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_START_WORD,
            source(0xa400_0508, SP_STATUS_PHYSICAL_ADDRESS),
        );
        assert!(!started.halt());
        assert!(!started.single_step());
        assert!(!started.broke());
        assert!(!started.interrupt_pending());
        assert!(!started.interrupt_on_break());

        let final_halt = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_FINAL_HALT_WORD,
            source(0x8000_01a0, SP_STATUS_PHYSICAL_ADDRESS),
        );
        assert_eq!(final_halt.command_word(), 0x00aa_aaae);
        assert!(final_halt.halt());
        assert!(!final_halt.single_step());
        assert!(!final_halt.broke());
        assert!(!final_halt.interrupt_pending());
        assert!(!final_halt.interrupt_on_break());
        assert_eq!(final_halt.signals(), [false; 8]);
    }

    #[test]
    fn sp_owner_starts_unavailable_and_replaces_exact_states() {
        let mut sp = Sp::default();
        assert_eq!(sp.status_state(), None);
        assert_eq!(sp.pc_state(), None);
        assert!(matches!(
            sp.semaphore_state(),
            Some(state)
                if state.clear()
                    && matches!(
                        state.source(),
                        MachineSpSemaphoreSource::SourceDefinedReset
                    )
        ));
        let status = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_HALT_CONFIGURE_WORD,
            source(0xa400_0490, SP_STATUS_PHYSICAL_ADDRESS),
        );
        let pc = MachineSpPcState::from_x105_zero(source(0xa400_04cc, SP_PC_PHYSICAL_ADDRESS));
        let semaphore = MachineSpSemaphoreState::from_x105_clear(source(
            0x8000_00b0,
            SP_SEMAPHORE_PHYSICAL_ADDRESS,
        ));
        sp.apply_status_store(status);
        sp.apply_pc_store(pc);
        sp.apply_semaphore_store(semaphore);
        assert_eq!(sp.status_state(), Some(status));
        assert_eq!(sp.pc_state(), Some(pc));
        assert_eq!(sp.semaphore_state(), Some(semaphore));
        assert!(semaphore.clear());
        assert!(matches!(
            semaphore.source(),
            MachineSpSemaphoreSource::CpuStore(source)
                if source.physical_address() == 0x0404_001c
        ));
    }

    #[test]
    fn general_status_commands_preserve_untouched_truth_and_reject_conflicts() {
        let source = source(0x800d_5a98, SP_STATUS_PHYSICAL_ADDRESS);
        let initial =
            MachineSpStatusState::from_x105_command(SP_STATUS_X105_HALT_CONFIGURE_WORD, source);
        let configured = MachineSpStatusState::from_command(0x0000_2b00, source, Some(initial))
            .expect("runtime task configuration command is source-defined");
        assert!(configured.halt());
        assert!(configured.single_step());
        assert!(configured.interrupt_on_break());
        assert_eq!(configured.signals(), [false; 8]);

        let started = MachineSpStatusState::from_command(0x0000_0125, source, Some(configured))
            .expect("runtime task start command is source-defined");
        assert!(!started.halt());
        assert!(!started.broke());
        assert!(!started.single_step());
        assert!(started.interrupt_on_break());
        assert_eq!(started.signals(), [false; 8]);

        assert_eq!(
            MachineSpStatusState::from_command(3, source, Some(started)),
            None
        );
        assert_eq!(
            MachineSpStatusState::from_command(0x0200_0000, source, Some(started)),
            None
        );
    }

    #[test]
    fn dma_record_dma_length_dma_count_dma_skip_fields_derive_one_bounded_record_and_advance_owner_state(
    ) {
        let memory = MachineSpMemoryAddressState::from_cpu_word(
            0x0400_1fc7,
            source(0x800d_0600, SP_MEMORY_ADDRESS_PHYSICAL_ADDRESS),
        );
        let dram = MachineSpDramAddressState::from_cpu_word(
            0xff12_bac7,
            source(0x800d_0610, SP_DRAM_ADDRESS_PHYSICAL_ADDRESS),
        );
        assert_eq!(memory.local_address(), 0x1fc0);
        assert_eq!(dram.physical_address(), 0x0012_bac0);

        let record = MachineSpDmaRecord::rdram_to_sp(
            0x0010_1038,
            memory,
            dram,
            MachineSpRegisterWriteSource::CpuStore(source(
                0x800d_0640,
                SP_READ_LENGTH_PHYSICAL_ADDRESS,
            )),
        );
        assert_eq!(record.direction(), MachineSpDmaDirection::RdramToSp);
        assert_eq!(record.block_length_bytes(), 64);
        assert_eq!(record.block_count(), 2);
        assert_eq!(record.dram_skip_bytes(), 1);
        assert_eq!(record.transferred_byte_count(), 128);
        assert_eq!(record.initial_local_address(), 0x1fc0);
        assert_eq!(record.final_local_address(), 0x1040);
        assert_eq!(record.initial_rdram_address(), 0x0012_bac0);
        assert_eq!(record.final_rdram_address(), 0x0012_bb42);

        let mut sp = Sp::default();
        sp.apply_memory_address_store(memory);
        sp.apply_dram_address_store(dram);
        sp.apply_dma(record);
        assert_eq!(sp.dma_record_count(), 1);
        assert_eq!(sp.last_dma(), Some(record));
        assert_eq!(sp.memory_address_state().unwrap().local_address(), 0x1040);
        assert_eq!(
            sp.dram_address_state().unwrap().physical_address(),
            0x0012_bb40
        );
    }
}
