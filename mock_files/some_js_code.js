// Some single line comment
/*
    Some multi line comment

    Blah blah blah
*/
const fibonacci = (fib_len) => {
    if (fib_len == 0) {
        return 0;
    }

    // Some more comments
    let first = 0;
    let second = 1;
    let temp;
    for (let i = 1; i < fib_len; i++) {
        temp = first + second;
        first = second;
        second = temp;
    }

    return second;
};

const assert = (val) => {
    if (!val) {
        throw new Error("Assertion failed");
    }
};

console.log("Hello, JS");

const fib_len = 19;

const res = fibonacci(fib_len);

assert(res == 4181);
// 23 lines without comments
// 31 with comments. No docs
