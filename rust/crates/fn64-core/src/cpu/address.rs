use crate::rdram::RDRAM_SIZE_BYTES;
use core::fmt;

const DIRECT_SEGMENT_MASK: u32 = 0xe000_0000;
const DIRECT_SEGMENT_OFFSET_MASK: u32 = 0x1fff_ffff;
const KSEG0_RDRAM_BASE: u32 = 0x8000_0000;
const KSEG1_RDRAM_BASE: u32 = 0xa000_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuAddress(u32);

impl CpuAddress {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RdramOffset(u32);

impl RdramOffset {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuAddressTarget {
    DirectRdram(RdramOffset),
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuDataAccessKind {
    Read,
    Write,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuAddressErrorKind {
    AddressErrorLoad,
    AddressErrorStore,
}

impl CpuAddressErrorKind {
    pub const fn cause_exception_code(self) -> u8 {
        match self {
            Self::AddressErrorLoad => 4,
            Self::AddressErrorStore => 5,
        }
    }

    pub const fn short_name(self) -> &'static str {
        match self {
            Self::AddressErrorLoad => "AdEL",
            Self::AddressErrorStore => "AdES",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuDataWidth {
    Byte,
    Halfword,
    Word,
    Doubleword,
}

impl CpuDataWidth {
    pub const fn bytes(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Halfword => 2,
            Self::Word => 4,
            Self::Doubleword => 8,
        }
    }

    const fn alignment_mask(self) -> u32 {
        match self {
            Self::Byte => 0,
            Self::Halfword => 0x1,
            Self::Word => 0x3,
            Self::Doubleword => 0x7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuDataAlignmentError {
    access_kind: CpuDataAccessKind,
    address: CpuAddress,
    width: CpuDataWidth,
}

impl CpuDataAlignmentError {
    pub const fn access_kind(self) -> CpuDataAccessKind {
        self.access_kind
    }

    pub const fn address(self) -> CpuAddress {
        self.address
    }

    pub const fn width(self) -> CpuDataWidth {
        self.width
    }
}

impl fmt::Display for CpuDataAlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU data {:?} requires naturally aligned {}-byte address: {}",
            self.access_kind,
            self.width.bytes(),
            self.address.value()
        )
    }
}

impl std::error::Error for CpuDataAlignmentError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuDataAddressError {
    address: CpuAddress,
    width: CpuDataWidth,
    access_kind: CpuDataAccessKind,
    exception_kind: CpuAddressErrorKind,
}

impl CpuDataAddressError {
    pub const fn address(self) -> CpuAddress {
        self.address
    }

    pub const fn bad_vaddr(self) -> CpuAddress {
        self.address
    }

    pub const fn width(self) -> CpuDataWidth {
        self.width
    }

    pub const fn access_kind(self) -> CpuDataAccessKind {
        self.access_kind
    }

    pub const fn exception_kind(self) -> CpuAddressErrorKind {
        self.exception_kind
    }

    pub const fn cause_exception_code(self) -> u8 {
        self.exception_kind.cause_exception_code()
    }
}

impl From<CpuDataAlignmentError> for CpuDataAddressError {
    fn from(error: CpuDataAlignmentError) -> Self {
        select_cpu_data_address_error(error)
    }
}

impl fmt::Display for CpuDataAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU data {:?} selected {} for {}-byte address error at {}",
            self.access_kind,
            self.exception_kind.short_name(),
            self.width.bytes(),
            self.address.value()
        )
    }
}

impl std::error::Error for CpuDataAddressError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuAddressErrorExceptionEntryError {
    pc: CpuAddress,
    next_pc: CpuAddress,
    status: u32,
}

impl CpuAddressErrorExceptionEntryError {
    pub const fn new(pc: CpuAddress, next_pc: CpuAddress, status: u32) -> Self {
        Self {
            pc,
            next_pc,
            status,
        }
    }

    pub const fn pc(self) -> CpuAddress {
        self.pc
    }

    pub const fn next_pc(self) -> CpuAddress {
        self.next_pc
    }

