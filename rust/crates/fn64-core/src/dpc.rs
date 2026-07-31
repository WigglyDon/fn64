use crate::rsp::{
    classify_instruction_source, MachineRspInstructionSource, MachineRspScalarRegisterSource,
};
use crate::sp_imem::SpImemByteProvenance;

pub const DPC_COUNTER_VALUE_MASK: u32 = 0x00ff_ffff;
pub const DPC_STATUS_CLEAR_TMEM_COUNTER: u32 = 0x0000_0040;
pub const DPC_STATUS_CLEAR_PIPE_COUNTER: u32 = 0x0000_0080;
pub const DPC_STATUS_CLEAR_COMMAND_COUNTER: u32 = 0x0000_0100;
pub const DPC_STATUS_CLEAR_CLOCK_COUNTER: u32 = 0x0000_0200;
pub const DPC_STATUS_X105_COUNTER_CLEAR_COMMAND: u32 =
    DPC_STATUS_CLEAR_TMEM_COUNTER | DPC_STATUS_CLEAR_CLOCK_COUNTER;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineDpcCounterIdentity {
    Clock,
    CommandBusy,
    PipeBusy,
    TmemLoad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineDpcCounterUnavailableSource {
    ConstructionOrResetUndefined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineDpcStatusClearSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    control_register_index: u8,
    raw_command_word: u32,
    counter_clear_bit: u32,
    counter: MachineDpcCounterIdentity,
}

impl MachineDpcStatusClearSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn control_register_index(&self) -> u8 {
        self.control_register_index
    }

    pub const fn raw_command_word(&self) -> u32 {
        self.raw_command_word
    }

    pub const fn counter_clear_bit(&self) -> u32 {
        self.counter_clear_bit
    }

    pub const fn counter(&self) -> MachineDpcCounterIdentity {
        self.counter
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineDpcCounterSource {
    RspMtc0StatusClear(Box<MachineDpcStatusClearSource>),
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

impl MachineDpcCounterSource {
    pub fn rsp_mtc0_status_clear(&self) -> Option<&MachineDpcStatusClearSource> {
        match self {
            Self::RspMtc0StatusClear(source) => Some(source),
            #[cfg(test)]
            Self::GeneratedMachineTestStaging => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineDpcCounterState {
    Available {
        value: u32,
        source: MachineDpcCounterSource,
    },
    Unavailable {
        source: MachineDpcCounterUnavailableSource,
    },
}

impl MachineDpcCounterState {
    pub const fn value(&self) -> Option<u32> {
        match self {
            Self::Available { value, .. } => Some(*value),
            Self::Unavailable { .. } => None,
        }
    }

    pub fn source(&self) -> Option<MachineDpcCounterSource> {
        match self {
            Self::Available { source, .. } => Some(source.clone()),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn unavailable_source(&self) -> Option<MachineDpcCounterUnavailableSource> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { source } => Some(*source),
        }
    }

    const fn construction_or_reset_undefined() -> Self {
        Self::Unavailable {
            source: MachineDpcCounterUnavailableSource::ConstructionOrResetUndefined,
        }
    }

    fn invariant_holds(&self) -> bool {
        match self {
            Self::Available { value, .. } => value & !DPC_COUNTER_VALUE_MASK == 0,
            Self::Unavailable { .. } => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MachineDpcStatusClearRejection {
    UnsupportedCommand {
        raw_command_word: u32,
    },
    MalformedCounter {
        counter: MachineDpcCounterIdentity,
        value: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineDpcStatusClearPlan {
    clock_counter: MachineDpcCounterState,
    tmem_load_counter: MachineDpcCounterState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Dpc {
    clock_counter: MachineDpcCounterState,
    command_busy_counter: MachineDpcCounterState,
    pipe_busy_counter: MachineDpcCounterState,
    tmem_load_counter: MachineDpcCounterState,
}

impl Default for Dpc {
    fn default() -> Self {
        Self {
            clock_counter: MachineDpcCounterState::construction_or_reset_undefined(),
            command_busy_counter: MachineDpcCounterState::construction_or_reset_undefined(),
            pipe_busy_counter: MachineDpcCounterState::construction_or_reset_undefined(),
            tmem_load_counter: MachineDpcCounterState::construction_or_reset_undefined(),
        }
    }
}

impl Dpc {
    pub(crate) const fn clock_counter(&self) -> &MachineDpcCounterState {
        &self.clock_counter
    }

    pub(crate) const fn command_busy_counter(&self) -> &MachineDpcCounterState {
        &self.command_busy_counter
    }

    pub(crate) const fn pipe_busy_counter(&self) -> &MachineDpcCounterState {
        &self.pipe_busy_counter
    }

    pub(crate) const fn tmem_load_counter(&self) -> &MachineDpcCounterState {
        &self.tmem_load_counter
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn plan_rsp_mtc0_status_clear(
        &self,
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        source_gpr: u8,
        source_value: u32,
        source: MachineRspScalarRegisterSource,
        control_register_index: u8,
    ) -> Result<MachineDpcStatusClearPlan, MachineDpcStatusClearRejection> {
        self.validate_counter(MachineDpcCounterIdentity::Clock, &self.clock_counter)?;
        self.validate_counter(
            MachineDpcCounterIdentity::CommandBusy,
            &self.command_busy_counter,
        )?;
        self.validate_counter(MachineDpcCounterIdentity::PipeBusy, &self.pipe_busy_counter)?;
        self.validate_counter(MachineDpcCounterIdentity::TmemLoad, &self.tmem_load_counter)?;

        if source_value != DPC_STATUS_X105_COUNTER_CLEAR_COMMAND {
            return Err(MachineDpcStatusClearRejection::UnsupportedCommand {
                raw_command_word: source_value,
            });
        }

        let cleared_counter = |counter, counter_clear_bit| MachineDpcCounterState::Available {
            value: 0,
            source: MachineDpcCounterSource::RspMtc0StatusClear(Box::new(
                MachineDpcStatusClearSource {
                    instruction_pc,
                    instruction_provenance,
                    source_gpr,
                    source_value,
                    source: source.clone(),
                    control_register_index,
                    raw_command_word: source_value,
                    counter_clear_bit,
                    counter,
                },
            )),
        };

        Ok(MachineDpcStatusClearPlan {
            clock_counter: cleared_counter(
                MachineDpcCounterIdentity::Clock,
                DPC_STATUS_CLEAR_CLOCK_COUNTER,
            ),
            tmem_load_counter: cleared_counter(
                MachineDpcCounterIdentity::TmemLoad,
                DPC_STATUS_CLEAR_TMEM_COUNTER,
            ),
        })
    }

    pub(crate) fn apply_rsp_mtc0_status_clear(&mut self, plan: MachineDpcStatusClearPlan) {
        self.clock_counter = plan.clock_counter;
        self.tmem_load_counter = plan.tmem_load_counter;
    }

    fn validate_counter(
        &self,
        counter: MachineDpcCounterIdentity,
        state: &MachineDpcCounterState,
    ) -> Result<(), MachineDpcStatusClearRejection> {
        if state.invariant_holds() {
            return Ok(());
        }
        let value = state
            .value()
            .expect("only an available DPC counter can violate the 24-bit invariant");
        Err(MachineDpcStatusClearRejection::MalformedCounter { counter, value })
    }

    #[cfg(test)]
    pub(crate) fn replace_counter_for_test(
        &mut self,
        counter: MachineDpcCounterIdentity,
        state: MachineDpcCounterState,
    ) {
        match counter {
            MachineDpcCounterIdentity::Clock => self.clock_counter = state,
            MachineDpcCounterIdentity::CommandBusy => self.command_busy_counter = state,
            MachineDpcCounterIdentity::PipeBusy => self.pipe_busy_counter = state,
            MachineDpcCounterIdentity::TmemLoad => self.tmem_load_counter = state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_provenance() -> [SpImemByteProvenance; 4] {
        [SpImemByteProvenance::GeneratedMachineTestStaging; 4]
    }

    #[test]
    fn dpc_counter_owner_starts_with_four_independent_unavailable_states() {
        let dpc = Dpc::default();
        for counter in [
            dpc.clock_counter(),
            dpc.command_busy_counter(),
            dpc.pipe_busy_counter(),
            dpc.tmem_load_counter(),
        ] {
            assert_eq!(counter.value(), None);
            assert_eq!(
                counter.unavailable_source(),
                Some(MachineDpcCounterUnavailableSource::ConstructionOrResetUndefined)
            );
        }
    }

    #[test]
    fn dpc_counter_width_invariant_rejects_more_than_twenty_four_bits() {
        let mut dpc = Dpc::default();
        dpc.replace_counter_for_test(
            MachineDpcCounterIdentity::Clock,
            MachineDpcCounterState::Available {
                value: DPC_COUNTER_VALUE_MASK + 1,
                source: MachineDpcCounterSource::GeneratedMachineTestStaging,
            },
        );

        assert_eq!(
            dpc.plan_rsp_mtc0_status_clear(
                0x098,
                generated_provenance(),
                3,
                DPC_STATUS_X105_COUNTER_CLEAR_COMMAND,
                MachineRspScalarRegisterSource::ArchitecturalZero,
                11,
            ),
            Err(MachineDpcStatusClearRejection::MalformedCounter {
                counter: MachineDpcCounterIdentity::Clock,
                value: DPC_COUNTER_VALUE_MASK + 1,
            })
        );
    }

    #[test]
    fn dpc_clock_dpc_tmem_counter_clear_preserves_dpc_pipe_and_dpc_command_counter_truth() {
        let mut dpc = Dpc::default();
        for (counter, value) in [
            (MachineDpcCounterIdentity::Clock, 0x0001_2345),
            (MachineDpcCounterIdentity::CommandBusy, 0x0002_3456),
            (MachineDpcCounterIdentity::PipeBusy, 0x0003_4567),
            (MachineDpcCounterIdentity::TmemLoad, 0x0004_5678),
        ] {
            dpc.replace_counter_for_test(
                counter,
                MachineDpcCounterState::Available {
                    value,
                    source: MachineDpcCounterSource::GeneratedMachineTestStaging,
                },
            );
        }
        let command_before = dpc.command_busy_counter().clone();
        let pipe_before = dpc.pipe_busy_counter().clone();
        let plan = dpc
            .plan_rsp_mtc0_status_clear(
                0x098,
                generated_provenance(),
                3,
                DPC_STATUS_X105_COUNTER_CLEAR_COMMAND,
                MachineRspScalarRegisterSource::ArchitecturalZero,
                11,
            )
            .unwrap();
        dpc.apply_rsp_mtc0_status_clear(plan);

        assert_eq!(dpc.clock_counter().value(), Some(0));
        assert_eq!(dpc.tmem_load_counter().value(), Some(0));
        assert_eq!(dpc.command_busy_counter(), &command_before);
        assert_eq!(dpc.pipe_busy_counter(), &pipe_before);
        for (counter, identity, clear_bit) in [
            (
                dpc.clock_counter(),
                MachineDpcCounterIdentity::Clock,
                DPC_STATUS_CLEAR_CLOCK_COUNTER,
            ),
            (
                dpc.tmem_load_counter(),
                MachineDpcCounterIdentity::TmemLoad,
                DPC_STATUS_CLEAR_TMEM_COUNTER,
            ),
        ] {
            let source = counter
                .source()
                .unwrap()
                .rsp_mtc0_status_clear()
                .unwrap()
                .clone();
            assert_eq!(source.instruction_pc(), 0x098);
            assert_eq!(source.instruction_provenance(), generated_provenance());
            assert_eq!(source.source_gpr(), 3);
            assert_eq!(source.source_value(), DPC_STATUS_X105_COUNTER_CLEAR_COMMAND);
            assert_eq!(source.control_register_index(), 11);
            assert_eq!(
                source.raw_command_word(),
                DPC_STATUS_X105_COUNTER_CLEAR_COMMAND
            );
            assert_eq!(source.counter_clear_bit(), clear_bit);
            assert_eq!(source.counter(), identity);
        }

        let first_clock_clear = dpc.clock_counter().clone();
        let first_tmem_clear = dpc.tmem_load_counter().clone();
        let repeated = dpc
            .plan_rsp_mtc0_status_clear(
                0x0a8,
                generated_provenance(),
                3,
                DPC_STATUS_X105_COUNTER_CLEAR_COMMAND,
                MachineRspScalarRegisterSource::ArchitecturalZero,
                11,
            )
            .unwrap();
        dpc.apply_rsp_mtc0_status_clear(repeated);
        assert_eq!(dpc.clock_counter().value(), Some(0));
        assert_eq!(dpc.tmem_load_counter().value(), Some(0));
        assert_ne!(dpc.clock_counter(), &first_clock_clear);
        assert_ne!(dpc.tmem_load_counter(), &first_tmem_clear);
        assert_eq!(
            dpc.clock_counter()
                .source()
                .unwrap()
                .rsp_mtc0_status_clear()
                .unwrap()
                .instruction_pc(),
            0x0a8
        );
        assert_eq!(
            dpc.tmem_load_counter()
                .source()
                .unwrap()
                .rsp_mtc0_status_clear()
                .unwrap()
                .instruction_pc(),
            0x0a8
        );
        assert_eq!(dpc.command_busy_counter(), &command_before);
        assert_eq!(dpc.pipe_busy_counter(), &pipe_before);
    }

    #[test]
    fn every_non_public_dpc_status_command_rejects_without_mutation() {
        for command in [
            0,
            DPC_STATUS_CLEAR_TMEM_COUNTER,
            DPC_STATUS_CLEAR_CLOCK_COUNTER,
            DPC_STATUS_CLEAR_PIPE_COUNTER,
            DPC_STATUS_CLEAR_COMMAND_COUNTER,
            0x0000_0001,
            0x0000_0002,
            0x0000_0004,
            0x0000_0008,
            0x0000_0010,
            0x0000_0020,
            0x8000_0240,
        ] {
            let dpc = Dpc::default();
            let before = dpc.clone();
            assert_eq!(
                dpc.plan_rsp_mtc0_status_clear(
                    0x098,
                    generated_provenance(),
                    3,
                    command,
                    MachineRspScalarRegisterSource::ArchitecturalZero,
                    11,
                ),
                Err(MachineDpcStatusClearRejection::UnsupportedCommand {
                    raw_command_word: command,
                })
            );
            assert_eq!(dpc, before);
        }
    }
}
