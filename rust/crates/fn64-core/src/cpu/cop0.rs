use core::fmt;

use super::Cpu;
use super::{MachineCop0TagState, MachineCop0TagWriteProvenance};
use crate::cpu::address::{
    CpuAddress, CpuAddressErrorExceptionEntryError, CpuAddressErrorKind, CpuDataAddressError,
};

const COP0_STATUS_INTERRUPT_ENABLE: u32 = 0x0000_0001;
const COP0_STATUS_EXL: u32 = 0x0000_0002;
const COP0_STATUS_ERL: u32 = 0x0000_0004;
const COP0_STATUS_INTERRUPT_MASK: u32 = 0x0000_ff00;
const COP0_CAUSE_SOFTWARE_INTERRUPT_PENDING_MASK: u32 = 0x0000_0300;
const COP0_CAUSE_RCP_INTERRUPT_PENDING: u32 = 0x0000_0400;
const COP0_CAUSE_TIMER_INTERRUPT_PENDING: u32 = 0x0000_8000;
const COP0_CAUSE_EXCEPTION_CODE_SHIFT: u32 = 2;
const COP0_CAUSE_BRANCH_DELAY: u32 = 0x8000_0000;
const COP0_EXCEPTION_CODE_SIGNED_OVERFLOW: u8 = 12;
const LOCAL_EXCEPTION_VECTOR_PC: u32 = 0x8000_0180;
const LOCAL_EXCEPTION_VECTOR_NEXT_PC: u32 = 0x8000_0184;
const COP0_ENTRY_HI_DEFINED_32_BIT_MASK: u32 = 0xffff_e0ff;
const COP0_ENTRY_LO_DEFINED_32_BIT_MASK: u32 = 0x3fff_ffff;
const COP0_INDEX_DEFINED_32_BIT_MASK: u32 = 0x8000_003f;
const COP0_PAGE_MASK_DEFINED_32_BIT_MASK: u32 = 0x01ff_e000;
const COP0_CONTEXT_PTE_BASE_MASK: u32 = 0xff80_0000;
pub const COP0_TLB_ENTRY_COUNT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop0TlbEntry {
    page_mask: u32,
    entry_hi: u32,
    entry_lo0: u32,
    entry_lo1: u32,
}

impl MachineCop0TlbEntry {
    const fn new(page_mask: u32, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) -> Self {
        Self {
            page_mask,
            entry_hi,
            entry_lo0,
            entry_lo1,
        }
    }

    pub const fn page_mask(self) -> u32 {
        self.page_mask
    }

    pub const fn entry_hi(self) -> u32 {
        self.entry_hi
    }

    pub const fn entry_lo0(self) -> u32 {
        self.entry_lo0
    }

    pub const fn entry_lo1(self) -> u32 {
        self.entry_lo1
    }

    pub const fn global(self) -> bool {
        self.entry_lo0 & 1 != 0 && self.entry_lo1 & 1 != 0
    }

