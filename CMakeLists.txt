cmake_minimum_required(VERSION 4.0)
project(mlog_interpreter C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 20)

add_executable(mlog_interpreter
        src/main.cpp
        src/lexer.h
        src/error.h
        src/variable_parse.h
)
