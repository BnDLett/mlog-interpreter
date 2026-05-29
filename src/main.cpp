//
// Created by lett on 9/26/25.
//
#include "lexer.h"
#include "error.h"
#include "variable_parse.h"

#include <string>
#include <iostream>
#include <chrono>

#include "executor.h"
#include "libraries/std.h"
using namespace std;

void print_error(const struct Error *error) {
    cout << "Error encountered on line " << error->position + 1 << ".\n\t" << error->message << endl;
}

#define VALIDATE if (error != nullptr) { print_error(error); return 1; }

int main() {
    // printf("lorem ipsum\n");

    // string example_code[1024] = {
    //     "set world \"phosphophyllite\"",
    //     "set sit 69.420",
    //     "print sit",
    //     "print \"Hello, Phos!\"",
    //     "printflush message1",
    // };

    // string example_code[1024] = {
    //     "set x 4",
    //     "set y 0",
    //     "op add x 13 5",
    //     "op sub y 13 5",
    //     "print x",
    //     "printflush message1",
    //     "print y",
    //     "printflush message2"
    // };

    // string example_code[1024] = {
    //     "set accum 0",
    //     // "print accum",
    //     // "printflush message1",
    //     "op add accum accum 1",
    //     "jump 1 notEqual accum 100000",
    //     "print accum",
    //     "printflush message1"
    // };

    string example_code[1024] = {
        "set first_num 1",
        "set second_num 1",
        "set new_num 0",
        "set limit 1476",
        "set accum 2",

        "op add new_num first_num second_num",
        "op add second_num 0 first_num",
        "op add first_num 0 new_num",

        "op add accum accum 1",
        "jump 5 notEqual accum limit",

        "print \"\"",
        "print \"fib_n:\n\"",
        "print accum",
        "print \"\n\n\"",

        "print \"fib_latest:\n\"",
        "print first_num",
        "print \"\n\n\"",

        "print \"fib_previous:\n\"",
        "print second_num",
        "print \"\n-------------------------------\"",

        "printflush message1",

        "stop"
    };
    struct GlobalState global_state;

    init_variable_parser(&global_state);
    init_std(&global_state);

    const vector<Line*> lexed = lex_many(example_code, &global_state);
    // printf("Lexing Result: %p\n", lexed[0].values[0].variable);

    const Error *error = get_error(lexed);
    VALIDATE;

    parse_variables(&lexed);

    // for (const Variable* variable : global_state.variables) {
    //     if (variable->value == nullptr) {
    //         continue;
    //     }
    //
    //     cout << variable->name << ": " << variable->value->str_value << endl;
    // }

    const auto start_time = chrono::high_resolution_clock::now();

    execute_many(lexed, &global_state);

    const auto finish_time = chrono::high_resolution_clock::now();

    auto duration = start_time.time_since_epoch();
    const double start = static_cast<double>(chrono::duration_cast<chrono::microseconds>(duration).count()) / 1000;

    duration = finish_time.time_since_epoch();
    const double finish = static_cast<double>(chrono::duration_cast<chrono::microseconds>(duration).count()) / 1000;

    cout << finish - start << " ms" << endl;
    cout << global_state.accumulator << " instructions ran.";

    return 0;
}