    fn matches(self, entry_hi: u32) -> bool {
        let vpn_comparison_mask = !(self.page_mask | 0x0000_1fff);
        (self.entry_hi & vpn_comparison_mask) == (entry_hi & vpn_comparison_mask)
            && (self.global() || (self.entry_hi as u8) == (entry_hi as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop0TlbOperationError {
    IndexUnavailable,
    IndexOutOfRange { index: u8 },
    EntryUnavailable { index: u8 },
    WorkingRegistersUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuCop0ExceptionReturnError {
    ErrorEpcUnavailable { status: u32 },
}

impl fmt::Display for CpuCop0ExceptionReturnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorEpcUnavailable { status } => write!(
                f,
                "CPU COP0 ERET requires unavailable ErrorEPC while Status.ERL is set: status=0x{status:08X}"
            ),
        }
    }
}

impl std::error::Error for CpuCop0ExceptionReturnError {}

pub(super) struct Cop0 {
    count: u32,
    compare: u32,
    timer_interrupt_pending: bool,
    status: u32,
    software_interrupt_pending: u32,
    software_interrupt_pending_known: bool,
    rcp_interrupt_pending: bool,
    epc: u32,
    bad_vaddr: u32,
    index: Option<u32>,
    random: u8,
    entry_lo0: Option<u32>,
    entry_lo1: Option<u32>,
    context: Option<u32>,
    page_mask: Option<u32>,
    wired: u8,
    entry_hi: u32,
    tlb_entries: [Option<MachineCop0TlbEntry>; COP0_TLB_ENTRY_COUNT],
    exception_code: u8,
    exception_branch_delay: bool,
    tag_lo: Option<MachineCop0TagState>,
    tag_hi: Option<MachineCop0TagState>,
}

impl Cop0 {
    pub(super) fn new() -> Self {
        Self {
            count: 0,
            compare: 0,
            timer_interrupt_pending: false,
            status: 0,
            software_interrupt_pending: 0,
            software_interrupt_pending_known: false,
            rcp_interrupt_pending: false,
            epc: 0,
            bad_vaddr: 0,
            index: None,
            random: (COP0_TLB_ENTRY_COUNT - 1) as u8,
            entry_lo0: None,
            entry_lo1: None,
            context: None,
            page_mask: None,
            wired: 0,
            entry_hi: 0,
            tlb_entries: [None; COP0_TLB_ENTRY_COUNT],
            exception_code: 0,
            exception_branch_delay: false,
            tag_lo: None,
            tag_hi: None,
        }
    }

    fn count(&self) -> u32 {
        self.count
    }

    fn compare(&self) -> u32 {
        self.compare
    }

    fn timer_interrupt_pending(&self) -> bool {
        self.timer_interrupt_pending
    }

    #[allow(dead_code)]
    fn advance_count_for_committed_step(&mut self) {
        self.count = self.count.wrapping_add(1);
        if self.count == self.compare {
            self.timer_interrupt_pending = true;
        }
    }

    fn status(&self) -> u32 {
        self.status
    }

    fn software_interrupt_pending(&self) -> u32 {
        self.software_interrupt_pending
    }

    fn software_interrupt_pending_known(&self) -> bool {
        self.software_interrupt_pending_known
    }

    fn write_cause_software_interrupt_pending(&mut self, value: u32) {
        self.software_interrupt_pending = value & COP0_CAUSE_SOFTWARE_INTERRUPT_PENDING_MASK;
        self.software_interrupt_pending_known = true;
    }

    fn pending_interrupt_word(&self) -> u32 {
        self.software_interrupt_pending
            | if self.rcp_interrupt_pending {
                COP0_CAUSE_RCP_INTERRUPT_PENDING
            } else {
                0
            }
            | if self.timer_interrupt_pending {
                COP0_CAUSE_TIMER_INTERRUPT_PENDING
            } else {
                0
            }
    }

    fn write_count(&mut self, value: u32) {
        self.count = value;
    }

    fn write_compare(&mut self, value: u32) {
        self.compare = value;
        self.timer_interrupt_pending = false;
    }

    fn epc(&self) -> u32 {
        self.epc
    }

    fn bad_vaddr(&self) -> u32 {
        self.bad_vaddr
    }

    fn index(&self) -> Option<u32> {
        self.index
    }

    fn random(&self) -> u8 {
        self.random
    }

    fn entry_lo0(&self) -> Option<u32> {
        self.entry_lo0
    }

    fn entry_lo1(&self) -> Option<u32> {
        self.entry_lo1
    }

    fn context(&self) -> Option<u32> {
        self.context
    }

    fn page_mask(&self) -> Option<u32> {
        self.page_mask
    }

    fn wired(&self) -> u8 {
        self.wired
    }

    fn entry_hi(&self) -> u32 {
        self.entry_hi
    }

    fn write_index(&mut self, value: u32) {
        self.index = Some(value & COP0_INDEX_DEFINED_32_BIT_MASK);
    }

    fn write_entry_lo0(&mut self, value: u32) {
        self.entry_lo0 = Some(value & COP0_ENTRY_LO_DEFINED_32_BIT_MASK);
    }

    fn write_entry_lo1(&mut self, value: u32) {
        self.entry_lo1 = Some(value & COP0_ENTRY_LO_DEFINED_32_BIT_MASK);
    }

    fn write_context(&mut self, value: u32) {
        let previous_bad_vpn2 = self.context.unwrap_or(0) & 0x007f_fff0;
        self.context = Some((value & COP0_CONTEXT_PTE_BASE_MASK) | previous_bad_vpn2);
    }

    fn write_page_mask(&mut self, value: u32) {
        self.page_mask = Some(value & COP0_PAGE_MASK_DEFINED_32_BIT_MASK);
    }

    fn write_wired(&mut self, value: u32) {
        self.wired = (value & 0x3f) as u8;
        self.random = (COP0_TLB_ENTRY_COUNT - 1) as u8;
    }

    fn write_entry_hi(&mut self, value: u32) {
        self.entry_hi = value & COP0_ENTRY_HI_DEFINED_32_BIT_MASK;
    }

    fn advance_random_for_committed_instruction(&mut self) {
        if self.random <= self.wired {
            self.random = (COP0_TLB_ENTRY_COUNT - 1) as u8;
        } else {
            self.random -= 1;
        }
    }

    fn tlb_entry(&self, index: usize) -> Option<MachineCop0TlbEntry> {
        self.tlb_entries.get(index).copied().flatten()
    }

    fn working_tlb_entry(&self) -> Result<MachineCop0TlbEntry, MachineCop0TlbOperationError> {
        let Some(page_mask) = self.page_mask else {
            return Err(MachineCop0TlbOperationError::WorkingRegistersUnavailable);
        };
        let Some(entry_lo0) = self.entry_lo0 else {
            return Err(MachineCop0TlbOperationError::WorkingRegistersUnavailable);
        };
        let Some(entry_lo1) = self.entry_lo1 else {
            return Err(MachineCop0TlbOperationError::WorkingRegistersUnavailable);
        };

        Ok(MachineCop0TlbEntry::new(
            page_mask,
            self.entry_hi,
            entry_lo0,
            entry_lo1,
        ))
    }

    fn indexed_tlb_slot(&self) -> Result<usize, MachineCop0TlbOperationError> {
        let Some(index_word) = self.index else {
            return Err(MachineCop0TlbOperationError::IndexUnavailable);
        };
        let index = (index_word & 0x3f) as u8;
        if usize::from(index) >= COP0_TLB_ENTRY_COUNT {
            return Err(MachineCop0TlbOperationError::IndexOutOfRange { index });
        }
        Ok(usize::from(index))
    }

    fn write_tlb_indexed(&mut self) -> Result<u8, MachineCop0TlbOperationError> {
        let index = self.indexed_tlb_slot()?;
        let entry = self.working_tlb_entry()?;
        self.tlb_entries[index] = Some(entry);
        Ok(index as u8)
    }

    fn write_tlb_random(&mut self) -> Result<u8, MachineCop0TlbOperationError> {
        let index = usize::from(self.random);
        let entry = self.working_tlb_entry()?;
        self.tlb_entries[index] = Some(entry);
        Ok(index as u8)
    }

    fn read_tlb_indexed(&mut self) -> Result<u8, MachineCop0TlbOperationError> {
        let index = self.indexed_tlb_slot()?;
        let Some(entry) = self.tlb_entries[index] else {
            return Err(MachineCop0TlbOperationError::EntryUnavailable { index: index as u8 });
        };
        self.page_mask = Some(entry.page_mask());
        self.entry_hi = entry.entry_hi();
        self.entry_lo0 = Some(entry.entry_lo0());
        self.entry_lo1 = Some(entry.entry_lo1());
        Ok(index as u8)
    }

    fn probe_tlb(&mut self) -> Option<u8> {
        let match_index = self
            .tlb_entries
            .iter()
            .position(|entry| entry.is_some_and(|entry| entry.matches(self.entry_hi)));
        self.index = Some(match match_index {
            Some(index) => index as u32,
            None => 0x8000_0000,
        });
        match_index.map(|index| index as u8)
    }

    fn exception_code(&self) -> u8 {
        self.exception_code
    }

    fn exception_branch_delay(&self) -> bool {
        self.exception_branch_delay
    }

    fn tag_lo(&self) -> Option<MachineCop0TagState> {
        self.tag_lo
    }

    fn tag_hi(&self) -> Option<MachineCop0TagState> {
        self.tag_hi
    }

    fn write_tag_lo(&mut self, value: u32, provenance: MachineCop0TagWriteProvenance) {
        self.tag_lo = Some(MachineCop0TagState::new(value, provenance));
    }

    fn write_tag_hi(&mut self, value: u32, provenance: MachineCop0TagWriteProvenance) {
        self.tag_hi = Some(MachineCop0TagState::new(value, provenance));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuArithmeticOverflowExceptionEntryError {
    pc: CpuAddress,
    next_pc: CpuAddress,
    status: u32,
}

impl CpuArithmeticOverflowExceptionEntryError {
    pub(crate) const fn new(pc: CpuAddress, next_pc: CpuAddress, status: u32) -> Self {
        Self {
            pc,
            next_pc,
            status,
        }
    }

    pub(crate) const fn pc(self) -> CpuAddress {
        self.pc
    }

    pub(crate) const fn next_pc(self) -> CpuAddress {
        self.next_pc
    }

    pub(crate) const fn status(self) -> u32 {
        self.status
    }
}

impl fmt::Display for CpuArithmeticOverflowExceptionEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU arithmetic overflow exception entry blocked: pc={} next_pc={} status={}",
            self.pc.value(),
            self.next_pc.value(),
            self.status
        )
    }
}

impl std::error::Error for CpuArithmeticOverflowExceptionEntryError {}

impl Cpu {
    pub fn cop0_count(&self) -> u32 {
        self.cop0.count()
    }

