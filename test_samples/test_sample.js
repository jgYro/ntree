function simpleFunction() {
    console.log("Hello JavaScript");
}

function functionWithIf(x) {
    if (x > 0) {
        return x + 1;
    } else {
        return 0;
    }
}

function complexFunction(a, b) {
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

const arrowFunction = (x, y) => {
    if (x && y) {
        return x * y;
    }
    return 0;
};