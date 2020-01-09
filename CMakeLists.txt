cmake_minimum_required(VERSION 3.15)
project(t)

set(CMAKE_CXX_STANDARD 17)

add_executable(t src/main.cpp src/cxxopts.hpp src/functions.hpp src/sha256/sha256.cpp src/sha256/sha256.h)