// Defining and using function pointers.
// https://stackoverflow.com/questions/840501/how-do-function-pointers-in-c-work

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}

int calculate(int (*operation)(int, int), int x, int y) {
    return operation(x, y);
}

int main() {
    int a = 1;
    int b = 2;

    int c = calculate(add, a, b);
    int d = calculate(subtract, a, b);

    assert(c == 3);
    assert(d == -1);

    return 0;
}
