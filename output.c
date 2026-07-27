#include <stdio.h>
#include <stdbool.h>

int main() {
    int x = 5;
    int y = 10;
    printf("%d\n", ((x < y) && (((x + 5) == y) || (y > 100))));
    return 0;
}
