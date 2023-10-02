// int demo_main(int argc, char **argv) {
//   int foo = 10;
//   int bar = 20;

//   if (foo == 10) {
//     return 0;
//   } else {
//     int baz = 30;
//     int qux = 20;
//   }
// }

#include <stdio.h>

// Generic swap macro
#define SWAP(x, y, type)                                                       \
  do {                                                                         \
    type SWAP_temp = x;                                                        \
    x = y;                                                                     \
    y = SWAP_temp;                                                             \
  } while (0)

int main() {
  int a = 5, b = 10;
  printf("Before swap: a = %d, b = %d\n", a, b);
  SWAP(a, b, double);
  printf("After swap: a = %d, b = %d\n", a, b);

  double c = 5.5, d = 10.5;
  printf("Before swap: c = %lf, d = %lf\n", c, d);
  SWAP(c, d, double);
  printf("After swap: c = %lf, d = %lf\n", c, d);

  return 0;
}

// int qux(int x) { return x; }

// // TODO: update case to change type to an array of char pointers, right now
// data type is being parsed/lowered as just char pointer
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
//     invalid x;  // Error: Unexpected token 'invalid', expected one of: 'int',
//     'char', 'short', ... return 0;
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

// TODO: properly handle parsing function declarations without types (i.e. int
// qux(x) { return x; }) and emitting a proper error message "missing type for
// function parameter 'x' in function 'qux'"
// int qux(int x) {}

// int qux(int x) {}

// int qux(int x) {}

// int qux(int x) {}
// int qux(int x) {}

// int x[5] = {1, 2, 3, 4, 5};

// int x = 20;

// 🔥

// TODO: properly parse this, and emit a proper error message
// e.g.
// testdata/parse/b.c:74:1: error: type specifier missing, defaults to
// 'int'; ISO C99 and later do not support implicit int [-Wimplicit-int]
// main(int argc, char **argv)
// ^
// int
// main(int argc, char **argv)

//   if (foo == 10) {
//     printf("foo is 10\n");
//   } else {
//     printf("foo is not 10\n");
//   }

//   int baz = foo + bar;
// int x = 3;

// int x, y, z = 3;

// TODO: implement parsing routine to handle top level struct declarations
// struct Point {
//     int x;
//     int y;
// };
