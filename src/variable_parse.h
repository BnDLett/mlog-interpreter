//
// Created by lett on 11/3/25.
//

#ifndef MLOG_INTERPRETER_VARIABLE_PARSE_H
#define MLOG_INTERPRETER_VARIABLE_PARSE_H
#include "lexer.h"
#endif //MLOG_INTERPRETER_VARIABLE_PARSE_H

#define SET_KEYWORD "set"

static struct Line *get_declarations(const struct Line *program) {
    int var_count = 0;
    struct Line *declarations = malloc(sizeof(struct Line) * VAR_LIMIT);

    for (int i = 0; i < PROGRAM_SIZE_LIMIT; i++) {
        const struct Line line = program[i];

        if (line.callback == NULL || line.callback->name == NULL) {
            continue;
        }

        if (strcmp(line.callback->name, SET_KEYWORD) != 0) {
            continue;
        }

        declarations[i] = line;
        var_count++;
    }

    return declarations;
}

static void parse_variables(const struct Line *program, struct GlobalState* global_state) {
    const struct Line *declarations = get_declarations(program);

    for (int i = 0; i < VAR_LIMIT; i++) {
        const struct Line declaration = declarations[i];

        const struct Value name = declaration.values[0];
        struct Value value = declaration.values[1];

        struct Variable *variable = malloc(sizeof(struct Variable));
        strncpy(variable->name, name.string, VAR_NAME_LIMIT);
        variable->value = &value;

        global_state->variables[global_state->var_index++] = *variable;
    }

    free((void*) declarations);
}

// empty since its functionality is handled at compile-time
static void set_callback() {}

static void init_variable_parser(struct GlobalState* global_state) {
    // struct Callback *callback = malloc(sizeof(struct Callback));
    // callback->name = SET_KEYWORD;
    // callback->parameters = 2;
    // callback->callback = &set_callback;
    //
    // for (int i = 0; i < CALLBACK_LIMIT; i++) {
    //     // if name's null, then it's inaccessible and can be overwritten.
    //     if (global_state.callbacks[i].name != NULL) continue;
    //
    //     global_state.callbacks[i] = *callback;
    //     return;
    // }

    create_keyword("set", 2, set_callback, global_state);
}
