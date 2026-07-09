use core::fmt;

use super::Cpu;
use crate::cpu::address::{
    CpuAddress, CpuAddressErrorExceptionEntryError, CpuAddressErrorKind, CpuDataAddressError,
};

const COP0_STATUS_EXL: u32 = 0x0000_0002;
const COP0_EXCEPTION_CODE_SIGNED_OVERFLOW: u8 = 12;
const LOCAL_EXCEPTION_VECTOR_PC: u32 = 0x8000_0180;
const LOCAL_EXCEPTION_VECTOR_NEXT_PC: u32 = 0x8000_0184;

pub(super) struct Cop0 {
    count: u32,
    compare: u32,
    timer_interrupt_pending: bool,
    status: u32,
    software_interrupt_pending: u32,
    epc: u32,
    bad_vaddr: u32,
    exception_code: u8,
    exception_branch_delay: bool,
}

impl Cop0 {
    pub(super) fn new() -> Self {
        Self {
            count: 0,
            compare: 0,
            timer_interrupt_pending: false,
            status: 0,
            software_interrupt_pending: 0,
            epc: 0,
            bad_vaddr: 0,
            exception_code: 0,
            exception_branch_delay: false,
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

    fn epc(&self) -> u32 {
        self.epc
    }

    fn bad_vaddr(&self) -> u32 {
        self.bad_vaddr
    }

    fn exception_code(&self) -> u8 {
        self.exception_code
    }

    fn exception_branch_delay(&self) -> bool {
        self.exception_branch_delay
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

    pub fn cop0_epc(&self) -> u32 {
        self.cop0.epc()
    }

    pub fn cop0_bad_vaddr(&self) -> u32 {
        self.cop0.bad_vaddr()
    }

    pub fn cop0_exception_code(&self) -> u8 {
        self.cop0.exception_code()
    }

    pub fn cop0_exception_branch_delay(&self) -> bool {
        self.cop0.exception_branch_delay()
    }

    #[allow(dead_code)]
    pub(crate) fn advance_count_for_committed_step(&mut self) {
        self.cop0.advance_count_for_committed_step();
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

    pub fn enter_data_address_error_exception(
        &mut self,
        address_error: CpuDataAddressError,
    ) -> Result<(), CpuAddressErrorExceptionEntryError> {
        let branch_delay = if self.local_synchronous_exception_entry_allowed() {
            false
        } else if self.local_delay_slot_synchronous_exception_entry_allowed() {
            true
        } else {
            return Err(CpuAddressErrorExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        };

        self.cop0.epc = if branch_delay {
            self.pc.wrapping_sub(4)
        } else {
            self.pc
        };
        self.cop0.bad_vaddr = address_error.bad_vaddr().value();
        self.cop0.exception_code = address_error.cause_exception_code();
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;

        Ok(())
    }

    pub(crate) fn enter_instruction_fetch_address_error_exception(
        &mut self,
        bad_vaddr: CpuAddress,
    ) -> Result<(), CpuAddressErrorExceptionEntryError> {
        if !self.local_synchronous_exception_entry_allowed() {
            return Err(CpuAddressErrorExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        }

        self.cop0.epc = self.pc;
        self.cop0.bad_vaddr = bad_vaddr.value();
        self.cop0.exception_code = CpuAddressErrorKind::AddressErrorLoad.cause_exception_code();
        self.cop0.exception_branch_delay = false;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn enter_arithmetic_overflow_exception(
        &mut self,
    ) -> Result<(), CpuArithmeticOverflowExceptionEntryError> {
        let branch_delay = if self.local_synchronous_exception_entry_allowed() {
            false
        } else if self.local_delay_slot_synchronous_exception_entry_allowed() {
            true
        } else {
            return Err(CpuArithmeticOverflowExceptionEntryError::new(
                CpuAddress::new(self.pc),
                CpuAddress::new(self.next_pc),
                self.cop0.status,
            ));
        };

        self.cop0.epc = if branch_delay {
            self.pc.wrapping_sub(4)
        } else {
            self.pc
        };
        self.cop0.exception_code = COP0_EXCEPTION_CODE_SIGNED_OVERFLOW;
        self.cop0.exception_branch_delay = branch_delay;
        self.cop0.status |= COP0_STATUS_EXL;
        self.pc = LOCAL_EXCEPTION_VECTOR_PC;
        self.next_pc = LOCAL_EXCEPTION_VECTOR_NEXT_PC;

        Ok(())
    }

    fn local_synchronous_exception_entry_allowed(&self) -> bool {
        self.next_pc == self.pc.wrapping_add(4) && (self.cop0.status & COP0_STATUS_EXL) == 0
    }

    fn local_delay_slot_synchronous_exception_entry_allowed(&self) -> bool {
        self.next_pc != self.pc.wrapping_add(4)
            && (self.cop0.status & COP0_STATUS_EXL) == 0
            && (self.pc & 0x3) == 0
            && self.pc >= 4
    }
}

#[cfg(test)]
mod tests {
    use super::{
        COP0_EXCEPTION_CODE_SIGNED_OVERFLOW, COP0_STATUS_EXL, LOCAL_EXCEPTION_VECTOR_NEXT_PC,
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
        assert_eq!(cpu.cop0_epc(), 0);
        assert_eq!(cpu.cop0_bad_vaddr(), 0);
        assert_eq!(cpu.cop0_exception_code(), 0);
        assert!(!cpu.cop0_exception_branch_delay());
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

        assert_eq!(cpu.enter_arithmetic_overflow_exception(), Ok(()));

        assert_eq!(
            cpu.cop0_exception_code(),
            COP0_EXCEPTION_CODE_SIGNED_OVERFLOW
        );
        assert!(cpu.cop0_exception_branch_delay());
        assert_eq!(cpu.cop0_epc(), 0x8000_1200);
        assert_eq!(cpu.pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(cpu.next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
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
    fn arithmetic_overflow_exception_entry_blocks_unsupported_delay_slot_context_without_mutation()
    {
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
    fn data_address_error_entry_blocks_unsupported_delay_slot_context_without_mutation() {
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
}
