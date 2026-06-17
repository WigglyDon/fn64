cmake_minimum_required(VERSION 3.25)

project(fn64 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(fn64_core
  src/core/rom.cpp
  src/core/cartridge.cpp
  src/core/machine.cpp
  src/core/machine_cpu.cpp
)

target_include_directories(fn64_core PUBLIC src/core)

add_library(fn64_proof
  src/proof/bootstrap.cpp
  src/proof/bootstrap_common.cpp
  src/proof/bootstrap_arith.cpp
  src/proof/bootstrap_data.cpp
  src/proof/bootstrap_traps.cpp
  src/proof/bootstrap_control.cpp
  src/proof/bootstrap_control_jumps.cpp
  src/proof/bootstrap_control_unsupported.cpp
  src/proof/bootstrap_control_ordinary_branches.cpp
  src/proof/bootstrap_control_branch_likely.cpp
  src/proof/bootstrap_control_register_jump_demos.cpp
)

target_include_directories(fn64_proof PUBLIC src/proof)
target_link_libraries(fn64_proof PUBLIC fn64_core)

add_executable(fn64_inspect
  src/host/cli/inspect_main.cpp
)

target_link_libraries(fn64_inspect PRIVATE fn64_core)

add_executable(fn64_selftest
  src/proof/selftest_main.cpp
)

target_link_libraries(fn64_selftest PRIVATE fn64_core fn64_proof)

find_package(SDL3 CONFIG)

if (SDL3_FOUND)
  add_executable(fn64
    src/host/sdl/main.cpp
    src/host/sdl/app.cpp
  )

  target_include_directories(fn64 PRIVATE src/host/sdl)
  target_link_libraries(fn64 PRIVATE SDL3::SDL3 fn64_core)
else()
  message(STATUS "SDL3 not found; fn64 SDL host executable will not be built")
endif()

if (CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  target_compile_options(fn64_core PRIVATE -Wall -Wextra -Wpedantic)
  target_compile_options(fn64_proof PRIVATE -Wall -Wextra -Wpedantic)
  target_compile_options(fn64_inspect PRIVATE -Wall -Wextra -Wpedantic)
  target_compile_options(fn64_selftest PRIVATE -Wall -Wextra -Wpedantic)
  if (TARGET fn64)
    target_compile_options(fn64 PRIVATE -Wall -Wextra -Wpedantic)
  endif()
endif()
