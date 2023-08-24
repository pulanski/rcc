// Minimal C program to test parsing of translation units.

// Function to calculate the factorial of a number using recursion.
int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// Function to calculate the nth Fibonacci number using recursion.
int fibonacci(int n) {
    if (n <= 0) {
        return 0;
    } else if (n == 1) {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// Structure to represent a point in 2D space.
struct Point {
    float x;
    float y;
};

// Enum to represent the days of the week.
enum Day {
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY
};

int main() {
    // Declare and initialize variables of various types.
    int integerVar = 42;
    float floatVar = 3.14;
    char charVar = 'A';
    double doubleVar = 2.71828;

    // Conditional statement.
    if (integerVar > 0) {
        printf("integerVar is positive.\n");
    } else {
        printf("integerVar is non-positive.\n");
    }

    // Looping statements.
    for (int i = 0; i < 5; i++) {
        printf("Iteration %d\n", i);
    }

    // Function calls and expressions.
    int factResult = factorial(5);
    int fibResult = fibonacci(6);

    printf("Factorial of 5 is %d\n", factResult);
    printf("Fibonacci(6) is %d\n", fibResult);

    // Using a structure.
    struct Point point1;
    point1.x = 1.0;
    point1.y = 2.0;

    printf("Point1: (%.2f, %.2f)\n", point1.x, point1.y);

    // Using an enum.
    enum Day today = MONDAY;
    printf("Today is Monday (Enum Value: %d)\n", today);

    return 0;
}
