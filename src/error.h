//
// Created by lett on 9/29/25.
//

#ifndef MLOG_INTERPRETER_ERROR_H
#define MLOG_INTERPRETER_ERROR_H
#include "lexer.h"

#endif //MLOG_INTERPRETER_ERROR_H

struct Error {
    const char *string;
    unsigned int position;
};

static struct Error *get_error(const struct Line *tokens) {
    for (unsigned int i = 0; i < PROGRAM_SIZE_LIMIT; i++) {
        const struct Line line = tokens[i];

        const char *string = line.error;
        const unsigned int position = line.position;

        if (string != NULL) {
            struct Error *error = malloc(sizeof(struct Error));

            error->string = string;
            error->position = position;

            return error;
        }
    }

    return NULL;
};
