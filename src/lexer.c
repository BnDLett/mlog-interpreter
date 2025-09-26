//
// Created by lett on 9/26/25.
//

#define BUF_LENGTH 1024
#define CALLBACK_LIMIT 256
#define VAR_LIMIT 128
#define MAX_PARAMETERS 16 // you're mental if you need anything over 16
#define PROGRAM_SIZE_LIMIT 1024

#define LEN(arr) ((int) (sizeof (arr) / sizeof (arr)[0]))

#include <string.h>

// TODO: move these?
struct Variable {
    struct Value *value;
    char *name;
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
};

// union Optional {
//     T Some{};
//     char None;
// };

// actual functions

struct Callback *find_callback(const char *name, struct GlobalState *global_state) {
    for (int i = 0; i < CALLBACK_LIMIT; i++) {
        struct Callback *callback = &global_state->callbacks[i];

        if (strcmp(callback->name, name) == 0) {
            return callback;
        }
    }

    return NULL;
}

struct Line *lex_many(char program[PROGRAM_SIZE_LIMIT][BUF_LENGTH], struct GlobalState *global_state) {
    size_t n = sizeof(program) / sizeof(program[0]);

    for (int i = 0; i < PROGRAM_SIZE_LIMIT; i++) {
        char *line = program[i];

        if (line == "") {
            continue;
        }

        lex(line, global_state);
    }
}

struct Line lex(const char *code, struct GlobalState *global_state) {
    const unsigned long len = strlen(code);
    char word[BUF_LENGTH] = "";
    unsigned int word_index = 0;

    unsigned char in_string = 0;
    struct Value parameters[MAX_PARAMETERS];
    unsigned int parameter_index = 0;
    struct Callback callback;

    for (int i = 0; i < len; i++) {
        char c = code[i];

        if (c == '"' || c == '\'') {
            in_string = !in_string;
            word[word_index++] = c;

            if (!in_string) {
                strcpy(word, "");
                word_index = 0;
                struct Value new_value;

                strncpy(new_value.string, word, BUF_LENGTH);
                new_value.value = 0;
                new_value.variable = NULL;

                parameters[parameter_index++] = new_value;
            }

            continue;
        }

        const struct Callback *callback_result = find_callback(word, global_state);

        if (callback_result != NULL) {
            callback = *callback_result;
        }

        word[word_index++] = c;
        word_index++;
    }

    const struct Line line = {
        .callback = &callback,
        .error = NULL,
        .values = parameters,
    };
    return line;
}
