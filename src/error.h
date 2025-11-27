//
// Created by lett on 9/29/25.
//

#ifndef MLOG_INTERPRETER_ERROR_H
#define MLOG_INTERPRETER_ERROR_H
#include "lexer.h"

#endif //MLOG_INTERPRETER_ERROR_H

class Error {
    public:
    string message;
        unsigned int position;

        Error(const string& message, const unsigned int position) {
            this->message = message;
            this->position = position;
        }
};

/// Checks tokens for any errors that may have resulted during the compilation process.
/// @param tokens The tokens to check for errors.
/// @return The first error that is discovered.
inline Error *get_error(vector<Line*> tokens) {
    for (unsigned int i = 0; i < tokens.capacity(); i++) {
        Line* line = tokens.at(i);

        const string error_message = line->error;
        const unsigned int position = line->position;

        if (!error_message.empty()) {
            // struct Error *error = malloc(sizeof(struct Error));
            //
            // error->string = error_message;
            // error->position = position;
            Error* error = new Error(error_message, position);

            return error;
        }
    }

    return nullptr;
};
