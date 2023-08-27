// A define macro with a single argument.
#define FOO(x) x

// A define macro with multiple arguments.
#define BAR(x, y) x + y

// A define macro with no arguments.
#define BAZ 1

// Use the macros.
int a = FOO(1);
int b = BAR(1, 2);
int c = BAZ;