    pub const fn status(self) -> u32 {
        self.status
    }
}

impl fmt::Display for CpuAddressErrorExceptionEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU data address-error exception entry blocked: pc={} next_pc={} status={}",
            self.pc.value(),
            self.next_pc.value(),
            self.status
        )
    }
}

impl std::error::Error for CpuAddressErrorExceptionEntryError {}

pub fn check_cpu_data_alignment(
    access_kind: CpuDataAccessKind,
    address: CpuAddress,
    width: CpuDataWidth,
) -> Result<(), CpuDataAlignmentError> {
    if (address.value() & width.alignment_mask()) != 0 {
        return Err(CpuDataAlignmentError {
            access_kind,
            address,
            width,
        });
    }

    Ok(())
}

pub fn select_cpu_data_address_error(error: CpuDataAlignmentError) -> CpuDataAddressError {
    select_cpu_data_address_error_for_access(error.access_kind(), error.address(), error.width())
}

pub fn select_cpu_data_address_error_for_access(
    access_kind: CpuDataAccessKind,
    address: CpuAddress,
    width: CpuDataWidth,
) -> CpuDataAddressError {
    let exception_kind = match access_kind {
        CpuDataAccessKind::Read => CpuAddressErrorKind::AddressErrorLoad,
        CpuDataAccessKind::Write => CpuAddressErrorKind::AddressErrorStore,
    };

    CpuDataAddressError {
        address,
        width,
        access_kind,
        exception_kind,
    }
}

pub fn classify_direct_rdram_address(cpu_address: CpuAddress, width: usize) -> CpuAddressTarget {
    let Some(physical_address) = translate_direct_cpu_physical_address(cpu_address) else {
        return CpuAddressTarget::Unsupported;
    };

    match translate_cpu_physical_rdram_address(physical_address, width) {
        Some(rdram_offset) => CpuAddressTarget::DirectRdram(rdram_offset),
        None => CpuAddressTarget::Unsupported,
    }
}

pub(crate) fn translate_direct_cpu_physical_address(cpu_address: CpuAddress) -> Option<u32> {
    let direct_segment = cpu_address.value() & DIRECT_SEGMENT_MASK;
    if direct_segment != KSEG0_RDRAM_BASE && direct_segment != KSEG1_RDRAM_BASE {
        return None;
    }

    Some(cpu_address.value() & DIRECT_SEGMENT_OFFSET_MASK)
}

