#include "bootstrap.hpp"

#include "machine.hpp"

namespace fn64::bootstrap_detail {

void run_arithmetic_demos(Machine& machine);
void run_data_demos(Machine& machine);
void run_trap_demos(Machine& machine);
void run_control_demos(Machine& machine);

}  // namespace fn64::bootstrap_detail

namespace fn64 {

void run_bootstrap_demos(Machine& machine) {
  bootstrap_detail::run_arithmetic_demos(machine);
  bootstrap_detail::run_data_demos(machine);
  bootstrap_detail::run_trap_demos(machine);
  bootstrap_detail::run_control_demos(machine);
}

}  // namespace fn64