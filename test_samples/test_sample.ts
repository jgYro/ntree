function simpleFunction(): void {
    console.log("Hello TypeScript");
}

function functionWithIf(x: number): number {
    if (x > 0) {
        return x + 1;
    } else {
        return 0;
    }
}

function complexFunction(a: number, b: number): number {
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

interface Calculator {
    calculate(a: number, b: number): number;
}

class MathCalculator implements Calculator {
    calculate(a: number, b: number): number {
        if (a > b) {
            return a - b;
        }
        return b - a;
    }
}