cmake_minimum_required(VERSION 3.25)

project(fn64 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(SDL3 REQUIRED CONFIG)

add_executable(fn64
  src/main.cpp
  src/app.cpp
  src/rom.cpp
  src/cartridge.cpp
  src/machine.cpp
)

target_include_directories(fn64 PRIVATE src)
target_link_libraries(fn64 PRIVATE SDL3::SDL3)

if (CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  target_compile_options(fn64 PRIVATE -Wall -Wextra -Wpedantic)
endif()