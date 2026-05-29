//
// Created by lett on 11/28/25.
//

#ifndef MLOG_INTERPRETER_STD_H
#define MLOG_INTERPRETER_STD_H

#endif //MLOG_INTERPRETER_STD_H

#include <string>
#include <sstream>

#include "../lexer.h"
// using namespace std;

static void mlog_runtime_error(string message, GlobalState* global_state) {
    cout << "Runtime error on line " << global_state->executor_index << ":\n\t" << message << "\n";
    global_state->executor_index = -1;
}

static void unrecognized_operation(const Value* operation, GlobalState* global_state) {
    stringstream message("");
    message << "Unrecognized operation: " << operation->variable->name;
    mlog_runtime_error(message.str(), global_state);
}

template <typename T> static T get_value(const Value* target) {
    T value = target->value;

    if (target->variable != nullptr) {
        value = target->variable->value->value;
    }

    return value;
}

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

    // double x_value = x->value;
    // double y_value = y->value;
    //
    // if (x->variable != nullptr) {
    //     x_value = x->variable->value->value;
    // }
    // if (y->variable != nullptr) {
    //     y_value = y->variable->value->value;
    // }

    double x_value = get_value<double>(x);
    double y_value = get_value<double>(y);

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
        unrecognized_operation(operation, global_state);
    }
}

static void jump_callback(vector<Value*> values, GlobalState* global_state) {
    // jump 6 notEqual accum limit
    const Value* line = values.at(0);
    const Value* operation = values.at(1);
    const Value* x = values.at(2);
    const Value* y = values.at(3);

    unsigned int line_value = get_value<unsigned int>(line);
    double x_value = get_value<double>(x);
    double y_value = get_value<double>(y);

    bool can_jump;
    switch (static_cast<int>(operation->variable->value->value)) {
    case 4:
        can_jump = x_value == y_value;
        break;
    case 5:
        can_jump = x_value != y_value;
        break;
    case 6:
        can_jump = true;
        break;
    default:
        unrecognized_operation(operation, global_state);
        return;
    }

    if (can_jump) {
        global_state->executor_index = line_value;
    }
}

#define OPERATION(name, id) {auto value = new Value("id", id, nullptr); auto variable = new Variable(value, name); global_state->variables.push_back(variable);}

static void init_std(GlobalState* global_state) {
    OPERATION("add", 0);
    OPERATION("sub", 1);
    OPERATION("mul", 2);
    OPERATION("div", 3);
    OPERATION("equal", 4);
    OPERATION("notEqual", 5);
    OPERATION("always", 6);

    create_keyword("print", 1, print_callback, global_state);
    create_keyword("printflush", 1, printflush_callback, global_state);
    create_keyword("op", 4, op_callback, global_state);
    create_keyword("jump", 4, jump_callback, global_state);
}
