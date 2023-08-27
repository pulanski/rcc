// int square(int x); // TODO: Get parsing working properly
// double circle_area(double radius); // TODO: Get parsing working properly

int main() {
    int side = 5;
    double r = 2.5;
    int result = square(side);
    double area = circle_area(r);
    return result + (int)area;
}
int square(int x) {
    return x * x;
}
double circle_area(double radius) {
    return 3.14 * radius * radius;
}
