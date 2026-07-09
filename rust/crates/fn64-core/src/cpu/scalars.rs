use super::Cpu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct CpuControlFlowSnapshot {
    pc: u32,
    next_pc: u32,
}

#[allow(dead_code)]
impl CpuControlFlowSnapshot {
    pub(crate) const fn pc(self) -> u32 {
        self.pc
    }

    pub(crate) const fn next_pc(self) -> u32 {
        self.next_pc
    }
}

impl Cpu {
    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn next_pc(&self) -> u32 {
        self.next_pc
    }

    pub fn hi(&self) -> u64 {
        self.hi
    }

    pub fn lo(&self) -> u64 {
        self.lo
    }

    pub fn stage_pc(&mut self, value: u32) {
        self.pc = value;
        // Mirrors C++ write_cpu_pc: staging PC also stages the sequential next PC.
        self.next_pc = sequential_instruction_address(value);
    }

    pub fn stage_next_pc(&mut self, value: u32) {
        self.next_pc = value;
    }

    pub fn stage_hi(&mut self, value: u64) {
        self.hi = value;
    }

    pub fn stage_lo(&mut self, value: u64) {
        self.lo = value;
    }

    #[allow(dead_code)]
    pub(crate) fn stage_next_sequential_pc_for_step(&mut self) {
        self.next_pc = sequential_instruction_address(self.next_pc);
    }