fn translate_cpu_physical_rdram_address(
    physical_address: u32,
    width: usize,
) -> Option<RdramOffset> {
    if width == 0 || width > RDRAM_SIZE_BYTES {
        return None;
    }

    let offset = physical_address as usize;
    let last_offset = RDRAM_SIZE_BYTES - width;
    if offset > last_offset {
        return None;
    }

    Some(RdramOffset::new(physical_address))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::Cartridge;
    use crate::cpu::{Cpu, NON_BOOT_RESET_VECTOR_NEXT_PC, NON_BOOT_RESET_VECTOR_PC};
    use crate::machine::Machine;

    fn assert_direct_rdram(cpu_address: u32, width: usize, expected_offset: u32) {
        assert_eq!(
            classify_direct_rdram_address(CpuAddress::new(cpu_address), width),
            CpuAddressTarget::DirectRdram(RdramOffset::new(expected_offset))
        );
    }

    fn assert_unsupported(cpu_address: u32, width: usize) {
        assert_eq!(
            classify_direct_rdram_address(CpuAddress::new(cpu_address), width),
            CpuAddressTarget::Unsupported
        );
    }

    fn assert_aligned(address: u32, width: CpuDataWidth) {
        assert_eq!(
            check_cpu_data_alignment(CpuDataAccessKind::Read, CpuAddress::new(address), width),
            Ok(())
        );
        assert_eq!(
            check_cpu_data_alignment(CpuDataAccessKind::Write, CpuAddress::new(address), width),
            Ok(())
        );
    }

    fn assert_unaligned(
        access_kind: CpuDataAccessKind,
        address: u32,
        width: CpuDataWidth,
    ) -> CpuDataAlignmentError {
        check_cpu_data_alignment(access_kind, CpuAddress::new(address), width).unwrap_err()
    }

    fn assert_address_error_selection(
        access_kind: CpuDataAccessKind,
        address: u32,
        width: CpuDataWidth,
        expected_kind: CpuAddressErrorKind,
        expected_code: u8,
    ) -> CpuDataAddressError {
        let alignment_error = assert_unaligned(access_kind, address, width);
        let address_error = select_cpu_data_address_error(alignment_error);

        assert_eq!(address_error.access_kind(), access_kind);
        assert_eq!(address_error.address(), CpuAddress::new(address));
        assert_eq!(address_error.bad_vaddr(), CpuAddress::new(address));
        assert_eq!(address_error.width(), width);
        assert_eq!(address_error.exception_kind(), expected_kind);
        assert_eq!(address_error.cause_exception_code(), expected_code);

        address_error
    }

    #[test]
    fn kseg0_and_kseg1_bases_map_to_rdram_offset_zero() {
        assert_direct_rdram(KSEG0_RDRAM_BASE, 1, 0);
        assert_direct_rdram(KSEG1_RDRAM_BASE, 1, 0);
    }

    #[test]
    fn kseg0_and_kseg1_last_byte_map_to_last_rdram_offset() {
        let last_offset = RDRAM_SIZE_BYTES as u32 - 1;

        assert_direct_rdram(KSEG0_RDRAM_BASE + last_offset, 1, last_offset);
        assert_direct_rdram(KSEG1_RDRAM_BASE + last_offset, 1, last_offset);
    }

    #[test]
    fn full_width_boundary_uses_last_valid_offset_for_requested_width() {
        let last_u32_offset = RDRAM_SIZE_BYTES as u32 - 4;
        let last_u64_offset = RDRAM_SIZE_BYTES as u32 - 8;

        assert_direct_rdram(KSEG0_RDRAM_BASE + last_u32_offset, 4, last_u32_offset);
        assert_direct_rdram(KSEG1_RDRAM_BASE + last_u64_offset, 8, last_u64_offset);
    }

    #[test]
    fn direct_rdram_classification_rejects_out_of_range_spans() {
        let first_invalid_offset = RDRAM_SIZE_BYTES as u32;

        assert_unsupported(KSEG0_RDRAM_BASE + first_invalid_offset, 1);
        assert_unsupported(KSEG1_RDRAM_BASE + first_invalid_offset, 1);
        assert_unsupported(KSEG0_RDRAM_BASE + first_invalid_offset - 3, 4);
        assert_unsupported(KSEG1_RDRAM_BASE + first_invalid_offset - 7, 8);
        assert_unsupported(KSEG0_RDRAM_BASE + first_invalid_offset + 1, 1);
    }

    #[test]
    fn non_direct_and_non_rdram_direct_addresses_are_unsupported() {
        assert_unsupported(0x0000_0000, 1);
        assert_unsupported(0x6000_0000, 1);
        assert_unsupported(0xc000_0000, 1);
        assert_unsupported(0xe000_0000, 1);
        assert_unsupported(0xbfc0_0000, 4);
    }

    #[test]
    fn direct_rdram_offset_calculation_masks_segment_bits() {
        assert_direct_rdram(0x8001_2345, 1, 0x0001_2345);
        assert_direct_rdram(0xa001_2345, 1, 0x0001_2345);
    }

    #[test]
    fn classification_accepts_unaligned_cpu_addresses_without_alignment_check() {
        assert_direct_rdram(0x8000_0003, 2, 3);
        assert_direct_rdram(0xa000_0005, 4, 5);
    }

    #[test]
    fn classification_rejects_zero_or_oversized_widths() {
        assert_unsupported(KSEG0_RDRAM_BASE, 0);
        assert_unsupported(KSEG1_RDRAM_BASE, RDRAM_SIZE_BYTES + 1);
    }

    #[test]
    fn byte_data_alignment_accepts_representative_addresses() {
        for address in [
            0x0000_0000,
            0x0000_0001,
            0x0000_0002,
            0x0000_0003,
            0x8000_0001,
            0xa000_0007,
            0xffff_ffff,
        ] {
            assert_aligned(address, CpuDataWidth::Byte);
        }
    }

    #[test]
    fn halfword_data_alignment_uses_low_bit_only() {
        assert_aligned(0x0000_0000, CpuDataWidth::Halfword);
        assert_aligned(0x8000_0002, CpuDataWidth::Halfword);
        assert_aligned(0xa000_0004, CpuDataWidth::Halfword);

        let read_error =
            assert_unaligned(CpuDataAccessKind::Read, 0x8000_0001, CpuDataWidth::Halfword);
        let write_error = assert_unaligned(
            CpuDataAccessKind::Write,
            0xa000_0003,
            CpuDataWidth::Halfword,
        );

        assert_eq!(read_error.access_kind(), CpuDataAccessKind::Read);
        assert_eq!(read_error.address(), CpuAddress::new(0x8000_0001));
        assert_eq!(read_error.width(), CpuDataWidth::Halfword);
        assert_eq!(write_error.access_kind(), CpuDataAccessKind::Write);
        assert_eq!(write_error.address(), CpuAddress::new(0xa000_0003));
        assert_eq!(write_error.width(), CpuDataWidth::Halfword);
    }

    #[test]
    fn word_data_alignment_rejects_low_bits_one_two_and_three() {
        assert_aligned(0x0000_0000, CpuDataWidth::Word);
        assert_aligned(0x8000_0004, CpuDataWidth::Word);
        assert_aligned(0xa000_0008, CpuDataWidth::Word);

        for low_bits in 1..=3 {
            let address = 0x8000_1000 | low_bits;
            let error = assert_unaligned(CpuDataAccessKind::Read, address, CpuDataWidth::Word);
            assert_eq!(error.access_kind(), CpuDataAccessKind::Read);
            assert_eq!(error.address(), CpuAddress::new(address));
            assert_eq!(error.width(), CpuDataWidth::Word);
        }
    }

    #[test]
    fn doubleword_data_alignment_rejects_low_bits_one_through_seven() {
        assert_aligned(0x0000_0000, CpuDataWidth::Doubleword);
        assert_aligned(0x8000_0008, CpuDataWidth::Doubleword);
        assert_aligned(0xa000_0010, CpuDataWidth::Doubleword);

        for low_bits in 1..=7 {
            let address = 0xa000_2000 | low_bits;
            let error =
                assert_unaligned(CpuDataAccessKind::Write, address, CpuDataWidth::Doubleword);
            assert_eq!(error.access_kind(), CpuDataAccessKind::Write);
            assert_eq!(error.address(), CpuAddress::new(address));
            assert_eq!(error.width(), CpuDataWidth::Doubleword);
        }
    }

    #[test]
    fn data_alignment_ignores_high_cpu_address_bits() {
        assert_aligned(0x0000_0008, CpuDataWidth::Doubleword);
        assert_aligned(0x8000_0008, CpuDataWidth::Doubleword);
        assert_aligned(0xa000_0008, CpuDataWidth::Doubleword);
        assert_aligned(0xe000_0008, CpuDataWidth::Doubleword);

        for address in [0x0000_0005, 0x8000_0005, 0xa000_0005, 0xe000_0005] {
            let error = assert_unaligned(CpuDataAccessKind::Read, address, CpuDataWidth::Word);
            assert_eq!(error.address(), CpuAddress::new(address));
            assert_eq!(error.width(), CpuDataWidth::Word);
        }
    }

    #[test]
    fn data_alignment_error_display_is_explicit_rust_api_safety() {
        let error = assert_unaligned(
            CpuDataAccessKind::Write,
            0xffff_ffff,
            CpuDataWidth::Doubleword,
        );

        assert_eq!(
            error.to_string(),
            "CPU data Write requires naturally aligned 8-byte address: 4294967295"
        );
    }

    #[test]
    fn read_alignment_fault_selects_address_error_load() {
        let address_error = assert_address_error_selection(
            CpuDataAccessKind::Read,
            0x8000_1001,
            CpuDataWidth::Halfword,
            CpuAddressErrorKind::AddressErrorLoad,
            4,
        );

        assert_eq!(address_error.exception_kind().short_name(), "AdEL");
    }

    #[test]
    fn write_alignment_fault_selects_address_error_store() {
        let address_error = assert_address_error_selection(
            CpuDataAccessKind::Write,
            0xa000_1003,
            CpuDataWidth::Word,
            CpuAddressErrorKind::AddressErrorStore,
            5,
        );

        assert_eq!(address_error.exception_kind().short_name(), "AdES");
    }

    #[test]
    fn halfword_address_error_preserves_fault_payload() {
        let address_error = assert_address_error_selection(
            CpuDataAccessKind::Read,
            0x0000_0001,
            CpuDataWidth::Halfword,
            CpuAddressErrorKind::AddressErrorLoad,
            4,
        );

        assert_eq!(
            address_error.to_string(),
            "CPU data Read selected AdEL for 2-byte address error at 1"
        );
    }

    #[test]
    fn word_address_error_preserves_every_rejected_low_bit_payload() {
        for low_bits in 1..=3 {
            let address = 0x8000_2000 | low_bits;
            assert_address_error_selection(
                CpuDataAccessKind::Write,
                address,
                CpuDataWidth::Word,
                CpuAddressErrorKind::AddressErrorStore,
                5,
            );
        }
    }

    #[test]
    fn doubleword_address_error_preserves_every_rejected_low_bit_payload() {
        for low_bits in 1..=7 {
            let address = 0xa000_3000 | low_bits;
            assert_address_error_selection(
                CpuDataAccessKind::Read,
                address,
                CpuDataWidth::Doubleword,
                CpuAddressErrorKind::AddressErrorLoad,
                4,
            );
        }
    }

    #[test]
    fn address_error_selection_does_not_mutate_cpu_or_machine_state() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_1004);
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);

        let cpu_state_before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.hi(),
            cpu.lo(),
            cpu.gpr(0),
            cpu.gpr(8),
        );
        let cpu_cop0_before = (
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

        let machine = Machine::from_cartridge(Cartridge::default());
        let machine_core_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.rdram().size_bytes(),
            machine.rdram().read_u8(0),
            machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1),
        );
        let machine_cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
        );
        let machine_cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let alignment_error = assert_unaligned(
            CpuDataAccessKind::Write,
            0x8000_4007,
            CpuDataWidth::Doubleword,
        );
        let address_error = CpuDataAddressError::from(alignment_error);

        assert_eq!(
            address_error.exception_kind(),
            CpuAddressErrorKind::AddressErrorStore
        );
        assert_eq!(
            (
                cpu.pc(),
                cpu.next_pc(),
                cpu.hi(),
                cpu.lo(),
                cpu.gpr(0),
                cpu.gpr(8),
            ),
            cpu_state_before
        );
        assert_eq!(
            (
                cpu.cop0_count(),
                cpu.cop0_compare(),
                cpu.cop0_timer_interrupt_pending(),
                cpu.cop0_status(),
                cpu.cop0_software_interrupt_pending(),
                cpu.cop0_epc(),
                cpu.cop0_bad_vaddr(),
                cpu.cop0_exception_code(),
                cpu.cop0_exception_branch_delay(),
            ),
            cpu_cop0_before
        );
        assert_eq!(
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.rdram().size_bytes(),
                machine.rdram().read_u8(0),
                machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1),
            ),
            machine_core_before
        );
        assert_eq!(
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
            ),
            machine_cpu_before
        );
        assert_eq!(
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            ),
            machine_cop0_before
        );
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
    }

    #[test]
    fn byte_alignment_has_no_address_error_selection_source() {
        for access_kind in [CpuDataAccessKind::Read, CpuDataAccessKind::Write] {
            assert_eq!(
                check_cpu_data_alignment(
                    access_kind,
                    CpuAddress::new(0xffff_ffff),
                    CpuDataWidth::Byte,
                ),
                Ok(())
            );
        }
    }
}
