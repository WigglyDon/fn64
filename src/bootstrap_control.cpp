#include "bootstrap_common.hpp"

namespace fn64::bootstrap_detail {

void run_ordinary_jump_demos(Machine& machine);
void run_unsupported_instruction_demos(Machine& machine);
void run_ordinary_branch_demos(Machine& machine);
void run_branch_likely_demos(Machine& machine);
void run_control_register_jump_demos(Machine& machine);

void run_control_demos(Machine& machine) {
  run_ordinary_jump_demos(machine);
  run_unsupported_instruction_demos(machine);
  run_ordinary_branch_demos(machine);
  run_branch_likely_demos(machine);
  run_control_register_jump_demos(machine);
}

}  // namespace fn64::bootstrap_detail