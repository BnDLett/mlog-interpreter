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
        auto value = to_print->variable->value;
        if (value->value_updated) {
            value->str_value = to_string(value->value);
        }

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
    global_state->print_buffer.clear();
}

// this pains me. Deeply.
static void op_callback(vector<Value*> values, GlobalState* global_state) {
    // op add new_num first_num second_num
    const Value* operation = values.at(0);
    const Value* variable = values.at(1);
    const Value* x = values.at(2);
    const Value* y = values.at(3);

    double x_value = x->value;
    double y_value = y->value;

    if (x->variable != nullptr) {
        x_value = x->variable->value->value;
    }
    if (y->variable != nullptr) {
        y_value = y->variable->value->value;
    }

    // cout << static_cast<int>(operation->variable->value->value) << "\n";
    variable->variable->value->value_updated = true;

    #define OP(op) variable->variable->value->value = x_value op y_value

    switch (static_cast<int>(operation->variable->value->value)) {
    case 0:
        OP(+);
        break;
    case 1:
        OP(-);
        break;
    case 2:
        OP(*);
        break;
    case 3:
        OP(/);
        break;

    default:
        // unexpected case, so break out of the program
        global_state->executor_index = -1;
        cout << "Unrecognized operation on line " << global_state->executor_index << ".";
    }
}

#define OPERATION(name, id) {auto value = new Value("id", id, nullptr); auto variable = new Variable(value, name); global_state->variables.push_back(variable);}

static void init_std(struct GlobalState* global_state) {
    OPERATION("add", 0);
    OPERATION("sub", 1);
    OPERATION("mul", 2);
    OPERATION("div", 3);

    create_keyword("print", 1, print_callback, global_state);
    create_keyword("printflush", 1, printflush_callback, global_state);
    create_keyword("op", 4, op_callback, global_state);
}
