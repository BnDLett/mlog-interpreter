//
// Created by lett on 9/26/25.
//
#pragma once

#define BUF_LENGTH 1024
#define CALLBACK_LIMIT 256
#define VAR_LIMIT 128
#define VAR_NAME_LIMIT 64
#define MAX_PARAMETERS 16 // you're mental if you need anything over 16
#define PROGRAM_SIZE_LIMIT 1024

#define LEN(arr) ((int) (sizeof (arr) / sizeof (arr)[0]))

#include <endian.h>
#include <stdlib.h>
#include <string.h>

// TODO: move these?
struct Variable {
    struct Value *value;
    char name[VAR_NAME_LIMIT];
};

struct Value {
    char string[BUF_LENGTH];
    double value;
    struct Variable *variable;
};

struct Callback {
    void (*callback)();
    int parameters;
    char *name;
};

struct Line {
    struct Callback *callback;
    char *error;
    struct Value *values;
};

struct GlobalState {
    struct Callback callbacks[CALLBACK_LIMIT];
    struct Variable variables[VAR_LIMIT];
    unsigned int var_index;
};

// union Optional {
//     T Some{};
//     char None;
// };

// actual functions

static struct Callback *find_callback(const char *fn_name, struct GlobalState *global_state) {
    for (int i = 0; i < CALLBACK_LIMIT; i++) {
        struct Callback *callback = &global_state->callbacks[i];

        if (callback->callback == NULL) {
            return NULL;
        }

        if (strcmp(callback->name, fn_name) == 0) {
            return callback;
        }
    }

    return NULL;
}

static void reset(char *a) {
    const int len = sizeof(a) / sizeof(a[0]);

    for (int i = 0; i < len; i++) {
        a[i] = *"";
    }
}

// A highly specialized macro. Don't use unless you know what you're doing.
#define RESET reset(word); word_index = 0; continue

static struct Line lex(const char *code, struct GlobalState *global_state) {
    const unsigned long len = strlen(code);
    char word[BUF_LENGTH] = "";
    char error[BUF_LENGTH];
    unsigned int word_index = 0;

    unsigned char in_string = 0;
    struct Value *parameters = malloc(sizeof(struct Value) * MAX_PARAMETERS);
    unsigned int parameter_index = 0;
    struct Callback callback = {
        .name = NULL
    };

    for (int i = 0; i < len; i++) {
        const char c = code[i];
        // printf("(%s) c: %c\n", word, c);

        if (c == '"' || c == '\'') {
            in_string = !in_string;
            word[word_index++] = c;

            if (!in_string) {
                struct Value new_value;

                strncpy(new_value.string, word, BUF_LENGTH);
                new_value.value = 0;
                new_value.variable = NULL;

                // parameters[parameter_index++] = new_value;
                // printf("%s\n", word);

                RESET;
            }

            continue;
        }

        if ((c == ' ' && !in_string) || i == len - 1) {
            if (i == len - 1) {
                word[word_index] = c;
            }

            if (strcmp(word, "") != 0) { // if it isn't "" — then it's likely not consumed/used.
                struct Variable *new_variable = malloc(sizeof(struct Variable));
                strncpy(new_variable->name, word, VAR_NAME_LIMIT);

                const struct Value new_value = {
                    .variable = new_variable
                };

                printf("%d\n", global_state->var_index);

                global_state->variables[global_state->var_index++] = *new_variable;
                parameters[parameter_index++] = new_value;
            }

            RESET;
        }

        const struct Callback *callback_result = find_callback(word, global_state);

        if (callback_result != NULL) {
            callback = *callback_result;
            RESET;
        }

        word[word_index++] = c;
    }

    if (callback.name == NULL) {
        reset(error);
        strcpy(error, "Couldn't find valid callback.");
    }

    const struct Line line = {
        .callback = &callback,
        .error = error,
        .values = parameters,
    };
    return line;
}

static struct Line *lex_many(char program[PROGRAM_SIZE_LIMIT][BUF_LENGTH], struct GlobalState *global_state) {
    // size_t n = sizeof(program) / sizeof(program[0]);
    struct Line *lexed = malloc(PROGRAM_SIZE_LIMIT * sizeof(struct Line));
    unsigned int position = 0;

    for (int i = 0; i < PROGRAM_SIZE_LIMIT; i++) {
        const char *line = program[i];

        if (strcmp(line, "") == 0) {
            continue;
        }

        // printf(line);
        lexed[position++] = lex(line, global_state);
    }

    return lexed;
}
