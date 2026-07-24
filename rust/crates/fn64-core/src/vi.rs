use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const VI_BASE_PHYSICAL_ADDRESS: u32 = 0x0440_0000;
pub const VI_CURRENT_PHYSICAL_ADDRESS: u32 = VI_BASE_PHYSICAL_ADDRESS + 0x10;
pub const VI_NTSC_HALF_LINES_PER_FIELD: u16 = 525;
pub const VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE: u16 = 1_500;
const VI_REGISTER_COUNT: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineViRegister {
    Control,
    Origin,
    Width,
    VerticalInterrupt,
    Current,
    Burst,
    VerticalSync,
    HorizontalSync,
    Leap,
    HorizontalStart,
    VerticalStart,
    VerticalBurst,
    XScale,
    YScale,
}

impl MachineViRegister {
    pub const fn physical_address(self) -> u32 {
        VI_BASE_PHYSICAL_ADDRESS
            + match self {
                Self::Control => 0x00,
                Self::Origin => 0x04,
                Self::Width => 0x08,
                Self::VerticalInterrupt => 0x0c,
                Self::Current => 0x10,
                Self::Burst => 0x14,
                Self::VerticalSync => 0x18,
                Self::HorizontalSync => 0x1c,
                Self::Leap => 0x20,
                Self::HorizontalStart => 0x24,
                Self::VerticalStart => 0x28,
                Self::VerticalBurst => 0x2c,
                Self::XScale => 0x30,
                Self::YScale => 0x34,
            }
    }

    const fn index(self) -> usize {
        match self {
            Self::Control => 0,
            Self::Origin => 1,
            Self::Width => 2,
            Self::VerticalInterrupt => 3,
            Self::Current => 4,
            Self::Burst => 5,
            Self::VerticalSync => 6,
            Self::HorizontalSync => 7,
            Self::Leap => 8,
            Self::HorizontalStart => 9,
            Self::VerticalStart => 10,
            Self::VerticalBurst => 11,
            Self::XScale => 12,
            Self::YScale => 13,
        }
    }

    pub const fn defined_mask(self) -> u32 {
        match self {
            // Retail mode tables carry bits 12..15 (commonly 0b0011) even
            // though the pinned SDK header labels them reserved. Preserve
            // those bits as uninterpreted raw request truth; no VI rendering
            // effect is derived from them.
            Self::Control => 0x0001_f35f,
            Self::Origin => 0x00ff_ffff,
            Self::Width => 0x0000_0fff,
            Self::VerticalInterrupt | Self::VerticalSync => 0x0000_03ff,
            Self::Current => u32::MAX,
            Self::Burst => 0x3fff_ffff,
            Self::HorizontalSync => 0x001f_0fff,
            Self::Leap
            | Self::HorizontalStart
            | Self::VerticalStart
            | Self::VerticalBurst
            | Self::XScale
            | Self::YScale => 0x0fff_0fff,
        }
    }
}

