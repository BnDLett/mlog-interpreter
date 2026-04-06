//
// Created by lett on 11/3/25.
//

#ifndef MLOG_INTERPRETER_VARIABLE_PARSE_H
#define MLOG_INTERPRETER_VARIABLE_PARSE_H
#include "lexer.h"
#endif //MLOG_INTERPRETER_VARIABLE_PARSE_H

#define SET_KEYWORD "set"

inline vector<Line*> get_declarations(const vector<Line*>& program) {
    vector<Line*> declarations = {};

    for (Line* line : program) {
        // if (line.callback == NULL || line.callback->name == NULL) {
        //     continue;
        // }

        if (line->callback->name != SET_KEYWORD) {
            continue;
        }

        declarations.push_back(line);
    }

    return declarations;
}

static void parse_variables(const vector<Line*>* program) {
    const vector<Line*> declarations = get_declarations(*program);

    for (int i = 0; i < declarations.capacity(); i++) {
        const Line* declaration = declarations[i];

        if (declaration == nullptr) continue;

        Variable* variable = declaration->values[0]->variable;
        variable->value = declaration->values.at(1);

        // global_state->variables.push_back(variable);
    }
}

// empty since its functionality is handled at compile-time
static void set_callback(vector<Value*>, GlobalState*) {}

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
