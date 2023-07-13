cmake_minimum_required(VERSION 3.22)

project(
  PlotterCXX
  VERSION 1.0
  LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4
)
FetchContent_MakeAvailable(Corrosion)

include(cmake/corrosion_cxx.cmake)
set(Rust_CARGO_TARGET x86_64-unknown-linux-gnu)
add_library_rust(PATH rust-lib NAMESPACE rust)

add_executable(main src/main.cpp)

target_link_libraries(main
  PUBLIC
  rust::rust-lib
)
