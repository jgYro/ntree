#include <iostream>

void simpleFunction() {
    std::cout << "Hello C++" << std::endl;
}

int functionWithIf(int x) {
    if (x > 0) {
        return x + 1;
    } else {
        return 0;
    }
}

int complexFunction(int a, int b) {
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

class Calculator {
public:
    int add(int a, int b) {
        return a + b;
    }

    int multiply(int x, int y) {
        if (x == 0 || y == 0) {
            return 0;
        }
        return x * y;
    }
};