    pub fn cop0_compare(&self) -> u32 {
        self.cop0.compare()
    }

    pub fn cop0_timer_interrupt_pending(&self) -> bool {
        self.cop0.timer_interrupt_pending()
    }

    pub fn cop0_status(&self) -> u32 {
        self.cop0.status()
    }

    pub fn cop0_software_interrupt_pending(&self) -> u32 {
        self.cop0.software_interrupt_pending()
    }

    pub fn cop0_software_interrupt_pending_known(&self) -> bool {
        self.cop0.software_interrupt_pending_known()
    }

    pub fn cop0_rcp_interrupt_pending(&self) -> bool {
        self.cop0.rcp_interrupt_pending
    }

    pub fn cop0_pending_interrupt_word(&self) -> u32 {
        self.cop0.pending_interrupt_word()
    }

    pub fn cop0_cause_word(&self) -> Option<u32> {
        if !self.cop0.software_interrupt_pending_known() {
            return None;
        }

        Some(
            self.cop0.pending_interrupt_word()
                | (u32::from(self.cop0.exception_code()) << COP0_CAUSE_EXCEPTION_CODE_SHIFT)
                | if self.cop0.exception_branch_delay() {
                    COP0_CAUSE_BRANCH_DELAY
                } else {
                    0
                },
        )
    }

    pub fn cop0_epc(&self) -> u32 {
        self.cop0.epc()
    }

    pub fn cop0_bad_vaddr(&self) -> u32 {
        self.cop0.bad_vaddr()
    }

    pub fn cop0_index(&self) -> Option<u32> {
        self.cop0.index()
    }

    pub fn cop0_random(&self) -> u8 {
        self.cop0.random()
    }

    pub fn cop0_entry_lo0(&self) -> Option<u32> {
        self.cop0.entry_lo0()
    }

    pub fn cop0_entry_lo1(&self) -> Option<u32> {
        self.cop0.entry_lo1()
    }

