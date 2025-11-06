//
// Created by lett on 9/26/25.
//
#include <stdio.h>
#include "main.h"
#include "lexer.h"
#include "error.h"
#include "variable_parse.h"

void print_error(const struct Error *error) {
    printf("Error encountered on line %d.\n\t%s", error->position + 1, error->string);
}

#define VALIDATE if (error != NULL) { print_error(error); return 1; }

int main() {
    // printf("lorem ipsum\n");

    char example_code[1024][1024] = {{"set world \"phosphophyllite\""}, {"set sit \"amet\""}};
    struct GlobalState global_state;
    global_state.var_index = 0;

    init_variable_parser(&global_state);

    const struct Line *lexed = lex_many(example_code, &global_state);
    // printf("Lexing Result: %p\n", lexed[0].values[0].variable);

    const struct Error *error = get_error(lexed);
    // VALIDATE;

    parse_variables(lexed, &global_state);

    free((void*) lexed);  // NOTE: this doesn't clear all memory referenced by `lexed`. However, the closing of the
                          // program should cause all memory to be freed by the kernel.

    return 0;
}
