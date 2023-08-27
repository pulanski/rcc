// Type aliasing (e.g. `typedef int my_int;`) and typedefs (e.g. `typedef int (*ArithmeticOperation)(int, int);`).
//
typedef int (*ArithmeticOperation)(int, int);

ArithmeticOperation get_operation(char op) {
    if (op == '+') {
        return add;
    } else {
        return subtract;
    }
}

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}
