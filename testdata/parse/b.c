// // TODO: update case to change type to an array of char pointers, right now data type is being parsed/lowered as just char pointer
// // int main(int argc, char *argv[]) {}

// #include <stdio.h>

// int add(int x, char **y) {
//     return x + y;
// }

// x = 2;
// int x = 2;

// Invalid C code
// int main() {
//     // Attempting to declare a variable with an invalid declaration specifier
//     invalid x;  // Error: Unexpected token 'invalid', expected one of: 'int', 'char', 'short', ...
//     return 0;
// }

// int main(int argc, char **argv) {
// int x = 10;

// if (x == 10) {
//     printf("x is 10\n");
// } else {
//     printf("x is not 10\n");
// }

// int x[5] = {1, 2, 3, 4, 5};

// Declare an array of integers with designated initializers.
// int arr[5] = {
//     [2] = 42,   // Initialize the element at index 2 to 42
//     [0] = 10,   // Initialize the element at index 0 to 10
//     [4] = 99    // Initialize the element at index 4 to 99
// };

// return 0;
// }

// int foo(int x) {
//     return x;
// }

// int bar(int x) {
//     return x;
// }

// int baz(int x) {
//     foo bar baz
// }

// TODO: properly handle parsing function declarations without types (i.e. int qux(x) { return x; })
// and emitting a proper error message "missing type for function parameter 'x' in function 'qux'"
int qux(int x) {}

int qux(int x) {}

int qux(int x) {}

int qux(int x) {}
// int qux(int x) {}

// int qux(int x) {
//     return x;
// }

// int x[5] = {1, 2, 3, 4, 5};

// int x = 20;

// 🔥

int main(int argc, char **argv)
{

}

int x = 3;

// TODO: implement parsing routine to handle top level struct declarations
// struct Point {
//     int x;
//     int y;
// };
