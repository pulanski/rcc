// Complex conditional function call.

int max(int a, int b, int c) {
    if (a > b) {
        if (a > c) {
            return a;
        } else {
            return c;
        }
    } else {
        if (b > c) {
            return b;
        } else {
            return c;
        }
    }
}

int main() {
    int a = 1;
    int b = 2;
    int c = 3;

    int d = max(a, b, c);
    return 0;
}
