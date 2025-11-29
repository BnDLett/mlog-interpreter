//
// Created by lett on 11/28/25.
//

#ifndef MLOG_INTERPRETER_STD_H
#define MLOG_INTERPRETER_STD_H

#endif //MLOG_INTERPRETER_STD_H

#include "../lexer.h"

static void print_callback(vector<Value*> values, GlobalState* global_state) {
    Value* to_print = values.at(0);

    if (to_print->variable != nullptr) {
        global_state->print_buffer.push_back(to_print->variable->value->str_value);
        return;
    }

    if (to_print->value_updated) {
        to_print->str_value = to_string(to_print->value);
    }

    global_state->print_buffer.push_back(to_print->str_value);
}

static void printflush_callback(vector<Value*> values, GlobalState* global_state) {
    cout << values.at(0)->target_block << ": ";

    for (const string& to_print : global_state->print_buffer) {
        cout << to_print;
    }

    cout << endl;
}

static void init_std(struct GlobalState* global_state) {
    create_keyword("print", 1, print_callback, global_state);
    create_keyword("printflush", 1, printflush_callback, global_state);
}
