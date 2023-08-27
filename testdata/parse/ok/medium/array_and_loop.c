// Uses an array and a loop to calculate the sum of its elements.
int sum(int array[], int size) {
    int result = 0;
    for (int i = 0; i < size; i++) {
        result += array[i];
    }
    return result;
}
