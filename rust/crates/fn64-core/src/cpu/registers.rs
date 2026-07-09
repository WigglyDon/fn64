use core::fmt;

use super::{Cpu, CPU_GPR_COUNT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuRegisterIndexError {
    index: usize,
}

impl CpuRegisterIndexError {
    pub fn index(&self) -> usize {
        self.index
    }
}

impl fmt::Display for CpuRegisterIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CPU GPR index out of range: {}", self.index)
    }
}

impl std::error::Error for CpuRegisterIndexError {}

impl Cpu {
    pub fn gpr(&self, index: usize) -> Option<u64> {
        if index >= CPU_GPR_COUNT {
            return None;
        }

        if index == 0 {
            return Some(0);
        }

        Some(self.gprs[index])
    }

    pub fn set_gpr(&mut self, index: usize, value: u64) -> Result<(), CpuRegisterIndexError> {
        if index >= CPU_GPR_COUNT {
            return Err(CpuRegisterIndexError { index });
        }

        if index == 0 {
            return Ok(());
        }

        self.gprs[index] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{NON_BOOT_RESET_VECTOR_NEXT_PC, NON_BOOT_RESET_VECTOR_PC};

    #[test]
    fn gpr_write_updates_only_the_selected_nonzero_register() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(cpu.gpr(7), Some(0));
        assert_eq!(cpu.gpr(9), Some(0));
        assert_eq!(cpu.gpr(31), Some(0));
        assert_eq!(cpu.pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(cpu.next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(cpu.hi(), 0);
        assert_eq!(cpu.lo(), 0);
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
    fn gpr_zero_write_is_ignored_without_changing_other_state() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0xfeed_face_cafe_beef), Ok(()));
        assert_eq!(cpu.set_gpr(0, u64::MAX), Ok(()));

        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0xfeed_face_cafe_beef));
        assert_eq!(cpu.pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(cpu.next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(cpu.hi(), 0);
        assert_eq!(cpu.lo(), 0);
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
    fn gpr_write_invalid_index_is_explicit_rust_api_safety() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(31, 0x1234), Ok(()));
        let error = cpu.set_gpr(CPU_GPR_COUNT, 0x5678).unwrap_err();

        assert_eq!(error.index(), CPU_GPR_COUNT);
        assert_eq!(error.to_string(), "CPU GPR index out of range: 32");
        assert_eq!(cpu.gpr(31), Some(0x1234));
        assert_eq!(cpu.gpr(CPU_GPR_COUNT), None);
        assert_eq!(cpu.pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(cpu.next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(cpu.hi(), 0);
        assert_eq!(cpu.lo(), 0);
    }
}
