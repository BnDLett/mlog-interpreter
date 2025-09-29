//
// Created by lett on 9/26/25.
//
#include <stdio.h>
#include "main.h"
#include "lexer.h"

int main() {
    printf("lorem ipsum\n");

    char example_code[1024][1024] = {{"lorem \"ipsum\" phosphophyllite"}, {"dolor \"sit amet\""}};
    struct GlobalState global_state;
    global_state.var_index = 0;

    const struct Line *lexed = lex_many(example_code, &global_state);
    printf("Lexing Result: %p\n", lexed[0].values[0].variable);
    free(lexed);

    return 0;
}
