#include <stdio.h>

void simple_function() {
    printf("Hello C\n");
}

int function_with_if(int x) {
    if (x > 0) {
        return x + 1;
    } else {
        return 0;
    }
}

int complex_function(int a, int b) {
    if (a > b) {
        if (a > 10) {
            return a * 2;
        } else {
            return a + b;
        }
    } else {
        if (b > 10) {
            return b * 3;
        } else {
            return a - b;
        }
    }
}

int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}