    pub fn cop0_context(&self) -> Option<u32> {
        self.cop0.context()
    }

    pub fn cop0_page_mask(&self) -> Option<u32> {
        self.cop0.page_mask()
    }

    pub fn cop0_wired(&self) -> u8 {
        self.cop0.wired()
    }

    pub fn cop0_entry_hi(&self) -> u32 {
        self.cop0.entry_hi()
    }

    pub fn cop0_tlb_entry(&self, index: usize) -> Option<MachineCop0TlbEntry> {
        self.cop0.tlb_entry(index)
    }

    pub fn cop0_exception_code(&self) -> u8 {
        self.cop0.exception_code()
    }

    pub fn cop0_exception_branch_delay(&self) -> bool {
        self.cop0.exception_branch_delay()
    }

    pub fn cop0_tag_lo(&self) -> Option<MachineCop0TagState> {
        self.cop0.tag_lo()
    }

    pub fn cop0_tag_hi(&self) -> Option<MachineCop0TagState> {
        self.cop0.tag_hi()
    }

    pub(crate) fn stage_cop0_status_for_bootstrap(&mut self, status: u32) {
        self.cop0.status = status;
    }

    pub(crate) fn stage_public_synthetic_cold_x105_page_mask(&mut self) {
        self.cop0.page_mask = Some(0);
    }

    #[allow(dead_code)]
    pub(crate) fn advance_count_for_committed_step(&mut self) {
        self.cop0.advance_count_for_committed_step();
        self.cop0.advance_random_for_committed_instruction();
    }

    pub(crate) fn write_cop0_cause_software_interrupt_pending(&mut self, value: u32) {
        self.cop0.write_cause_software_interrupt_pending(value);
    }

    pub(crate) fn set_cop0_rcp_interrupt_pending(&mut self, pending: bool) {
        self.cop0.rcp_interrupt_pending = pending;
    }

    pub(crate) fn cop0_interrupt_should_enter(&self) -> bool {
        let status = self.cop0.status;
        (status & COP0_STATUS_INTERRUPT_ENABLE) != 0
            && (status & (COP0_STATUS_EXL | COP0_STATUS_ERL)) == 0
            && (status & COP0_STATUS_INTERRUPT_MASK & self.cop0.pending_interrupt_word()) != 0
    }

    pub(crate) fn write_cop0_status(&mut self, value: u32) {
        self.cop0.status = value;
    }

    pub(crate) fn write_cop0_count(&mut self, value: u32) {
        self.cop0.write_count(value);
    }

    pub(crate) fn write_cop0_compare(&mut self, value: u32) {
        self.cop0.write_compare(value);
    }

    pub(crate) fn write_cop0_epc(&mut self, value: u32) {
        self.cop0.epc = value;
    }

    pub(crate) fn write_cop0_index(&mut self, value: u32) {
        self.cop0.write_index(value);
    }

    pub(crate) fn write_cop0_entry_lo0(&mut self, value: u32) {
        self.cop0.write_entry_lo0(value);
    }

    pub(crate) fn write_cop0_entry_lo1(&mut self, value: u32) {
        self.cop0.write_entry_lo1(value);
    }

    pub(crate) fn write_cop0_context(&mut self, value: u32) {
        self.cop0.write_context(value);
    }

    pub(crate) fn write_cop0_page_mask(&mut self, value: u32) {
        self.cop0.write_page_mask(value);
    }

    pub(crate) fn write_cop0_wired(&mut self, value: u32) {
        self.cop0.write_wired(value);
    }

    pub(crate) fn write_cop0_entry_hi(&mut self, value: u32) {
        self.cop0.write_entry_hi(value);
    }

    pub(crate) fn execute_cop0_tlb_read(&mut self) -> Result<u8, MachineCop0TlbOperationError> {
        self.cop0.read_tlb_indexed()
    }

    pub(crate) fn execute_cop0_tlb_write_indexed(
        &mut self,
    ) -> Result<u8, MachineCop0TlbOperationError> {
        self.cop0.write_tlb_indexed()
    }

    pub(crate) fn execute_cop0_tlb_write_random(
        &mut self,
    ) -> Result<u8, MachineCop0TlbOperationError> {
        self.cop0.write_tlb_random()
    }

    pub(crate) fn execute_cop0_tlb_probe(&mut self) -> Option<u8> {
        self.cop0.probe_tlb()
    }

    pub(crate) fn execute_cop0_exception_return(
        &mut self,
    ) -> Result<(), CpuCop0ExceptionReturnError> {
        let status = self.cop0.status;
        if (status & COP0_STATUS_ERL) != 0 {
            return Err(CpuCop0ExceptionReturnError::ErrorEpcUnavailable { status });
        }

        self.cop0.status &= !COP0_STATUS_EXL;
        self.stage_pc(self.cop0.epc);
        Ok(())
    }

    pub(crate) fn write_cop0_tag_lo(
        &mut self,
        value: u32,
        provenance: MachineCop0TagWriteProvenance,
    ) {
        self.cop0.write_tag_lo(value, provenance);
    }

    pub(crate) fn write_cop0_tag_hi(
        &mut self,
        value: u32,
        provenance: MachineCop0TagWriteProvenance,
    ) {
        self.cop0.write_tag_hi(value, provenance);
    }