    #[allow(dead_code)]
    pub(crate) const fn capture_control_flow(&self) -> CpuControlFlowSnapshot {
        CpuControlFlowSnapshot {
            pc: self.pc,
            next_pc: self.next_pc,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn restore_control_flow(&mut self, snapshot: CpuControlFlowSnapshot) {
        self.pc = snapshot.pc;
        self.next_pc = snapshot.next_pc;
    }

    #[allow(dead_code)]
    pub(crate) fn commit_staged_step_control_flow(&mut self, snapshot: CpuControlFlowSnapshot) {
        self.pc = snapshot.next_pc;
    }
}

fn sequential_instruction_address(address: u32) -> u32 {
    address.wrapping_add(4)
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    fn assert_cop0_construction_state(cpu: &Cpu) {
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
    fn stage_pc_sets_pc_and_sequential_next_pc_without_touching_other_cpu_state() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        cpu.stage_next_pc(0x1357_9bdf);

        cpu.stage_pc(0x8000_1000);

        assert_eq!(cpu.pc(), 0x8000_1000);
        assert_eq!(cpu.next_pc(), 0x8000_1004);
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn stage_pc_uses_cpp_u32_wrapping_for_next_pc() {
        for (pc, next_pc) in [
            (0xffff_fffc, 0x0000_0000),
            (0xffff_fffd, 0x0000_0001),
            (0xffff_fffe, 0x0000_0002),
            (0xffff_ffff, 0x0000_0003),
        ] {
            let mut cpu = Cpu::new();

            cpu.stage_pc(pc);

            assert_eq!(cpu.pc(), pc);
            assert_eq!(cpu.next_pc(), next_pc);
        }
    }

    #[test]
    fn stage_next_pc_updates_only_next_pc_without_validation() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0xfeed_face_cafe_beef), Ok(()));
        cpu.stage_hi(0x0102_0304_0506_0708);
        cpu.stage_lo(0x1112_1314_1516_1718);
        let original_pc = cpu.pc();

        cpu.stage_next_pc(0x8000_0002);

        assert_eq!(cpu.pc(), original_pc);
        assert_eq!(cpu.next_pc(), 0x8000_0002);
        assert_eq!(cpu.hi(), 0x0102_0304_0506_0708);
        assert_eq!(cpu.lo(), 0x1112_1314_1516_1718);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0xfeed_face_cafe_beef));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn stage_hi_updates_only_hi() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0101_0202_0303_0404), Ok(()));
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        cpu.stage_lo(0x9999_aaaa_bbbb_cccc);

        cpu.stage_hi(0x1234_5678_9abc_def0);

        assert_eq!(cpu.pc(), 0x8000_1000);
        assert_eq!(cpu.next_pc(), 0x8000_2000);
        assert_eq!(cpu.hi(), 0x1234_5678_9abc_def0);
        assert_eq!(cpu.lo(), 0x9999_aaaa_bbbb_cccc);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0101_0202_0303_0404));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn stage_lo_updates_only_lo() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0a0b_0c0d_0e0f_1011), Ok(()));
        cpu.stage_pc(0x8000_3000);
        cpu.stage_next_pc(0x8000_4000);
        cpu.stage_hi(0x1234_5678_9abc_def0);

        cpu.stage_lo(0x0fed_cba9_8765_4321);

        assert_eq!(cpu.pc(), 0x8000_3000);
        assert_eq!(cpu.next_pc(), 0x8000_4000);
        assert_eq!(cpu.hi(), 0x1234_5678_9abc_def0);
        assert_eq!(cpu.lo(), 0x0fed_cba9_8765_4321);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0a0b_0c0d_0e0f_1011));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn stage_next_sequential_pc_for_step_uses_current_next_pc_without_changing_pc() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);

        cpu.stage_next_sequential_pc_for_step();

        assert_eq!(cpu.pc(), 0x8000_1000);
        assert_eq!(cpu.next_pc(), 0x8000_2004);
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn stage_next_sequential_pc_for_step_wraps_like_cpp_u32_address_add() {
        for (next_pc, expected_next_pc) in [
            (0xffff_fffc, 0x0000_0000),
            (0xffff_fffd, 0x0000_0001),
            (0xffff_fffe, 0x0000_0002),
            (0xffff_ffff, 0x0000_0003),
        ] {
            let mut cpu = Cpu::new();

            cpu.stage_pc(0x8000_1000);
            cpu.stage_next_pc(next_pc);
            cpu.stage_next_sequential_pc_for_step();

            assert_eq!(cpu.pc(), 0x8000_1000);
            assert_eq!(cpu.next_pc(), expected_next_pc);
        }
    }

    #[test]
    fn control_flow_snapshot_captures_current_pc_and_next_pc() {
        let mut cpu = Cpu::new();

        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);

        let snapshot = cpu.capture_control_flow();

        assert_eq!(snapshot.pc(), 0x8000_1000);
        assert_eq!(snapshot.next_pc(), 0x8000_2000);
    }

    #[test]
    fn control_flow_restore_restores_only_pc_and_next_pc() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        let snapshot = cpu.capture_control_flow();

        cpu.stage_pc(0x8000_3000);
        cpu.stage_next_pc(0x8000_4000);
        cpu.restore_control_flow(snapshot);

        assert_eq!(cpu.pc(), 0x8000_1000);
        assert_eq!(cpu.next_pc(), 0x8000_2000);
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn commit_staged_step_control_flow_sets_pc_to_snapshot_next_pc() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        cpu.stage_hi(0x1111_2222_3333_4444);
        cpu.stage_lo(0x5555_6666_7777_8888);
        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0x8000_2000);
        let snapshot = cpu.capture_control_flow();

        cpu.stage_next_sequential_pc_for_step();
        cpu.commit_staged_step_control_flow(snapshot);

        assert_eq!(cpu.pc(), 0x8000_2000);
        assert_eq!(cpu.next_pc(), 0x8000_2004);
        assert_eq!(cpu.hi(), 0x1111_2222_3333_4444);
        assert_eq!(cpu.lo(), 0x5555_6666_7777_8888);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_cop0_construction_state(&cpu);
    }

    #[test]
    fn commit_staged_step_control_flow_preserves_already_staged_next_pc() {
        let mut cpu = Cpu::new();

        cpu.stage_pc(0x8000_1000);
        cpu.stage_next_pc(0xffff_fffc);
        let snapshot = cpu.capture_control_flow();

        cpu.stage_next_sequential_pc_for_step();
        cpu.commit_staged_step_control_flow(snapshot);

        assert_eq!(cpu.pc(), 0xffff_fffc);
        assert_eq!(cpu.next_pc(), 0x0000_0000);
    }
}
