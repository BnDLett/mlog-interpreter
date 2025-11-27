//
// Created by lett on 9/26/25.
//
#pragma once
#define PROGRAM_SIZE_LIMIT 1024

#include <string>
#include <vector>
class Value;
using namespace std;

// TODO: move these?
// struct Variable {
//     struct Value *value;
//     char name[VAR_NAME_LIMIT];
// };

class Variable {
    public:
        Value* value;
	    string name;

        Variable(Value* value, const string& name) {
            this->value = value;
            this->name = name;
        }
};

//struct Value {
//    char string[BUF_LENGTH];
//    double value;
//    struct Variable *variable;
//};

class Value {
    public:
        string str_value;
        double value;
        Variable* variable;

        Value(const string& str_value, const double value, Variable* variable) {
            this->str_value = str_value;
            this->value = value;
            this->variable = variable;
        }

        explicit Value(Variable* variable) {
            this->variable = variable;
            this->value = 0;
        }
};

// struct Callback {
//     void (*callback)();
//     int parameters;
//     char *name;
// };

class Callback {
    public:
        void (*callback)();
        int parameters;
        string name;

        Callback(void callback(), int parameters, const string& name) {
            this->callback = callback;
            this->parameters = parameters;
            this->name = name;
        }

};

// struct Line {
//     struct Callback *callback;
//     char *error;
//     struct Value *values;
//     unsigned int position;
// };

class Line {
    public:
        Callback *callback;
        string error;
        vector<Value*> values;
        unsigned int position;

        Line(const unsigned int position, Callback *callback, string error, const vector<Value*>& values) {
            this->callback = callback;
            this->error = error;
            this->values = values;
            this->position = position;
        }
};

struct GlobalState {
    // struct Callback callbacks[CALLBACK_LIMIT];
    // struct Variable variables[VAR_LIMIT];
    vector<Callback*> callbacks;
    vector<Variable*> variables;
    unsigned int var_index;
};

// union Optional {
//     T Some{};
//     char None;
// };

// actual functions

inline Callback *find_callback(const string& fn_name, const struct GlobalState *global_state) {
    for (Callback* callback : global_state->callbacks) {
        // printf("%d: %p\n", i, callback);

        if (callback->callback == nullptr) {
            return nullptr;
        }

        if (callback->name == fn_name) {
            return callback;
        }
    }

    return nullptr;
}

static void create_keyword(const char *name, const int parameters, void callback(), struct GlobalState* global_state) {
    Callback* keyword = new Callback(callback, parameters, name);
    global_state->callbacks.push_back(keyword);
}

inline bool numbers_only(const string& target_string) {
    const string numbers = "0123456789";
    bool is_float = false;

    for (const char c : target_string) {
        if (c == '.') {
            if (is_float) {
                return false;
            }

            is_float = true;
            continue;
        }

        if (numbers.find(c) == string::npos) {
            return false;
        }
    }

    return true;
}

/**
 * Checks whether the name of a variable is valid.
 * @param name The name of the variable.
 * @return `true` if the variable is valid, otherwise `false`.
 */
inline bool variable_name_valid(const string& name) {
    constexpr char banned_characters[] = {' ', '.', ','};
    bool contains_banned = false;

    for (const char c : banned_characters) {
        if (name.find(c) != string::npos) {
            contains_banned = true;
            break;
        }
    }

    return !contains_banned;
}

// static void reset(string *a) {
//     a->clear();
// }

// A highly specialized macro. Don't use unless you know what you're doing.
#define RESET word.clear(); word_index = 0; continue

inline Line *lex(const string& line, struct GlobalState *global_state, const int position) {
    string error = "\0";
    string word;
    vector<Value*> parameters = {};
    Callback *callback = nullptr;

    bool in_string = false;
    unsigned int word_index = 0;
    const unsigned long len = line.length();

    for (int i = 0; i < len; i++) {
        const char c = line.at(i);
        // printf("(%s) c: %c\n", word, c);

        if (c == '"' || c == '\'') {
            in_string = !in_string;
            word.push_back(c);

            if (!in_string) {
                Value *new_value = new Value(word, 0, nullptr);
                parameters.push_back(new_value);

                RESET;
            }

            continue;
        }

        if ((c == ' ' && !in_string) || i == len - 1) {
            if (i == len - 1) {
                word.push_back(c);
            }

            Callback *callback_result = find_callback(word, global_state);

            if (callback_result != nullptr) {
                callback = callback_result;
                RESET;
            }

            if (!word.empty() && numbers_only(word)) {
                Value* new_value = new Value(word, stod(word), nullptr);
                parameters.push_back(new_value);
                RESET;
            }

            // assumes unrecognized value is a variable.
            if (!word.empty() && variable_name_valid(word)) { // if it isn't "" — then it's likely not consumed/used.
                Variable *new_variable = new Variable(nullptr, word);
                Value *new_value = new Value(new_variable);

                global_state->variables.push_back(new_variable);
                parameters.push_back(new_value);
                RESET;
            }

            error = "Could not find a valid representation for token: \"" + word + '"';
            RESET;
        }

        word.push_back(c);
    }

    if (callback == nullptr) {
        error = "Couldn't find valid callback.";
    }

    Line *lexed_line = new Line(position, callback, error, parameters);
    return lexed_line;
}

inline vector<Line*> lex_many(string program[PROGRAM_SIZE_LIMIT], struct GlobalState *global_state) {
    // size_t n = sizeof(program) / sizeof(program[0]);
    vector<Line*> lexed_program = {};

    for (int i = 0; i < PROGRAM_SIZE_LIMIT; i++) {
        string line = program[i];

        if (line.empty()) {
            continue;
        }

        // printf(line);
        lexed_program.push_back(lex(line, global_state, i));
    }

    return lexed_program;
}
