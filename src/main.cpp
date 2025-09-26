//
// Created by lett on 9/26/25.
//
#include <stdio.h>
#include <iostream>
// #include "main.h"

using namespace std;

struct rah {
    int a;
    int b;
};

int main() {
    cout << "Hello, world!" << endl;

    struct rah v = {
        5,
        6
    };

    printf("%d", v.a + v.b);

    return 0;
}
