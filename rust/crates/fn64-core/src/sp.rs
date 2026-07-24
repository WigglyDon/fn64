use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

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
pub struct MachineSpStatusState {
    command_word: u32,
    halt: bool,
    broke: bool,
    interrupt_pending: bool,
    single_step: bool,
    interrupt_on_break: bool,
    signals: [bool; 8],
    source: MachineSpCpuStoreProvenance,
}

impl MachineSpStatusState {
    pub(crate) fn from_command(
        command_word: u32,
        source: MachineSpCpuStoreProvenance,
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

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
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
    clear: bool,
    source: MachineSpCpuStoreProvenance,
}

impl MachineSpSemaphoreState {
    pub(crate) const fn from_x105_clear(source: MachineSpCpuStoreProvenance) -> Self {
        Self {
            clear: true,
            source,
        }
    }
    pub const fn clear(self) -> bool {
        self.clear
    }
    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpPcState {
    raw_low_field: u32,
    source: MachineSpCpuStoreProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpMemoryAddressState {
    transfer_word: u32,
    local_address: u16,
    source: MachineSpCpuStoreProvenance,
}

impl MachineSpMemoryAddressState {
    pub(crate) const fn from_cpu_word(
        transfer_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        Self {
            transfer_word,
            local_address: (transfer_word as u16) & 0x1ff8,
            source,
        }
    }

    pub const fn transfer_word(self) -> u32 {
        self.transfer_word
    }

    pub const fn local_address(self) -> u16 {
        self.local_address
    }

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDramAddressState {
    transfer_word: u32,
    physical_address: u32,
    source: MachineSpCpuStoreProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmaDirection {
    RdramToSp,
    SpToRdram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmaRecord {
    direction: MachineSpDmaDirection,
    raw_length_word: u32,
    block_length_bytes: u16,
    block_count: u16,
    dram_skip_bytes: u16,
    initial_local_address: u16,
    initial_rdram_address: u32,
    final_local_address: u16,
    final_rdram_address: u32,
    transferred_byte_count: u32,
    trigger: MachineSpCpuStoreProvenance,
}

impl MachineSpDmaRecord {
    pub(crate) const fn rdram_to_sp(
        raw_length_word: u32,
        memory_address: MachineSpMemoryAddressState,
        dram_address: MachineSpDramAddressState,
        trigger: MachineSpCpuStoreProvenance,
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
            direction: MachineSpDmaDirection::RdramToSp,
            raw_length_word,
            block_length_bytes,
            block_count,
            dram_skip_bytes,
            initial_local_address,
            initial_rdram_address,
            final_local_address,
            final_rdram_address,
            transferred_byte_count,
            trigger,
        }
    }

    pub const fn direction(self) -> MachineSpDmaDirection {
        self.direction
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

    pub const fn initial_rdram_address(self) -> u32 {
        self.initial_rdram_address
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

    pub const fn trigger(self) -> MachineSpCpuStoreProvenance {
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
    pub(crate) const fn from_cpu_word(
        transfer_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        Self {
            transfer_word,
            physical_address: transfer_word & 0x00ff_fff8,
            source,
        }
    }

    pub const fn transfer_word(self) -> u32 {
        self.transfer_word
    }

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Sp {
    status: Option<MachineSpStatusState>,
    pc: Option<MachineSpPcState>,
    semaphore: Option<MachineSpSemaphoreState>,
    memory_address: Option<MachineSpMemoryAddressState>,
    dram_address: Option<MachineSpDramAddressState>,
    dma_records: [Option<MachineSpDmaRecord>; SP_DMA_RECORD_CAPACITY],
    dma_record_count: u8,
}

impl Sp {
    pub(crate) const fn status_state(self) -> Option<MachineSpStatusState> {
        self.status
    }

    pub(crate) const fn status_word(self) -> u32 {
        match self.status {
            Some(status) => status.read_word(),
            None => 1,
        }
    }

    pub(crate) const fn pc_state(self) -> Option<MachineSpPcState> {
        self.pc
    }

    pub(crate) const fn semaphore_state(self) -> Option<MachineSpSemaphoreState> {
        self.semaphore
    }

    pub(crate) const fn memory_address_state(self) -> Option<MachineSpMemoryAddressState> {
        self.memory_address
    }

    pub(crate) const fn dram_address_state(self) -> Option<MachineSpDramAddressState> {
        self.dram_address
    }

    pub(crate) const fn last_dma(self) -> Option<MachineSpDmaRecord> {
        if self.dma_record_count == 0 {
            None
        } else {
            self.dma_records[self.dma_record_count as usize - 1]
        }
    }

    pub(crate) const fn dma_record_count(self) -> usize {
        self.dma_record_count as usize
    }

    pub(crate) const fn dma_record(self, index: usize) -> Option<MachineSpDmaRecord> {
        if index < self.dma_record_count as usize {
            self.dma_records[index]
        } else {
            None
        }
    }

    pub(crate) const fn can_record_dma(self) -> bool {
        (self.dma_record_count as usize) < SP_DMA_RECORD_CAPACITY
    }

    pub(crate) fn apply_status_store(&mut self, state: MachineSpStatusState) {
        self.status = Some(state);
    }

    pub(crate) fn apply_pc_store(&mut self, state: MachineSpPcState) {
        self.pc = Some(state);
    }

    pub(crate) fn apply_semaphore_store(&mut self, state: MachineSpSemaphoreState) {
        self.semaphore = Some(state);
    }

    pub(crate) fn apply_memory_address_store(&mut self, state: MachineSpMemoryAddressState) {
        self.memory_address = Some(state);
    }

    pub(crate) fn apply_dram_address_store(&mut self, state: MachineSpDramAddressState) {
        self.dram_address = Some(state);
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
        if let Some(mut state) = self.dram_address {
            state.physical_address = record.final_rdram_address() & 0x00ff_fff8;
            self.dram_address = Some(state);
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
        assert_eq!(sp.semaphore_state(), None);
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
        assert_eq!(semaphore.source().physical_address(), 0x0404_001c);
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
    fn dma_address_and_length_fields_derive_one_bounded_record_and_advance_owner_state() {
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
            source(0x800d_0640, SP_READ_LENGTH_PHYSICAL_ADDRESS),
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