    #[cfg(test)]
    pub(crate) fn stage_cop0_count_compare_timer_for_test(
        &mut self,
        count: u32,
        compare: u32,
        timer_interrupt_pending: bool,
    ) {
        self.cop0.count = count;
        self.cop0.compare = compare;
        self.cop0.timer_interrupt_pending = timer_interrupt_pending;
    }

    #[cfg(test)]
    pub(crate) fn stage_cop0_bad_vaddr_for_test(&mut self, bad_vaddr: u32) {
        self.cop0.bad_vaddr = bad_vaddr;
    }

    #[cfg(test)]
    pub(crate) fn stage_cop0_epc_for_test(&mut self, epc: u32) {
        self.cop0.epc = epc;
    }

    #[cfg(test)]
    pub(crate) fn stage_cop0_cause_state_for_test(
        &mut self,
        software_interrupt_pending: u32,
        software_interrupt_pending_known: bool,
        exception_code: u8,
        exception_branch_delay: bool,
    ) {
        self.cop0.software_interrupt_pending =
            software_interrupt_pending & COP0_CAUSE_SOFTWARE_INTERRUPT_PENDING_MASK;
        self.cop0.software_interrupt_pending_known = software_interrupt_pending_known;
        self.cop0.exception_code = exception_code;
        self.cop0.exception_branch_delay = exception_branch_delay;
    }

