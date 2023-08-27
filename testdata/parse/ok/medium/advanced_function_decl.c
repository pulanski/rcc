int (*function_pointer)(int, int);

int add(int a, int b);
int subtract(int a, int b);

int main() {
    function_pointer = add;
    int result1 = function_pointer(5, 3);
    function_pointer = subtract;
    int result2 = function_pointer(5, 3);

    printf("Result1: %d\n", result1);
    printf("Result2: %d\n", result2);

    return 0;
}
