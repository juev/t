cmake_minimum_required(VERSION 3.15)
project(t)

set(CMAKE_CXX_STANDARD 17)

add_executable(t src/main.cpp src/opts/cxxopts.hpp src/functions.hpp)