use core::fmt;

use super::{
    Cpu, CpuCop0ExceptionReturnError, CpuRegisterIndexError, MachineCop0TlbOperationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuInstructionWord(u32);

impl CpuInstructionWord {
    pub const fn new(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuInstructionFields {
    raw: CpuInstructionWord,
    opcode: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    sa: u8,
    funct: u8,
    immediate_u16: u16,
    jump_target: u32,
}

impl CpuInstructionFields {
    pub const fn raw(self) -> CpuInstructionWord {
        self.raw
    }

    pub const fn opcode(self) -> u8 {
        self.opcode
    }

    pub const fn rs(self) -> u8 {
        self.rs
    }

    pub const fn rt(self) -> u8 {
        self.rt
    }

    pub const fn rd(self) -> u8 {
        self.rd
    }

    pub const fn sa(self) -> u8 {
        self.sa
    }

    pub const fn funct(self) -> u8 {
        self.funct
    }

    pub const fn immediate_u16(self) -> u16 {
        self.immediate_u16
    }

    pub const fn jump_target(self) -> u32 {
        self.jump_target
    }
}

pub const fn decode_cpu_instruction_word(raw: CpuInstructionWord) -> CpuInstructionFields {
    let bits = raw.bits();
    CpuInstructionFields {
        raw,
        opcode: ((bits >> 26) & 0x3f) as u8,
        rs: ((bits >> 21) & 0x1f) as u8,
        rt: ((bits >> 16) & 0x1f) as u8,
        rd: ((bits >> 11) & 0x1f) as u8,
        sa: ((bits >> 6) & 0x1f) as u8,
        funct: (bits & 0x3f) as u8,
        immediate_u16: (bits & 0xffff) as u16,
        jump_target: bits & 0x03ff_ffff,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuInstructionIdentity {
    UnknownPrimary,
    SpecialUnknown,
    RegimmUnknown,

    SpecialSll,
    SpecialSrl,
    SpecialSra,
    SpecialSllv,
    SpecialSrlv,
    SpecialSrav,
    SpecialJr,
    SpecialJalr,
    SpecialSyscall,
    SpecialBreak,
    SpecialSync,
    SpecialMfhi,
    SpecialMthi,
    SpecialMflo,
    SpecialMtlo,
    SpecialDsllv,
    SpecialDsrlv,
    SpecialDsrav,
    SpecialMult,
    SpecialMultu,
    SpecialDiv,
    SpecialDivu,
    SpecialDmult,
    SpecialDmultu,
    SpecialDdiv,
    SpecialDdivu,
    SpecialAdd,
    SpecialAddu,
    SpecialSub,
    SpecialSubu,
    SpecialAnd,
    SpecialOr,
    SpecialXor,
    SpecialNor,
    SpecialSlt,
    SpecialSltu,
    SpecialDadd,
    SpecialDaddu,
    SpecialDsub,
    SpecialDsubu,
    SpecialTge,
    SpecialTgeu,
    SpecialTlt,
    SpecialTltu,
    SpecialTeq,
    SpecialTne,
    SpecialDsll,
    SpecialDsrl,
    SpecialDsra,
    SpecialDsll32,
    SpecialDsrl32,
    SpecialDsra32,

    RegimmBltz,
    RegimmBgez,
    RegimmBltzl,
    RegimmBgezl,
    RegimmTgei,
    RegimmTgeiu,
    RegimmTlti,
    RegimmTltiu,
    RegimmTeqi,
    RegimmTnei,
    RegimmBltzal,
    RegimmBgezal,
    RegimmBltzall,
    RegimmBgezall,

    J,
    Jal,
    Beq,
    Bne,
    Blez,
    Bgtz,
    Addi,
    Addiu,
    Slti,
    Sltiu,
    Andi,
    Ori,
    Xori,
    Lui,
    Cop0,
    Cop0Mfc0,
    Cop0Mtc0,
    Cop0Tlbr,
    Cop0Tlbwi,
    Cop0Tlbwr,
    Cop0Tlbp,
    Cop0Eret,
    Cop1,
    Cop1Cfc1,
    Cop1Ctc1,
    Cop2,
    Cop3,
    Beql,
    Bnel,
    Blezl,
    Bgtzl,
    Daddi,
    Daddiu,
    Ldl,
    Ldr,
    Lb,
    Lh,
    Lwl,
    Lw,
    Lbu,
    Lhu,
    Lwr,
    Lwu,
    Sb,
    Sh,
    Swl,
    Sw,
    Sdl,
    Sdr,
    Swr,
    Cache,
    Ll,
    Lwc1,
    Lwc2,
    Lld,
    Ldc1,
    Ldc2,
    Ld,
    Sc,
    Swc1,
    Swc2,
    Scd,
    Sdc1,
    Sdc2,
    Sd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuLocalExecutedHelperFamily {
    NoEffectSync,
    SpecialShift,
    SpecialBitwiseLogical,
    SpecialHiLoTransfer,
    SpecialMultiply,
    SpecialDivide,
    SpecialNonTrappingInteger,
    SpecialTrappingInteger,
    ImmediateTrappingInteger,
    ImmediateNonTrappingInteger,
    ImmediateComparison,
    ImmediateBitwiseLogical,
    UpperImmediateLui,
    Cop0Tlb,
    Cop0ExceptionReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuLocalExecutedHelperSelection {
    identity: CpuInstructionIdentity,
    family: CpuLocalExecutedHelperFamily,
}

impl CpuLocalExecutedHelperSelection {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    #[allow(dead_code)]
    pub(crate) const fn family(self) -> CpuLocalExecutedHelperFamily {
        self.family
    }
}

pub const fn identify_cpu_instruction(instruction: CpuInstructionFields) -> CpuInstructionIdentity {
    match instruction.opcode() {
        0x00 => match instruction.funct() {
            0x00 => CpuInstructionIdentity::SpecialSll,
            0x02 => CpuInstructionIdentity::SpecialSrl,
            0x03 => CpuInstructionIdentity::SpecialSra,
            0x04 => CpuInstructionIdentity::SpecialSllv,
            0x06 => CpuInstructionIdentity::SpecialSrlv,
            0x07 => CpuInstructionIdentity::SpecialSrav,
            0x08 => CpuInstructionIdentity::SpecialJr,
            0x09 => CpuInstructionIdentity::SpecialJalr,
            0x0c => CpuInstructionIdentity::SpecialSyscall,
            0x0d => CpuInstructionIdentity::SpecialBreak,
            0x0f => CpuInstructionIdentity::SpecialSync,
            0x10 => CpuInstructionIdentity::SpecialMfhi,
            0x11 => CpuInstructionIdentity::SpecialMthi,
            0x12 => CpuInstructionIdentity::SpecialMflo,
            0x13 => CpuInstructionIdentity::SpecialMtlo,
            0x14 => CpuInstructionIdentity::SpecialDsllv,
            0x16 => CpuInstructionIdentity::SpecialDsrlv,
            0x17 => CpuInstructionIdentity::SpecialDsrav,
            0x18 => CpuInstructionIdentity::SpecialMult,
            0x19 => CpuInstructionIdentity::SpecialMultu,
            0x1a => CpuInstructionIdentity::SpecialDiv,
            0x1b => CpuInstructionIdentity::SpecialDivu,
            0x1c => CpuInstructionIdentity::SpecialDmult,
            0x1d => CpuInstructionIdentity::SpecialDmultu,
            0x1e => CpuInstructionIdentity::SpecialDdiv,
            0x1f => CpuInstructionIdentity::SpecialDdivu,
            0x20 => CpuInstructionIdentity::SpecialAdd,
            0x21 => CpuInstructionIdentity::SpecialAddu,
            0x22 => CpuInstructionIdentity::SpecialSub,
            0x23 => CpuInstructionIdentity::SpecialSubu,
            0x24 => CpuInstructionIdentity::SpecialAnd,
            0x25 => CpuInstructionIdentity::SpecialOr,
            0x26 => CpuInstructionIdentity::SpecialXor,
            0x27 => CpuInstructionIdentity::SpecialNor,
            0x2a => CpuInstructionIdentity::SpecialSlt,
            0x2b => CpuInstructionIdentity::SpecialSltu,
            0x2c => CpuInstructionIdentity::SpecialDadd,
            0x2d => CpuInstructionIdentity::SpecialDaddu,
            0x2e => CpuInstructionIdentity::SpecialDsub,
            0x2f => CpuInstructionIdentity::SpecialDsubu,
            0x30 => CpuInstructionIdentity::SpecialTge,
            0x31 => CpuInstructionIdentity::SpecialTgeu,
            0x32 => CpuInstructionIdentity::SpecialTlt,
            0x33 => CpuInstructionIdentity::SpecialTltu,
            0x34 => CpuInstructionIdentity::SpecialTeq,
            0x36 => CpuInstructionIdentity::SpecialTne,
            0x38 => CpuInstructionIdentity::SpecialDsll,
            0x3a => CpuInstructionIdentity::SpecialDsrl,
            0x3b => CpuInstructionIdentity::SpecialDsra,
            0x3c => CpuInstructionIdentity::SpecialDsll32,
            0x3e => CpuInstructionIdentity::SpecialDsrl32,
            0x3f => CpuInstructionIdentity::SpecialDsra32,
            _ => CpuInstructionIdentity::SpecialUnknown,
        },
        0x01 => match instruction.rt() {
            0x00 => CpuInstructionIdentity::RegimmBltz,
            0x01 => CpuInstructionIdentity::RegimmBgez,
            0x02 => CpuInstructionIdentity::RegimmBltzl,
            0x03 => CpuInstructionIdentity::RegimmBgezl,
            0x08 => CpuInstructionIdentity::RegimmTgei,
            0x09 => CpuInstructionIdentity::RegimmTgeiu,
            0x0a => CpuInstructionIdentity::RegimmTlti,
            0x0b => CpuInstructionIdentity::RegimmTltiu,
            0x0c => CpuInstructionIdentity::RegimmTeqi,
            0x0e => CpuInstructionIdentity::RegimmTnei,
            0x10 => CpuInstructionIdentity::RegimmBltzal,
            0x11 => CpuInstructionIdentity::RegimmBgezal,
            0x12 => CpuInstructionIdentity::RegimmBltzall,
            0x13 => CpuInstructionIdentity::RegimmBgezall,
            _ => CpuInstructionIdentity::RegimmUnknown,
        },
        0x02 => CpuInstructionIdentity::J,
        0x03 => CpuInstructionIdentity::Jal,
        0x04 => CpuInstructionIdentity::Beq,
        0x05 => CpuInstructionIdentity::Bne,
        0x06 => CpuInstructionIdentity::Blez,
        0x07 => CpuInstructionIdentity::Bgtz,
        0x08 => CpuInstructionIdentity::Addi,
        0x09 => CpuInstructionIdentity::Addiu,
        0x0a => CpuInstructionIdentity::Slti,
        0x0b => CpuInstructionIdentity::Sltiu,
        0x0c => CpuInstructionIdentity::Andi,
        0x0d => CpuInstructionIdentity::Ori,
        0x0e => CpuInstructionIdentity::Xori,
        0x0f => CpuInstructionIdentity::Lui,
        0x10 => match instruction.rs() {
            0x00 => CpuInstructionIdentity::Cop0Mfc0,
            0x04 => CpuInstructionIdentity::Cop0Mtc0,
            0x10 => match instruction.raw().bits() {
                0x4200_0001 => CpuInstructionIdentity::Cop0Tlbr,
                0x4200_0002 => CpuInstructionIdentity::Cop0Tlbwi,
                0x4200_0006 => CpuInstructionIdentity::Cop0Tlbwr,
                0x4200_0008 => CpuInstructionIdentity::Cop0Tlbp,
                0x4200_0018 => CpuInstructionIdentity::Cop0Eret,
                _ => CpuInstructionIdentity::Cop0,
            },
            _ => CpuInstructionIdentity::Cop0,
        },
        0x11 => match instruction.rs() {
            0x02 => CpuInstructionIdentity::Cop1Cfc1,
            0x06 => CpuInstructionIdentity::Cop1Ctc1,
            _ => CpuInstructionIdentity::Cop1,
        },
        0x12 => CpuInstructionIdentity::Cop2,
        0x13 => CpuInstructionIdentity::Cop3,
        0x14 => CpuInstructionIdentity::Beql,
        0x15 => CpuInstructionIdentity::Bnel,
        0x16 => CpuInstructionIdentity::Blezl,
        0x17 => CpuInstructionIdentity::Bgtzl,
        0x18 => CpuInstructionIdentity::Daddi,
        0x19 => CpuInstructionIdentity::Daddiu,
        0x1a => CpuInstructionIdentity::Ldl,
        0x1b => CpuInstructionIdentity::Ldr,
        0x20 => CpuInstructionIdentity::Lb,
        0x21 => CpuInstructionIdentity::Lh,
        0x22 => CpuInstructionIdentity::Lwl,
        0x23 => CpuInstructionIdentity::Lw,
        0x24 => CpuInstructionIdentity::Lbu,
        0x25 => CpuInstructionIdentity::Lhu,
        0x26 => CpuInstructionIdentity::Lwr,
        0x27 => CpuInstructionIdentity::Lwu,
        0x28 => CpuInstructionIdentity::Sb,
        0x29 => CpuInstructionIdentity::Sh,
        0x2a => CpuInstructionIdentity::Swl,
        0x2b => CpuInstructionIdentity::Sw,
        0x2c => CpuInstructionIdentity::Sdl,
        0x2d => CpuInstructionIdentity::Sdr,
        0x2e => CpuInstructionIdentity::Swr,
        0x2f => CpuInstructionIdentity::Cache,
        0x30 => CpuInstructionIdentity::Ll,
        0x31 => CpuInstructionIdentity::Lwc1,
        0x32 => CpuInstructionIdentity::Lwc2,
        0x34 => CpuInstructionIdentity::Lld,
        0x35 => CpuInstructionIdentity::Ldc1,
        0x36 => CpuInstructionIdentity::Ldc2,
        0x37 => CpuInstructionIdentity::Ld,
        0x38 => CpuInstructionIdentity::Sc,
        0x39 => CpuInstructionIdentity::Swc1,
        0x3a => CpuInstructionIdentity::Swc2,
        0x3c => CpuInstructionIdentity::Scd,
        0x3d => CpuInstructionIdentity::Sdc1,
        0x3e => CpuInstructionIdentity::Sdc2,
        0x3f => CpuInstructionIdentity::Sd,
        _ => CpuInstructionIdentity::UnknownPrimary,
    }
}

#[allow(dead_code)]
pub(crate) const fn select_cpu_local_executed_helper(
    identity: CpuInstructionIdentity,
) -> Option<CpuLocalExecutedHelperSelection> {
    let family = match identity {
        CpuInstructionIdentity::SpecialSync => Some(CpuLocalExecutedHelperFamily::NoEffectSync),
        CpuInstructionIdentity::SpecialSll
        | CpuInstructionIdentity::SpecialSrl
        | CpuInstructionIdentity::SpecialSra
        | CpuInstructionIdentity::SpecialSllv
        | CpuInstructionIdentity::SpecialSrlv
        | CpuInstructionIdentity::SpecialSrav
        | CpuInstructionIdentity::SpecialDsll
        | CpuInstructionIdentity::SpecialDsrl
        | CpuInstructionIdentity::SpecialDsra
        | CpuInstructionIdentity::SpecialDsll32
        | CpuInstructionIdentity::SpecialDsrl32
        | CpuInstructionIdentity::SpecialDsra32
        | CpuInstructionIdentity::SpecialDsllv
        | CpuInstructionIdentity::SpecialDsrlv
        | CpuInstructionIdentity::SpecialDsrav => Some(CpuLocalExecutedHelperFamily::SpecialShift),
        CpuInstructionIdentity::SpecialAnd
        | CpuInstructionIdentity::SpecialOr
        | CpuInstructionIdentity::SpecialXor
        | CpuInstructionIdentity::SpecialNor => {
            Some(CpuLocalExecutedHelperFamily::SpecialBitwiseLogical)
        }
        CpuInstructionIdentity::SpecialMfhi
        | CpuInstructionIdentity::SpecialMthi
        | CpuInstructionIdentity::SpecialMflo
        | CpuInstructionIdentity::SpecialMtlo => {
            Some(CpuLocalExecutedHelperFamily::SpecialHiLoTransfer)
        }
        CpuInstructionIdentity::SpecialMultu | CpuInstructionIdentity::SpecialDmultu => {
            Some(CpuLocalExecutedHelperFamily::SpecialMultiply)
        }
        CpuInstructionIdentity::SpecialDiv | CpuInstructionIdentity::SpecialDdivu => {
            Some(CpuLocalExecutedHelperFamily::SpecialDivide)
        }
        CpuInstructionIdentity::SpecialAddu
        | CpuInstructionIdentity::SpecialSubu
        | CpuInstructionIdentity::SpecialDaddu
        | CpuInstructionIdentity::SpecialDsubu
        | CpuInstructionIdentity::SpecialSlt
        | CpuInstructionIdentity::SpecialSltu => {
            Some(CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger)
        }
        CpuInstructionIdentity::SpecialAdd
        | CpuInstructionIdentity::SpecialSub
        | CpuInstructionIdentity::SpecialDadd
        | CpuInstructionIdentity::SpecialDsub => {
            Some(CpuLocalExecutedHelperFamily::SpecialTrappingInteger)
        }
        CpuInstructionIdentity::Addi | CpuInstructionIdentity::Daddi => {
            Some(CpuLocalExecutedHelperFamily::ImmediateTrappingInteger)
        }
        CpuInstructionIdentity::Addiu | CpuInstructionIdentity::Daddiu => {
            Some(CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger)
        }
        CpuInstructionIdentity::Slti | CpuInstructionIdentity::Sltiu => {
            Some(CpuLocalExecutedHelperFamily::ImmediateComparison)
        }
        CpuInstructionIdentity::Andi
        | CpuInstructionIdentity::Ori
        | CpuInstructionIdentity::Xori => {
            Some(CpuLocalExecutedHelperFamily::ImmediateBitwiseLogical)
        }
        CpuInstructionIdentity::Lui => Some(CpuLocalExecutedHelperFamily::UpperImmediateLui),
        CpuInstructionIdentity::Cop0Tlbr
        | CpuInstructionIdentity::Cop0Tlbwi
        | CpuInstructionIdentity::Cop0Tlbwr
        | CpuInstructionIdentity::Cop0Tlbp => Some(CpuLocalExecutedHelperFamily::Cop0Tlb),
        CpuInstructionIdentity::Cop0Eret => Some(CpuLocalExecutedHelperFamily::Cop0ExceptionReturn),
        _ => None,
    };

    match family {
        Some(family) => Some(CpuLocalExecutedHelperSelection { identity, family }),
        None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuLocalExecutedHelperExecutedInstruction {
    identity: CpuInstructionIdentity,
    family: CpuLocalExecutedHelperFamily,
}

impl CpuLocalExecutedHelperExecutedInstruction {
    #[cfg(test)]
    pub(crate) const fn new_for_test(
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) -> Self {
        Self { identity, family }
    }

    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    #[allow(dead_code)]
    pub(crate) const fn family(self) -> CpuLocalExecutedHelperFamily {
        self.family
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuLocalExecutedHelperArithmeticOverflow {
    SpecialTrappingInteger(CpuSpecialTrappingIntegerOverflow),
    ImmediateTrappingInteger(CpuImmediateTrappingIntegerOverflow),
}

impl CpuLocalExecutedHelperArithmeticOverflow {
    #[cfg(test)]
    pub(crate) const fn special_trapping_integer_for_test(
        identity: CpuInstructionIdentity,
        rd: u8,
        rs_value: u64,
        rt_value: u64,
    ) -> Self {
        Self::SpecialTrappingInteger(CpuSpecialTrappingIntegerOverflow {
            identity,
            rd,
            rs_value,
            rt_value,
        })
    }

    #[cfg(test)]
    pub(crate) const fn immediate_trapping_integer_for_test(
        identity: CpuInstructionIdentity,
        rt: u8,
        rs_value: u64,
        immediate_u16: u16,
        immediate_value: u64,
    ) -> Self {
        Self::ImmediateTrappingInteger(CpuImmediateTrappingIntegerOverflow {
            identity,
            rt,
            rs_value,
            immediate_u16,
            immediate_value,
        })
    }

    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        match self {
            Self::SpecialTrappingInteger(overflow) => overflow.identity(),
            Self::ImmediateTrappingInteger(overflow) => overflow.identity(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn family(self) -> CpuLocalExecutedHelperFamily {
        match self {
            Self::SpecialTrappingInteger(_) => CpuLocalExecutedHelperFamily::SpecialTrappingInteger,
            Self::ImmediateTrappingInteger(_) => {
                CpuLocalExecutedHelperFamily::ImmediateTrappingInteger
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuLocalExecutedHelperInvocationOutcome {
    Executed(CpuLocalExecutedHelperExecutedInstruction),
    ArithmeticOverflow(CpuLocalExecutedHelperArithmeticOverflow),
}

impl CpuLocalExecutedHelperInvocationOutcome {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        match self {
            Self::Executed(executed) => executed.identity(),
            Self::ArithmeticOverflow(overflow) => overflow.identity(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn family(self) -> CpuLocalExecutedHelperFamily {
        match self {
            Self::Executed(executed) => executed.family(),
            Self::ArithmeticOverflow(overflow) => overflow.family(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn is_executed(self) -> bool {
        matches!(self, Self::Executed(_))
    }

    #[allow(dead_code)]
    pub(crate) const fn is_arithmetic_overflow(self) -> bool {
        matches!(self, Self::ArithmeticOverflow(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuLocalExecutedHelperInvocationError {
    HelperRejectedSelection(CpuLocalExecutedHelperSelection),
    RegisterIndex(CpuRegisterIndexError),
    Cop0Tlb {
        identity: CpuInstructionIdentity,
        error: MachineCop0TlbOperationError,
    },
    Cop0ExceptionReturn(CpuCop0ExceptionReturnError),
}

impl fmt::Display for CpuLocalExecutedHelperInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HelperRejectedSelection(selection) => {
                write!(
                    f,
                    "CPU local executed helper rejected selection: identity={:?} family={:?}",
                    selection.identity(),
                    selection.family()
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
            Self::Cop0Tlb { identity, error } => {
                write!(f, "CPU {identity:?} TLB operation rejected: {error:?}")
            }
            Self::Cop0ExceptionReturn(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuLocalExecutedHelperInvocationError {}

impl From<CpuRegisterIndexError> for CpuLocalExecutedHelperInvocationError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialShiftExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuSpecialShiftExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialShiftExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuSpecialShiftExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU SPECIAL shift execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuSpecialShiftExecutionError {}

impl From<CpuRegisterIndexError> for CpuSpecialShiftExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialBitwiseLogicalExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuSpecialBitwiseLogicalExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialBitwiseLogicalExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuSpecialBitwiseLogicalExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU SPECIAL bitwise logical execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuSpecialBitwiseLogicalExecutionError {}

impl From<CpuRegisterIndexError> for CpuSpecialBitwiseLogicalExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialHiLoTransferExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuSpecialHiLoTransferExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialHiLoTransferExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuSpecialHiLoTransferExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU SPECIAL HI/LO transfer execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuSpecialHiLoTransferExecutionError {}

impl From<CpuRegisterIndexError> for CpuSpecialHiLoTransferExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialNonTrappingIntegerExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuSpecialNonTrappingIntegerExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialNonTrappingIntegerExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuSpecialNonTrappingIntegerExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU SPECIAL non-trapping integer execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuSpecialNonTrappingIntegerExecutionError {}

impl From<CpuRegisterIndexError> for CpuSpecialNonTrappingIntegerExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialTrappingIntegerExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuSpecialTrappingIntegerExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuSpecialTrappingIntegerOverflow {
    identity: CpuInstructionIdentity,
    rd: u8,
    rs_value: u64,
    rt_value: u64,
}

impl CpuSpecialTrappingIntegerOverflow {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    #[allow(dead_code)]
    pub(crate) const fn rd(self) -> u8 {
        self.rd
    }

    #[allow(dead_code)]
    pub(crate) const fn rs_value(self) -> u64 {
        self.rs_value
    }

    #[allow(dead_code)]
    pub(crate) const fn rt_value(self) -> u64 {
        self.rt_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialTrappingIntegerExecutionOutcome {
    Executed(CpuSpecialTrappingIntegerExecutedInstruction),
    Overflow(CpuSpecialTrappingIntegerOverflow),
}

impl CpuSpecialTrappingIntegerExecutionOutcome {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        match self {
            Self::Executed(executed) => executed.identity(),
            Self::Overflow(overflow) => overflow.identity(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn is_executed(self) -> bool {
        matches!(self, Self::Executed(_))
    }

    #[allow(dead_code)]
    pub(crate) const fn is_overflow(self) -> bool {
        matches!(self, Self::Overflow(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSpecialTrappingIntegerExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuSpecialTrappingIntegerExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU SPECIAL trapping integer execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuSpecialTrappingIntegerExecutionError {}

impl From<CpuRegisterIndexError> for CpuSpecialTrappingIntegerExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuImmediateTrappingIntegerExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuImmediateTrappingIntegerExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuImmediateTrappingIntegerOverflow {
    identity: CpuInstructionIdentity,
    rt: u8,
    rs_value: u64,
    immediate_u16: u16,
    immediate_value: u64,
}

impl CpuImmediateTrappingIntegerOverflow {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    #[allow(dead_code)]
    pub(crate) const fn rt(self) -> u8 {
        self.rt
    }

    #[allow(dead_code)]
    pub(crate) const fn rs_value(self) -> u64 {
        self.rs_value
    }

    #[allow(dead_code)]
    pub(crate) const fn immediate_u16(self) -> u16 {
        self.immediate_u16
    }

    #[allow(dead_code)]
    pub(crate) const fn immediate_value(self) -> u64 {
        self.immediate_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuImmediateTrappingIntegerExecutionOutcome {
    Executed(CpuImmediateTrappingIntegerExecutedInstruction),
    Overflow(CpuImmediateTrappingIntegerOverflow),
}

impl CpuImmediateTrappingIntegerExecutionOutcome {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        match self {
            Self::Executed(executed) => executed.identity(),
            Self::Overflow(overflow) => overflow.identity(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn is_executed(self) -> bool {
        matches!(self, Self::Executed(_))
    }

    #[allow(dead_code)]
    pub(crate) const fn is_overflow(self) -> bool {
        matches!(self, Self::Overflow(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuImmediateTrappingIntegerExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuImmediateTrappingIntegerExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU immediate trapping integer execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuImmediateTrappingIntegerExecutionError {}

impl From<CpuRegisterIndexError> for CpuImmediateTrappingIntegerExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuImmediateNonTrappingIntegerExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuImmediateNonTrappingIntegerExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuImmediateNonTrappingIntegerExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuImmediateNonTrappingIntegerExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU immediate non-trapping integer execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuImmediateNonTrappingIntegerExecutionError {}

impl From<CpuRegisterIndexError> for CpuImmediateNonTrappingIntegerExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuImmediateComparisonExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuImmediateComparisonExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuImmediateComparisonExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuImmediateComparisonExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU immediate comparison execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuImmediateComparisonExecutionError {}

impl From<CpuRegisterIndexError> for CpuImmediateComparisonExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuImmediateBitwiseLogicalExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuImmediateBitwiseLogicalExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuImmediateBitwiseLogicalExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuImmediateBitwiseLogicalExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU immediate bitwise logical execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuImmediateBitwiseLogicalExecutionError {}

impl From<CpuRegisterIndexError> for CpuImmediateBitwiseLogicalExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuUpperImmediateExecutedInstruction {
    identity: CpuInstructionIdentity,
}

impl CpuUpperImmediateExecutedInstruction {
    #[allow(dead_code)]
    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuUpperImmediateExecutionError {
    UnsupportedIdentity(CpuInstructionIdentity),
    RegisterIndex(CpuRegisterIndexError),
}

impl fmt::Display for CpuUpperImmediateExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedIdentity(identity) => {
                write!(
                    f,
                    "CPU upper-immediate execution unsupported for {identity:?}"
                )
            }
            Self::RegisterIndex(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CpuUpperImmediateExecutionError {}

impl From<CpuRegisterIndexError> for CpuUpperImmediateExecutionError {
    fn from(error: CpuRegisterIndexError) -> Self {
        Self::RegisterIndex(error)
    }
}

impl Cpu {
    #[allow(dead_code)]
    pub(crate) fn invoke_cpu_local_executed_helper(
        &mut self,
        instruction: CpuInstructionFields,
        selection: CpuLocalExecutedHelperSelection,
    ) -> Result<CpuLocalExecutedHelperInvocationOutcome, CpuLocalExecutedHelperInvocationError>
    {
        let identity = selection.identity();
        let family = selection.family();

        match family {
            CpuLocalExecutedHelperFamily::NoEffectSync => {
                if identity != CpuInstructionIdentity::SpecialSync {
                    return Err(
                        CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection),
                    );
                }
            }
            CpuLocalExecutedHelperFamily::SpecialShift => {
                if let Err(error) = self.execute_special_shift_instruction(identity, instruction) {
                    return Err(map_special_shift_invocation_error(error, selection));
                }
            }
            CpuLocalExecutedHelperFamily::SpecialBitwiseLogical => {
                if let Err(error) =
                    self.execute_special_bitwise_logical_instruction(identity, instruction)
                {
                    return Err(map_special_bitwise_logical_invocation_error(
                        error, selection,
                    ));
                }
            }
            CpuLocalExecutedHelperFamily::SpecialHiLoTransfer => {
                if let Err(error) =
                    self.execute_special_hi_lo_transfer_instruction(identity, instruction)
                {
                    return Err(map_special_hi_lo_transfer_invocation_error(
                        error, selection,
                    ));
                }
            }
            CpuLocalExecutedHelperFamily::SpecialMultiply => match identity {
                CpuInstructionIdentity::SpecialMultu => {
                    let product = u64::from(read_gpr_word(self, instruction.rs()))
                        .wrapping_mul(u64::from(read_gpr_word(self, instruction.rt())));
                    self.hi = sign_extend_u32_to_cpu_value((product >> 32) as u32);
                    self.lo = sign_extend_u32_to_cpu_value(product as u32);
                }
                CpuInstructionIdentity::SpecialDmultu => {
                    let product = u128::from(
                        self.gpr(usize::from(instruction.rs()))
                            .expect("decoded CPU register index is five bits"),
                    ) * u128::from(
                        self.gpr(usize::from(instruction.rt()))
                            .expect("decoded CPU register index is five bits"),
                    );
                    self.hi = (product >> 64) as u64;
                    self.lo = product as u64;
                }
                _ => {
                    return Err(
                        CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection),
                    );
                }
            },
            CpuLocalExecutedHelperFamily::SpecialDivide => match identity {
                CpuInstructionIdentity::SpecialDiv => {
                    let dividend = read_gpr_word(self, instruction.rs()) as i32;
                    let divisor = read_gpr_word(self, instruction.rt()) as i32;
                    let (quotient, remainder) = if divisor == 0 {
                        (if dividend < 0 { 1 } else { -1 }, dividend)
                    } else if dividend == i32::MIN && divisor == -1 {
                        (i32::MIN, 0)
                    } else {
                        (dividend / divisor, dividend % divisor)
                    };
                    self.hi = sign_extend_u32_to_cpu_value(remainder as u32);
                    self.lo = sign_extend_u32_to_cpu_value(quotient as u32);
                }
                CpuInstructionIdentity::SpecialDdivu => {
                    let dividend = self
                        .gpr(usize::from(instruction.rs()))
                        .expect("decoded CPU register index is five bits");
                    let divisor = self
                        .gpr(usize::from(instruction.rt()))
                        .expect("decoded CPU register index is five bits");
                    if divisor == 0 {
                        self.hi = dividend;
                        self.lo = u64::MAX;
                    } else {
                        self.hi = dividend % divisor;
                        self.lo = dividend / divisor;
                    }
                }
                _ => {
                    return Err(
                        CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection),
                    );
                }
            },
            CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger => {
                if let Err(error) =
                    self.execute_special_non_trapping_integer_instruction(identity, instruction)
                {
                    return Err(map_special_non_trapping_integer_invocation_error(
                        error, selection,
                    ));
                }
            }
            CpuLocalExecutedHelperFamily::SpecialTrappingInteger => {
                match self.execute_special_trapping_integer_instruction(identity, instruction) {
                    Ok(CpuSpecialTrappingIntegerExecutionOutcome::Executed(_)) => {}
                    Ok(CpuSpecialTrappingIntegerExecutionOutcome::Overflow(overflow)) => {
                        return Ok(CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(
                            CpuLocalExecutedHelperArithmeticOverflow::SpecialTrappingInteger(
                                overflow,
                            ),
                        ));
                    }
                    Err(error) => {
                        return Err(map_special_trapping_integer_invocation_error(
                            error, selection,
                        ));
                    }
                }
            }
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger => {
                match self.execute_immediate_trapping_integer_instruction(identity, instruction) {
                    Ok(CpuImmediateTrappingIntegerExecutionOutcome::Executed(_)) => {}
                    Ok(CpuImmediateTrappingIntegerExecutionOutcome::Overflow(overflow)) => {
                        return Ok(CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(
                            CpuLocalExecutedHelperArithmeticOverflow::ImmediateTrappingInteger(
                                overflow,
                            ),
                        ));
                    }
                    Err(error) => {
                        return Err(map_immediate_trapping_integer_invocation_error(
                            error, selection,
                        ));
                    }
                }
            }
            CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger => {
                if let Err(error) =
                    self.execute_immediate_non_trapping_integer_instruction(identity, instruction)
                {
                    return Err(map_immediate_non_trapping_integer_invocation_error(
                        error, selection,
                    ));
                }
            }
            CpuLocalExecutedHelperFamily::ImmediateComparison => {
                if let Err(error) =
                    self.execute_immediate_comparison_instruction(identity, instruction)
                {
                    return Err(map_immediate_comparison_invocation_error(error, selection));
                }
            }
            CpuLocalExecutedHelperFamily::ImmediateBitwiseLogical => {
                if let Err(error) =
                    self.execute_immediate_bitwise_logical_instruction(identity, instruction)
                {
                    return Err(map_immediate_bitwise_logical_invocation_error(
                        error, selection,
                    ));
                }
            }
            CpuLocalExecutedHelperFamily::UpperImmediateLui => {
                if let Err(error) = self.execute_upper_immediate_instruction(identity, instruction)
                {
                    return Err(map_upper_immediate_invocation_error(error, selection));
                }
            }
            CpuLocalExecutedHelperFamily::Cop0Tlb => match identity {
                CpuInstructionIdentity::Cop0Tlbr => {
                    self.execute_cop0_tlb_read().map_err(|error| {
                        CpuLocalExecutedHelperInvocationError::Cop0Tlb { identity, error }
                    })?;
                }
                CpuInstructionIdentity::Cop0Tlbwi => {
                    self.execute_cop0_tlb_write_indexed().map_err(|error| {
                        CpuLocalExecutedHelperInvocationError::Cop0Tlb { identity, error }
                    })?;
                }
                CpuInstructionIdentity::Cop0Tlbwr => {
                    self.execute_cop0_tlb_write_random().map_err(|error| {
                        CpuLocalExecutedHelperInvocationError::Cop0Tlb { identity, error }
                    })?;
                }
                CpuInstructionIdentity::Cop0Tlbp => {
                    self.execute_cop0_tlb_probe();
                }
                _ => {
                    return Err(
                        CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection),
                    );
                }
            },
            CpuLocalExecutedHelperFamily::Cop0ExceptionReturn => {
                if identity != CpuInstructionIdentity::Cop0Eret {
                    return Err(
                        CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection),
                    );
                }
                self.execute_cop0_exception_return()
                    .map_err(CpuLocalExecutedHelperInvocationError::Cop0ExceptionReturn)?;
            }
        }

        Ok(CpuLocalExecutedHelperInvocationOutcome::Executed(
            CpuLocalExecutedHelperExecutedInstruction { identity, family },
        ))
    }

    #[allow(dead_code)]
    pub(crate) fn execute_special_shift_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuSpecialShiftExecutedInstruction, CpuSpecialShiftExecutionError> {
        let value = match identity {
            CpuInstructionIdentity::SpecialSll => sign_extend_u32_to_cpu_value(
                read_gpr_word(self, instruction.rt()) << instruction.sa(),
            ),
            CpuInstructionIdentity::SpecialSrl => sign_extend_u32_to_cpu_value(
                read_gpr_word(self, instruction.rt()) >> instruction.sa(),
            ),
            CpuInstructionIdentity::SpecialSra => sign_extend_u32_to_cpu_value(
                arithmetic_shift_right_u32(read_gpr_word(self, instruction.rt()), instruction.sa()),
            ),
            CpuInstructionIdentity::SpecialSllv => {
                let sa = variable_shift_amount_u32(read_gpr_word(self, instruction.rs()));
                sign_extend_u32_to_cpu_value(read_gpr_word(self, instruction.rt()) << sa)
            }
            CpuInstructionIdentity::SpecialSrlv => {
                let sa = variable_shift_amount_u32(read_gpr_word(self, instruction.rs()));
                sign_extend_u32_to_cpu_value(read_gpr_word(self, instruction.rt()) >> sa)
            }
            CpuInstructionIdentity::SpecialSrav => {
                let sa = variable_shift_amount_u32(read_gpr_word(self, instruction.rs()));
                sign_extend_u32_to_cpu_value(arithmetic_shift_right_u32(
                    read_gpr_word(self, instruction.rt()),
                    sa,
                ))
            }
            CpuInstructionIdentity::SpecialDsll => {
                read_gpr_value(self, instruction.rt()) << instruction.sa()
            }
            CpuInstructionIdentity::SpecialDsrl => {
                read_gpr_value(self, instruction.rt()) >> instruction.sa()
            }
            CpuInstructionIdentity::SpecialDsra => arithmetic_shift_right_cpu_value(
                read_gpr_value(self, instruction.rt()),
                instruction.sa(),
            ),
            CpuInstructionIdentity::SpecialDsll32 => {
                let sa = instruction.sa() + 32;
                read_gpr_value(self, instruction.rt()) << sa
            }
            CpuInstructionIdentity::SpecialDsrl32 => {
                let sa = instruction.sa() + 32;
                read_gpr_value(self, instruction.rt()) >> sa
            }
            CpuInstructionIdentity::SpecialDsra32 => {
                let sa = instruction.sa() + 32;
                arithmetic_shift_right_cpu_value(read_gpr_value(self, instruction.rt()), sa)
            }
            CpuInstructionIdentity::SpecialDsllv => {
                let sa = variable_shift_amount_cpu_value(read_gpr_value(self, instruction.rs()));
                read_gpr_value(self, instruction.rt()) << sa
            }
            CpuInstructionIdentity::SpecialDsrlv => {
                let sa = variable_shift_amount_cpu_value(read_gpr_value(self, instruction.rs()));
                read_gpr_value(self, instruction.rt()) >> sa
            }
            CpuInstructionIdentity::SpecialDsrav => {
                let sa = variable_shift_amount_cpu_value(read_gpr_value(self, instruction.rs()));
                arithmetic_shift_right_cpu_value(read_gpr_value(self, instruction.rt()), sa)
            }
            _ => return Err(CpuSpecialShiftExecutionError::UnsupportedIdentity(identity)),
        };

        self.set_gpr(usize::from(instruction.rd()), value)?;

        Ok(CpuSpecialShiftExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_special_bitwise_logical_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuSpecialBitwiseLogicalExecutedInstruction, CpuSpecialBitwiseLogicalExecutionError>
    {
        let value = match identity {
            CpuInstructionIdentity::SpecialAnd
            | CpuInstructionIdentity::SpecialOr
            | CpuInstructionIdentity::SpecialXor
            | CpuInstructionIdentity::SpecialNor => {
                let rs = read_gpr_value(self, instruction.rs());
                let rt = read_gpr_value(self, instruction.rt());
                match identity {
                    CpuInstructionIdentity::SpecialAnd => rs & rt,
                    CpuInstructionIdentity::SpecialOr => rs | rt,
                    CpuInstructionIdentity::SpecialXor => rs ^ rt,
                    CpuInstructionIdentity::SpecialNor => !(rs | rt),
                    _ => unreachable!("identity was constrained by the outer match"),
                }
            }
            _ => {
                return Err(CpuSpecialBitwiseLogicalExecutionError::UnsupportedIdentity(
                    identity,
                ))
            }
        };

        self.set_gpr(usize::from(instruction.rd()), value)?;

        Ok(CpuSpecialBitwiseLogicalExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_special_hi_lo_transfer_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuSpecialHiLoTransferExecutedInstruction, CpuSpecialHiLoTransferExecutionError>
    {
        match identity {
            CpuInstructionIdentity::SpecialMfhi => {
                self.set_gpr(usize::from(instruction.rd()), self.hi())?;
            }
            CpuInstructionIdentity::SpecialMthi => {
                let value = read_gpr_value(self, instruction.rs());
                self.stage_hi(value);
            }
            CpuInstructionIdentity::SpecialMflo => {
                self.set_gpr(usize::from(instruction.rd()), self.lo())?;
            }
            CpuInstructionIdentity::SpecialMtlo => {
                let value = read_gpr_value(self, instruction.rs());
                self.stage_lo(value);
            }
            _ => {
                return Err(CpuSpecialHiLoTransferExecutionError::UnsupportedIdentity(
                    identity,
                ))
            }
        }

        Ok(CpuSpecialHiLoTransferExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_special_non_trapping_integer_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<
        CpuSpecialNonTrappingIntegerExecutedInstruction,
        CpuSpecialNonTrappingIntegerExecutionError,
    > {
        let value = match identity {
            CpuInstructionIdentity::SpecialAddu
            | CpuInstructionIdentity::SpecialSubu
            | CpuInstructionIdentity::SpecialDaddu
            | CpuInstructionIdentity::SpecialDsubu
            | CpuInstructionIdentity::SpecialSlt
            | CpuInstructionIdentity::SpecialSltu => {
                let rs = read_gpr_value(self, instruction.rs());
                let rt = read_gpr_value(self, instruction.rt());
                match identity {
                    CpuInstructionIdentity::SpecialAddu => {
                        sign_extend_u32_to_cpu_value((rs as u32).wrapping_add(rt as u32))
                    }
                    CpuInstructionIdentity::SpecialSubu => {
                        sign_extend_u32_to_cpu_value((rs as u32).wrapping_sub(rt as u32))
                    }
                    CpuInstructionIdentity::SpecialDaddu => rs.wrapping_add(rt),
                    CpuInstructionIdentity::SpecialDsubu => rs.wrapping_sub(rt),
                    CpuInstructionIdentity::SpecialSlt => {
                        cpu_value_from_bool(signed_cpu_value_less_than(rs, rt))
                    }
                    CpuInstructionIdentity::SpecialSltu => cpu_value_from_bool(rs < rt),
                    _ => unreachable!("identity was constrained by the outer match"),
                }
            }
            _ => {
                return Err(
                    CpuSpecialNonTrappingIntegerExecutionError::UnsupportedIdentity(identity),
                )
            }
        };

        self.set_gpr(usize::from(instruction.rd()), value)?;

        Ok(CpuSpecialNonTrappingIntegerExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_special_trapping_integer_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuSpecialTrappingIntegerExecutionOutcome, CpuSpecialTrappingIntegerExecutionError>
    {
        let value = match identity {
            CpuInstructionIdentity::SpecialAdd
            | CpuInstructionIdentity::SpecialSub
            | CpuInstructionIdentity::SpecialDadd
            | CpuInstructionIdentity::SpecialDsub => {
                let rs = read_gpr_value(self, instruction.rs());
                let rt = read_gpr_value(self, instruction.rt());
                match identity {
                    CpuInstructionIdentity::SpecialAdd => {
                        let value = i64::from(i32_from_u32_bits(rs as u32))
                            + i64::from(i32_from_u32_bits(rt as u32));
                        if signed_i32_result_out_of_range(value) {
                            return Ok(CpuSpecialTrappingIntegerExecutionOutcome::Overflow(
                                CpuSpecialTrappingIntegerOverflow {
                                    identity,
                                    rd: instruction.rd(),
                                    rs_value: rs,
                                    rt_value: rt,
                                },
                            ));
                        }
                        sign_extend_u32_to_cpu_value(u32_bits_from_i32_value(value))
                    }
                    CpuInstructionIdentity::SpecialSub => {
                        let value = i64::from(i32_from_u32_bits(rs as u32))
                            - i64::from(i32_from_u32_bits(rt as u32));
                        if signed_i32_result_out_of_range(value) {
                            return Ok(CpuSpecialTrappingIntegerExecutionOutcome::Overflow(
                                CpuSpecialTrappingIntegerOverflow {
                                    identity,
                                    rd: instruction.rd(),
                                    rs_value: rs,
                                    rt_value: rt,
                                },
                            ));
                        }
                        sign_extend_u32_to_cpu_value(u32_bits_from_i32_value(value))
                    }
                    CpuInstructionIdentity::SpecialDadd => {
                        let value = rs.wrapping_add(rt);
                        if signed_cpu_add_overflows(rs, rt, value) {
                            return Ok(CpuSpecialTrappingIntegerExecutionOutcome::Overflow(
                                CpuSpecialTrappingIntegerOverflow {
                                    identity,
                                    rd: instruction.rd(),
                                    rs_value: rs,
                                    rt_value: rt,
                                },
                            ));
                        }
                        value
                    }
                    CpuInstructionIdentity::SpecialDsub => {
                        let value = rs.wrapping_sub(rt);
                        if signed_cpu_sub_overflows(rs, rt, value) {
                            return Ok(CpuSpecialTrappingIntegerExecutionOutcome::Overflow(
                                CpuSpecialTrappingIntegerOverflow {
                                    identity,
                                    rd: instruction.rd(),
                                    rs_value: rs,
                                    rt_value: rt,
                                },
                            ));
                        }
                        value
                    }
                    _ => unreachable!("identity was constrained by the outer match"),
                }
            }
            _ => {
                return Err(CpuSpecialTrappingIntegerExecutionError::UnsupportedIdentity(identity))
            }
        };

        self.set_gpr(usize::from(instruction.rd()), value)?;

        Ok(CpuSpecialTrappingIntegerExecutionOutcome::Executed(
            CpuSpecialTrappingIntegerExecutedInstruction { identity },
        ))
    }

    #[allow(dead_code)]
    pub(crate) fn execute_immediate_trapping_integer_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<
        CpuImmediateTrappingIntegerExecutionOutcome,
        CpuImmediateTrappingIntegerExecutionError,
    > {
        let rs = read_gpr_value(self, instruction.rs());
        let immediate_u16 = instruction.immediate_u16();
        let immediate_value = sign_extend_u16_to_cpu_value(immediate_u16);

        let value = match identity {
            CpuInstructionIdentity::Addi => {
                let value = i64::from(i32_from_u32_bits(rs as u32))
                    + i64::from(i16_from_u16_bits(immediate_u16));
                if signed_i32_result_out_of_range(value) {
                    return Ok(CpuImmediateTrappingIntegerExecutionOutcome::Overflow(
                        CpuImmediateTrappingIntegerOverflow {
                            identity,
                            rt: instruction.rt(),
                            rs_value: rs,
                            immediate_u16,
                            immediate_value,
                        },
                    ));
                }
                sign_extend_u32_to_cpu_value(u32_bits_from_i32_value(value))
            }
            CpuInstructionIdentity::Daddi => {
                let value = rs.wrapping_add(immediate_value);
                if signed_cpu_add_overflows(rs, immediate_value, value) {
                    return Ok(CpuImmediateTrappingIntegerExecutionOutcome::Overflow(
                        CpuImmediateTrappingIntegerOverflow {
                            identity,
                            rt: instruction.rt(),
                            rs_value: rs,
                            immediate_u16,
                            immediate_value,
                        },
                    ));
                }
                value
            }
            _ => {
                return Err(
                    CpuImmediateTrappingIntegerExecutionError::UnsupportedIdentity(identity),
                )
            }
        };

        self.set_gpr(usize::from(instruction.rt()), value)?;

        Ok(CpuImmediateTrappingIntegerExecutionOutcome::Executed(
            CpuImmediateTrappingIntegerExecutedInstruction { identity },
        ))
    }

    #[allow(dead_code)]
    pub(crate) fn execute_immediate_non_trapping_integer_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<
        CpuImmediateNonTrappingIntegerExecutedInstruction,
        CpuImmediateNonTrappingIntegerExecutionError,
    > {
        let rs = read_gpr_value(self, instruction.rs());
        let immediate_u16 = instruction.immediate_u16();

        let value = match identity {
            CpuInstructionIdentity::Addiu => sign_extend_u32_to_cpu_value(
                (rs as u32).wrapping_add(sign_extend_u16_to_u32(immediate_u16)),
            ),
            CpuInstructionIdentity::Daddiu => {
                rs.wrapping_add(sign_extend_u16_to_cpu_value(immediate_u16))
            }
            _ => {
                return Err(
                    CpuImmediateNonTrappingIntegerExecutionError::UnsupportedIdentity(identity),
                )
            }
        };

        self.set_gpr(usize::from(instruction.rt()), value)?;

        Ok(CpuImmediateNonTrappingIntegerExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_immediate_comparison_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuImmediateComparisonExecutedInstruction, CpuImmediateComparisonExecutionError>
    {
        let rs = read_gpr_value(self, instruction.rs());
        let immediate_value = sign_extend_u16_to_cpu_value(instruction.immediate_u16());

        let value = match identity {
            CpuInstructionIdentity::Slti => {
                cpu_value_from_bool(signed_cpu_value_less_than(rs, immediate_value))
            }
            CpuInstructionIdentity::Sltiu => cpu_value_from_bool(rs < immediate_value),
            _ => {
                return Err(CpuImmediateComparisonExecutionError::UnsupportedIdentity(
                    identity,
                ))
            }
        };

        self.set_gpr(usize::from(instruction.rt()), value)?;

        Ok(CpuImmediateComparisonExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_immediate_bitwise_logical_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<
        CpuImmediateBitwiseLogicalExecutedInstruction,
        CpuImmediateBitwiseLogicalExecutionError,
    > {
        let rs = read_gpr_value(self, instruction.rs());
        let immediate_value = u64::from(instruction.immediate_u16());

        let value = match identity {
            CpuInstructionIdentity::Andi => rs & immediate_value,
            CpuInstructionIdentity::Ori => rs | immediate_value,
            CpuInstructionIdentity::Xori => rs ^ immediate_value,
            _ => {
                return Err(CpuImmediateBitwiseLogicalExecutionError::UnsupportedIdentity(identity))
            }
        };

        self.set_gpr(usize::from(instruction.rt()), value)?;

        Ok(CpuImmediateBitwiseLogicalExecutedInstruction { identity })
    }

    #[allow(dead_code)]
    pub(crate) fn execute_upper_immediate_instruction(
        &mut self,
        identity: CpuInstructionIdentity,
        instruction: CpuInstructionFields,
    ) -> Result<CpuUpperImmediateExecutedInstruction, CpuUpperImmediateExecutionError> {
        let value = match identity {
            CpuInstructionIdentity::Lui => {
                sign_extend_u32_to_cpu_value(u32::from(instruction.immediate_u16()) << 16)
            }
            _ => {
                return Err(CpuUpperImmediateExecutionError::UnsupportedIdentity(
                    identity,
                ))
            }
        };

        self.set_gpr(usize::from(instruction.rt()), value)?;

        Ok(CpuUpperImmediateExecutedInstruction { identity })
    }
}

fn map_special_shift_invocation_error(
    error: CpuSpecialShiftExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuSpecialShiftExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuSpecialShiftExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_special_bitwise_logical_invocation_error(
    error: CpuSpecialBitwiseLogicalExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuSpecialBitwiseLogicalExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuSpecialBitwiseLogicalExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_special_hi_lo_transfer_invocation_error(
    error: CpuSpecialHiLoTransferExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuSpecialHiLoTransferExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuSpecialHiLoTransferExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_special_non_trapping_integer_invocation_error(
    error: CpuSpecialNonTrappingIntegerExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuSpecialNonTrappingIntegerExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuSpecialNonTrappingIntegerExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_special_trapping_integer_invocation_error(
    error: CpuSpecialTrappingIntegerExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuSpecialTrappingIntegerExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuSpecialTrappingIntegerExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_immediate_trapping_integer_invocation_error(
    error: CpuImmediateTrappingIntegerExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuImmediateTrappingIntegerExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuImmediateTrappingIntegerExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_immediate_non_trapping_integer_invocation_error(
    error: CpuImmediateNonTrappingIntegerExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuImmediateNonTrappingIntegerExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuImmediateNonTrappingIntegerExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_immediate_comparison_invocation_error(
    error: CpuImmediateComparisonExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuImmediateComparisonExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuImmediateComparisonExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_immediate_bitwise_logical_invocation_error(
    error: CpuImmediateBitwiseLogicalExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuImmediateBitwiseLogicalExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuImmediateBitwiseLogicalExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn map_upper_immediate_invocation_error(
    error: CpuUpperImmediateExecutionError,
    selection: CpuLocalExecutedHelperSelection,
) -> CpuLocalExecutedHelperInvocationError {
    match error {
        CpuUpperImmediateExecutionError::UnsupportedIdentity(_) => {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection)
        }
        CpuUpperImmediateExecutionError::RegisterIndex(error) => {
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error)
        }
    }
}

fn read_gpr_word(cpu: &Cpu, index: u8) -> u32 {
    cpu.gpr(usize::from(index))
        .expect("decoded CPU register index is five bits") as u32
}

fn read_gpr_value(cpu: &Cpu, index: u8) -> u64 {
    cpu.gpr(usize::from(index))
        .expect("decoded CPU register index is five bits")
}

fn variable_shift_amount_u32(value: u32) -> u8 {
    (value & 0x1f) as u8
}

fn variable_shift_amount_cpu_value(value: u64) -> u8 {
    (value & 0x3f) as u8
}

fn arithmetic_shift_right_u32(value: u32, sa: u8) -> u32 {
    ((value as i32) >> u32::from(sa)) as u32
}

fn arithmetic_shift_right_cpu_value(value: u64, sa: u8) -> u64 {
    ((value as i64) >> u32::from(sa)) as u64
}

pub(crate) fn signed_cpu_value_less_than(lhs: u64, rhs: u64) -> bool {
    (lhs as i64) < (rhs as i64)
}

fn signed_cpu_add_overflows(lhs: u64, rhs: u64, result: u64) -> bool {
    const SIGN_BIT: u64 = 0x8000_0000_0000_0000;
    ((!(lhs ^ rhs) & (lhs ^ result)) & SIGN_BIT) != 0
}

fn signed_cpu_sub_overflows(lhs: u64, rhs: u64, result: u64) -> bool {
    const SIGN_BIT: u64 = 0x8000_0000_0000_0000;
    (((lhs ^ rhs) & (lhs ^ result)) & SIGN_BIT) != 0
}

fn i32_from_u32_bits(value: u32) -> i32 {
    value as i32
}

fn i16_from_u16_bits(value: u16) -> i16 {
    value as i16
}

fn signed_i32_result_out_of_range(value: i64) -> bool {
    value < i64::from(i32::MIN) || value > i64::from(i32::MAX)
}

fn u32_bits_from_i32_value(value: i64) -> u32 {
    (value as i32) as u32
}

fn cpu_value_from_bool(value: bool) -> u64 {
    u64::from(value)
}

fn sign_extend_u32_to_cpu_value(value: u32) -> u64 {
    if (value & 0x8000_0000) == 0 {
        u64::from(value)
    } else {
        0xffff_ffff_0000_0000 | u64::from(value)
    }
}

fn sign_extend_u16_to_u32(value: u16) -> u32 {
    if (value & 0x8000) == 0 {
        u32::from(value)
    } else {
        0xffff_0000 | u32::from(value)
    }
}

fn sign_extend_u16_to_cpu_value(value: u16) -> u64 {
    if (value & 0x8000) == 0 {
        u64::from(value)
    } else {
        0xffff_ffff_ffff_0000 | u64::from(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cartridge, Machine};

    fn decode(bits: u32) -> CpuInstructionFields {
        decode_cpu_instruction_word(CpuInstructionWord::new(bits))
    }

    fn identify(bits: u32) -> CpuInstructionIdentity {
        identify_cpu_instruction(decode(bits))
    }

    fn with_primary_opcode(opcode: u8) -> u32 {
        u32::from(opcode) << 26
    }

    fn with_special_funct(funct: u8) -> u32 {
        (0x05 << 21) | (0x06 << 16) | (0x07 << 11) | (0x08 << 6) | u32::from(funct)
    }

    fn with_regimm_rt(rt: u8) -> u32 {
        (0x01 << 26) | (0x05 << 21) | (u32::from(rt) << 16) | 0x89ab
    }

    fn special_shift_word(rs: u8, rt: u8, rd: u8, sa: u8, funct: u8) -> u32 {
        (u32::from(rs) << 21)
            | (u32::from(rt) << 16)
            | (u32::from(rd) << 11)
            | (u32::from(sa) << 6)
            | u32::from(funct)
    }

    fn immediate_word(opcode: u8, rs: u8, rt: u8, immediate: u16) -> u32 {
        (u32::from(opcode) << 26)
            | (u32::from(rs) << 21)
            | (u32::from(rt) << 16)
            | u32::from(immediate)
    }

    fn execute_special_shift(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuSpecialShiftExecutedInstruction, CpuSpecialShiftExecutionError> {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_special_shift_instruction(identity, fields)
    }

    fn execute_special_bitwise_logical(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuSpecialBitwiseLogicalExecutedInstruction, CpuSpecialBitwiseLogicalExecutionError>
    {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_special_bitwise_logical_instruction(identity, fields)
    }

    fn execute_special_hi_lo_transfer(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuSpecialHiLoTransferExecutedInstruction, CpuSpecialHiLoTransferExecutionError>
    {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_special_hi_lo_transfer_instruction(identity, fields)
    }

    fn execute_special_non_trapping_integer(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<
        CpuSpecialNonTrappingIntegerExecutedInstruction,
        CpuSpecialNonTrappingIntegerExecutionError,
    > {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_special_non_trapping_integer_instruction(identity, fields)
    }

    fn execute_special_trapping_integer(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuSpecialTrappingIntegerExecutionOutcome, CpuSpecialTrappingIntegerExecutionError>
    {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_special_trapping_integer_instruction(identity, fields)
    }

    fn execute_immediate_trapping_integer(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<
        CpuImmediateTrappingIntegerExecutionOutcome,
        CpuImmediateTrappingIntegerExecutionError,
    > {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_immediate_trapping_integer_instruction(identity, fields)
    }

    fn execute_immediate_non_trapping_integer(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<
        CpuImmediateNonTrappingIntegerExecutedInstruction,
        CpuImmediateNonTrappingIntegerExecutionError,
    > {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_immediate_non_trapping_integer_instruction(identity, fields)
    }

    fn execute_immediate_comparison(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuImmediateComparisonExecutedInstruction, CpuImmediateComparisonExecutionError>
    {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_immediate_comparison_instruction(identity, fields)
    }

    fn execute_immediate_bitwise_logical(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<
        CpuImmediateBitwiseLogicalExecutedInstruction,
        CpuImmediateBitwiseLogicalExecutionError,
    > {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_immediate_bitwise_logical_instruction(identity, fields)
    }

    fn execute_upper_immediate(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuUpperImmediateExecutedInstruction, CpuUpperImmediateExecutionError> {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);

        cpu.execute_upper_immediate_instruction(identity, fields)
    }

    fn assert_cpu_local_executed_helper_selection(
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) {
        let selection = select_cpu_local_executed_helper(identity)
            .expect("identity should select a sealed CPU-local executed helper family");

        assert_eq!(selection.identity(), identity);
        assert_eq!(selection.family(), family);
    }

    fn select_cpu_local_executed_helper_for_test(
        identity: CpuInstructionIdentity,
    ) -> CpuLocalExecutedHelperSelection {
        select_cpu_local_executed_helper(identity)
            .expect("identity should select a sealed CPU-local executed helper family")
    }

    fn invoke_cpu_local_executed_helper(
        cpu: &mut Cpu,
        bits: u32,
    ) -> Result<CpuLocalExecutedHelperInvocationOutcome, CpuLocalExecutedHelperInvocationError>
    {
        let fields = decode(bits);
        let identity = identify_cpu_instruction(fields);
        let selection = select_cpu_local_executed_helper_for_test(identity);

        cpu.invoke_cpu_local_executed_helper(fields, selection)
    }

    fn cpu_control_and_cop0_state(cpu: &Cpu) -> (u32, u32, u32, u32, bool, u32, u32, u8, bool) {
        (
            cpu.pc(),
            cpu.next_pc(),
            cpu.cop0_count(),
            cpu.cop0_compare(),
            cpu.cop0_timer_interrupt_pending(),
            cpu.cop0_status(),
            cpu.cop0_epc(),
            cpu.cop0_exception_code(),
            cpu.cop0_exception_branch_delay(),
        )
    }

    fn cpu_gprs(cpu: &Cpu) -> [u64; 32] {
        core::array::from_fn(|index| cpu.gpr(index).expect("index is within the GPR file"))
    }

    fn assert_executed_invocation(
        outcome: CpuLocalExecutedHelperInvocationOutcome,
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) {
        assert_eq!(outcome.identity(), identity);
        assert_eq!(outcome.family(), family);
        assert!(outcome.is_executed());
        assert!(!outcome.is_arithmetic_overflow());
        match outcome {
            CpuLocalExecutedHelperInvocationOutcome::Executed(executed) => {
                assert_eq!(executed.identity(), identity);
                assert_eq!(executed.family(), family);
            }
            CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(_) => {
                panic!("expected executed invocation outcome")
            }
        }
    }

    fn assert_arithmetic_overflow_invocation(
        outcome: CpuLocalExecutedHelperInvocationOutcome,
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) -> CpuLocalExecutedHelperArithmeticOverflow {
        assert_eq!(outcome.identity(), identity);
        assert_eq!(outcome.family(), family);
        assert!(!outcome.is_executed());
        assert!(outcome.is_arithmetic_overflow());
        match outcome {
            CpuLocalExecutedHelperInvocationOutcome::Executed(_) => {
                panic!("expected arithmetic overflow invocation outcome")
            }
            CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(overflow) => {
                assert_eq!(overflow.identity(), identity);
                assert_eq!(overflow.family(), family);
                overflow
            }
        }
    }

    #[test]
    fn raw_word_is_preserved_exactly() {
        let fields = decode(0x8cc5_0104);

        assert_eq!(fields.raw(), CpuInstructionWord::new(0x8cc5_0104));
        assert_eq!(fields.raw().bits(), 0x8cc5_0104);
    }

    #[test]
    fn all_zero_word_decodes_all_fields_to_zero() {
        let fields = decode(0);

        assert_eq!(fields.raw().bits(), 0);
        assert_eq!(fields.opcode(), 0);
        assert_eq!(fields.rs(), 0);
        assert_eq!(fields.rt(), 0);
        assert_eq!(fields.rd(), 0);
        assert_eq!(fields.sa(), 0);
        assert_eq!(fields.funct(), 0);
        assert_eq!(fields.immediate_u16(), 0);
        assert_eq!(fields.jump_target(), 0);
    }

    #[test]
    fn all_one_word_decodes_cpp_field_masks() {
        let fields = decode(u32::MAX);

        assert_eq!(fields.opcode(), 0x3f);
        assert_eq!(fields.rs(), 0x1f);
        assert_eq!(fields.rt(), 0x1f);
        assert_eq!(fields.rd(), 0x1f);
        assert_eq!(fields.sa(), 0x1f);
        assert_eq!(fields.funct(), 0x3f);
        assert_eq!(fields.immediate_u16(), 0xffff);
        assert_eq!(fields.jump_target(), 0x03ff_ffff);
    }

    #[test]
    fn individual_field_extraction_uses_cpp_bit_positions() {
        let raw = (0x15 << 26) | (0x0a << 21) | (0x0b << 16) | (0x0c << 11) | (0x0d << 6) | 0x2e;
        let fields = decode(raw);

        assert_eq!(fields.opcode(), 0x15);
        assert_eq!(fields.rs(), 0x0a);
        assert_eq!(fields.rt(), 0x0b);
        assert_eq!(fields.rd(), 0x0c);
        assert_eq!(fields.sa(), 0x0d);
        assert_eq!(fields.funct(), 0x2e);
        assert_eq!(fields.immediate_u16(), (raw & 0xffff) as u16);
        assert_eq!(fields.jump_target(), raw & 0x03ff_ffff);
    }

    #[test]
    fn representative_r_type_word_extracts_raw_fields() {
        let fields = decode(0x00a6_3820);

        assert_eq!(fields.opcode(), 0x00);
        assert_eq!(fields.rs(), 0x05);
        assert_eq!(fields.rt(), 0x06);
        assert_eq!(fields.rd(), 0x07);
        assert_eq!(fields.sa(), 0x00);
        assert_eq!(fields.funct(), 0x20);
        assert_eq!(fields.immediate_u16(), 0x3820);
        assert_eq!(fields.jump_target(), 0x00a6_3820);
    }

    #[test]
    fn representative_i_type_word_extracts_raw_fields() {
        let fields = decode(0x34c4_1234);

        assert_eq!(fields.opcode(), 0x0d);
        assert_eq!(fields.rs(), 0x06);
        assert_eq!(fields.rt(), 0x04);
        assert_eq!(fields.rd(), 0x02);
        assert_eq!(fields.sa(), 0x08);
        assert_eq!(fields.funct(), 0x34);
        assert_eq!(fields.immediate_u16(), 0x1234);
        assert_eq!(fields.jump_target(), 0x00c4_1234);
    }

    #[test]
    fn representative_j_type_word_extracts_raw_target() {
        let fields = decode(0x0812_3456);

        assert_eq!(fields.opcode(), 0x02);
        assert_eq!(fields.jump_target(), 0x0012_3456);
        assert_eq!(fields.immediate_u16(), 0x3456);
    }

    #[test]
    fn primary_opcode_identity_classification_matches_cpp_switch() {
        use CpuInstructionIdentity::*;

        let cases = [
            (0x02, J),
            (0x03, Jal),
            (0x04, Beq),
            (0x05, Bne),
            (0x06, Blez),
            (0x07, Bgtz),
            (0x08, Addi),
            (0x09, Addiu),
            (0x0a, Slti),
            (0x0b, Sltiu),
            (0x0c, Andi),
            (0x0d, Ori),
            (0x0e, Xori),
            (0x0f, Lui),
            (0x11, Cop1),
            (0x12, Cop2),
            (0x13, Cop3),
            (0x14, Beql),
            (0x15, Bnel),
            (0x16, Blezl),
            (0x17, Bgtzl),
            (0x18, Daddi),
            (0x19, Daddiu),
            (0x1a, Ldl),
            (0x1b, Ldr),
            (0x20, Lb),
            (0x21, Lh),
            (0x22, Lwl),
            (0x23, Lw),
            (0x24, Lbu),
            (0x25, Lhu),
            (0x26, Lwr),
            (0x27, Lwu),
            (0x28, Sb),
            (0x29, Sh),
            (0x2a, Swl),
            (0x2b, Sw),
            (0x2c, Sdl),
            (0x2d, Sdr),
            (0x2e, Swr),
            (0x2f, Cache),
            (0x30, Ll),
            (0x31, Lwc1),
            (0x32, Lwc2),
            (0x34, Lld),
            (0x35, Ldc1),
            (0x36, Ldc2),
            (0x37, Ld),
            (0x38, Sc),
            (0x39, Swc1),
            (0x3a, Swc2),
            (0x3c, Scd),
            (0x3d, Sdc1),
            (0x3e, Sdc2),
            (0x3f, Sd),
        ];

        for (opcode, expected) in cases {
            assert_eq!(identify(with_primary_opcode(opcode)), expected);
        }
    }

    #[test]
    fn special_opcode_uses_funct_field_and_unknown_matches_cpp_switch() {
        use CpuInstructionIdentity::*;

        let cases = [
            (0x00, SpecialSll),
            (0x02, SpecialSrl),
            (0x03, SpecialSra),
            (0x04, SpecialSllv),
            (0x06, SpecialSrlv),
            (0x07, SpecialSrav),
            (0x08, SpecialJr),
            (0x09, SpecialJalr),
            (0x0c, SpecialSyscall),
            (0x0d, SpecialBreak),
            (0x0f, SpecialSync),
            (0x10, SpecialMfhi),
            (0x11, SpecialMthi),
            (0x12, SpecialMflo),
            (0x13, SpecialMtlo),
            (0x14, SpecialDsllv),
            (0x16, SpecialDsrlv),
            (0x17, SpecialDsrav),
            (0x18, SpecialMult),
            (0x19, SpecialMultu),
            (0x1a, SpecialDiv),
            (0x1b, SpecialDivu),
            (0x1c, SpecialDmult),
            (0x1d, SpecialDmultu),
            (0x1e, SpecialDdiv),
            (0x1f, SpecialDdivu),
            (0x20, SpecialAdd),
            (0x21, SpecialAddu),
            (0x22, SpecialSub),
            (0x23, SpecialSubu),
            (0x24, SpecialAnd),
            (0x25, SpecialOr),
            (0x26, SpecialXor),
            (0x27, SpecialNor),
            (0x2a, SpecialSlt),
            (0x2b, SpecialSltu),
            (0x2c, SpecialDadd),
            (0x2d, SpecialDaddu),
            (0x2e, SpecialDsub),
            (0x2f, SpecialDsubu),
            (0x30, SpecialTge),
            (0x31, SpecialTgeu),
            (0x32, SpecialTlt),
            (0x33, SpecialTltu),
            (0x34, SpecialTeq),
            (0x36, SpecialTne),
            (0x38, SpecialDsll),
            (0x3a, SpecialDsrl),
            (0x3b, SpecialDsra),
            (0x3c, SpecialDsll32),
            (0x3e, SpecialDsrl32),
            (0x3f, SpecialDsra32),
        ];

        for (funct, expected) in cases {
            assert_eq!(identify(with_special_funct(funct)), expected);
        }
        assert_eq!(identify(with_special_funct(0x01)), SpecialUnknown);
    }

    #[test]
    fn regimm_opcode_uses_rt_field_and_unknown_matches_cpp_switch() {
        use CpuInstructionIdentity::*;

        let cases = [
            (0x00, RegimmBltz),
            (0x01, RegimmBgez),
            (0x02, RegimmBltzl),
            (0x03, RegimmBgezl),
            (0x08, RegimmTgei),
            (0x09, RegimmTgeiu),
            (0x0a, RegimmTlti),
            (0x0b, RegimmTltiu),
            (0x0c, RegimmTeqi),
            (0x0e, RegimmTnei),
            (0x10, RegimmBltzal),
            (0x11, RegimmBgezal),
            (0x12, RegimmBltzall),
            (0x13, RegimmBgezall),
        ];

        for (rt, expected) in cases {
            assert_eq!(identify(with_regimm_rt(rt)), expected);
        }
        assert_eq!(identify(with_regimm_rt(0x04)), RegimmUnknown);
    }

    #[test]
    fn cop0_identity_classification_uses_rs_and_exact_eret_raw_word() {
        use CpuInstructionIdentity::*;

        assert_eq!(identify(0x10 << 26), Cop0Mfc0);
        assert_eq!(identify((0x10 << 26) | (0x04 << 21)), Cop0Mtc0);
        assert_eq!(identify(0x4200_0018), Cop0Eret);
        assert_eq!(identify(0x4200_0019), Cop0);
        assert_eq!(identify((0x10 << 26) | (0x01 << 21)), Cop0);
    }

    #[test]
    fn nop_classifies_as_special_sll_like_cpp() {
        assert_eq!(identify(0), CpuInstructionIdentity::SpecialSll);
    }

    #[test]
    fn unknown_primary_opcode_matches_cpp_default() {
        assert_eq!(
            identify(with_primary_opcode(0x1c)),
            CpuInstructionIdentity::UnknownPrimary
        );
    }

    #[test]
    fn identity_classification_does_not_interpret_operands() {
        assert_eq!(identify(0x2084_ffff), CpuInstructionIdentity::Addi);
        assert_eq!(identify(0x2084_0001), CpuInstructionIdentity::Addi);
        assert_eq!(identify(0x0800_0001), CpuInstructionIdentity::J);
        assert_eq!(identify(0x0bff_ffff), CpuInstructionIdentity::J);
    }

    #[test]
    fn local_executed_helper_selection_maps_sealed_shift_identities() {
        use CpuInstructionIdentity::*;

        for identity in [
            SpecialSll,
            SpecialSrl,
            SpecialSra,
            SpecialSllv,
            SpecialSrlv,
            SpecialSrav,
            SpecialDsll,
            SpecialDsrl,
            SpecialDsra,
            SpecialDsll32,
            SpecialDsrl32,
            SpecialDsra32,
            SpecialDsllv,
            SpecialDsrlv,
            SpecialDsrav,
        ] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialShift,
            );
        }
    }

    #[test]
    fn local_executed_helper_selection_maps_sealed_special_register_families() {
        use CpuInstructionIdentity::*;

        for identity in [SpecialAnd, SpecialOr, SpecialXor, SpecialNor] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialBitwiseLogical,
            );
        }

        for identity in [SpecialMfhi, SpecialMthi, SpecialMflo, SpecialMtlo] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialHiLoTransfer,
            );
        }

        for identity in [SpecialMultu, SpecialDmultu] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialMultiply,
            );
        }

        for identity in [SpecialDiv, SpecialDdivu] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialDivide,
            );
        }

        for identity in [
            SpecialAddu,
            SpecialSubu,
            SpecialDaddu,
            SpecialDsubu,
            SpecialSlt,
            SpecialSltu,
        ] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger,
            );
        }

        for identity in [SpecialAdd, SpecialSub, SpecialDadd, SpecialDsub] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::SpecialTrappingInteger,
            );
        }
    }

    #[test]
    fn local_executed_helper_selection_maps_sealed_immediate_and_no_effect_families() {
        use CpuInstructionIdentity::*;

        assert_cpu_local_executed_helper_selection(
            SpecialSync,
            CpuLocalExecutedHelperFamily::NoEffectSync,
        );

        for identity in [Addi, Daddi] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::ImmediateTrappingInteger,
            );
        }

        for identity in [Addiu, Daddiu] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger,
            );
        }

        for identity in [Slti, Sltiu] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::ImmediateComparison,
            );
        }

        for identity in [Andi, Ori, Xori] {
            assert_cpu_local_executed_helper_selection(
                identity,
                CpuLocalExecutedHelperFamily::ImmediateBitwiseLogical,
            );
        }

        assert_cpu_local_executed_helper_selection(
            Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );
    }

    #[test]
    fn local_executed_helper_selection_excludes_unsealed_or_non_executed_identities() {
        use CpuInstructionIdentity::*;

        let excluded = [
            UnknownPrimary,
            SpecialUnknown,
            RegimmUnknown,
            SpecialJr,
            SpecialJalr,
            SpecialSyscall,
            SpecialBreak,
            SpecialMult,
            SpecialDivu,
            SpecialDmult,
            SpecialDdiv,
            SpecialTge,
            SpecialTgeu,
            SpecialTlt,
            SpecialTltu,
            SpecialTeq,
            SpecialTne,
            RegimmBltz,
            RegimmBgez,
            RegimmBltzl,
            RegimmBgezl,
            RegimmTgei,
            RegimmTgeiu,
            RegimmTlti,
            RegimmTltiu,
            RegimmTeqi,
            RegimmTnei,
            RegimmBltzal,
            RegimmBgezal,
            RegimmBltzall,
            RegimmBgezall,
            J,
            Jal,
            Beq,
            Bne,
            Blez,
            Bgtz,
            Cop0,
            Cop0Mfc0,
            Cop0Mtc0,
            Cop1,
            Cop2,
            Cop3,
            Beql,
            Bnel,
            Blezl,
            Bgtzl,
            Ldl,
            Ldr,
            Lb,
            Lh,
            Lwl,
            Lw,
            Lbu,
            Lhu,
            Lwr,
            Lwu,
            Sb,
            Sh,
            Swl,
            Sw,
            Sdl,
            Sdr,
            Swr,
            Cache,
            Ll,
            Lwc1,
            Lwc2,
            Lld,
            Ldc1,
            Ldc2,
            Ld,
            Sc,
            Swc1,
            Swc2,
            Scd,
            Sdc1,
            Sdc2,
            Sd,
        ];

        for identity in excluded {
            assert_eq!(select_cpu_local_executed_helper(identity), None);
        }
    }

    #[test]
    fn local_executed_helper_selection_performs_no_cpu_mutation() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let control_before = (
            cpu.pc(),
            cpu.next_pc(),
            cpu.hi(),
            cpu.lo(),
            cpu.gpr(0),
            cpu.gpr(8),
        );
        let cop0_before = (
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

        assert_cpu_local_executed_helper_selection(
            CpuInstructionIdentity::SpecialSll,
            CpuLocalExecutedHelperFamily::SpecialShift,
        );
        assert_cpu_local_executed_helper_selection(
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );
        assert_eq!(
            select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialSyscall),
            None
        );

        assert_eq!(
            control_before,
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
                cpu.cop0_count(),
                cpu.cop0_compare(),
                cpu.cop0_timer_interrupt_pending(),
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
    fn local_executed_helper_invocation_no_effect_sync_mutates_nothing() {
        let mut cpu = Cpu::new();
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let control_cop0_before = cpu_control_and_cop0_state(&cpu);
        let gprs_before = cpu_gprs(&cpu);
        let hi_lo_before = (cpu.hi(), cpu.lo());

        let outcome = invoke_cpu_local_executed_helper(&mut cpu, 0x0000_000f)
            .expect("SYNC should invoke as a no-effect executed helper");

        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialSync,
            CpuLocalExecutedHelperFamily::NoEffectSync,
        );
        assert_eq!(control_cop0_before, cpu_control_and_cop0_state(&cpu));
        assert_eq!(hi_lo_before, (cpu.hi(), cpu.lo()));
        assert_eq!(gprs_before, cpu_gprs(&cpu));
    }

    #[test]
    fn local_executed_helper_invocation_calls_representative_executed_helpers() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(2, 0x0000_0000_0000_0003), Ok(()));
        let before = cpu_control_and_cop0_state(&cpu);
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(0, 2, 3, 2, 0x00))
                .expect("SLL should invoke through the shift helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialSll,
            CpuLocalExecutedHelperFamily::SpecialShift,
        );
        assert_eq!(cpu.gpr(3), Some(0x0000_0000_0000_000c));
        assert_eq!(before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 65), Ok(()));
        assert_eq!(cpu.set_gpr(2, 0x8000_0000_0000_0000), Ok(()));
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x17))
                .expect("DSRAV should invoke through the shift helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialDsrav,
            CpuLocalExecutedHelperFamily::SpecialShift,
        );
        assert_eq!(cpu.gpr(3), Some(0xc000_0000_0000_0000));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0x0101_0000_ffff_0000), Ok(()));
        assert_eq!(cpu.set_gpr(2, 0xf0f0_0000_0000_1234), Ok(()));
        let before = cpu_control_and_cop0_state(&cpu);
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x25))
                .expect("OR should invoke through the logical helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialOr,
            CpuLocalExecutedHelperFamily::SpecialBitwiseLogical,
        );
        assert_eq!(cpu.gpr(3), Some(0xf1f1_0000_ffff_1234));
        assert_eq!(before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        cpu.stage_hi(0x1234_5678_9abc_def0);
        let before = cpu_control_and_cop0_state(&cpu);
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(0, 0, 4, 0, 0x10))
                .expect("MFHI should invoke through the HI/LO transfer helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialMfhi,
            CpuLocalExecutedHelperFamily::SpecialHiLoTransfer,
        );
        assert_eq!(cpu.gpr(4), Some(0x1234_5678_9abc_def0));
        assert_eq!(cpu.hi(), 0x1234_5678_9abc_def0);
        assert_eq!(cpu.lo(), 0);
        assert_eq!(before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(5, 0xcafe_babe_dead_beef), Ok(()));
        let gprs_before = cpu_gprs(&cpu);
        let before = cpu_control_and_cop0_state(&cpu);
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(5, 0, 0, 0, 0x11))
                .expect("MTHI should invoke through the HI/LO transfer helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialMthi,
            CpuLocalExecutedHelperFamily::SpecialHiLoTransfer,
        );
        assert_eq!(cpu.hi(), 0xcafe_babe_dead_beef);
        assert_eq!(cpu.lo(), 0);
        assert_eq!(gprs_before, cpu_gprs(&cpu));
        assert_eq!(before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0xffff_ffff_ffff_ffff), Ok(()));
        assert_eq!(cpu.set_gpr(2, 2), Ok(()));
        let before = cpu_control_and_cop0_state(&cpu);
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x21))
                .expect("ADDU should invoke through the non-trapping integer helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialAddu,
            CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger,
        );
        assert_eq!(cpu.gpr(3), Some(1));
        assert_eq!(before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0xffff_ffff_ffff_fffe), Ok(()));
        assert_eq!(cpu.set_gpr(2, 1), Ok(()));
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x2b))
                .expect("SLTU should invoke through the non-trapping integer helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialSltu,
            CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger,
        );
        assert_eq!(cpu.gpr(3), Some(0));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0x0000_0000_0000_0004), Ok(()));
        assert_eq!(cpu.set_gpr(2, 0x0000_0000_0000_0005), Ok(()));
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x20))
                .expect("ADD should invoke through the trapping integer helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialAdd,
            CpuLocalExecutedHelperFamily::SpecialTrappingInteger,
        );
        assert_eq!(cpu.gpr(3), Some(9));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0x0000_0000_0000_0005), Ok(()));
        let outcome = invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x08, 1, 2, 4))
            .expect("ADDI should invoke through the immediate trapping helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::Addi,
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger,
        );
        assert_eq!(cpu.gpr(2), Some(9));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0xffff_ffff_ffff_ffff), Ok(()));
        let outcome = invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x19, 1, 2, 2))
            .expect("DADDIU should invoke through the immediate non-trapping helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::Daddiu,
            CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger,
        );
        assert_eq!(cpu.gpr(2), Some(1));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 4), Ok(()));
        let outcome = invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x0b, 1, 2, 5))
            .expect("SLTIU should invoke through the immediate comparison helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::Sltiu,
            CpuLocalExecutedHelperFamily::ImmediateComparison,
        );
        assert_eq!(cpu.gpr(2), Some(1));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0xffff_0000_0000_0000), Ok(()));
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x0e, 1, 2, 0xffff))
                .expect("XORI should invoke through the immediate logical helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::Xori,
            CpuLocalExecutedHelperFamily::ImmediateBitwiseLogical,
        );
        assert_eq!(cpu.gpr(2), Some(0xffff_0000_0000_ffff));

        let mut cpu = Cpu::new();
        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x0f, 0, 2, 0x8000))
                .expect("LUI should invoke through the upper-immediate helper");
        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );
        assert_eq!(cpu.gpr(2), Some(0xffff_ffff_8000_0000));
    }

    #[test]
    fn local_executed_helper_invocation_returns_overflow_without_exception_entry() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0x0000_0000_7fff_ffff), Ok(()));
        assert_eq!(cpu.set_gpr(2, 1), Ok(()));
        let gprs_before = cpu_gprs(&cpu);
        let control_cop0_before = cpu_control_and_cop0_state(&cpu);

        let outcome =
            invoke_cpu_local_executed_helper(&mut cpu, special_shift_word(1, 2, 3, 0, 0x20))
                .expect("ADD overflow should return a local overflow outcome");
        let overflow = assert_arithmetic_overflow_invocation(
            outcome,
            CpuInstructionIdentity::SpecialAdd,
            CpuLocalExecutedHelperFamily::SpecialTrappingInteger,
        );
        match overflow {
            CpuLocalExecutedHelperArithmeticOverflow::SpecialTrappingInteger(overflow) => {
                assert_eq!(overflow.rd(), 3);
                assert_eq!(overflow.rs_value(), 0x0000_0000_7fff_ffff);
                assert_eq!(overflow.rt_value(), 1);
            }
            CpuLocalExecutedHelperArithmeticOverflow::ImmediateTrappingInteger(_) => {
                panic!("expected SPECIAL overflow")
            }
        }
        assert_eq!(gprs_before, cpu_gprs(&cpu));
        assert_eq!(control_cop0_before, cpu_control_and_cop0_state(&cpu));

        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(1, 0x0000_0000_7fff_ffff), Ok(()));
        let gprs_before = cpu_gprs(&cpu);
        let control_cop0_before = cpu_control_and_cop0_state(&cpu);

        let outcome = invoke_cpu_local_executed_helper(&mut cpu, immediate_word(0x08, 1, 2, 1))
            .expect("ADDI overflow should return a local overflow outcome");
        let overflow = assert_arithmetic_overflow_invocation(
            outcome,
            CpuInstructionIdentity::Addi,
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger,
        );
        match overflow {
            CpuLocalExecutedHelperArithmeticOverflow::SpecialTrappingInteger(_) => {
                panic!("expected immediate overflow")
            }
            CpuLocalExecutedHelperArithmeticOverflow::ImmediateTrappingInteger(overflow) => {
                assert_eq!(overflow.rt(), 2);
                assert_eq!(overflow.rs_value(), 0x0000_0000_7fff_ffff);
                assert_eq!(overflow.immediate_u16(), 1);
                assert_eq!(overflow.immediate_value(), 1);
            }
        }
        assert_eq!(gprs_before, cpu_gprs(&cpu));
        assert_eq!(control_cop0_before, cpu_control_and_cop0_state(&cpu));
    }

    #[test]
    fn local_executed_helper_invocation_rejects_forged_invalid_selection() {
        let mut cpu = Cpu::new();
        let fields = decode(0x0000_000f);
        let forged = CpuLocalExecutedHelperSelection {
            identity: CpuInstructionIdentity::SpecialBreak,
            family: CpuLocalExecutedHelperFamily::NoEffectSync,
        };
        let error = cpu
            .invoke_cpu_local_executed_helper(fields, forged)
            .expect_err("forged NoEffectSync selection should be rejected");

        assert_eq!(
            error,
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(forged)
        );
    }

    #[test]
    fn local_executed_helper_invocation_error_maps_to_machine_rejection_plan() {
        use crate::machine::{
            classify_cpu_local_invocation_step_action, MachineCpuLocalInvocationStepAction,
        };

        let forged = CpuLocalExecutedHelperSelection {
            identity: CpuInstructionIdentity::SpecialBreak,
            family: CpuLocalExecutedHelperFamily::NoEffectSync,
        };
        let error = CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(forged);

        let plan = classify_cpu_local_invocation_step_action(Err(error));

        assert_eq!(
            plan.action(),
            MachineCpuLocalInvocationStepAction::RejectInvocationError
        );
        assert_eq!(plan.invocation_error(), Some(error));
        assert_eq!(plan.executed(), None);
        assert_eq!(plan.cadence_plan(), None);
        assert_eq!(plan.overflow(), None);
        assert!(!plan.mutates_state());
    }

    #[test]
    fn non_selected_identities_are_not_invokable_through_local_helper_path() {
        use CpuInstructionIdentity::*;

        for identity in [
            SpecialSyscall,
            SpecialBreak,
            UnknownPrimary,
            SpecialUnknown,
            RegimmUnknown,
            Cop1,
            SpecialJr,
            Beq,
            Lw,
            Sw,
            Cop0Mfc0,
            Cop0Mtc0,
            Ll,
            Sc,
        ] {
            assert_eq!(select_cpu_local_executed_helper(identity), None);
        }
    }

    #[test]
    fn decoding_does_not_mutate_cpu_machine_or_rdram_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();
        let pc_before = machine.cpu().pc();
        let next_pc_before = machine.cpu().next_pc();
        let gpr_before = machine.cpu().gpr(8);
        let cop0_status_before = machine.cpu().cop0_status();
        let cop0_bad_vaddr_before = machine.cpu().cop0_bad_vaddr();
        let cop0_exception_code_before = machine.cpu().cop0_exception_code();
        let rdram_before = machine.rdram().read_u32_be(0).unwrap();

        let fields = decode(0x8cc5_0104);

        assert_eq!(fields.opcode(), 0x23);
        assert_eq!(machine.cpu().pc(), pc_before);
        assert_eq!(machine.cpu().next_pc(), next_pc_before);
        assert_eq!(machine.cpu().gpr(8), gpr_before);
        assert_eq!(machine.cpu().cop0_status(), cop0_status_before);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), cop0_bad_vaddr_before);
        assert_eq!(
            machine.cpu().cop0_exception_code(),
            cop0_exception_code_before
        );
        assert_eq!(machine.rdram().read_u32_be(0).unwrap(), rdram_before);
    }

    #[test]
    fn identifying_does_not_mutate_cpu_machine_or_rdram_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, 0x5566_7788).unwrap();
        let pc_before = machine.cpu().pc();
        let next_pc_before = machine.cpu().next_pc();
        let gpr_before = machine.cpu().gpr(8);
        let cop0_status_before = machine.cpu().cop0_status();
        let cop0_bad_vaddr_before = machine.cpu().cop0_bad_vaddr();
        let cop0_exception_code_before = machine.cpu().cop0_exception_code();
        let rdram_before = machine.rdram().read_u32_be(0).unwrap();

        let identity = identify(0x8cc5_0104);

        assert_eq!(identity, CpuInstructionIdentity::Lw);
        assert_eq!(machine.cpu().pc(), pc_before);
        assert_eq!(machine.cpu().next_pc(), next_pc_before);
        assert_eq!(machine.cpu().gpr(8), gpr_before);
        assert_eq!(machine.cpu().cop0_status(), cop0_status_before);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), cop0_bad_vaddr_before);
        assert_eq!(
            machine.cpu().cop0_exception_code(),
            cop0_exception_code_before
        );
        assert_eq!(machine.rdram().read_u32_be(0).unwrap(), rdram_before);
    }

    #[test]
    fn special_sll_writes_sign_extended_word_result() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_4000_0001), Ok(()));

        let executed = execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, 1, 0x00))
            .expect("SLL should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSll);
        assert_eq!(cpu.gpr(4), Some(0x0000_0000_4000_0001));
        assert_eq!(cpu.gpr(5), Some(0xffff_ffff_8000_0002));
    }

    #[test]
    fn special_sll_zero_raw_nop_writes_zero_register_and_preserves_gprs() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0123_4567_89ab_cdef), Ok(()));
        assert_eq!(cpu.set_gpr(5, 0xfedc_ba98_7654_3210), Ok(()));

        let executed = execute_special_shift(&mut cpu, 0).expect("NOP/SLL should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSll);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(4), Some(0x0123_4567_89ab_cdef));
        assert_eq!(cpu.gpr(5), Some(0xfedc_ba98_7654_3210));
    }

    #[test]
    fn special_sll_reads_rt_before_writing_aliased_rd() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_0000_0001), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(0, 4, 4, 1, 0x00))
            .expect("SLL should execute");

        assert_eq!(cpu.gpr(4), Some(0x0000_0000_0000_0002));
    }

    #[test]
    fn special_sll_write_to_zero_register_is_ignored() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_1234_5678), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(0, 4, 0, 4, 0x00))
            .expect("SLL should execute");

        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(4), Some(0x0000_0000_1234_5678));
    }

    #[test]
    fn special_sll_sa_zero_copies_word_to_nonzero_rd_with_sign_extension() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0xffff_ffff_89ab_cdef), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, 0, 0x00))
            .expect("SLL should execute");

        assert_eq!(cpu.gpr(5), Some(0xffff_ffff_89ab_cdef));
    }

    #[test]
    fn special_srl_writes_logical_right_shift_sign_extended_word_result() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0xffff_ffff_8000_0000), Ok(()));

        let executed = execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, 1, 0x02))
            .expect("SRL should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSrl);
        assert_eq!(cpu.gpr(5), Some(0x0000_0000_4000_0000));
    }

    #[test]
    fn special_sra_writes_arithmetic_right_shift_sign_extended_word_result() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_8000_0000), Ok(()));

        let executed = execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, 4, 0x03))
            .expect("SRA should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSra);
        assert_eq!(cpu.gpr(5), Some(0xffff_ffff_f800_0000));
    }

    #[test]
    fn special_variable_shifts_use_rs_low_five_bits() {
        let cases = [
            (
                CpuInstructionIdentity::SpecialSllv,
                0x04,
                0x0000_0000_0000_0003,
                0x0000_0000_0000_0021,
                0x0000_0000_0000_0006,
            ),
            (
                CpuInstructionIdentity::SpecialSrlv,
                0x06,
                0xffff_ffff_8000_0000,
                0x0000_0000_0000_0021,
                0x0000_0000_4000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialSrav,
                0x07,
                0x0000_0000_8000_0000,
                0x0000_0000_0000_0024,
                0xffff_ffff_f800_0000,
            ),
        ];

        for (identity, funct, rt_value, rs_value, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rt_value), Ok(()));
            assert_eq!(cpu.set_gpr(5, rs_value), Ok(()));

            let executed = execute_special_shift(&mut cpu, special_shift_word(5, 4, 6, 0, funct))
                .expect("variable shift should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(6), Some(expected));
        }
    }

    #[test]
    fn special_variable_shift_reads_sources_before_writing_aliased_rd() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_0000_0021), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(4, 4, 4, 0, 0x04))
            .expect("SLLV should execute");

        assert_eq!(cpu.gpr(4), Some(0x0000_0000_0000_0042));
    }

    #[test]
    fn special_fixed_64_bit_shifts_write_full_width_results() {
        let cases = [
            (
                CpuInstructionIdentity::SpecialDsll,
                0x38,
                0x0000_0001_0000_0001,
                4,
                0x0000_0010_0000_0010,
            ),
            (
                CpuInstructionIdentity::SpecialDsrl,
                0x3a,
                0x8000_0000_0000_0000,
                4,
                0x0800_0000_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsra,
                0x3b,
                0x8000_0000_0000_0000,
                4,
                0xf800_0000_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsll32,
                0x3c,
                0x0000_0000_0000_0001,
                1,
                0x0000_0002_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsrl32,
                0x3e,
                0x8000_0000_0000_0000,
                1,
                0x0000_0000_4000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsra32,
                0x3f,
                0x8000_0000_0000_0000,
                1,
                0xffff_ffff_c000_0000,
            ),
        ];

        for (identity, funct, rt_value, sa, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rt_value), Ok(()));

            let executed = execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, sa, funct))
                .expect("fixed 64-bit shift should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(5), Some(expected));
        }
    }

    #[test]
    fn special_fixed_64_bit_shift_amount_boundaries_match_cpp() {
        let cases = [
            (
                CpuInstructionIdentity::SpecialDsll,
                0x38,
                0x0123_4567_89ab_cdef,
                0,
                0x0123_4567_89ab_cdef,
            ),
            (
                CpuInstructionIdentity::SpecialDsll,
                0x38,
                0x0000_0000_0000_0001,
                31,
                0x0000_0000_8000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsll32,
                0x3c,
                0x0000_0000_0000_0001,
                0,
                0x0000_0001_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsll32,
                0x3c,
                0x0000_0000_0000_0001,
                31,
                0x8000_0000_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsrl32,
                0x3e,
                0x8000_0000_0000_0000,
                31,
                0x0000_0000_0000_0001,
            ),
            (
                CpuInstructionIdentity::SpecialDsra32,
                0x3f,
                0x8000_0000_0000_0000,
                31,
                0xffff_ffff_ffff_ffff,
            ),
        ];

        for (identity, funct, rt_value, sa, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rt_value), Ok(()));

            let executed = execute_special_shift(&mut cpu, special_shift_word(0, 4, 5, sa, funct))
                .expect("fixed 64-bit shift boundary should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(5), Some(expected));
        }
    }

    #[test]
    fn special_variable_64_bit_shifts_use_rs_low_six_bits() {
        let cases = [
            (
                CpuInstructionIdentity::SpecialDsllv,
                0x14,
                0x0000_0000_0000_0003,
                0x0000_0000_0000_0041,
                0x0000_0000_0000_0006,
            ),
            (
                CpuInstructionIdentity::SpecialDsrlv,
                0x16,
                0x8000_0000_0000_0000,
                0x0000_0000_0000_0041,
                0x4000_0000_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsrav,
                0x17,
                0x8000_0000_0000_0000,
                0x0000_0000_0000_0044,
                0xf800_0000_0000_0000,
            ),
            (
                CpuInstructionIdentity::SpecialDsrlv,
                0x16,
                0x8000_0000_0000_0000,
                0xffff_ffff_ffff_ffff,
                0x0000_0000_0000_0001,
            ),
        ];

        for (identity, funct, rt_value, rs_value, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rt_value), Ok(()));
            assert_eq!(cpu.set_gpr(5, rs_value), Ok(()));

            let executed = execute_special_shift(&mut cpu, special_shift_word(5, 4, 6, 0, funct))
                .expect("variable 64-bit shift should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(6), Some(expected));
        }
    }

    #[test]
    fn special_fixed_64_bit_shift_reads_rt_before_writing_aliased_rd() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_0000_0001), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(0, 4, 4, 1, 0x38))
            .expect("DSLL should execute");

        assert_eq!(cpu.gpr(4), Some(0x0000_0000_0000_0002));
    }

    #[test]
    fn special_variable_64_bit_shift_reads_sources_before_writing_aliased_rd() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_0000_0041), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(4, 4, 4, 0, 0x14))
            .expect("DSLLV should execute");

        assert_eq!(cpu.gpr(4), Some(0x0000_0000_0000_0082));
    }

    #[test]
    fn special_64_bit_shift_write_to_zero_register_is_ignored() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_0000_0001), Ok(()));

        execute_special_shift(&mut cpu, special_shift_word(0, 4, 0, 1, 0x38))
            .expect("DSLL should execute");

        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(4), Some(0x0000_0000_0000_0001));
    }

    #[test]
    fn special_bitwise_logical_writes_full_width_results() {
        let cases = [
            (
                CpuInstructionIdentity::SpecialAnd,
                0x24,
                0xffff_0000_ffff_0000,
                0x0f0f_0f0f_ffff_ffff,
                0x0f0f_0000_ffff_0000,
            ),
            (
                CpuInstructionIdentity::SpecialOr,
                0x25,
                0x8000_0000_0000_0001,
                0x0000_0000_ffff_0000,
                0x8000_0000_ffff_0001,
            ),
            (
                CpuInstructionIdentity::SpecialXor,
                0x26,
                0xffff_0000_aaaa_5555,
                0x00ff_00ff_aaaa_ffff,
                0xff00_00ff_0000_aaaa,
            ),
            (
                CpuInstructionIdentity::SpecialNor,
                0x27,
                0x0000_ffff_0000_ffff,
                0x00ff_00ff_0000_0000,
                0xff00_0000_ffff_0000,
            ),
        ];

        for (identity, funct, rs_value, rt_value, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rs_value), Ok(()));
            assert_eq!(cpu.set_gpr(5, rt_value), Ok(()));

            let executed =
                execute_special_bitwise_logical(&mut cpu, special_shift_word(4, 5, 6, 0, funct))
                    .expect("SPECIAL bitwise logical instruction should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(6), Some(expected));
        }
    }

    #[test]
    fn special_bitwise_logical_reads_sources_before_writing_aliased_rd() {
        let mut rs_aliased = Cpu::new();
        assert_eq!(rs_aliased.set_gpr(4, 0xffff_0000_ffff_0000), Ok(()));
        assert_eq!(rs_aliased.set_gpr(5, 0x0f0f_0f0f_ffff_ffff), Ok(()));

        execute_special_bitwise_logical(&mut rs_aliased, special_shift_word(4, 5, 4, 0, 0x24))
            .expect("AND should execute with rs == rd");

        assert_eq!(rs_aliased.gpr(4), Some(0x0f0f_0000_ffff_0000));

        let mut rt_aliased = Cpu::new();
        assert_eq!(rt_aliased.set_gpr(4, 0xffff_0000_aaaa_5555), Ok(()));
        assert_eq!(rt_aliased.set_gpr(5, 0x00ff_00ff_aaaa_ffff), Ok(()));

        execute_special_bitwise_logical(&mut rt_aliased, special_shift_word(4, 5, 5, 0, 0x26))
            .expect("XOR should execute with rt == rd");

        assert_eq!(rt_aliased.gpr(5), Some(0xff00_00ff_0000_aaaa));
    }

    #[test]
    fn special_bitwise_logical_zero_register_source_and_destination_rules_match_gpr_semantics() {
        let mut source_zero = Cpu::new();
        assert_eq!(source_zero.set_gpr(5, 0x1234_5678_9abc_def0), Ok(()));

        execute_special_bitwise_logical(&mut source_zero, special_shift_word(0, 5, 6, 0, 0x25))
            .expect("OR with r0 source should execute");

        assert_eq!(source_zero.gpr(6), Some(0x1234_5678_9abc_def0));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 0xffff_0000_ffff_0000), Ok(()));
        assert_eq!(destination_zero.set_gpr(5, 0x0f0f_0f0f_ffff_ffff), Ok(()));

        execute_special_bitwise_logical(
            &mut destination_zero,
            special_shift_word(4, 5, 0, 0, 0x24),
        )
        .expect("AND writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
        assert_eq!(destination_zero.gpr(4), Some(0xffff_0000_ffff_0000));
        assert_eq!(destination_zero.gpr(5), Some(0x0f0f_0f0f_ffff_ffff));
    }

    #[test]
    fn special_bitwise_logical_rejects_non_logical_identity_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0xffff_0000_ffff_0000), Ok(()));
        assert_eq!(cpu.set_gpr(5, 0x0f0f_0f0f_ffff_ffff), Ok(()));
        let fields = decode(special_shift_word(4, 5, 6, 0, 0x24));

        let error = cpu
            .execute_special_bitwise_logical_instruction(CpuInstructionIdentity::SpecialSll, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuSpecialBitwiseLogicalExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::SpecialSll
            )
        );
        assert_eq!(cpu.gpr(4), Some(0xffff_0000_ffff_0000));
        assert_eq!(cpu.gpr(5), Some(0x0f0f_0f0f_ffff_ffff));
        assert_eq!(cpu.gpr(6), Some(0));
    }

    #[test]
    fn special_hi_lo_transfer_moves_full_width_values() {
        let mut mfhi = Cpu::new();
        mfhi.stage_hi(0x0123_4567_89ab_cdef);
        mfhi.stage_lo(0xfedc_ba98_7654_3210);

        let executed =
            execute_special_hi_lo_transfer(&mut mfhi, special_shift_word(0, 0, 6, 0, 0x10))
                .expect("MFHI should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialMfhi);
        assert_eq!(mfhi.gpr(6), Some(0x0123_4567_89ab_cdef));
        assert_eq!(mfhi.hi(), 0x0123_4567_89ab_cdef);
        assert_eq!(mfhi.lo(), 0xfedc_ba98_7654_3210);

        let mut mflo = Cpu::new();
        mflo.stage_hi(0x1111_2222_3333_4444);
        mflo.stage_lo(0x8888_7777_6666_5555);

        let executed =
            execute_special_hi_lo_transfer(&mut mflo, special_shift_word(0, 0, 7, 0, 0x12))
                .expect("MFLO should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialMflo);
        assert_eq!(mflo.gpr(7), Some(0x8888_7777_6666_5555));
        assert_eq!(mflo.hi(), 0x1111_2222_3333_4444);
        assert_eq!(mflo.lo(), 0x8888_7777_6666_5555);

        let mut mthi = Cpu::new();
        mthi.stage_hi(0x1111_1111_1111_1111);
        mthi.stage_lo(0x2222_2222_2222_2222);
        assert_eq!(mthi.set_gpr(4, 0xffff_0000_aaaa_5555), Ok(()));

        let executed =
            execute_special_hi_lo_transfer(&mut mthi, special_shift_word(4, 0, 0, 0, 0x11))
                .expect("MTHI should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialMthi);
        assert_eq!(mthi.hi(), 0xffff_0000_aaaa_5555);
        assert_eq!(mthi.lo(), 0x2222_2222_2222_2222);
        assert_eq!(mthi.gpr(4), Some(0xffff_0000_aaaa_5555));

        let mut mtlo = Cpu::new();
        mtlo.stage_hi(0x3333_3333_3333_3333);
        mtlo.stage_lo(0x4444_4444_4444_4444);
        assert_eq!(mtlo.set_gpr(5, 0x8000_0000_0000_0001), Ok(()));

        let executed =
            execute_special_hi_lo_transfer(&mut mtlo, special_shift_word(5, 0, 0, 0, 0x13))
                .expect("MTLO should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialMtlo);
        assert_eq!(mtlo.hi(), 0x3333_3333_3333_3333);
        assert_eq!(mtlo.lo(), 0x8000_0000_0000_0001);
        assert_eq!(mtlo.gpr(5), Some(0x8000_0000_0000_0001));
    }

    #[test]
    fn special_hi_lo_transfer_zero_register_writeback_is_ignored() {
        let mut mfhi = Cpu::new();
        mfhi.stage_hi(0x0123_4567_89ab_cdef);

        execute_special_hi_lo_transfer(&mut mfhi, special_shift_word(0, 0, 0, 0, 0x10))
            .expect("MFHI writing r0 should execute");

        assert_eq!(mfhi.gpr(0), Some(0));
        assert_eq!(mfhi.hi(), 0x0123_4567_89ab_cdef);

        let mut mflo = Cpu::new();
        mflo.stage_lo(0xfedc_ba98_7654_3210);

        execute_special_hi_lo_transfer(&mut mflo, special_shift_word(0, 0, 0, 0, 0x12))
            .expect("MFLO writing r0 should execute");

        assert_eq!(mflo.gpr(0), Some(0));
        assert_eq!(mflo.lo(), 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn special_hi_lo_transfer_rejects_non_transfer_identity_without_mutation() {
        let mut cpu = Cpu::new();
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(cpu.set_gpr(4, 0xffff_0000_ffff_0000), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(special_shift_word(4, 0, 6, 0, 0x10));

        let error = cpu
            .execute_special_hi_lo_transfer_instruction(CpuInstructionIdentity::SpecialAnd, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuSpecialHiLoTransferExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::SpecialAnd
            )
        );
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(4), Some(0xffff_0000_ffff_0000));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn special_multu_multiplies_unsigned_low_words_and_sign_extends_hi_lo_halves() {
        let cases = [
            (
                0xffff_ffff_ffff_ffff,
                0xaaaa_bbbb_ffff_ffff,
                0xffff_ffff_ffff_fffe,
                0x0000_0000_0000_0001,
            ),
            (
                0x1234_5678_8000_0000,
                0xdead_beef_0000_0002,
                0x0000_0000_0000_0001,
                0x0000_0000_0000_0000,
            ),
            (0, u64::MAX, 0, 0),
        ];

        for (rs_value, rt_value, expected_hi, expected_lo) in cases {
            let mut cpu = Cpu::new();
            cpu.stage_hi(0x1111_2222_3333_4444);
            cpu.stage_lo(0x5555_6666_7777_8888);
            cpu.set_gpr(4, rs_value).unwrap();
            cpu.set_gpr(5, rt_value).unwrap();
            let fields = decode(special_shift_word(4, 5, 0, 0, 0x19));
            let selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialMultu)
                .expect("MULTU has one exact CPU-local helper");

            let outcome = cpu
                .invoke_cpu_local_executed_helper(fields, selection)
                .expect("MULTU should execute");

            assert_executed_invocation(
                outcome,
                CpuInstructionIdentity::SpecialMultu,
                CpuLocalExecutedHelperFamily::SpecialMultiply,
            );
            assert_eq!(cpu.hi(), expected_hi);
            assert_eq!(cpu.lo(), expected_lo);
            assert_eq!(cpu.gpr(4), Some(rs_value));
            assert_eq!(cpu.gpr(5), Some(rt_value));
            assert_eq!(cpu.gpr(0), Some(0));
        }
    }

    #[test]
    fn special_dmultu_multiplies_complete_unsigned_64_bit_sources() {
        let mut cpu = Cpu::new();
        cpu.set_gpr(4, u64::MAX).unwrap();
        cpu.set_gpr(5, 2).unwrap();
        let fields = decode(special_shift_word(4, 5, 0, 0, 0x1d));
        let selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialDmultu)
            .expect("DMULTU has one exact CPU-local helper");

        let outcome = cpu
            .invoke_cpu_local_executed_helper(fields, selection)
            .expect("DMULTU should execute");

        assert_executed_invocation(
            outcome,
            CpuInstructionIdentity::SpecialDmultu,
            CpuLocalExecutedHelperFamily::SpecialMultiply,
        );
        assert_eq!(cpu.hi(), 1);
        assert_eq!(cpu.lo(), 0xffff_ffff_ffff_fffe);
        assert_eq!(cpu.gpr(4), Some(u64::MAX));
        assert_eq!(cpu.gpr(5), Some(2));
    }

    #[test]
    fn special_div_owns_signed_word_quotient_remainder_and_architectural_edges() {
        let cases = [
            (
                (-7_i32) as u32 as u64,
                3_u64,
                u64::MAX,
                0xffff_ffff_ffff_fffe,
            ),
            (7, 0, 7, u64::MAX),
            (
                i32::MIN as u32 as u64,
                u32::MAX as u64,
                0,
                0xffff_ffff_8000_0000,
            ),
        ];

        for (dividend, divisor, expected_hi, expected_lo) in cases {
            let mut cpu = Cpu::new();
            cpu.set_gpr(4, dividend).unwrap();
            cpu.set_gpr(5, divisor).unwrap();
            let fields = decode(special_shift_word(4, 5, 0, 0, 0x1a));
            let selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialDiv)
                .expect("DIV has one exact CPU-local helper");

            let outcome = cpu
                .invoke_cpu_local_executed_helper(fields, selection)
                .expect("DIV should execute");

            assert_executed_invocation(
                outcome,
                CpuInstructionIdentity::SpecialDiv,
                CpuLocalExecutedHelperFamily::SpecialDivide,
            );
            assert_eq!(cpu.hi(), expected_hi);
            assert_eq!(cpu.lo(), expected_lo);
        }
    }

    #[test]
    fn special_ddivu_owns_full_width_unsigned_division_and_zero_divisor() {
        for (dividend, divisor, expected_hi, expected_lo) in [
            (10, 3, 1, 3),
            (0x1234_5678_9abc_def0, 0, 0x1234_5678_9abc_def0, u64::MAX),
        ] {
            let mut cpu = Cpu::new();
            cpu.set_gpr(4, dividend).unwrap();
            cpu.set_gpr(5, divisor).unwrap();
            let fields = decode(special_shift_word(4, 5, 0, 0, 0x1f));
            let selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialDdivu)
                .expect("DDIVU has one exact CPU-local helper");

            let outcome = cpu
                .invoke_cpu_local_executed_helper(fields, selection)
                .expect("DDIVU should execute");

            assert_executed_invocation(
                outcome,
                CpuInstructionIdentity::SpecialDdivu,
                CpuLocalExecutedHelperFamily::SpecialDivide,
            );
            assert_eq!(cpu.hi(), expected_hi);
            assert_eq!(cpu.lo(), expected_lo);
        }
    }

    #[test]
    fn special_non_trapping_integer_word_arithmetic_wraps_and_sign_extends() {
        let mut addu = Cpu::new();
        assert_eq!(addu.set_gpr(4, 0xffff_ffff_7fff_ffff), Ok(()));
        assert_eq!(addu.set_gpr(5, 0x0000_0000_0000_0001), Ok(()));

        let executed =
            execute_special_non_trapping_integer(&mut addu, special_shift_word(4, 5, 6, 0, 0x21))
                .expect("ADDU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialAddu);
        assert_eq!(addu.gpr(6), Some(0xffff_ffff_8000_0000));

        let mut addu_wrap = Cpu::new();
        assert_eq!(addu_wrap.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));
        assert_eq!(addu_wrap.set_gpr(5, 0x0000_0000_0000_0001), Ok(()));

        execute_special_non_trapping_integer(&mut addu_wrap, special_shift_word(4, 5, 6, 0, 0x21))
            .expect("ADDU should wrap the low word");

        assert_eq!(addu_wrap.gpr(6), Some(0));

        let mut subu = Cpu::new();
        assert_eq!(subu.set_gpr(4, 0), Ok(()));
        assert_eq!(subu.set_gpr(5, 1), Ok(()));

        let executed =
            execute_special_non_trapping_integer(&mut subu, special_shift_word(4, 5, 6, 0, 0x23))
                .expect("SUBU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSubu);
        assert_eq!(subu.gpr(6), Some(0xffff_ffff_ffff_ffff));
    }

    #[test]
    fn special_non_trapping_integer_doubleword_arithmetic_wraps_full_width() {
        let mut daddu = Cpu::new();
        assert_eq!(daddu.set_gpr(4, u64::MAX), Ok(()));
        assert_eq!(daddu.set_gpr(5, 2), Ok(()));

        let executed =
            execute_special_non_trapping_integer(&mut daddu, special_shift_word(4, 5, 6, 0, 0x2d))
                .expect("DADDU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialDaddu);
        assert_eq!(daddu.gpr(6), Some(1));

        let mut dsubu = Cpu::new();
        assert_eq!(dsubu.set_gpr(4, 0), Ok(()));
        assert_eq!(dsubu.set_gpr(5, 1), Ok(()));

        let executed =
            execute_special_non_trapping_integer(&mut dsubu, special_shift_word(4, 5, 6, 0, 0x2f))
                .expect("DSUBU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialDsubu);
        assert_eq!(dsubu.gpr(6), Some(u64::MAX));
    }

    #[test]
    fn special_non_trapping_integer_compares_signed_and_unsigned_full_width_values() {
        let mut slt_true = Cpu::new();
        assert_eq!(slt_true.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));
        assert_eq!(slt_true.set_gpr(5, 1), Ok(()));

        let executed = execute_special_non_trapping_integer(
            &mut slt_true,
            special_shift_word(4, 5, 6, 0, 0x2a),
        )
        .expect("SLT should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSlt);
        assert_eq!(slt_true.gpr(6), Some(1));

        let mut slt_false = Cpu::new();
        assert_eq!(slt_false.set_gpr(4, 0x7fff_ffff_ffff_ffff), Ok(()));
        assert_eq!(slt_false.set_gpr(5, 0x8000_0000_0000_0000), Ok(()));

        execute_special_non_trapping_integer(&mut slt_false, special_shift_word(4, 5, 6, 0, 0x2a))
            .expect("SLT should execute for high-bit comparison");

        assert_eq!(slt_false.gpr(6), Some(0));

        let mut sltu_false = Cpu::new();
        assert_eq!(sltu_false.set_gpr(4, u64::MAX), Ok(()));
        assert_eq!(sltu_false.set_gpr(5, 1), Ok(()));

        let executed = execute_special_non_trapping_integer(
            &mut sltu_false,
            special_shift_word(4, 5, 6, 0, 0x2b),
        )
        .expect("SLTU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSltu);
        assert_eq!(sltu_false.gpr(6), Some(0));

        let mut sltu_true = Cpu::new();
        assert_eq!(sltu_true.set_gpr(4, 1), Ok(()));
        assert_eq!(sltu_true.set_gpr(5, u64::MAX), Ok(()));

        execute_special_non_trapping_integer(&mut sltu_true, special_shift_word(4, 5, 6, 0, 0x2b))
            .expect("SLTU should execute for unsigned high-bit comparison");

        assert_eq!(sltu_true.gpr(6), Some(1));
    }

    #[test]
    fn special_non_trapping_integer_reads_sources_before_writing_aliased_rd() {
        let mut rs_aliased = Cpu::new();
        assert_eq!(rs_aliased.set_gpr(4, u64::MAX), Ok(()));
        assert_eq!(rs_aliased.set_gpr(5, 2), Ok(()));

        execute_special_non_trapping_integer(&mut rs_aliased, special_shift_word(4, 5, 4, 0, 0x2d))
            .expect("DADDU should execute with rs == rd");

        assert_eq!(rs_aliased.gpr(4), Some(1));

        let mut rt_aliased = Cpu::new();
        assert_eq!(rt_aliased.set_gpr(4, 0), Ok(()));
        assert_eq!(rt_aliased.set_gpr(5, 1), Ok(()));

        execute_special_non_trapping_integer(&mut rt_aliased, special_shift_word(4, 5, 5, 0, 0x2f))
            .expect("DSUBU should execute with rt == rd");

        assert_eq!(rt_aliased.gpr(5), Some(u64::MAX));
    }

    #[test]
    fn special_non_trapping_integer_zero_register_source_and_destination_rules_match_gpr_semantics()
    {
        let mut source_zero = Cpu::new();
        assert_eq!(source_zero.set_gpr(5, 0x0000_0000_8000_0000), Ok(()));

        execute_special_non_trapping_integer(
            &mut source_zero,
            special_shift_word(0, 5, 6, 0, 0x21),
        )
        .expect("ADDU with r0 source should execute");

        assert_eq!(source_zero.gpr(6), Some(0xffff_ffff_8000_0000));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, u64::MAX), Ok(()));
        assert_eq!(destination_zero.set_gpr(5, 1), Ok(()));

        execute_special_non_trapping_integer(
            &mut destination_zero,
            special_shift_word(4, 5, 0, 0, 0x2d),
        )
        .expect("DADDU writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
        assert_eq!(destination_zero.gpr(4), Some(u64::MAX));
        assert_eq!(destination_zero.gpr(5), Some(1));
    }

    #[test]
    fn special_non_trapping_integer_rejects_trapping_identity_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(cpu.set_gpr(5, 1), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(special_shift_word(4, 5, 6, 0, 0x21));

        let error = cpu
            .execute_special_non_trapping_integer_instruction(
                CpuInstructionIdentity::SpecialAdd,
                fields,
            )
            .unwrap_err();

        assert_eq!(
            error,
            CpuSpecialNonTrappingIntegerExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::SpecialAdd
            )
        );
        assert_eq!(cpu.gpr(4), Some(0x7fff_ffff));
        assert_eq!(cpu.gpr(5), Some(1));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn special_trapping_integer_word_arithmetic_writes_non_overflow_sign_extended_results() {
        let mut add = Cpu::new();
        assert_eq!(add.set_gpr(4, 0xffff_ffff_7fff_fffe), Ok(()));
        assert_eq!(add.set_gpr(5, 1), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut add, special_shift_word(4, 5, 6, 0, 0x20))
                .expect("ADD should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialAdd);
        assert!(outcome.is_executed());
        assert_eq!(add.gpr(6), Some(0x0000_0000_7fff_ffff));

        let mut add_negative = Cpu::new();
        assert_eq!(add_negative.set_gpr(4, 0xffff_ffff), Ok(()));
        assert_eq!(add_negative.set_gpr(5, 0xffff_ffff), Ok(()));

        execute_special_trapping_integer(&mut add_negative, special_shift_word(4, 5, 6, 0, 0x20))
            .expect("ADD negative result should execute without overflow");

        assert_eq!(add_negative.gpr(6), Some(0xffff_ffff_ffff_fffe));

        let mut sub = Cpu::new();
        assert_eq!(sub.set_gpr(4, 1), Ok(()));
        assert_eq!(sub.set_gpr(5, 2), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut sub, special_shift_word(4, 5, 6, 0, 0x22))
                .expect("SUB should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialSub);
        assert!(outcome.is_executed());
        assert_eq!(sub.gpr(6), Some(0xffff_ffff_ffff_ffff));
    }

    #[test]
    fn special_trapping_integer_word_overflow_is_detected_before_writeback() {
        let mut add = Cpu::new();
        assert_eq!(add.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(add.set_gpr(5, 1), Ok(()));
        assert_eq!(add.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut add, special_shift_word(4, 5, 6, 0, 0x20))
                .expect("ADD overflow should return an outcome");

        assert!(outcome.is_overflow());
        match outcome {
            CpuSpecialTrappingIntegerExecutionOutcome::Overflow(overflow) => {
                assert_eq!(overflow.identity(), CpuInstructionIdentity::SpecialAdd);
                assert_eq!(overflow.rd(), 6);
                assert_eq!(overflow.rs_value(), 0x7fff_ffff);
                assert_eq!(overflow.rt_value(), 1);
            }
            CpuSpecialTrappingIntegerExecutionOutcome::Executed(_) => {
                panic!("ADD overflow must not execute")
            }
        }
        assert_eq!(add.gpr(6), Some(0x0123_4567_89ab_cdef));

        let mut sub = Cpu::new();
        assert_eq!(sub.set_gpr(4, 0x8000_0000), Ok(()));
        assert_eq!(sub.set_gpr(5, 1), Ok(()));
        assert_eq!(sub.set_gpr(6, 0xfedc_ba98_7654_3210), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut sub, special_shift_word(4, 5, 6, 0, 0x22))
                .expect("SUB overflow should return an outcome");

        assert!(outcome.is_overflow());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialSub);
        assert_eq!(sub.gpr(6), Some(0xfedc_ba98_7654_3210));
    }

    #[test]
    fn special_trapping_integer_doubleword_arithmetic_writes_non_overflow_full_width_results() {
        let mut dadd = Cpu::new();
        assert_eq!(dadd.set_gpr(4, 0x7fff_ffff_ffff_fffe), Ok(()));
        assert_eq!(dadd.set_gpr(5, 1), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut dadd, special_shift_word(4, 5, 6, 0, 0x2c))
                .expect("DADD should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialDadd);
        assert!(outcome.is_executed());
        assert_eq!(dadd.gpr(6), Some(0x7fff_ffff_ffff_ffff));

        let mut dsub = Cpu::new();
        assert_eq!(dsub.set_gpr(4, 0x8000_0000_0000_0001), Ok(()));
        assert_eq!(dsub.set_gpr(5, 1), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut dsub, special_shift_word(4, 5, 6, 0, 0x2e))
                .expect("DSUB should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialDsub);
        assert!(outcome.is_executed());
        assert_eq!(dsub.gpr(6), Some(0x8000_0000_0000_0000));
    }

    #[test]
    fn special_trapping_integer_doubleword_overflow_is_detected_before_writeback() {
        let mut dadd = Cpu::new();
        assert_eq!(dadd.set_gpr(4, 0x7fff_ffff_ffff_ffff), Ok(()));
        assert_eq!(dadd.set_gpr(5, 1), Ok(()));
        assert_eq!(dadd.set_gpr(6, 0x1111_2222_3333_4444), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut dadd, special_shift_word(4, 5, 6, 0, 0x2c))
                .expect("DADD overflow should return an outcome");

        assert!(outcome.is_overflow());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialDadd);
        assert_eq!(dadd.gpr(6), Some(0x1111_2222_3333_4444));

        let mut dsub = Cpu::new();
        assert_eq!(dsub.set_gpr(4, 0x8000_0000_0000_0000), Ok(()));
        assert_eq!(dsub.set_gpr(5, 1), Ok(()));
        assert_eq!(dsub.set_gpr(6, 0x5555_6666_7777_8888), Ok(()));

        let outcome =
            execute_special_trapping_integer(&mut dsub, special_shift_word(4, 5, 6, 0, 0x2e))
                .expect("DSUB overflow should return an outcome");

        assert!(outcome.is_overflow());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialDsub);
        assert_eq!(dsub.gpr(6), Some(0x5555_6666_7777_8888));
    }

    #[test]
    fn special_trapping_integer_alias_and_zero_register_rules_match_gpr_semantics() {
        let mut rs_aliased = Cpu::new();
        assert_eq!(rs_aliased.set_gpr(4, 1), Ok(()));
        assert_eq!(rs_aliased.set_gpr(5, 2), Ok(()));

        execute_special_trapping_integer(&mut rs_aliased, special_shift_word(4, 5, 4, 0, 0x20))
            .expect("ADD should execute with rs == rd");

        assert_eq!(rs_aliased.gpr(4), Some(3));

        let mut rt_aliased = Cpu::new();
        assert_eq!(rt_aliased.set_gpr(4, 4), Ok(()));
        assert_eq!(rt_aliased.set_gpr(5, 1), Ok(()));

        execute_special_trapping_integer(&mut rt_aliased, special_shift_word(4, 5, 5, 0, 0x2e))
            .expect("DSUB should execute with rt == rd");

        assert_eq!(rt_aliased.gpr(5), Some(3));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 1), Ok(()));
        assert_eq!(destination_zero.set_gpr(5, 2), Ok(()));

        execute_special_trapping_integer(
            &mut destination_zero,
            special_shift_word(4, 5, 0, 0, 0x20),
        )
        .expect("ADD writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));

        let mut overflow_zero = Cpu::new();
        assert_eq!(overflow_zero.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(overflow_zero.set_gpr(5, 1), Ok(()));

        let outcome = execute_special_trapping_integer(
            &mut overflow_zero,
            special_shift_word(4, 5, 0, 0, 0x20),
        )
        .expect("ADD overflow writing r0 should return overflow");

        assert!(outcome.is_overflow());
        assert_eq!(overflow_zero.gpr(0), Some(0));
    }

    #[test]
    fn special_trapping_integer_rejects_non_trapping_identity_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 1), Ok(()));
        assert_eq!(cpu.set_gpr(5, 2), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(special_shift_word(4, 5, 6, 0, 0x20));

        let error = cpu
            .execute_special_trapping_integer_instruction(
                CpuInstructionIdentity::SpecialAddu,
                fields,
            )
            .unwrap_err();

        assert_eq!(
            error,
            CpuSpecialTrappingIntegerExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::SpecialAddu
            )
        );
        assert_eq!(cpu.gpr(4), Some(1));
        assert_eq!(cpu.gpr(5), Some(2));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn immediate_trapping_integer_addi_writes_non_overflow_sign_extended_results() {
        let mut positive = Cpu::new();
        assert_eq!(positive.set_gpr(4, 1), Ok(()));

        let outcome =
            execute_immediate_trapping_integer(&mut positive, immediate_word(0x08, 4, 6, 2))
                .expect("ADDI positive immediate should execute");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::Addi);
        assert!(outcome.is_executed());
        assert_eq!(positive.gpr(6), Some(3));

        let mut negative = Cpu::new();
        assert_eq!(negative.set_gpr(4, 0), Ok(()));

        execute_immediate_trapping_integer(&mut negative, immediate_word(0x08, 4, 6, 0xffff))
            .expect("ADDI negative immediate should execute");

        assert_eq!(negative.gpr(6), Some(0xffff_ffff_ffff_ffff));

        let mut low_word_source = Cpu::new();
        assert_eq!(low_word_source.set_gpr(4, 0x1234_5678_ffff_fffe), Ok(()));

        execute_immediate_trapping_integer(&mut low_word_source, immediate_word(0x08, 4, 6, 1))
            .expect("ADDI should use low word source");

        assert_eq!(low_word_source.gpr(6), Some(0xffff_ffff_ffff_ffff));
    }

    #[test]
    fn immediate_trapping_integer_addi_overflow_is_detected_before_writeback() {
        let mut positive_overflow = Cpu::new();
        assert_eq!(positive_overflow.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(positive_overflow.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));

        let outcome = execute_immediate_trapping_integer(
            &mut positive_overflow,
            immediate_word(0x08, 4, 6, 1),
        )
        .expect("ADDI overflow should return an outcome");

        assert!(outcome.is_overflow());
        match outcome {
            CpuImmediateTrappingIntegerExecutionOutcome::Overflow(overflow) => {
                assert_eq!(overflow.identity(), CpuInstructionIdentity::Addi);
                assert_eq!(overflow.rt(), 6);
                assert_eq!(overflow.rs_value(), 0x7fff_ffff);
                assert_eq!(overflow.immediate_u16(), 1);
                assert_eq!(overflow.immediate_value(), 1);
            }
            CpuImmediateTrappingIntegerExecutionOutcome::Executed(_) => {
                panic!("ADDI overflow must not execute")
            }
        }
        assert_eq!(positive_overflow.gpr(6), Some(0x0123_4567_89ab_cdef));

        let mut negative_overflow = Cpu::new();
        assert_eq!(negative_overflow.set_gpr(4, 0x8000_0000), Ok(()));
        assert_eq!(negative_overflow.set_gpr(6, 0xfedc_ba98_7654_3210), Ok(()));

        let outcome = execute_immediate_trapping_integer(
            &mut negative_overflow,
            immediate_word(0x08, 4, 6, 0xffff),
        )
        .expect("ADDI negative overflow should return an outcome");

        assert!(outcome.is_overflow());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::Addi);
        assert_eq!(negative_overflow.gpr(6), Some(0xfedc_ba98_7654_3210));
    }

    #[test]
    fn immediate_trapping_integer_daddi_writes_non_overflow_full_width_results() {
        let mut positive = Cpu::new();
        assert_eq!(positive.set_gpr(4, 0x7fff_ffff_ffff_fffe), Ok(()));

        let outcome =
            execute_immediate_trapping_integer(&mut positive, immediate_word(0x18, 4, 6, 1))
                .expect("DADDI positive immediate should execute");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::Daddi);
        assert!(outcome.is_executed());
        assert_eq!(positive.gpr(6), Some(0x7fff_ffff_ffff_ffff));

        let mut negative = Cpu::new();
        assert_eq!(negative.set_gpr(4, 0x10), Ok(()));

        execute_immediate_trapping_integer(&mut negative, immediate_word(0x18, 4, 6, 0xfffe))
            .expect("DADDI negative immediate should execute");

        assert_eq!(negative.gpr(6), Some(0x0e));
    }

    #[test]
    fn immediate_trapping_integer_daddi_overflow_is_detected_before_writeback() {
        let mut positive_overflow = Cpu::new();
        assert_eq!(positive_overflow.set_gpr(4, 0x7fff_ffff_ffff_ffff), Ok(()));
        assert_eq!(positive_overflow.set_gpr(6, 0x1111_2222_3333_4444), Ok(()));

        let outcome = execute_immediate_trapping_integer(
            &mut positive_overflow,
            immediate_word(0x18, 4, 6, 1),
        )
        .expect("DADDI overflow should return an outcome");

        assert!(outcome.is_overflow());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::Daddi);
        assert_eq!(positive_overflow.gpr(6), Some(0x1111_2222_3333_4444));

        let mut negative_overflow = Cpu::new();
        assert_eq!(negative_overflow.set_gpr(4, 0x8000_0000_0000_0000), Ok(()));
        assert_eq!(negative_overflow.set_gpr(6, 0x5555_6666_7777_8888), Ok(()));

        let outcome = execute_immediate_trapping_integer(
            &mut negative_overflow,
            immediate_word(0x18, 4, 6, 0xffff),
        )
        .expect("DADDI negative overflow should return an outcome");

        assert!(outcome.is_overflow());
        match outcome {
            CpuImmediateTrappingIntegerExecutionOutcome::Overflow(overflow) => {
                assert_eq!(overflow.identity(), CpuInstructionIdentity::Daddi);
                assert_eq!(overflow.rt(), 6);
                assert_eq!(overflow.rs_value(), 0x8000_0000_0000_0000);
                assert_eq!(overflow.immediate_u16(), 0xffff);
                assert_eq!(overflow.immediate_value(), 0xffff_ffff_ffff_ffff);
            }
            CpuImmediateTrappingIntegerExecutionOutcome::Executed(_) => {
                panic!("DADDI overflow must not execute")
            }
        }
        assert_eq!(negative_overflow.gpr(6), Some(0x5555_6666_7777_8888));
    }

    #[test]
    fn immediate_trapping_integer_alias_and_zero_register_rules_match_gpr_semantics() {
        let mut addi_aliased = Cpu::new();
        assert_eq!(addi_aliased.set_gpr(4, 41), Ok(()));

        execute_immediate_trapping_integer(&mut addi_aliased, immediate_word(0x08, 4, 4, 1))
            .expect("ADDI should execute with rs == rt");

        assert_eq!(addi_aliased.gpr(4), Some(42));

        let mut daddi_aliased = Cpu::new();
        assert_eq!(daddi_aliased.set_gpr(4, 2), Ok(()));

        execute_immediate_trapping_integer(&mut daddi_aliased, immediate_word(0x18, 4, 4, 0xffff))
            .expect("DADDI should execute with rs == rt");

        assert_eq!(daddi_aliased.gpr(4), Some(1));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 1), Ok(()));

        execute_immediate_trapping_integer(&mut destination_zero, immediate_word(0x08, 4, 0, 1))
            .expect("ADDI writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));

        execute_immediate_trapping_integer(&mut destination_zero, immediate_word(0x18, 4, 0, 1))
            .expect("DADDI writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
    }

    #[test]
    fn immediate_trapping_integer_rejects_non_trapping_identity_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 1), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(immediate_word(0x08, 4, 6, 1));

        let error = cpu
            .execute_immediate_trapping_integer_instruction(CpuInstructionIdentity::Addiu, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuImmediateTrappingIntegerExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::Addiu
            )
        );
        assert_eq!(cpu.gpr(4), Some(1));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn immediate_non_trapping_integer_addiu_writes_sign_extended_wrapping_word_results() {
        let mut positive = Cpu::new();
        assert_eq!(positive.set_gpr(4, 0x10), Ok(()));

        let executed =
            execute_immediate_non_trapping_integer(&mut positive, immediate_word(0x09, 4, 6, 2))
                .expect("ADDIU positive immediate should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Addiu);
        assert_eq!(positive.gpr(6), Some(0x12));

        let mut negative = Cpu::new();
        assert_eq!(negative.set_gpr(4, 0x10), Ok(()));

        execute_immediate_non_trapping_integer(&mut negative, immediate_word(0x09, 4, 6, 0xffff))
            .expect("ADDIU negative immediate should execute");

        assert_eq!(negative.gpr(6), Some(0x0f));

        let mut sign_extended = Cpu::new();
        assert_eq!(sign_extended.set_gpr(4, 0x7fff_ffff), Ok(()));

        execute_immediate_non_trapping_integer(&mut sign_extended, immediate_word(0x09, 4, 6, 1))
            .expect("ADDIU should sign-extend the wrapped word result");

        assert_eq!(sign_extended.gpr(6), Some(0xffff_ffff_8000_0000));

        let mut low_word_source = Cpu::new();
        assert_eq!(low_word_source.set_gpr(4, 0x1234_5678_0000_0001), Ok(()));

        execute_immediate_non_trapping_integer(
            &mut low_word_source,
            immediate_word(0x09, 4, 6, 0xffff),
        )
        .expect("ADDIU should use low word source arithmetic");

        assert_eq!(low_word_source.gpr(6), Some(0));
    }

    #[test]
    fn immediate_non_trapping_integer_addiu_wraps_without_overflow_exception() {
        let mut positive_wrap = Cpu::new();
        assert_eq!(positive_wrap.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(positive_wrap.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));

        let executed = execute_immediate_non_trapping_integer(
            &mut positive_wrap,
            immediate_word(0x09, 4, 6, 1),
        )
        .expect("ADDIU positive signed overflow shape should still execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Addiu);
        assert_eq!(positive_wrap.gpr(6), Some(0xffff_ffff_8000_0000));

        let mut word_wrap = Cpu::new();
        assert_eq!(word_wrap.set_gpr(4, 0xffff_ffff), Ok(()));

        execute_immediate_non_trapping_integer(&mut word_wrap, immediate_word(0x09, 4, 6, 1))
            .expect("ADDIU word wrap should execute");

        assert_eq!(word_wrap.gpr(6), Some(0));
    }

    #[test]
    fn immediate_non_trapping_integer_daddiu_writes_full_width_wrapping_results() {
        let mut positive = Cpu::new();
        assert_eq!(positive.set_gpr(4, 0x7fff_ffff_ffff_fffe), Ok(()));

        let executed =
            execute_immediate_non_trapping_integer(&mut positive, immediate_word(0x19, 4, 6, 1))
                .expect("DADDIU positive immediate should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Daddiu);
        assert_eq!(positive.gpr(6), Some(0x7fff_ffff_ffff_ffff));

        let mut negative = Cpu::new();
        assert_eq!(negative.set_gpr(4, 0x10), Ok(()));

        execute_immediate_non_trapping_integer(&mut negative, immediate_word(0x19, 4, 6, 0xfffe))
            .expect("DADDIU negative immediate should execute");

        assert_eq!(negative.gpr(6), Some(0x0e));

        let mut wrap = Cpu::new();
        assert_eq!(wrap.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));

        execute_immediate_non_trapping_integer(&mut wrap, immediate_word(0x19, 4, 6, 1))
            .expect("DADDIU full-width wrap should execute");

        assert_eq!(wrap.gpr(6), Some(0));
    }

    #[test]
    fn immediate_non_trapping_integer_alias_and_zero_register_rules_match_gpr_semantics() {
        let mut addiu_aliased = Cpu::new();
        assert_eq!(addiu_aliased.set_gpr(4, 41), Ok(()));

        execute_immediate_non_trapping_integer(&mut addiu_aliased, immediate_word(0x09, 4, 4, 1))
            .expect("ADDIU should execute with rs == rt");

        assert_eq!(addiu_aliased.gpr(4), Some(42));

        let mut daddiu_aliased = Cpu::new();
        assert_eq!(daddiu_aliased.set_gpr(4, 2), Ok(()));

        execute_immediate_non_trapping_integer(
            &mut daddiu_aliased,
            immediate_word(0x19, 4, 4, 0xffff),
        )
        .expect("DADDIU should execute with rs == rt");

        assert_eq!(daddiu_aliased.gpr(4), Some(1));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 1), Ok(()));

        execute_immediate_non_trapping_integer(
            &mut destination_zero,
            immediate_word(0x09, 4, 0, 1),
        )
        .expect("ADDIU writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));

        execute_immediate_non_trapping_integer(
            &mut destination_zero,
            immediate_word(0x19, 4, 0, 1),
        )
        .expect("DADDIU writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
    }

    #[test]
    fn immediate_non_trapping_integer_rejects_other_immediate_identities_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 1), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(immediate_word(0x09, 4, 6, 1));

        let error = cpu
            .execute_immediate_non_trapping_integer_instruction(
                CpuInstructionIdentity::Addi,
                fields,
            )
            .unwrap_err();

        assert_eq!(
            error,
            CpuImmediateNonTrappingIntegerExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::Addi
            )
        );
        assert_eq!(cpu.gpr(4), Some(1));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn immediate_comparison_slti_writes_signed_less_than_result() {
        let mut less = Cpu::new();
        assert_eq!(less.set_gpr(4, 0xffff_ffff_ffff_fffe), Ok(()));

        let executed = execute_immediate_comparison(&mut less, immediate_word(0x0a, 4, 6, 0))
            .expect("SLTI signed less-than should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Slti);
        assert_eq!(less.gpr(6), Some(1));

        let mut equal = Cpu::new();
        assert_eq!(equal.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));

        execute_immediate_comparison(&mut equal, immediate_word(0x0a, 4, 6, 0xffff))
            .expect("SLTI signed equal should execute");

        assert_eq!(equal.gpr(6), Some(0));

        let mut greater = Cpu::new();
        assert_eq!(greater.set_gpr(4, 0x7fff_ffff_ffff_ffff), Ok(()));

        execute_immediate_comparison(&mut greater, immediate_word(0x0a, 4, 6, 0xffff))
            .expect("SLTI signed greater-than should execute");

        assert_eq!(greater.gpr(6), Some(0));
    }

    #[test]
    fn immediate_comparison_slti_negative_immediate_and_high_bit_source_are_signed() {
        let mut negative_less = Cpu::new();
        assert_eq!(negative_less.set_gpr(4, 0xffff_ffff_ffff_fffd), Ok(()));

        execute_immediate_comparison(&mut negative_less, immediate_word(0x0a, 4, 6, 0xfffe))
            .expect("SLTI negative immediate should execute");

        assert_eq!(negative_less.gpr(6), Some(1));

        let mut positive_greater = Cpu::new();
        assert_eq!(positive_greater.set_gpr(4, 0x8000_0000_0000_0000), Ok(()));

        execute_immediate_comparison(&mut positive_greater, immediate_word(0x0a, 4, 6, 0))
            .expect("SLTI high-bit source should execute");

        assert_eq!(positive_greater.gpr(6), Some(1));
    }

    #[test]
    fn immediate_comparison_sltiu_writes_unsigned_less_than_result() {
        let mut less = Cpu::new();
        assert_eq!(less.set_gpr(4, 0x0000_0000_0000_0001), Ok(()));

        let executed = execute_immediate_comparison(&mut less, immediate_word(0x0b, 4, 6, 2))
            .expect("SLTIU unsigned less-than should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Sltiu);
        assert_eq!(less.gpr(6), Some(1));

        let mut equal = Cpu::new();
        assert_eq!(equal.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));

        execute_immediate_comparison(&mut equal, immediate_word(0x0b, 4, 6, 0xffff))
            .expect("SLTIU unsigned equal should execute");

        assert_eq!(equal.gpr(6), Some(0));

        let mut greater = Cpu::new();
        assert_eq!(greater.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));

        execute_immediate_comparison(&mut greater, immediate_word(0x0b, 4, 6, 0xfffe))
            .expect("SLTIU unsigned greater-than should execute");

        assert_eq!(greater.gpr(6), Some(0));
    }

    #[test]
    fn immediate_comparison_sltiu_sign_extends_immediate_before_unsigned_compare() {
        let mut sign_extended = Cpu::new();
        assert_eq!(sign_extended.set_gpr(4, 0x0000_0000_0001_0000), Ok(()));

        execute_immediate_comparison(&mut sign_extended, immediate_word(0x0b, 4, 6, 0xffff))
            .expect("SLTIU sign-extended immediate should execute");

        assert_eq!(sign_extended.gpr(6), Some(1));

        let mut high_bit = Cpu::new();
        assert_eq!(high_bit.set_gpr(4, 0x8000_0000_0000_0000), Ok(()));

        execute_immediate_comparison(&mut high_bit, immediate_word(0x0b, 4, 6, 0))
            .expect("SLTIU high-bit source should execute");

        assert_eq!(high_bit.gpr(6), Some(0));
    }

    #[test]
    fn immediate_comparison_alias_and_zero_register_rules_match_gpr_semantics() {
        let mut slti_aliased = Cpu::new();
        assert_eq!(slti_aliased.set_gpr(4, 0), Ok(()));

        execute_immediate_comparison(&mut slti_aliased, immediate_word(0x0a, 4, 4, 1))
            .expect("SLTI should execute with rs == rt");

        assert_eq!(slti_aliased.gpr(4), Some(1));

        let mut sltiu_aliased = Cpu::new();
        assert_eq!(sltiu_aliased.set_gpr(4, 0), Ok(()));

        execute_immediate_comparison(&mut sltiu_aliased, immediate_word(0x0b, 4, 4, 1))
            .expect("SLTIU should execute with rs == rt");

        assert_eq!(sltiu_aliased.gpr(4), Some(1));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 0), Ok(()));

        execute_immediate_comparison(&mut destination_zero, immediate_word(0x0a, 4, 0, 1))
            .expect("SLTI writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));

        execute_immediate_comparison(&mut destination_zero, immediate_word(0x0b, 4, 0, 1))
            .expect("SLTIU writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
    }

    #[test]
    fn immediate_comparison_rejects_other_immediate_identities_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 1), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(immediate_word(0x0a, 4, 6, 1));

        let error = cpu
            .execute_immediate_comparison_instruction(CpuInstructionIdentity::Addiu, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuImmediateComparisonExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::Addiu
            )
        );
        assert_eq!(cpu.gpr(4), Some(1));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn immediate_bitwise_logical_writes_expected_full_width_results() {
        let cases = [
            (
                CpuInstructionIdentity::Andi,
                0x0c,
                0xffff_ffff_ffff_f0f0,
                0x0ff0,
                0x0000_0000_0000_00f0,
            ),
            (
                CpuInstructionIdentity::Ori,
                0x0d,
                0xffff_0000_1234_0000,
                0x8001,
                0xffff_0000_1234_8001,
            ),
            (
                CpuInstructionIdentity::Xori,
                0x0e,
                0xffff_0000_1234_ffff,
                0xffff,
                0xffff_0000_1234_0000,
            ),
        ];

        for (identity, opcode, rs_value, immediate, expected) in cases {
            let mut cpu = Cpu::new();
            assert_eq!(cpu.set_gpr(4, rs_value), Ok(()));

            let executed = execute_immediate_bitwise_logical(
                &mut cpu,
                immediate_word(opcode, 4, 6, immediate),
            )
            .expect("immediate bitwise logical instruction should execute");

            assert_eq!(executed.identity(), identity);
            assert_eq!(cpu.gpr(6), Some(expected));
        }
    }

    #[test]
    fn immediate_bitwise_logical_zero_extends_raw_immediate_u16_only_for_family() {
        let mut high_bit = Cpu::new();
        assert_eq!(high_bit.set_gpr(4, 0), Ok(()));

        execute_immediate_bitwise_logical(&mut high_bit, immediate_word(0x0d, 4, 6, 0x8000))
            .expect("ORI high-bit immediate should execute");

        assert_eq!(high_bit.gpr(6), Some(0x0000_0000_0000_8000));

        let mut all_bits = Cpu::new();
        assert_eq!(all_bits.set_gpr(4, 0xffff_ffff_ffff_0000), Ok(()));

        execute_immediate_bitwise_logical(&mut all_bits, immediate_word(0x0c, 4, 6, 0xffff))
            .expect("ANDI all-ones immediate should execute");

        assert_eq!(all_bits.gpr(6), Some(0));

        let mut zero = Cpu::new();
        assert_eq!(zero.set_gpr(4, 0x0123_4567_89ab_cdef), Ok(()));

        execute_immediate_bitwise_logical(&mut zero, immediate_word(0x0e, 4, 6, 0))
            .expect("XORI zero immediate should execute");

        assert_eq!(zero.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn immediate_bitwise_logical_alias_and_zero_register_rules_match_gpr_semantics() {
        let mut aliased = Cpu::new();
        assert_eq!(aliased.set_gpr(4, 0xffff_0000_1234_0000), Ok(()));

        execute_immediate_bitwise_logical(&mut aliased, immediate_word(0x0d, 4, 4, 0x8001))
            .expect("ORI should execute with rs == rt");

        assert_eq!(aliased.gpr(4), Some(0xffff_0000_1234_8001));

        let mut source_zero = Cpu::new();

        execute_immediate_bitwise_logical(&mut source_zero, immediate_word(0x0d, 0, 6, 0xabcd))
            .expect("ORI should execute with r0 source");

        assert_eq!(source_zero.gpr(6), Some(0x0000_0000_0000_abcd));

        let mut destination_zero = Cpu::new();
        assert_eq!(destination_zero.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));

        execute_immediate_bitwise_logical(
            &mut destination_zero,
            immediate_word(0x0c, 4, 0, 0xffff),
        )
        .expect("ANDI writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
        assert_eq!(destination_zero.gpr(4), Some(0xffff_ffff_ffff_ffff));
    }

    #[test]
    fn immediate_bitwise_logical_rejects_other_immediate_identities_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0xffff_0000_1234_0000), Ok(()));
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(immediate_word(0x0d, 4, 6, 0x8001));

        let error = cpu
            .execute_immediate_bitwise_logical_instruction(CpuInstructionIdentity::Sltiu, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuImmediateBitwiseLogicalExecutionError::UnsupportedIdentity(
                CpuInstructionIdentity::Sltiu
            )
        );
        assert_eq!(cpu.gpr(4), Some(0xffff_0000_1234_0000));
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn upper_immediate_lui_writes_source_clear_shifted_sign_extended_result() {
        let cases = [
            (0x0000, 0x0000_0000_0000_0000),
            (0x0001, 0x0000_0000_0001_0000),
            (0x7fff, 0x0000_0000_7fff_0000),
            (0x8000, 0xffff_ffff_8000_0000),
            (0xffff, 0xffff_ffff_ffff_0000),
        ];

        for (immediate, expected) in cases {
            let mut cpu = Cpu::new();

            let executed = execute_upper_immediate(&mut cpu, immediate_word(0x0f, 0, 6, immediate))
                .expect("LUI should execute");

            assert_eq!(executed.identity(), CpuInstructionIdentity::Lui);
            assert_eq!(cpu.gpr(6), Some(expected));
        }
    }

    #[test]
    fn upper_immediate_lui_ignores_rs_field_and_preserves_zero_register() {
        let mut ignored_rs = Cpu::new();
        assert_eq!(ignored_rs.set_gpr(31, 0x0123_4567_89ab_cdef), Ok(()));

        execute_upper_immediate(&mut ignored_rs, immediate_word(0x0f, 31, 6, 0x1234))
            .expect("LUI should ignore nonzero rs field");

        assert_eq!(ignored_rs.gpr(31), Some(0x0123_4567_89ab_cdef));
        assert_eq!(ignored_rs.gpr(6), Some(0x0000_0000_1234_0000));

        let mut destination_zero = Cpu::new();

        execute_upper_immediate(&mut destination_zero, immediate_word(0x0f, 31, 0, 0xffff))
            .expect("LUI writing r0 should execute");

        assert_eq!(destination_zero.gpr(0), Some(0));
    }

    #[test]
    fn upper_immediate_lui_rejects_other_immediate_identities_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        let fields = decode(immediate_word(0x0f, 0, 6, 0xffff));

        let error = cpu
            .execute_upper_immediate_instruction(CpuInstructionIdentity::Ori, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuUpperImmediateExecutionError::UnsupportedIdentity(CpuInstructionIdentity::Ori)
        );
        assert_eq!(cpu.gpr(6), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn special_shift_rejects_non_shift_identity_without_mutation() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.set_gpr(4, 0x0000_0000_1234_5678), Ok(()));
        let fields = decode(special_shift_word(0, 4, 5, 1, 0x00));

        let error = cpu
            .execute_special_shift_instruction(CpuInstructionIdentity::SpecialSync, fields)
            .unwrap_err();

        assert_eq!(
            error,
            CpuSpecialShiftExecutionError::UnsupportedIdentity(CpuInstructionIdentity::SpecialSync)
        );
        assert_eq!(cpu.gpr(4), Some(0x0000_0000_1234_5678));
        assert_eq!(cpu.gpr(5), Some(0));
    }
}
