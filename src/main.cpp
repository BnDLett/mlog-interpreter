//
// Created by lett on 9/26/25.
//
#include "lexer.h"
#include "error.h"
#include "variable_parse.h"

#include <string>
#include <iostream>
using namespace std;

void print_error(const struct Error *error) {
    cout << "Error encountered on line " << error->position + 1 << ".\n\t" << *error->message << endl;
}

#define VALIDATE if (error != nullptr) { print_error(error); return 1; }

int main() {
    // printf("lorem ipsum\n");

    cout << "lorem";

    string example_code[1024] = {"set world \"phosphophyllite\"", "set sit \"amet\""};
    struct GlobalState global_state;
    global_state.var_index = 0;

    cout << "lorem";

    init_variable_parser(&global_state);

    vector<Line*> lexed = lex_many(example_code, &global_state);
    // printf("Lexing Result: %p\n", lexed[0].values[0].variable);

    const struct Error *error = get_error(lexed);
    VALIDATE;

    parse_variables(lexed, &global_state);

    return 0;
}