    pub fn enter_data_address_error_exception(
        &mut self,
        address_error: CpuDataAddressError,
    ) -> Result<(), CpuAddressErrorExceptionEntryError> {
        let Some((epc, branch_delay)) = self.local_synchronous_exception_lineage() else {
            return Err(CpuAddressErrorExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        };

        self.cop0.epc = epc;
        self.cop0.bad_vaddr = address_error.bad_vaddr().value();
        self.cop0.exception_code = address_error.cause_exception_code();
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;
        self.clear_delay_slot_context();

        Ok(())
    }

    pub(crate) fn enter_instruction_fetch_address_error_exception(
        &mut self,
        bad_vaddr: CpuAddress,
    ) -> Result<(), CpuAddressErrorExceptionEntryError> {
        let Some((epc, branch_delay)) = self.local_synchronous_exception_lineage() else {
            return Err(CpuAddressErrorExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        };

        self.cop0.epc = epc;
        self.cop0.bad_vaddr = bad_vaddr.value();
        self.cop0.exception_code = CpuAddressErrorKind::AddressErrorLoad.cause_exception_code();
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;
        self.clear_delay_slot_context();

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn enter_arithmetic_overflow_exception(
        &mut self,
    ) -> Result<(), CpuArithmeticOverflowExceptionEntryError> {
        let Some((epc, branch_delay)) = self.local_synchronous_exception_lineage() else {
            return Err(CpuArithmeticOverflowExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        };

        self.cop0.epc = epc;
        self.cop0.exception_code = COP0_EXCEPTION_CODE_SIGNED_OVERFLOW;
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;
        self.clear_delay_slot_context();

        Ok(())
    }

    pub(crate) fn enter_interrupt_exception(&mut self) {
        debug_assert!(self.cop0_interrupt_should_enter());
        let (epc, branch_delay) = match self.delay_slot_context() {
            Some(context) => (context.branch_or_jump_pc(), true),
            None => (self.pc, false),
        };
        self.cop0.epc = epc;
        self.cop0.exception_code = 0;
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;
        self.clear_delay_slot_context();
    }

    fn local_synchronous_exception_lineage(&self) -> Option<(u32, bool)> {
        if (self.cop0.status & COP0_STATUS_EXL) != 0 {
            return None;
        }
        match self.delay_slot_context() {
            Some(context) => Some((context.branch_or_jump_pc(), true)),
            None if self.next_pc == self.pc.wrapping_add(4) => Some((self.pc, false)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CpuCop0ExceptionReturnError, MachineCop0TlbOperationError,
        COP0_EXCEPTION_CODE_SIGNED_OVERFLOW, COP0_STATUS_ERL, COP0_STATUS_EXL,
        COP0_STATUS_INTERRUPT_ENABLE, COP0_STATUS_INTERRUPT_MASK, LOCAL_EXCEPTION_VECTOR_NEXT_PC,
        LOCAL_EXCEPTION_VECTOR_PC,
    };
    use crate::cpu::address::{
        check_cpu_data_alignment, select_cpu_data_address_error, CpuAddress, CpuAddressErrorKind,
        CpuDataAccessKind, CpuDataWidth,
    };
    use crate::cpu::Cpu;

    fn data_address_error(
        access_kind: CpuDataAccessKind,
        address: u32,
        width: CpuDataWidth,
    ) -> crate::cpu::address::CpuDataAddressError {
        let alignment_error =
            check_cpu_data_alignment(access_kind, CpuAddress::new(address), width).unwrap_err();
        select_cpu_data_address_error(alignment_error)
    }

    #[test]
    fn new_cpu_zeroes_cpp_cop0_construction_state() {
        let cpu = Cpu::new();

        assert_eq!(cpu.cop0_count(), 0);
        assert_eq!(cpu.cop0_compare(), 0);
        assert!(!cpu.cop0_timer_interrupt_pending());
        assert_eq!(cpu.cop0_status(), 0);
        assert_eq!(cpu.cop0_software_interrupt_pending(), 0);
        assert!(!cpu.cop0_software_interrupt_pending_known());
        assert_eq!(cpu.cop0_epc(), 0);
        assert_eq!(cpu.cop0_bad_vaddr(), 0);
        assert_eq!(cpu.cop0_entry_hi(), 0);
        assert_eq!(cpu.cop0_exception_code(), 0);
        assert!(!cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_tag_lo(), None);
        assert_eq!(cpu.cop0_tag_hi(), None);
    }

    #[test]
    fn cop0_construction_access_does_not_change_earned_cpu_state() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);

        let observed = (
            cpu.cop0_count(),
            cpu.cop0_compare(),
            cpu.cop0_timer_interrupt_pending(),
            cpu.cop0_status(),
            cpu.cop0_software_interrupt_pending(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        assert_eq!(observed, (0, 0, false, 0, 0, 0, 0, 0, false));
        assert_eq!(cpu.pc(), 0x8000_1000);
        assert_eq!(cpu.next_pc(), 0x8000_2000);
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn count_advance_for_committed_step_increments_and_latches_timer_after_increment() {
        let mut cpu = Cpu::new();
        cpu.cop0.count = 0x0000_0010;
        cpu.cop0.compare = 0x0000_0011;

        cpu.advance_count_for_committed_step();

        assert_eq!(cpu.cop0_count(), 0x0000_0011);
        assert!(cpu.cop0_timer_interrupt_pending());
    }

    #[test]
    fn count_advance_for_committed_step_wraps_like_cpp_unsigned_count() {
        let mut cpu = Cpu::new();
        cpu.cop0.count = 0xffff_ffff;
        cpu.cop0.compare = 0x0000_0000;

        cpu.advance_count_for_committed_step();

        assert_eq!(cpu.cop0_count(), 0x0000_0000);
        assert!(cpu.cop0_timer_interrupt_pending());
    }

    #[test]
    fn count_advance_for_committed_step_preserves_timer_pending_when_not_matching_compare() {
        let mut cpu = Cpu::new();
        cpu.cop0.count = 0x0000_0007;
        cpu.cop0.compare = 0x0000_0009;

        cpu.advance_count_for_committed_step();

        assert_eq!(cpu.cop0_count(), 0x0000_0008);
        assert!(!cpu.cop0_timer_interrupt_pending());

        cpu.cop0.count = 0x0000_0020;
        cpu.cop0.compare = 0x0000_0040;
        cpu.cop0.timer_interrupt_pending = true;

        cpu.advance_count_for_committed_step();

        assert_eq!(cpu.cop0_count(), 0x0000_0021);
        assert!(cpu.cop0_timer_interrupt_pending());
    }

    #[test]
    fn count_advance_for_committed_step_mutates_only_count_and_timer_pending() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_1004);
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.cop0.count = 0x0000_00ff;
        cpu.cop0.compare = 0x0000_0100;
        cpu.cop0.status = 0x0000_0002;
        cpu.cop0.software_interrupt_pending = 0x0000_0100;
        cpu.cop0.epc = 0x8000_2000;
        cpu.cop0.bad_vaddr = 0x8000_3000;
        cpu.cop0.exception_code = 4;
        cpu.cop0.exception_branch_delay = true;

        let scalar_before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.hi(),
            cpu.lo(),
            cpu.gpr(0),
            cpu.gpr(8),
        );
        let cop0_before = (
            cpu.cop0_compare(),
            cpu.cop0_status(),
            cpu.cop0_software_interrupt_pending(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        cpu.advance_count_for_committed_step();

        assert_eq!(cpu.cop0_count(), 0x0000_0100);
        assert!(cpu.cop0_timer_interrupt_pending());
        assert_eq!(
            scalar_before,
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.hi(),
                cpu.lo(),
                cpu.gpr(0),
                cpu.gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                cpu.cop0_compare(),
                cpu.cop0_status(),
                cpu.cop0_software_interrupt_pending(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn arithmetic_overflow_exception_enters_local_exception_without_bad_vaddr_or_gpr_mutation() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_1004);
        cpu.cop0.count = 0x0000_0040;
        cpu.cop0.compare = 0x0000_0050;
        cpu.cop0.bad_vaddr = 0x8000_3000;
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        assert_eq!(cpu.enter_arithmetic_overflow_exception(), Ok(()));

        assert_eq!(
            cpu.cop0_exception_code(),
            COP0_EXCEPTION_CODE_SIGNED_OVERFLOW
        );
        assert!(!cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1000);
        assert_eq!(cpu.cop0_status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(cpu.cop0_count(), 0x0000_0040);
        assert_eq!(cpu.cop0_compare(), 0x0000_0050);
        assert_eq!(cpu.cop0_bad_vaddr(), 0x8000_3000);
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn arithmetic_overflow_delay_slot_entry_sets_branch_delay_epc() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1204);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_delay_slot_context_for_test(0x8000_1200);

        assert_eq!(cpu.enter_arithmetic_overflow_exception(), Ok(()));

        assert_eq!(
            cpu.cop0_exception_code(),
            COP0_EXCEPTION_CODE_SIGNED_OVERFLOW
        );
        assert!(cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1200);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(cpu.delay_slot_context(), None);
    }

    #[test]
    fn arithmetic_overflow_exception_entry_blocks_when_exl_is_already_set() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1300);
        cpu.stage_next_pc(0x8000_1304);

        assert_eq!(cpu.enter_arithmetic_overflow_exception(), Ok(()));

        let before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.cop0_status(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        let error = cpu.enter_arithmetic_overflow_exception().unwrap_err();

        assert_eq!(error.pc(), CpuAddress::new(LOCAL_EXCEPTION_VECTOR_PC));
        assert_eq!(
            error.next_pc(),
            CpuAddress::new(LOCAL_EXCEPTION_VECTOR_NEXT_PC)
        );
        assert_eq!(error.status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.cop0_status(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            ),
            before
        );
    }

    #[test]
    fn arithmetic_overflow_exception_entry_blocks_nonsequential_flow_without_explicit_context() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x0000_0002);
        cpu.stage_next_pc(0x8000_2000);
        cpu.cop0.bad_vaddr = 0x8000_4000;

        let before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.cop0_status(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        let error = cpu.enter_arithmetic_overflow_exception().unwrap_err();

        assert_eq!(error.pc(), CpuAddress::new(0x0000_0002));
        assert_eq!(error.next_pc(), CpuAddress::new(0x8000_2000));
        assert_eq!(error.status(), 0);
        assert_eq!(
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.cop0_status(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            ),
            before
        );
    }

    #[test]
    fn instruction_fetch_adel_explicit_delay_slot_entry_uses_owner_epc_and_clears_context() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1206);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_delay_slot_context_for_test(0x8000_1200);

        assert_eq!(
            cpu.enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1206)),
            Ok(())
        );

        assert_eq!(cpu.cop0_bad_vaddr(), 0x8000_1206);
        assert_eq!(cpu.cop0_exception_code(), 4);
        assert!(cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1200);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(cpu.delay_slot_context(), None);
    }

    #[test]
    fn data_address_error_load_enters_local_exception_without_gpr_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_pc(0x8000_1000);

        let address_error =
            data_address_error(CpuDataAccessKind::Read, 0x8000_2001, CpuDataWidth::Halfword);
        assert_eq!(
            address_error.exception_kind(),
            CpuAddressErrorKind::AddressErrorLoad
        );

        assert_eq!(
            cpu.enter_data_address_error_exception(address_error),
            Ok(())
        );

        assert_eq!(cpu.cop0_bad_vaddr(), 0x8000_2001);
        assert_eq!(cpu.cop0_exception_code(), 4);
        assert!(!cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1000);
        assert_eq!(cpu.cop0_status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn data_address_error_store_enters_local_exception_with_store_code() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1100);

        let address_error =
            data_address_error(CpuDataAccessKind::Write, 0x8000_3003, CpuDataWidth::Word);
        assert_eq!(
            address_error.exception_kind(),
            CpuAddressErrorKind::AddressErrorStore
        );

        assert_eq!(
            cpu.enter_data_address_error_exception(address_error),
            Ok(())
        );

        assert_eq!(cpu.cop0_bad_vaddr(), 0x8000_3003);
        assert_eq!(cpu.cop0_exception_code(), 5);
        assert!(!cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1100);
        assert_eq!(cpu.cop0_status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
    }

    #[test]
    fn data_address_error_delay_slot_entry_sets_branch_delay_epc() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1204);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_delay_slot_context_for_test(0x8000_1200);

