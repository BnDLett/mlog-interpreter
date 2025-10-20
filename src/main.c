//
// Created by lett on 9/26/25.
//
#include <stdio.h>
#include "main.h"
#include "lexer.h"
#include "error.h"

int main() {
    // printf("lorem ipsum\n");

    char example_code[1024][1024] = {{"lorem \"ipsum\" phosphophyllite"}, {"dolor \"sit amet\""}};
    struct GlobalState global_state;
    global_state.var_index = 0;

    const struct Line *lexed = lex_many(example_code, &global_state);
    // printf("Lexing Result: %p\n", lexed[0].values[0].variable);

    const struct Error *error = get_error(lexed);
    if (error != NULL) {
        printf("Error encountered on line %d.\n\t%s", error->position + 1, error->string);
        return 1;
    }

    free((void*) lexed);  // NOTE: this doesn't clear all memory referenced by `lexed`. However, the closing of the
                          // program should cause all memory to be freed by the kernel.

    return 0;
}
