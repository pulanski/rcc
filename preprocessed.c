# 1 "testdata/preprocessor/in/variadic_macro.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 415 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "testdata/preprocessor/in/variadic_macro.c" 2
# 10 "testdata/preprocessor/in/variadic_macro.c"
void test()
{
    printf("Hello, world!\n");
    printf("Hello, %s!\n", "world");
    printf("Hello, %s!\n", "world");
    int a = 1;
    int b = 1 + 1 + 2 + 1;
    int c = 1 + 1 + 3 + 1 + 1;
}
