#define PI 3.14159
#define RADTODEG(x) ((x)*57.29578)

int main()
{
    // Define struct Point before using it
    struct Point
    {
        int x;
        int y;
    };

    int x[] = {1, 2, 3};
    struct Point p = {.x = 10, .y = 20};

    // while (1)
    // {
    //     break;
    // }

    for (int i = 0; i < 10; i++)
    {
        continue;
    }

    return 0;
}