        let address_error = data_address_error(
            CpuDataAccessKind::Read,
            0x8000_4007,
            CpuDataWidth::Doubleword,
        );

        assert_eq!(
            cpu.enter_data_address_error_exception(address_error),
            Ok(())
        );

        assert_eq!(cpu.cop0_bad_vaddr(), 0x8000_4007);
        assert_eq!(cpu.cop0_exception_code(), 4);
        assert!(cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1200);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(cpu.delay_slot_context(), None);
    }

    #[test]
    fn data_address_error_entry_blocks_when_exl_is_already_set() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1300);

        let first_error =
            data_address_error(CpuDataAccessKind::Read, 0x8000_5001, CpuDataWidth::Halfword);
        assert_eq!(cpu.enter_data_address_error_exception(first_error), Ok(()));

        let blocked_error =
            data_address_error(CpuDataAccessKind::Write, 0x8000_6003, CpuDataWidth::Word);
        let before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.cop0_status(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        let error = cpu
            .enter_data_address_error_exception(blocked_error)
            .unwrap_err();

        assert_eq!(error.pc(), CpuAddress::new(LOCAL_EXCEPTION_VECTOR_PC));
        assert_eq!(
            error.next_pc(),
            CpuAddress::new(LOCAL_EXCEPTION_VECTOR_NEXT_PC)
        );
        assert_eq!(error.status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.cop0_status(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            ),
            before
        );
    }

    #[test]
    fn data_address_error_entry_blocks_nonsequential_flow_without_explicit_context() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x0000_0002);
        cpu.stage_next_pc(0x8000_2000);

        let address_error =
            data_address_error(CpuDataAccessKind::Read, 0x8000_7001, CpuDataWidth::Halfword);
        let before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.cop0_status(),
            cpu.cop0_epc(),
            cpu.cop0_bad_vaddr(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        );

        let error = cpu
            .enter_data_address_error_exception(address_error)
            .unwrap_err();

        assert_eq!(error.pc(), CpuAddress::new(0x0000_0002));
        assert_eq!(error.next_pc(), CpuAddress::new(0x8000_2000));
        assert_eq!(error.status(), 0);
        assert_eq!(
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.cop0_status(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            ),
            before
        );
    }

    #[test]
    fn indexed_tlb_write_read_and_probe_preserve_masked_machine_owned_truth() {
        let mut cpu = Cpu::new();
        assert_eq!(
            cpu.execute_cop0_tlb_write_indexed(),
            Err(MachineCop0TlbOperationError::IndexUnavailable)
        );

        cpu.write_cop0_index(3);
        cpu.write_cop0_page_mask(0x001f_ffff);
        cpu.write_cop0_entry_hi(0x1234_567a);
        cpu.write_cop0_entry_lo0(0xffff_ffff);
        cpu.write_cop0_entry_lo1(0x8000_0001);
        assert_eq!(cpu.execute_cop0_tlb_write_indexed(), Ok(3));

        let entry = cpu
            .cop0_tlb_entry(3)
            .expect("indexed write owns one TLB entry");
        assert_eq!(entry.page_mask(), 0x001f_e000);
        assert_eq!(entry.entry_hi(), 0x1234_407a);
        assert_eq!(entry.entry_lo0(), 0x3fff_ffff);
        assert_eq!(entry.entry_lo1(), 1);
        assert!(entry.global());

        cpu.write_cop0_page_mask(0);
        cpu.write_cop0_entry_hi(0);
        cpu.write_cop0_entry_lo0(0);
        cpu.write_cop0_entry_lo1(0);
        assert_eq!(cpu.execute_cop0_tlb_read(), Ok(3));
        assert_eq!(cpu.cop0_page_mask(), Some(entry.page_mask()));
        assert_eq!(cpu.cop0_entry_hi(), entry.entry_hi());
        assert_eq!(cpu.cop0_entry_lo0(), Some(entry.entry_lo0()));
        assert_eq!(cpu.cop0_entry_lo1(), Some(entry.entry_lo1()));

        assert_eq!(cpu.execute_cop0_tlb_probe(), Some(3));
        assert_eq!(cpu.cop0_index(), Some(3));
        cpu.write_cop0_entry_hi(0x4000_0000);
        assert_eq!(cpu.execute_cop0_tlb_probe(), None);
        assert_eq!(cpu.cop0_index(), Some(0x8000_0000));
    }

    #[test]
    fn eret_clears_exl_and_rejects_unavailable_error_epc_without_mutation() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(LOCAL_EXCEPTION_VECTOR_PC);
        cpu.write_cop0_status(COP0_STATUS_EXL | 0x0000_ff01);
        cpu.write_cop0_epc(0x8000_4320);

        assert_eq!(cpu.execute_cop0_exception_return(), Ok(()));
        assert_eq!(cpu.pc(), 0x8000_4320);
        assert_eq!(cpu.next_pc(), 0x8000_4324);
        assert_eq!(cpu.cop0_status(), 0x0000_ff01);

        cpu.write_cop0_status(COP0_STATUS_ERL | COP0_STATUS_EXL);
        cpu.stage_pc(0x8000_0180);
        let before = (cpu.pc(), cpu.next_pc(), cpu.cop0_status(), cpu.cop0_epc());
        assert_eq!(
            cpu.execute_cop0_exception_return(),
            Err(CpuCop0ExceptionReturnError::ErrorEpcUnavailable {
                status: COP0_STATUS_ERL | COP0_STATUS_EXL,
            })
        );
        assert_eq!(
            (cpu.pc(), cpu.next_pc(), cpu.cop0_status(), cpu.cop0_epc()),
            before
        );
    }

    #[test]
    fn interrupt_entry_uses_instruction_boundary_and_existing_delay_slot_owner() {
        let enabled_status =
            COP0_STATUS_INTERRUPT_ENABLE | (COP0_STATUS_INTERRUPT_MASK & 0x0000_0400);

        let mut ordinary = Cpu::new();
        ordinary.stage_pc(0x8000_2000);
        ordinary.write_cop0_status(enabled_status);
        ordinary.set_cop0_rcp_interrupt_pending(true);
        ordinary.stage_cop0_count_compare_timer_for_test(0x1234_5678, 0x8765_4321, false);
        assert!(ordinary.cop0_interrupt_should_enter());
        ordinary.enter_interrupt_exception();
        assert_eq!(ordinary.cop0_epc(), 0x8000_2000);
        assert!(!ordinary.cop0_exception_branch_delay());
        assert_eq!(ordinary.cop0_exception_code(), 0);
        assert_eq!(ordinary.cop0_count(), 0x1234_5678);
        assert_eq!(ordinary.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(ordinary.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);

        let mut delay = Cpu::new();
        delay.stage_pc(0x8000_3004);
        delay.stage_next_pc(0x8000_4000);
        delay.stage_delay_slot_context_for_test(0x8000_3000);
        delay.write_cop0_status(enabled_status);
        delay.set_cop0_rcp_interrupt_pending(true);
        delay.enter_interrupt_exception();
        assert_eq!(delay.cop0_epc(), 0x8000_3000);
        assert!(delay.cop0_exception_branch_delay());
        assert_eq!(delay.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(delay.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(delay.delay_slot_context(), None);
    }
}
