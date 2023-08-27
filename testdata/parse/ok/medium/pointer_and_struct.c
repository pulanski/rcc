// Declare a struct and a function that takes a pointer to it.
struct Point {
    int x;
    int y;
};

void move(struct Point *point, int dx, int dy) {
    point->x += dx;
    point->y += dy;
}
