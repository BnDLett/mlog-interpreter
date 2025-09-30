cmake_minimum_required(VERSION 4.0)
project(mlog_interpreter C CXX)

set(CMAKE_C_STANDARD 11)

add_executable(mlog_interpreter
        src/main.c
        src/lexer.h
        src/error.h
)
