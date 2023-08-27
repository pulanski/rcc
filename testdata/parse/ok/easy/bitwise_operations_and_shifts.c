// #include <stdio.h>

// Bitwise operations and shifts
// https://www.programiz.com/c-programming/bitwise-operators

int main() {
    int x = 5;
    int y = 3;
    int result1 = x & y;
    int result2 = x | y;
    int result3 = x << 2;
    int result4 = y >> 1;

    printf("Bitwise AND: %d\n", result1);
    printf("Bitwise OR: %d\n", result2);
    printf("Left Shift: %d\n", result3);
    printf("Right Shift: %d\n", result4);

    return 0;
}
