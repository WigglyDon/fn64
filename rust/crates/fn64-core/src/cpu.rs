pub mod address;
mod cache;
mod cop0;
mod instruction;
mod registers;
mod scalars;

use cop0::Cop0;
pub(crate) use cop0::CpuArithmeticOverflowExceptionEntryError;

pub use address::{
    CpuAddressErrorExceptionEntryError, CpuAddressErrorKind, CpuDataAccessKind,
    CpuDataAddressError, CpuDataAlignmentError, CpuDataWidth,
};
pub(crate) use cache::{
    primary_data_cache_line_index, primary_instruction_cache_line_index,
    MachinePrimaryDataCacheFillPlan, MachinePrimaryDataCacheStorePlan,
    MachinePrimaryDataCacheWritebackPlan, MachinePrimaryInstructionCacheFillPlan,
};
pub use cache::{
    MachineCop0TagState, MachineCop0TagWriteProvenance, MachinePrimaryCacheIndexStoreTagTarget,
    MachinePrimaryCacheOperationProvenance, MachinePrimaryCaches,
    MachinePrimaryDataCacheFillProvenance, MachinePrimaryDataCacheLineState,
    MachinePrimaryDataCacheStoreProvenance, MachinePrimaryDataCacheStoreWidth,
    MachinePrimaryInstructionCacheFillProvenance, MachinePrimaryInstructionCacheLineState,
    PRIMARY_DATA_CACHE_LINE_COUNT, PRIMARY_DATA_CACHE_LINE_SIZE_BYTES,
    PRIMARY_DATA_CACHE_SIZE_BYTES, PRIMARY_INSTRUCTION_CACHE_LINE_COUNT,
    PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES, PRIMARY_INSTRUCTION_CACHE_SIZE_BYTES,
};
#[cfg(test)]
pub(crate) use instruction::CpuLocalExecutedHelperFamily;
pub use instruction::{
    decode_cpu_instruction_word, identify_cpu_instruction, CpuInstructionFields,
    CpuInstructionIdentity, CpuInstructionWord,
};
pub(crate) use instruction::{select_cpu_local_executed_helper, signed_cpu_value_less_than};
pub(crate) use instruction::{
    CpuLocalExecutedHelperArithmeticOverflow, CpuLocalExecutedHelperExecutedInstruction,
    CpuLocalExecutedHelperInvocationError, CpuLocalExecutedHelperInvocationOutcome,
};
pub use registers::CpuRegisterIndexError;
pub(crate) use scalars::CpuControlFlowSnapshot;
pub use scalars::CpuDelaySlotContext;

pub const CPU_GPR_COUNT: usize = 32;
pub const NON_BOOT_RESET_VECTOR_PC: u32 = 0xbfc0_0000;
pub const NON_BOOT_RESET_VECTOR_NEXT_PC: u32 = 0xbfc0_0004;

pub struct Cpu {
    pc: u32,
    next_pc: u32,
    delay_slot_context: Option<CpuDelaySlotContext>,
    hi: u64,
    lo: u64,
    gprs: [u64; CPU_GPR_COUNT],
    cop0: Cop0,
    primary_caches: Box<MachinePrimaryCaches>,
}

#[allow(clippy::new_without_default)]
impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: NON_BOOT_RESET_VECTOR_PC,
            next_pc: NON_BOOT_RESET_VECTOR_NEXT_PC,
            delay_slot_context: None,
            hi: 0,
            lo: 0,
            gprs: [0; CPU_GPR_COUNT],
            cop0: Cop0::new(),
            primary_caches: Box::new(MachinePrimaryCaches::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cpu_starts_at_cpp_non_boot_pc_pair() {
        let cpu = Cpu::new();

        assert_eq!(cpu.pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(cpu.pc(), 0xbfc0_0000);
        assert_eq!(cpu.next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(cpu.next_pc(), 0xbfc0_0004);
    }

    #[test]
    fn new_cpu_zeroes_integer_register_state() {
        let cpu = Cpu::new();

        assert_eq!(cpu.hi(), 0);
        assert_eq!(cpu.lo(), 0);
        assert_eq!(cpu.gpr(0), Some(0));

        for index in 0..CPU_GPR_COUNT {
            assert_eq!(cpu.gpr(index), Some(0));
        }
    }

    #[test]
    fn new_cpu_exposes_cpp_gpr_count_boundary() {
        let cpu = Cpu::new();

        assert_eq!(CPU_GPR_COUNT, 32);
        assert_eq!(cpu.gpr(CPU_GPR_COUNT - 1), Some(0));
        assert_eq!(cpu.gpr(CPU_GPR_COUNT), None);
    }
}
