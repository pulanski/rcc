// Path: testdata/preprocessor/in/variadic_macro.c
//
// A variadic macro is a macro that can take a variable number of arguments.
#define LOG(format, ...) printf(format, ##__VA_ARGS__)
#define STRINGIFY(x) #x
#define FOO(x) x + 1
#define BAR(x, y) FOO(x) + FOO(y)

// Use the macro.
void test()
{
    LOG("Hello, world!\n");
    LOG("Hello, %s!\n", "world");
    LOG("Hello, %s!\n", STRINGIFY(world));
    int a = 1;
    int b = FOO(1) + FOO(2);
    int c = BAR(1, FOO(3));
}