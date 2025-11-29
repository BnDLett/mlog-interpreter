//
// Created by lett on 9/26/25.
//
#include "lexer.h"
#include "error.h"
#include "variable_parse.h"

#include <string>
#include <iostream>

#include "executor.h"
#include "libraries/std.h"
using namespace std;

void print_error(const struct Error *error) {
    cout << "Error encountered on line " << error->position + 1 << ".\n\t" << error->message << endl;
}

#define VALIDATE if (error != nullptr) { print_error(error); return 1; }

int main() {
    // printf("lorem ipsum\n");

    string example_code[1024] = {
        "set world \"phosphophyllite\"",
        "set sit 69.420",
        "print sit",
        "print \"Hello, Phos!\"",
        "printflush message1",
    };
    struct GlobalState global_state;

    init_variable_parser(&global_state);
    init_std(&global_state);

    const vector<Line*> lexed = lex_many(example_code, &global_state);
    // printf("Lexing Result: %p\n", lexed[0].values[0].variable);

    const Error *error = get_error(lexed);
    VALIDATE;

    parse_variables(&lexed, &global_state);

    // for (const Variable* variable : global_state.variables) {
    //     if (variable->value == nullptr) {
    //         continue;
    //     }
    //
    //     cout << variable->name << ": " << variable->value->str_value << endl;
    // }

    execute_many(lexed, &global_state);

    return 0;
}
