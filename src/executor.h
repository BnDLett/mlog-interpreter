//
// Created by bored on 11/28/2025.
//

#ifndef MLOG_INTERPRETER_EXECUTOR_H
#define MLOG_INTERPRETER_EXECUTOR_H
#include "lexer.h"

#endif //MLOG_INTERPRETER_EXECUTOR_H

inline void execute(const Line& line, GlobalState* global_state) {
    line.callback->callback(line.values, global_state);
}

inline void execute_many(Line program[PROGRAM_SIZE_LIMIT], GlobalState* global_state) {
    while (true) {
        if (global_state->executor_index > PROGRAM_SIZE_LIMIT - 1) {
            break;
        }

        execute(program[global_state->executor_index++], global_state);
    }
}
