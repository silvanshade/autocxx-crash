cmake_minimum_required(VERSION 3.22.1)

project(autocxx-crash LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_COMPILER "clang++")

add_library(autocxx-crash STATIC
  cxx/lib/cmake.cxx)

target_include_directories(autocxx-crash PUBLIC
  cxx/include)

target_compile_options(autocxx-crash PUBLIC
  -std=gnu++2b
)