pub const fn vi_register(physical_address: u32) -> Option<MachineViRegister> {
    match physical_address.checked_sub(VI_BASE_PHYSICAL_ADDRESS) {
        Some(0x00) => Some(MachineViRegister::Control),
        Some(0x04) => Some(MachineViRegister::Origin),
        Some(0x08) => Some(MachineViRegister::Width),
        Some(0x0c) => Some(MachineViRegister::VerticalInterrupt),
        Some(0x10) => Some(MachineViRegister::Current),
        Some(0x14) => Some(MachineViRegister::Burst),
        Some(0x18) => Some(MachineViRegister::VerticalSync),
        Some(0x1c) => Some(MachineViRegister::HorizontalSync),
        Some(0x20) => Some(MachineViRegister::Leap),
        Some(0x24) => Some(MachineViRegister::HorizontalStart),
        Some(0x28) => Some(MachineViRegister::VerticalStart),
        Some(0x2c) => Some(MachineViRegister::VerticalBurst),
        Some(0x30) => Some(MachineViRegister::XScale),
        Some(0x34) => Some(MachineViRegister::YScale),
        Some(_) | None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineViCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineViCpuStoreProvenance {
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
pub struct MachineViRegisterState {
    register: MachineViRegister,
    raw_word: u32,
    provenance: MachineViCpuStoreProvenance,
}

impl MachineViRegisterState {
    pub(crate) const fn new(
        register: MachineViRegister,
        raw_word: u32,
        provenance: MachineViCpuStoreProvenance,
    ) -> Self {
        Self {
            register,
            raw_word,
            provenance,
        }
    }

    pub const fn register(self) -> MachineViRegister {
        self.register
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn provenance(self) -> MachineViCpuStoreProvenance {
        self.provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineViCurrentState {
    half_line: u16,
    committed_step_phase: u16,
}

impl MachineViCurrentState {
    pub const fn half_line(self) -> u16 {
        self.half_line
    }

    pub const fn committed_step_phase(self) -> u16 {
        self.committed_step_phase
    }

    pub const fn read_word(self) -> u32 {
        self.half_line as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Vi {
    current: MachineViCurrentState,
    registers: [Option<MachineViRegisterState>; VI_REGISTER_COUNT],
}

impl Default for Vi {
    fn default() -> Self {
        Self {
            current: MachineViCurrentState {
                half_line: 0,
                committed_step_phase: 0,
            },
            registers: [None; VI_REGISTER_COUNT],
        }
    }
}

impl Vi {
    pub const fn current_state(self) -> MachineViCurrentState {
        self.current
    }

    pub const fn register_state(
        self,
        register: MachineViRegister,
    ) -> Option<MachineViRegisterState> {
        self.registers[register.index()]
    }

    pub(crate) fn apply_register_store(&mut self, state: MachineViRegisterState) -> bool {
        self.registers[state.register().index()] = Some(state);
        state.register() == MachineViRegister::Current
    }

    pub(crate) fn advance_for_committed_machine_step(&mut self) -> bool {
        let next_phase = self.current.committed_step_phase + 1;
        if next_phase == VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE {
            self.current.committed_step_phase = 0;
            let configured_half_lines = self
                .register_state(MachineViRegister::VerticalSync)
                .map(MachineViRegisterState::raw_word)
                .map(|word| (word & 0x03ff) as u16)
                .filter(|half_lines| *half_lines != 0)
                .unwrap_or(VI_NTSC_HALF_LINES_PER_FIELD);
            self.current.half_line = (self.current.half_line + 1) % configured_half_lines;
            return self
                .register_state(MachineViRegister::VerticalInterrupt)
                .is_some_and(|state| (state.raw_word() & 0x03ff) as u16 == self.current.half_line);
        } else {
            self.current.committed_step_phase = next_phase;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{Vi, VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE, VI_NTSC_HALF_LINES_PER_FIELD};

    #[test]
    fn hostless_current_line_advances_only_on_the_fixed_committed_step_cadence() {
        let mut vi = Vi::default();
        for _ in 1..VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE {
            assert!(!vi.advance_for_committed_machine_step());
        }
        assert_eq!(vi.current_state().half_line(), 0);
        assert_eq!(
            vi.current_state().committed_step_phase(),
            VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE - 1
        );

        assert!(!vi.advance_for_committed_machine_step());
        assert_eq!(vi.current_state().half_line(), 1);
        assert_eq!(vi.current_state().committed_step_phase(), 0);
    }

    #[test]
    fn hostless_current_line_wraps_at_the_fixed_ntsc_field_boundary() {
        let mut vi = Vi::default();
        for _ in 0..u32::from(VI_HOSTLESS_COMMITTED_STEPS_PER_HALF_LINE)
            * u32::from(VI_NTSC_HALF_LINES_PER_FIELD)
        {
            assert!(!vi.advance_for_committed_machine_step());
        }
        assert_eq!(vi.current_state().read_word(), 0);
        assert_eq!(vi.current_state().committed_step_phase(), 0);
    }
}
